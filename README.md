wip

## setup

as a prereq, you should have a `.runtime.env` file that sets the `LLAMA_SERVER_URL` variable as whatever your model endpoint is. (i guess more importantly you should have a [llama.cpp server](https://github.com/ggml-org/llama.cpp) running.)

if you want to be able to push changes from your docker environment, you should also have a `.build.env` file that sets the `GH_PAT` variable to whatever token you have. i would recommend provisioning one with just the rights to make prs.

get this thing going with

```sh
docker build --secret id=env,src=.build.env robotgirl .
docker run --init --env-file .runtime.env -p 3000:3000 robotgirl
```

ui should be up on [`localhost:3000`](http://localhost:3000)

