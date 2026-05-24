FROM rust:1.94.0-alpine AS builder
LABEL authors="issy"

RUN rustup target add riscv32imac-unknown-none-elf
RUN rustup target add wasm32-unknown-unknown

FROM builder

WORKDIR /app
COPY . .

WORKDIR /app/foundation

RUN cargo build --target riscv32imac-unknown-none-elf
RUN cargo build --target wasm32-unknown-unknown

WORKDIR /app/firmware

RUN cargo build --target riscv32imac-unknown-none-elf

WORKDIR /app/simulator

RUN cargo build --target wasm32-unknown-unknown
