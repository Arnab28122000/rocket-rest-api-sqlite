# 1. This tells docker to use the Rust official image
FROM  rust:1.49

# working directory
WORKDIR /app

# 2. Copy the files in your machine to the Docker image
#COPY ./ /app

# Build your program for release
RUN rustup default nightly-2022-08-11

RUN cargo install cargo-watch
# Run the binary
#CMD ["./target/release/rocket-rest-api"]