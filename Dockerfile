FROM rust:1.66.1 as builder

RUN USER=root cargo new --bin gpsfitnessapp
WORKDIR ./gpsfitnessapp
COPY ./Cargo.toml ./Cargo.toml
# RUN cargo build --release
# RUN rm src/*.rs

COPY . ./

#RUN rm ./target/release/deps/rust_docker_web*
RUN cargo build --release


FROM debian:bullseye-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && apt-get -y install curl \ 
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /gpsfitnessapp/target/release/gpsfitnessapp ${APP}/gpsfitnessapp

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./gpsfitnessapp"]