FROM rust:nightly

RUN USER=root cargo new --bin users-microservice
WORKDIR /users-microservice
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build

RUN rm src/*.rs
COPY ./src ./src
RUN rm ./target/debug/deps/users_microservice*
RUN cargo build

CMD ["/users-microservice/target/debug/users-microservice"]

EXPOSE 8080

