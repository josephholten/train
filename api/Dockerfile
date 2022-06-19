FROM rust:1.61
WORKDIR /app

# such that dependencies can be cached through updates to code
COPY dummy.rs .
COPY Cargo.toml .
#COPY Cargo.lock .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY . .

RUN cargo build
CMD ["./target/debug/api"]
#CMD ["cargo", "run"]
