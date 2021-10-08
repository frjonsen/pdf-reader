FROM node:slim as frontendbuilder

COPY frontend frontend
WORKDIR /frontend
RUN yarn install && yarn build

FROM rust:latest as backendbuilder

COPY backend backend
WORKDIR /backend
RUN cargo build --release

FROM nginx:mainline
COPY --from=frontendbuilder /frontend/build /usr/share/nginx/html
COPY --from=backendbuilder  /backend/target/release/pdf-reader /bin/pdf-reader
RUN mkdir /workdir
COPY deploy/entrypoint.sh /workdir
COPY deploy/nginx.conf /etc/nginx/conf.d/default.conf
WORKDIR /workdir
CMD ["./entrypoint.sh"]