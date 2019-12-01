#!/bin/bash
docker stop www-rust
docker rm www-rust
docker build -t iron .
docker run --name www-rust -p 3000:3000 -d iron
