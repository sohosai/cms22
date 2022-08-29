FROM rust:latest 
WORKDIR /usr/src/backend-api
COPY . .
RUN cargo install --path .
CMD ["backend-api"]
