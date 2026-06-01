FROM docker.io/library/rust:1.96.0

LABEL   org.opencontainers.image.authors="heathcliff@heathcliff.eu" \
        org.opencontainers.image.description="Builder and CI image for rust projects. Can be used to cross-compile." \
        org.opencontainers.image.source="https://github.com/heathcliff26/turbo-clicker" \
        org.opencontainers.image.licenses="Apache-2.0" \
        org.opencontainers.image.title="rust-qt-builder"

# renovate: datasource=github-releases depName=goreleaser/goreleaser extractVersion=^(?<version>.*)$
ARG GORELEASER_VERSION=v2.16.0

COPY install-deps.sh .
RUN ./install-deps.sh

WORKDIR /app
