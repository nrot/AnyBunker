import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";

console.log(import.meta.env);

export const baseUrl = import.meta.env.VITE_GRPCSERVER_BIND_URI;

export const GrpcTransport = new GrpcWebFetchTransport({
    baseUrl: "http://" + baseUrl,
    format: "binary"
});