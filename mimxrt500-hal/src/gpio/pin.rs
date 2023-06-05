use super::dynpin::{DynAlternate, DynPinId, DynPinMode};
use crate::pac::IOPCTL;
use core::marker::PhantomData;
use paste::paste;

macro_rules! alt_function {
    (
        $(
            $Name:ident
        ),+
    ) => {
        paste! {
            $(
                #[
                    doc = "Type-level variant of [`AlternateConfig`] for \
                    alternate pin function " $Name
                ]
                pub enum $Name {}
                impl private::Sealed for $Name {}
                impl AlternateConfig for $Name {
                    const DYN: DynAlternate = DynAlternate::$Name;
                }
                #[
                    doc = "Type-level variant of [`PinMode`] for alternate \
                    peripheral function [`" $Name "`]"
                ]
                pub type [<Alternate $Name>] = Alternate<$Name>;
            )+
        }
    };
}

alt_function!(
    Func1, Func2, Func3, Func4, Func5, Func6, Func7, Func8, Func9, Func10, Func11, Func12, Func13,
    Func14, Func15
);

/// Type-level variant of [`PinMode`] for alternate peripheral functions
///
/// Type `C` is an [`AlternateConfig`]
pub struct Alternate<C: AlternateConfig> {
    cfg: PhantomData<C>,
}

impl<C: AlternateConfig> private::Sealed for Alternate<C> {}

/// Type-level enum for alternate peripheral function configurations
pub trait AlternateConfig: private::Sealed {
    /// Corresponding [`DynAlternate`]
    const DYN: DynAlternate;
}

pub trait PinMode: private::Sealed {
    const DYN: DynPinMode;
}

/*
impl<C: DisabledConfig> PinMode for Disabled<C> {
    const DYN: DynPinMode = DynPinMode::Disabled(C::DYN);
}

impl<C: InputConfig> PinMode for Input<C> {
    const DYN: DynPinMode = DynPinMode::Input(C::DYN);
}

impl<C: InterruptConfig> PinMode for Interrupt<C> {
    const DYN: DynPinMode = DynPinMode::Interrupt(C::DYN);
}

impl<C: OutputConfig> PinMode for Output<C> {
    const DYN: DynPinMode = DynPinMode::Output(C::DYN);
}
*/

impl<C: AlternateConfig> PinMode for Alternate<C> {
    const DYN: DynPinMode = DynPinMode::Alternate(C::DYN);
}

/// Type-level enum for pin IDs
///
/// Valid options take the form `PIOX_YY`, where `X` is a number in `0`-`5` and
/// `YY` is a number between 00-31.
pub trait PinId: private::Sealed {
    /// Corresponding [`DynPinId`](DynPinId)
    const DYN: DynPinId;
}

/// Provide a safe register interface for [`Pin`]s
///
/// This `struct` takes ownership of a [`PinId`] and provides an API to
/// access the corresponding regsiters.
pub(in crate::gpio) struct Registers<I: PinId> {
    id: PhantomData<I>,
}

/// A type-level GPIO pin, parameterized by [`PinId`] and [`PinMode`] types
pub struct Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    pub(in crate::gpio) regs: Registers<I>,
    mode: PhantomData<M>,
}

mod private {
    /// Super trait used to mark traits with an exhaustive set of
    /// implementations
    pub trait Sealed {}
}
