FROM docker.io/rust:1-alpine3.22 AS build

COPY . .

RUN apk add openssl-dev musl-dev openssl-libs-static
RUN cargo build -r
RUN objcopy --compress-debug-sections target/release/robotgirl ./robotgirl

FROM docker.io/alpine:3.22.0

# server
COPY . .
COPY --from=build /robotgirl ./robotgirl

EXPOSE 3000
CMD ["./robotgirl"]

