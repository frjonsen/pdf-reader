FROM node:slim as frontendbuilder

COPY frontend frontend
WORKDIR /frontend
RUN yarn install && yarn next build && yarn next export

FROM rust:latest as backendbuilder

COPY backend backend
WORKDIR /backend
RUN cargo build --release
RUN strip target/release/pdfreader

FROM nginx:mainline
RUN apt update && apt install gosu
COPY --from=frontendbuilder /frontend/out /usr/share/nginx/html
COPY --from=backendbuilder  /backend/target/release/pdfreader /bin/pdfreader
RUN mkdir /workdir
COPY deploy/entrypoint.sh /workdir
COPY deploy/nginx.conf /etc/nginx/nginx.conf
COPY deploy/nginx-default.conf /etc/nginx/conf.d/default.conf
WORKDIR /workdir
COPY --from=backendbuilder /backend/migrations .
CMD ["./entrypoint.sh"]
