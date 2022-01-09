# Advent of code solutions.

My implementation of advent of code stuff.

## Running

I'm lazy, you need to be in the right directory.

```shell
$ cd 2021/day01/src/
$ cargo test
$ cargo run
```

## Building

```shell
podman build . -t rustadvent
podman run -it --rm -v $(pwd):$(pwd):rw -w $(pwd) rustadvent \
  cargo build

or 

../../cargowrapper.sh test
../../cargowrapper.sh build
```

## Copying

A BSD-style license.
