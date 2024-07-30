# syntax=docker/dockerfile:1.9


FROM golang:1.22 AS golang-builder

COPY --link . ./
RUN CGO_ENABLED=0 GOOS=linux go build -a -o /go_main ./golang/main.go


FROM rust:1.80 AS rust-builder

COPY --link . ./
RUN rm -rf rust-toolchain.toml
RUN cargo build --release && mv target/release/futures /rust_main


FROM debian:bookworm-slim

COPY --link --from=golang-builder /go_main /
COPY --link --from=rust-builder /rust_main /
