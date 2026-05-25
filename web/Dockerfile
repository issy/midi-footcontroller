FROM ghcr.io/pnpm/pnpm:11.2.2 AS base

WORKDIR /app

COPY . .
COPY --from=simulator-wasm-pack-artefacts /out/ /app/simulator-pkg/

RUN pnpm install --frozen-lockfile
RUN pnpm run build

FROM scratch AS web-artefacts

COPY --from=base /app/dist/ /out/
