# Fullstack Rust web application single binary example

This is an example of how to crate Rust full stack web application, using [seed](https://seed-rs.org/) for frontend and [actix-web](https://actix.rs/) for backend. We will utilize [actix-web static files](https://docs.rs/actix-web-static-files/3.0.5/actix_web_static_files/index.html) create to develop single binary embedding Seed frontend. We can then just run this binary to serve both frontend and API.

## 1. Frontend

Follow Seed [quickstart](https://github.com/seed-rs/seed-quickstart).

    cargo make watch

    cargo make serve

## 2. Server

Based on https://github.com/seed-rs/seed/blob/master/examples/server_integration/server/src/main.rs.

## 3. Actix-web static files

Follow instructions on the [actix-web static files](https://docs.rs/actix-web-static-files/3.0.5/actix_web_static_files/index.html) documentation.

## 4. Build and run

To see steps required for build have a look at `build_and_run.sh` script. Reproduced here:

```bash
    cd client
    rm -rf dist
    cargo make build_release
    mkdir dist
    cp index.html dist/
    cp -r pkg dist/
    cd ..

    cd server
    cargo build
    cargo run
```

We will build client using command `cargo make build_release`, then we will copy `index.html` and `pkg` directory into `dist` directory that will be embedded into server binary.

Then we go to the `server` and execute `cargo build`.