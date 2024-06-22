# rust-ci-test

A simple project to test and experiment with GitHub CI workflows on a Rust project.
How to build a binary for several platforms, and embed it in a docker container.

# Docker

I tested different Dockerfile approaches to build a container for a dedicated architecture (arm64, x86-64) from a Sillicon Mac.

```
docker build  -f docker/server/Dockerfile.x86-64 --platform linux/amd64 .
docker build  -f docker/server/Dockerfile.x86-64-qemu .
docker build  -f docker/server/Dockerfile.arm-64 .
```