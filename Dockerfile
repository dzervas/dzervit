FROM rustlang/rust:nightly-slim as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release --target-dir .

FROM debian:buster-slim

ENV ROCKET_ENV=production
ENV PUID=1000
ENV PGID=1000

COPY --from=builder /usr/src/app/release/dzervit /usr/local/bin/

USER ${PUID}:${PGID}

EXPOSE 8000/tcp
CMD [ "dzervit" ]
