#!/usr/bin/env sh
protoc --prost_out=./src/rpc --tonic_out=./src/rpc --prost-serde_out=./src/rpc proto/message.proto 
protoc --prost_out=./src/rpc/admin --tonic_out=./src/rpc/admin --prost-serde_out=./src/rpc/admin -I. proto/index.proto proto/admin.proto 


echo "Generate frontend part"
# Path to this plugin
PROTOC_GEN_TS_PATH="./node/node_modules/.bin/protoc-gen-ts"

# Directory to write generated code to (.js and .d.ts files)
OUT_DIR="./src/lib/trpc"

cd frontend/
npx protoc --ts_opt optimize_code_size --ts_out $OUT_DIR -I../ --proto_path ../proto proto/index.proto proto/admin.proto --experimental_allow_proto3_optional
cd  ..