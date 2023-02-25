use std::{error::Error, path::PathBuf, str::FromStr, sync::Arc};

use dptree::di::DependencySupplier;
use sea_orm::{ConnectionTrait, DatabaseConnection, FromQueryResult, Statement};
use sea_query::{Expr, PostgresQueryBuilder, Query};
use tokio::io::AsyncWriteExt;

use crate::{
    event,
    model::{self, log_index as LogIndex, log_log as LogLog, log_report as LogReport},
    utils::with_schema,
};

use sea_orm::prelude::*;

use chrono::{self, Duration};

use super::ReportPull;

pub async fn create_report(
    pull: ReportPull,
    db: DatabaseConnection,
    dp: dptree::di::DependencyMap,
) {
    let mut p = pull.write().await;
    for r in LogReport::Entity::find()
        .all(&db)
        .await
        .expect("Can`t get reports from database")
    {
        p.push(tokio::spawn(report_handler(db.clone(), r, dp.clone())))
    }
    drop(p);
    let s: Arc<event::SenderEvents> = dp.get();
    let mut r = s.as_ref().subscribe();
    loop {
        if let event::Events::NewReport { index } = r.recv().await.unwrap() {
            let m = match LogReport::Entity::find_by_id(index)
                .one(&db)
                .await
                .unwrap_or_else(|_| panic!("Can`t get report by id: {}", index))
            {
                Some(m) => m,
                None => {
                    log::warn!("Can`t get report by id: {}", index);
                    continue;
                }
            };
            let mut p = pull.write().await;
            p.push(tokio::spawn(report_handler(db.clone(), m, dp.clone())));
        }
    }
}

async fn report_handler(
    db: DatabaseConnection,
    mut r: LogReport::Model,
    dp: dptree::di::DependencyMap,
) {
    loop {
        let schedule = match cron::Schedule::from_str(&r.cron) {
            Ok(c) => c,
            Err(e) => {
                log::error!("Can`t parse cron: {:?}", e);
                return;
            }
        };
        let mut cr = schedule.upcoming(chrono::Utc);
        if let Some(model::sea_orm_active_enums::ReportSendType::Email) = r.send_type {
            log::error!("Email not implemented yet");
            return;
        }

        let to_sleep = cr.next().unwrap() - chrono::Utc::now();
        tokio::time::sleep(to_sleep.to_std().unwrap()).await;
        log::debug!("Schedule start: {:?}", r.id);
        if let Ok(Some(m)) = LogReport::Entity::find_by_id(r.id).one(&db).await {
            r = m;
        } else {
            continue;
        }
        match r.send_type.as_ref().unwrap() {
            model::sea_orm_active_enums::ReportSendType::Email => todo!(),
            model::sea_orm_active_enums::ReportSendType::File => {
                if let Err(e) = report_to_file(&r, dp.clone()).await {
                    log::error!("Report to file error: {}", e);
                }
            }
        }
    }
}

async fn report_to_file(
    r: &LogReport::Model,
    dp: dptree::di::DependencyMap,
) -> Result<(), Box<dyn Error>> {
    let interval = Duration::seconds(r.interval.unwrap() as i64);
    log::info!("interval: {}", interval);
    let now = chrono::Utc::now();
    let from_start = now - interval;
    log::info!("from start: {}", from_start);
    let db: Arc<DatabaseConnection> = dp.get();

    let index = match LogIndex::Entity::find_by_id(r.index.unwrap())
        .one(db.as_ref())
        .await?
    {
        Some(i) => i,
        None => {
            log::error!("Can`t find index by id: {:?}", r.index);
            return Err("".into());
        }
    };
    let mut pre_query = Query::select();
    pre_query
        .from(with_schema(&index.name.unwrap()))
        .columns(vec![
            LogLog::Column::Id,
            LogLog::Column::Timestamp,
            LogLog::Column::Data,
        ])
        .and_where(Expr::col(LogLog::Column::Timestamp).gte(from_start.naive_utc()))
        .and_where(Expr::cust(&format!(r#""data" @@ '{}'"#, &r.query)));
    let query = pre_query.build(PostgresQueryBuilder);
    log::info!("Query: {:?}", query);

    let nl_str = serde_json::json!([]);
    let col_value = r.columns.clone().unwrap_or(nl_str);
    let columns: Vec<Vec<String>> = if let Ok(v) = serde_json::from_value::<Vec<String>>(col_value)
    {
        v.into_iter()
            .map(|v| v.split('.').map(String::from).collect::<Vec<String>>())
            .collect()
    } else {
        log::error!("Can`t parse columns: {}", r.id);
        return Err("".into());
    };
    let path = PathBuf::from(now.format(r.send_to.as_ref().unwrap()).to_string());
    let mut f = match tokio::fs::File::create(path).await {
        Ok(f) => f,
        Err(e) => {
            log::error!("Can`t open file {}", e);
            return Err(e.into());
        }
    };
    let mut header = Vec::new();
    for c in &columns {
        header.push(format!("\"{}\"", c.join(".")));
    }

    //TODO Переделать на курсор БД
    let s = db
        .query_all(Statement::from_sql_and_values(
            db.get_database_backend(),
            &query.0,
            query.1,
        ))
        .await?;

    f.write_all(header.join(",").as_bytes()).await?;
    f.write_all(b"\r\n").await?;

    for q in s.into_iter() {
        let m = LogLog::Model::from_query_result(&q, "")?;
        log::info!("model: {:?}", m);
        let mut row = String::new();
        for column in &columns {
            let mut v = &m.data;
            for c in column {
                let t = if let Ok(i) = c.parse::<usize>() {
                    v.get(i)
                } else {
                    v.get(c)
                };
                if let Some(val) = t {
                    v = val;
                } else {
                    v = &serde_json::Value::Null;
                    break;
                }
            }
            match v {
                Json::String(s) => row += &format!("{},", serde_json::to_string(s)?),
                Json::Null => row += "",
                s => row += &format!("\"{}\",", serde_json::to_string(s)?.replace('\"', "\'")),
            };
        }
        f.write_all(row.trim_end_matches(',').as_bytes()).await?;
        f.write_all(b"\r\n").await?;
        f.flush().await?;
    }
    Ok(())
}
