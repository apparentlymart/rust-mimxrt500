use core::marker::PhantomData;
use super::{private, PinMode};
use super::super::dynpin::{DynInput, DynPinMode};

pub struct Input<C: InputConfig> {
    cfg: PhantomData<C>,
}
impl<C: InputConfig> PinMode for Input<C> {
    const DYN: DynPinMode = DynPinMode::Input(C::DYN);
}
impl<C: InputConfig> private::Sealed for Input<C> {}

pub trait InputConfig: private::Sealed {
    const DYN: DynInput;
}

pub enum Floating {}
impl InputConfig for Floating {
    const DYN: DynInput = DynInput::Floating;
}
impl private::Sealed for Floating {}

pub enum PullUp {}
impl InputConfig for PullUp {
    const DYN: DynInput = DynInput::PullUp;
}
impl private::Sealed for PullUp {}

pub enum PullDown {}
impl InputConfig for PullDown {
    const DYN: DynInput = DynInput::PullDown;
}
impl private::Sealed for PullDown {}

pub type InputFloating = Input<Floating>;
pub type InputPullUp = Input<PullUp>;
pub type InputPullDown = Input<PullDown>;
