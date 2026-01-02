FROM docker.io/rust:1.85-slim
RUN apt update && apt full-upgrade -y && apt clean

# At least build once to populate the registry in /usr/local/cargo
COPY . .
RUN cd 2021/day07/ && cargo test
