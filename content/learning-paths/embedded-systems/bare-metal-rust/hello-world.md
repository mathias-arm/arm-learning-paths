
---
# User change
title: Create and build a Hello World example project

weight: 4

# Do not modify these elements
layout: "learningpathall"
---

## Creating the project

The following command will create the `hello-word` directory with an empty project.

```console
cargo new hello-world
```

### Adding rust-toolchain.toml

Create the file `hello-world/rust-toolchain.toml` with the following content:

{{< include-code toml "content/learning-paths/embedded-systems/bare-metal-rust/source/hello-world/rust-toolchain.toml" >}}

This is equivalent to running the commands when required:

```console
rustup target add aarch64-unknown-none
rustup component add llvm-tools
```

### Adding link.x

Create `hello-world/link.x` with the following content:

{{< include-code C "content/learning-paths/embedded-systems/bare-metal-rust/source/hello-world/link.x" >}}

This is a simple linker script because the `aarch64-unknown-none` target does not have a default one.

### Adding build.rs

Create `hello-world/build.rs` with the following content:

{{< include-code rust "content/learning-paths/embedded-systems/bare-metal-rust/source/hello-world/build.rs" >}}

This script tells the Cargo build system to take into account changes to `link.x` when re-building the project.

### Adding .cargo/config.toml

Create `hello-world/.cargo/config.toml` with the following content:

{{< include-code toml "content/learning-paths/embedded-systems/bare-metal-rust/source/hello-world/.cargo/config.toml" >}}

This configuration file sets the default target and tell the Rust compiler to use the `link.x` linker script.

### Modifying Cargo.toml

Add to `hello-world/Cargo.toml` the following content:

```toml
[profile.release]
codegen-units = 1
lto = true
opt-level = "s"
panic = "abort"
```

This section changes the `release` profile to optimize for size with maximum link-time optimizations.

### Replacing src/main.rs

Replace `hello-world/src/main.rs` with following content:

{{< include-code rust "content/learning-paths/embedded-systems/bare-metal-rust/source/hello-world/src/main.rs" >}}

## Building the project

Run the following commands:

```
(cd hello-world ; \
    cargo build --release ;
    cargo objcopy --release -- -O binary hello-world.bin)
```

# Running the project

Run the following command:

```
shrinkwrap run bare-metal.yaml --rtvar BINARY=hello-world/hello-world.bin
```

The output should contain the lines:

```
[...]
NOTICE:  Booting Trusted Firmware
[...]
Hello world!
[...]
```

# Looking at the binary

Run the command:

```
(cd hello-world ; \
    cargo size --release ;
    cargo objdump --release -- -d)
```

The output should look like:

```
    Finished `release` profile [optimized] target(s) in 0.00s
   text	   data	    bss	    dec	    hex	filename
     61	      0	      0	     61	     3d	hello-world
    Finished `release` profile [optimized] target(s) in 0.00s

hello-world:	file format elf64-littleaarch64

Disassembly of section .text:

0000000000000000 <main>:
       0: aa1f03ea     	mov	x10, xzr
       4: d503201f     	nop
       8: 10000148     	adr	x8, 0x30
       c: 52a38129     	mov	w9, #0x1c090000         // =470351872
      10: 386a690b     	ldrb	w11, [x8, x10]
      14: 9100054a     	add	x10, x10, #0x1
      18: f100355f     	cmp	x10, #0xd
      1c: 3900012b     	strb	w11, [x9]
      20: 54ffff81     	b.ne	0x10 <main+0x10>
      24: 52800300     	mov	w0, #0x18               // =24
      28: d45e0000     	hlt	#0xf000
      2c: d4200020     	brk	#0x1
```

The resulting binary is only 61 bytes.
