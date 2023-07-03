use super::{private, PinMode};
use super::super::dynpin::{DynOutput, DynPinMode};

pub struct Output<const OPEN_DRAIN: bool, const FULL_DRIVE: bool, const SLOW_SLEW: bool> {
}
impl<const OPEN_DRAIN: bool, const FULL_DRIVE: bool, const SLOW_SLEW: bool> PinMode for Output<OPEN_DRAIN, FULL_DRIVE, SLOW_SLEW> {
    const DYN: DynPinMode = DynPinMode::Output(DynOutput {
        open_drain: OPEN_DRAIN,
        full_drive: FULL_DRIVE,
        slow_slew: SLOW_SLEW,
    });
}
impl<const OPEN_DRAIN: bool, const FULL_DRIVE: bool, const SLOW_SLEW: bool> private::Sealed for Output<OPEN_DRAIN, FULL_DRIVE, SLOW_SLEW> {}

/// An alias for [`Output`] in push-pull mode with all the other special features disabled.
pub type OutputPushPull = Output<false, false, false>;

/// An alias for [`Output`] in open drain mode with all the other special features disabled.
pub type OutputOpenDrain = Output<true, false, false>;
