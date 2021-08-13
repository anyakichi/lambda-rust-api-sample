# lambda-rust-api-sample

This is a rust application sample that works in AWS Lambda.

You can build it using [docker-buildenv](https://github.com/anyakichi/docker-buildenv).

```
$ mkdir lambda-rust-api-1 && cd $_
$ din anyakichi/lambda-rust-api-sample:main
[builder@lambda-rust-api-1 build]$ extract
[builder@lambda-rust-api-1 build]$ setup
[builder@lambda-rust-api-1 lambda-rust-api-sample]$ build
[builder@lambda-rust-api-1 lambda-rust-api-sample]$ package
```

You can share some directories with your host machine.

```
$ din \
    -e CARGO_HOME=/cargo -v $HOME/.cargo:/cargo \
    -v $HOME/.aws:/home/builder/.aws:ro \
    anyakichi/lambda-rust-api-sample:main
```

Upload the application to your Lambda environment.

```
[builder@lambda-rust-api-1 lambda-rust-api-sample]$ update

or use aws command directly

[builder@lambda-rust-api-1 lambda-rust-sample]$ aws lambda update-function-code \
    --function-name <function-name> --zip-file fileb://lambda-rust-sample.zip
```

See [docker-lambda-rust-builder](https://github.com/anyakichi/docker-lambda-rust-builder) for more information.
