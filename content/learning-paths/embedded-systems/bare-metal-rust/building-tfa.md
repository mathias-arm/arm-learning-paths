---
# User change
title: Building TF-A

weight: 3

# Do not modify these elements
layout: "learningpathall"
---

## Trusted Firmware-A (TF-A)

TF-A provides a reference implementation of secure world software for Armv8-A.

## Building TF-A

Create `bare-metal.yaml` and input the following content:

{{< include-code yaml "content/learning-paths/embedded-systems/bare-metal-rust/source/bare-metal.yaml" >}}

`bare-metal.yaml` is the configuration file Shrinkwrap will use to build TF-A and define FVP command parameters:
- TF-A will boot the binary loaded at address `0x84000000` with the [Linux Boot Protocol](https://docs.kernel.org/arch/arm64/booting.html).
- The DTB will be placed at address `0x82000000`.

Run the following command:

```shell
shrinkwrap build bare-metal.yaml
```
