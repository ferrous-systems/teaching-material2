In this exercise, we will extend our redis client with protocol buffers
functionality.

-   [Presentation slides](../redis-protobuf.html)

Preparation + Setup
----

Step 1  
-   Install the [protoc
    compiler](https://grpc.io/docs/protoc-installation/) (required by
    `prost-build`).

-   Create a binary project called `redis-client-protobuf`:
    `cargo new --bin redis-client-protobuf` and add your redis client
    library as a path dependency.

-   Copy the [protobuf
    definitions](https://github.com/ferrous-systems/teaching-material/tree/main/assignments/redis-protobuf)
    to your source directory.

Implementation
----

Step 2  
Generate Rust protobuf code for phone book and configuration, using
[prost\_build](https://docs.rs/prost-build/0.7.0/prost_build/)

Step 3  
Store and retrieve configuration and phone book entries in Redis.

Step 4  
Implement a wrapper around `RedisClient` that handles encoding and
decoding.

Bonus  
Optimize your build so that code generation is only run when the
protobuf definitions change.
