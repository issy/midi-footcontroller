FROM rust:1.93.1-slim AS base
LABEL authors="issy"

WORKDIR /app
COPY . .

FROM base AS firmware

WORKDIR /app/firmware
RUN cargo build --target riscv32imac-unknown-none-elf

WORKDIR /app/foundation
RUN cargo build --target riscv32imac-unknown-none-elf

FROM base AS simulator

WORKDIR /app/foundation
RUN cargo build --target wasm32-unknown-unknown

WORKDIR /app/simulator
RUN cargo build --target wasm32-unknown-unknown

RUN cargo install wasm-pack

# resulting wasm file will be in /app/simulator/pkg/simulator_bg.wasm
RUN wasm-pack build --target web .

FROM scratch AS simulator-wasm-pack-artefacts

COPY --from=simulator /app/simulator/pkg /out/

FROM base AS check-formatting

WORKDIR /app
CMD ["cargo", "fmt", "--all", "--check"]
