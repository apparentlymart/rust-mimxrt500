# Rust support for NXP i.MX RT500 series microcontrollers

This repository contains a number of different crates, at different levels of
abstraction, for programming chips in the NXP i.MX RT500 series of so-called
"crossover MCUs".

* [`mimxrt595s`](https://docs.rs/crate/mimxrt595s): Low-level peripheral access
crate for the RT595S chip.
* [`mimxrt500-rt`](https://docs.rs/crate/mimxrt500-rt): A fork of
[`cortex-m-rt`](https://docs.rs/crate/cortex-m-rt) that deals with the usual
extra requirements imposed by these chips' boot ROM, which expects extra
configuration blocks and metadata in NOR flash images.

    This _must_ typically be used with a board support package since RT500
    chips have no on-chip flash memory and so the board is responsible for
    providing some kind of boot media.

    This library currently assumes XIP boot from NOR flash. It may grow to
    support creating images for other boot media in future, which will at
    least require a different linker script that produces an appropriate
    image load header and maps code into addresses in RAM.
* [`mimxrt595-evk`](https://docs.rs/crate/mimxrt595-evk): Board support
package for NXP's official evaluation kit that includes, amongst other things,
the RT595S chip and an on-board NOR flash.
* [`mimxrt500-hal`](https://docs.rs/crate/mimxrt500-hal): `embedded-hal`
trait implementations and other higher-level abstractions for the RT500 chip
peripherals.

All of these crates are currently in an early stage of development and might
change API significantly before becoming stable.
