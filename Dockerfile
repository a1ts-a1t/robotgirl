FROM docker.io/rust:1-alpine3.22 AS build

COPY . .

RUN apk add openssl-dev musl-dev openssl-libs-static
RUN cargo build -r
RUN objcopy --compress-debug-sections target/release/robotgirl ./robotgirl

# server
FROM docker.io/alpine:3.22.0

RUN apk update
RUN apk add git openssh-client

WORKDIR /app
COPY . .
COPY --from=build /robotgirl ./robotgirl

RUN git config user.name RobotGirl
RUN git config user.email robotgirl@github.com
RUN --mount=type=secret,id=env \
    export $(cat /run/secrets/env | xargs) && \
    git config --global url."https://$GH_PAT@github.com/".insteadOf "https://github.com"

EXPOSE 3000
CMD ["./robotgirl"]

