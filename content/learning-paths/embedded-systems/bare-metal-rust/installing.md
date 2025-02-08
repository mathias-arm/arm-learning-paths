---
# User change
title: Installing

weight: 2 # 1 is first, 2 is second, etc.

# Do not modify these elements
layout: "learningpathall"
---

## Install the Rust toolchain

To install the Rust toolchain, refer to [Rust Install Guide](/install-guides/rust/).

```console
cargo install cargo-binutils
```

## Install Docker

To install Docker, refer to the [Docker install guide](/install-guides/docker/).

## Install and configure shrinkwrap

This learning path uses [Shrinkwrap](https://shrinkwrap.docs.arm.com/en/latest/). Shrinkwrap is a tool to simplify the process of building and running firmware on Arm Fixed Virtual Platforms (FVP).

To install Shrinkwrap's dependencies:

- On Ubuntu 22.04 (and other Debian-based distributions):

```
sudo apt-get install git netcat-openbsd python3 python3-pip \
    python3-virtualenv telnet
```

- On MacOS:

```
brew install git netcat virtualenv telnet
```

Create a Python virtual environment, install dependencies and clone `shrinkwrap` repository (this only needs to be done once):

```shell
virtualenv venv
source venv/bin/activate
pip3 install pyyaml termcolor tuxmake
git clone https://git.gitlab.arm.com/tooling/shrinkwrap.git
```

Load the environment for `shrinkwrap` (this needs to be done in every shell before executing the `shrinkwrap` command):

```shell
source venv/bin/activate
export PATH=$PATH:$PWD/shrinkwrap/shrinkwrap
export SHRINKWRAP_BUILD=$PWD/build
export SHRINKWRAP_PACKAGE=$PWD/package
```
