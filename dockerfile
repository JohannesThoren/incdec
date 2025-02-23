FROM rust:latest

WORKDIR .
COPY . .

RUN cargo install --path .
CMD [ "incdec" ]