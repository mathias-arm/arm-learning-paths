
---
# User change
title: Improved version

weight: 5

# Do not modify these elements
layout: "learningpathall"
---

## Improved version

The previous program only worked because it was very small (it only used registers). Let's make a better version, and start by copying the first program into a new folder and start improving it:

```console
(cd hello-world ; cargo --clean)
cp -r hello-world bare-metal
```

### Changing Cargo.toml

In `bare-metal/Cargo.toml`, change `hello-world` to `bare-metal`.

### Replacing link.x

Replace `bare-metal/link.x` with the following content:

{{< include-code C "content/learning-paths/embedded-systems/bare-metal-rust/source/bare-metal/link.x" >}}

### Replacing src/main.rs

Replace `bare-metal/src/main.rs` the following content:

{{< include-code rust "content/learning-paths/embedded-systems/bare-metal-rust/source/bare-metal/src/main.rs" >}}

### Creating src/entry.rs

Create `bare-metal/src/entry.rs` the following content:

{{< include-code rust "content/learning-paths/embedded-systems/bare-metal-rust/source/bare-metal/src/entry.rs" >}}

### Creating src/pl011.rs

Create `bare-metal/src/pl011.rs` the following content:

{{< include-code rust "content/learning-paths/embedded-systems/bare-metal-rust/source/bare-metal/src/pl011.rs" >}}

## Building the program

Run the following command:

```
(cd bare-metal ; \
    cargo build --release ; \
    cargo objcopy --release -- -O binary bare-metal.bin ; \
    cargo size --release)
```

## Building the program

Run the following command:

```
shrinkwrap run bare-metal.yaml --rtvar BINARY=bare-metal/bare-metal.bin
```

The output should look like:

```
[...]
NOTICE:  Booting Trusted Firmware
[...]
Hello world!
[...]
```
