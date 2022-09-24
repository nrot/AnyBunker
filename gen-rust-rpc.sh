#!/usr/bin/env sh
protoc --prost_out=./src/rpc --tonic_out=./src/rpc --prost-serde_out=./src/rpc proto/log.proto 
