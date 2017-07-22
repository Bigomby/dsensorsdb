# Dynamic Sensors Database

## Overview

Dynamic Sensors Database is a library to store Netflow templates. Is written in
Rust and there is a C interface is available.

The goal of the library is to store Netflow templates in a database which uses
Netflow concepts (sensors, observation ids, networks, etc.). The implementation
aims to be _fast_ and have low overhead. Check out the
[example](https://github.com/Bigomby/dsensorsdb/blob/master/examples/example.c).

All the sensors stored on the database can be managed through a REST API. For
example, to get all the sensors stored in the database you can `GET /sensors`.

**NOTE**: This library is currently WIP. Don't use it in production.

## Building

To build the library you need to have the Rust build system installed
(you can use [rustup](https://www.rustup.rs)).

Once you have it installed, just run:

```bash
make build
```

This will generate a dynamic c library on `target/debug` called
`libdsensorsdb.so`. You should not use this shared object in a final release.

## Installing

To generate a release version with optimizations, run:

```bash
make release
```

And then install the generated shared library and the header file to your
system:

```bash
make install
```

## Running test

To run tests, run the following command:

```bash
make test
```

## API Documentation

The public API docs are available in the
[header file](https://github.com/Bigomby/dsensorsdb/blob/master/headers/dsensorsdb.h).

## Roadmap

- [ ] Database module
- [ ] Sensor module
- [ ] Observation ID module
- [ ] REST API module
- [ ] Persistence (etcd, redis, kafka, etc.)
