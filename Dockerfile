FROM rust:1.71.1 as builder

RUN USER=root cargo new --bin phishing
WORKDIR ./phishing
COPY ./Cargo.toml ./Cargo.toml
COPY ./migration ./migration
RUN cargo build --release \
    && rm src/*.rs target/release/deps/phishing*

ADD . ./

RUN cargo build --release

# ===========================
FROM debian:buster-slim

ARG APP=/usr/src/app

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=pworker

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /phishing/target/release/phishing ${APP}/phishing

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

ENV RUST_LOG=info

# TODO: fix glibc
RUN sudo apt install libc6 -y

CMD ["./phishing"]