//! Generates a suitable default flash control block and boot stub for linking
//! into applications intended to run on the MIMXRT595-EVK, which is the
//! official evaluation kit for the NXP i.MX RT500 series of microcontrollers.
//!
//! With the `bootstub` feature enabled (which is on by default), linking this
//! crate into your application will use the [`bootstub_builtin`] and
//! [`fcb`] macros from crate `mimxrt500-bootstub` to produce symbols that
//! will be placed in the appropriate locations by the linker script provided
//! by the `mimxrt595-evk` crate. The flash control block uses the values
//! provided by `mimxrt595-evk`, in [`FCB`].
//!
//! To use this, declare a dependency on this crate and then ensure it will
//! be linked into your program by declaring it as an `extern crate`:
//!
//! ```
//! extern crate mimxrt595_evk_bootstub as _;
//! ```
//!
//! Your application must also use `cortex-m-rt` and a suitable build script
//! to arrange for using its linker script `link.x` when building your
//! application. That will include the `memory.x` linker script provided by
//! `mimxrt595-evk`, which will arrange for the boot stub code and flash
//! control block to be placed where the on-chip boot ROM expects to find them.
//!
//! This library exports no symbols itself. It exists only to add some special
//! static variables to your program. It's a separate crate so that applications
//! with different needs can still depend on `mimxrt595-evk` but generate
//! their own boot stub and flash control block, perhaps using the macros
//! provided in crate `mimxrt500-bootstub`.
#![no_std]

use mimxrt500_bootstub::{bootstub_builtin, fcb};
use mimxrt595_evk::bootstub::FCB;

#[cfg(feature = "bootstub")]
bootstub_builtin!();

#[cfg(feature = "bootstub")]
fcb!(FCB);
