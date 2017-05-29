# Retroboard

Retroboard is a sample Rust library that exposes functions that could be used to maintain agile _retrospective_ boards for developers. 

This is designed as a sample application to illustrate the use of Redis as a backing store for an application where we might otherwise have chosen a store like **MongoDB** or **mySQL**.

<font color="red">Warning</font> : because the Redis tests right now are integration tests, and each test purges when its done, if the tests are run concurrently they can fail. Until this ugliness is fixed, force a single-threaded test via:

```
 RUST_TEST_THREADS=1 cargo test 
 ```