#======================================================
#==================Base Image==========================
FROM rust:slim-buster as base
RUN \
	apt-get update && \
	apt-get upgrade -y && \
	apt-get install git pkg-config libssl-dev -y --no-install-recommends

#=======================================================
#=============Planner stage=============================
FROM base as planner

ARG MONITORED_DISTRICTS
ENV MONITORED_DISTRICTS=$MONITORED_DISTRICTS

WORKDIR vnb
# We only p	ay the installation cost once, 
# it will be cached from the second build onwards
# To ensure a reproducible build consider pinning 
# the cargo-chef version with `--version X.X.X`
RUN cargo install cargo-chef 
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

#======================================================
#===============Caching Stage==========================

FROM base as cacher
WORKDIR vnb
RUN cargo install cargo-chef
COPY --from=planner /vnb/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

#======================================================
#=================Builder Stage========================
FROM base as builder
WORKDIR vnb
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /vnb/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin vnb


#=======================================================
#==================Production Image=====================
FROM debian:buster-slim

ARG TELEOXIDE_TOKEN
ARG CHANNEL_ID
ARG MONITORED_DISTRICTS
ARG TZ=Asia/Kolkata

ENV \
    TELOXIDE_TOKEN=$TELEOXIDE_TOKEN \
    CHANNEL_ID=$CHANNEL_ID \
    MONITORED_DISTRICTS=$MONITORED_DISTRICTS \
    TZ=$TZ

RUN \
    apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y --no-install-recommends \
    pkg-config libssl-dev

WORKDIR vnb
COPY --from=builder /vnb/target/release/vnb /usr/local/bin
CMD ["/usr/local/bin/vnb"]
