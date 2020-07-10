# Rust Application State

A sample app using Actix Application State.

#### Usage

To run the app, do:

```bash
$ cargo run
```

waiting for installation and compilation to finish, then:

```bash
$ curl -H 'HOST: api.hello-rust.org' 0.0.0.0:3000/api/hello
Hello Rust Main times 1!
```

or

```bash
$ curl -H 'HOST: main.hello-rust.org' 0.0.0.0:3000/main/hello
Hello Rust API times 1!
```
