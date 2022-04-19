FROM rust:1.56 as project_builder

RUN USER=root cargo new --bin order_service
WORKDIR ./order_service
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

# Copy the project into the container to build
ADD . ./

RUN rm ./target/release/deps/order_service*
RUN cargo build --release


FROM debian:buster
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

RUN apt-get update \
    && apt-get install -y libpq-dev

#RUN apt-get update \
#    && apt-get install -y vim \
#    && apt install net-tools

EXPOSE 8081

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=project_builder /order_service/target/release/order_service ${APP}/order_service

RUN chown -R $APP_USER:$APP_USER ${APP}

COPY settings.toml ${APP}/settings.toml

ENV RUST_BACKTRACE=full
ENV DATABASE_URL=postgres://username:password@database/order_db

USER $APP_USER
WORKDIR ${APP}

CMD ["./order_service"]