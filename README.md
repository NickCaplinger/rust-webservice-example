# Rust Webservice Example

## Overview

The purpose of this project is to demonstrate how errors might be caught from a backend webservice running Actix Web.

## Setup

In order to build this application, you'll need Cargo, which you can install via [Rustup](https://rustup.rs/).

In order to build a container for deployment to the cloud, you'll need [Docker](https://www.docker.com/get-started).

## Developing locally

After installing the Rust toolchain, use `cargo build` to build the project. There are multiple flags that can be passed here, with `--release` being the notable one. See `cargo help build` for more info.

The `build` command will download dependencies, build them, and then build this project.

Invoking `cargo run` (again with an optional `--release` flag) will build _and_ run the project.

## Building and pushing a Docker container

In order to build a container, first make sure you have a configuration file in the default location listed in the Configuration step.

Then run:
`docker build . --tag your-tag-here:latest`

To push up the container, run:
`docker push your-tag-here`
