# Compiler/Builder (Only used to compile the application)
FROM rust:slim as builder

WORKDIR /home/performance_checker

COPY . .

RUN apt-get update; \
    apt-get install -y libsqlite3-dev; \
    rm -rf /var/lib/apt/lists/*

RUN rustc --version && \
    cargo --version && \
    cargo update && \
    cargo build --release

# Container Runtime (Used to run the application)
FROM debian

ARG APP=/home/performance_checker

RUN apt-get update; \
    apt-get install -y libsqlite3-dev; \
    mkdir -p ${APP} 

COPY --from=builder /home/performance_checker/target/release/riscv_performance_checker ${APP}/riscv_performance_checker
COPY --from=builder /etc/localtime /etc/localtime

# Used to avoid null copy error in multi-stage build
RUN true

COPY ./entrypoint.sh ${APP}/entrypoint.sh

RUN chmod +x ${APP}/entrypoint.sh

WORKDIR ${APP}

CMD ["./entrypoint.sh"]