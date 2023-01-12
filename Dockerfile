FROM node:slim as frontendbuilder

COPY frontend frontend
WORKDIR /frontend
RUN yarn install && yarn next build && yarn next export

# Using specific versions because glibc of the rust and nginx images need to match,
# which it happens to do for these two images
FROM rust:1.66.1-buster as backendbuilder

COPY backend backend
WORKDIR /pdfium
RUN curl -LO https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-linux-x64.tgz
RUN tar xzf pdfium-linux-x64.tgz
WORKDIR /backend
RUN cargo build --release
RUN strip target/release/pdfreader

FROM nginx:1.23.3
RUN apt update && apt install gosu
COPY --from=frontendbuilder /frontend/out /usr/share/nginx/html
COPY --from=backendbuilder  /backend/target/release/pdfreader /bin/pdfreader
COPY --from=backendbuilder /pdfium/lib/libpdfium.so /usr/lib/x86_64-linux-gnu/
RUN mkdir /workdir
COPY deploy/entrypoint.sh /workdir
COPY deploy/nginx.conf /etc/nginx/nginx.conf
COPY deploy/nginx-default.conf /etc/nginx/conf.d/default.conf
WORKDIR /workdir
COPY --from=backendbuilder /backend/migrations .
CMD ["./entrypoint.sh"]
