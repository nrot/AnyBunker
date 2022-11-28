use std::env;
use cached::proc_macro::once;


#[allow(non_snake_case)]
#[once]
pub fn postgres_schema()->String{
    log::debug!("Get POSTGRES_SCHEMA");
    env::var("POSTGRES_SCHEMA").unwrap_or_else(|_|String::from("indexes"))
}

#[allow(non_snake_case)]
#[once]
pub fn postgres_uri()->String{
    env::var("DATABASE_URL").expect("DATABASE_URL not set")
}

#[allow(non_snake_case)]
#[once]
pub fn postgres_connections()->u32{
    env::var("POSTGRES_CONNECTIONS").unwrap_or_else(|_|"64".into()).parse::<u32>().expect("Can`t parse POSTGRES_CONNECTIONS as u32")
}

#[allow(non_snake_case)]
#[once]
pub fn httpserver_bind_uri()->String{
    let r = env::var("HTTPSERVER_BIND_URI").unwrap_or_else(|_| "127.0.0.1:8500".into());
    log::info!("HTTPSERVER_BIND_URI: {}", r);
    r
}

#[allow(non_snake_case)]
#[once]
pub fn bus_size()->usize{
    env::var("BUS_SIZE").unwrap_or_else(|_| "1024".into()).parse::<usize>().expect("Can`t parse BUS_SIZE as usize (u32/u64)")
}

#[allow(non_snake_case)]
#[once]
pub fn grpcserver_bind_uri()->String{
    env::var("GRPCSERVER_BIND_URI").unwrap_or_else(|_| "[::1]:50051".into())
}