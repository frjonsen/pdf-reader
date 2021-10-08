FROM node:slim as frontendbuilder

COPY frontend frontend
WORKDIR /frontend
RUN yarn build

FROM rust:latest as backendbuilder

COPY backend backend
WORKDIR /backend
RUN cargo build --release

FROM nginx:mainline
COPY --from=frontendbuilder /frontend/build /usr/share/nginx/html
COPY --from=backendbuilder  /backend/target/release/pdf-reader /bin/pdf-reader