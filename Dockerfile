FROM rustlang/rust:nightly-slim as rust-builder

WORKDIR /usr/src/app
COPY src/ ./src
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release --target-dir .

FROM node:slim as node-builder

WORKDIR /usr/src/app
COPY web/ ./web
COPY .babelrc package.json package-lock.json webpack.config.js ./
RUN npm install && npm run build

FROM debian:buster-slim

ENV CARGO_MANIFEST_DIR=/usr/src/app
ENV ROCKET_ENV=production
ENV PUID=1000
ENV PGID=1000

WORKDIR ${CARGO_MANIFEST_DIR}
COPY --from=rust-builder /usr/src/app/release/dzervit /usr/src/app
COPY --from=node-builder /usr/src/app/bundle ./bundle/

USER ${PUID}:${PGID}

EXPOSE 8000/tcp
CMD [ "./dzervit" ]
