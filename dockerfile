FROM rust:latest

WORKDIR /app
COPY . /app

RUN cargo install --path .
RUN cat rocket.toml
RUN ls
CMD [ "incdec" ]