use core::marker::PhantomData;
use super::{private, PinMode};
use super::super::dynpin::{DynInput, DynPinMode};

/// The [`PinMode`] for pins in input modes.
pub struct Input<C: InputConfig> {
    cfg: PhantomData<C>,
}
impl<C: InputConfig> PinMode for Input<C> {
    const DYN: DynPinMode = DynPinMode::Input(C::DYN);
}
impl<C: InputConfig> private::Sealed for Input<C> {}

/// Type-level enum for the various different input sub-modes.
///
/// - [`Floating`]: A passive input pin.
/// - [`PullUp`]: An input pin with a weak pull-up.
/// - [`PullDown`]: An input pin with a weak pull-down.
pub trait InputConfig: private::Sealed {
    const DYN: DynInput;
}

/// The [`InputConfig`] for a passive input pin.
pub enum Floating {}
impl InputConfig for Floating {
    const DYN: DynInput = DynInput::Floating;
}
impl private::Sealed for Floating {}

/// The [`InputConfig`] for a pin with a weak pull-up.
pub enum PullUp {}
impl InputConfig for PullUp {
    const DYN: DynInput = DynInput::PullUp;
}
impl private::Sealed for PullUp {}

/// The [`InputConfig`] for a pin with a weak pull-down.
pub enum PullDown {}
impl InputConfig for PullDown {
    const DYN: DynInput = DynInput::PullDown;
}
impl private::Sealed for PullDown {}

/// Alias [`PinMode`] for a passive input pin.
pub type InputFloating = Input<Floating>;

/// Alias [`PinMode`] for an input pin with a weak pull-up.
pub type InputPullUp = Input<PullUp>;

/// Alias [`PinMode`] for an input pin with a weak pull-down.
pub type InputPullDown = Input<PullDown>;
