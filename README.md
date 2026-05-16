wip

## setup

get this going with

```sh
docker build -t robotgirl .
docker run --init --env-file .env -p 3000:3000 robotgirl
```

ui should be up on [`localhost:3000`](http://localhost:3000)

