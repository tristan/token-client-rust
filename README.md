

## Protobuf

```
protoc --rust_out src/signal protobuf/SignalService.proto
protoc --rust_out src/service/chat protobuf/WebSocketResources.proto
```


# TODOS:

 - impl address checksums: https://github.com/ethereum/EIPs/issues/55