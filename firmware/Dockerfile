FROM rust:1.94.0-alpine AS base
LABEL authors="issy"

WORKDIR /app
COPY . .

FROM base AS builder-riscv

RUN rustup target add riscv32imac-unknown-none-elf

FROM base AS builder-wasm32

RUN rustup target add wasm32-unknown-unknown

FROM builder-riscv AS firmware

WORKDIR /app/firmware
RUN cargo build --target riscv32imac-unknown-none-elf

WORKDIR /app/foundation
RUN cargo build --target riscv32imac-unknown-none-elf

FROM builder-wasm32 AS simulator

WORKDIR /app/simulator
RUN cargo build --target wasm32-unknown-unknown

WORKDIR /app/foundation
RUN cargo build --target wasm32-unknown-unknown
