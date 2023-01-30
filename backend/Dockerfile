FROM rust:1.66.1 as builder

RUN USER=root cargo new --bin gpsfitnessapp
WORKDIR ./gpsfitnessapp
COPY ./.cargo .cargo
COPY ./vendor vendor
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

#dummy build to cache dependencies
#make dummy main.rs file
RUN echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

# The last modified attribute of main.rs needs to be updated manually,
# otherwise cargo won't rebuild it.
RUN touch -a -m ./src/main.rs

#RUN rm ./target/release/deps/rust_docker_web*
RUN cargo build --release


FROM debian:bullseye-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata curl wget \
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