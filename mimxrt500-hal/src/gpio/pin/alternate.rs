use super::super::dynpin::{DynAltFunc, DynPinMode, DynSignalType};
use super::{private, PinMode};
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
                    doc = "Type-level variant of [`AltFunc`] for \
                    alternate pin function " $Name
                ]
                pub enum $Name {}
                impl private::Sealed for $Name {}
                impl AltFunc for $Name {
                    const DYN: DynAltFunc = DynAltFunc::$Name;
                }
                #[
                    doc = "Type-level variant of [`PinMode`] for alternate \
                    peripheral function [`" $Name "`] in digital mode"
                ]
                pub type [<Alt $Name Digital>] = Alternate<$Name, Digital>;
                #[
                    doc = "Type-level variant of [`PinMode`] for alternate \
                    peripheral function [`" $Name "`] in analog mode"
                ]
                pub type [<Alt $Name Analog>] = Alternate<$Name, Analog>;
            )+
        }
    };
}

alt_function!(
    Func1, Func2, Func3, Func4, Func5, Func6, Func7, Func8, Func9, Func10, Func11, Func12, Func13,
    Func14, Func15
);

/// Type-level variant of [`PinMode`] for alternate peripheral functions.
///
/// This is parameterized by both an [`AltFunc`] for function and a
/// [`SignalType`] to specify whether the pin is enabled for digital or analog
/// input.
pub struct Alternate<F: AltFunc, ST: SignalType> {
    cfg: PhantomData<(F, ST)>,
}

impl<C: AltFunc, ST: SignalType> private::Sealed for Alternate<C, ST> {}

impl<F: AltFunc, ST: SignalType> PinMode for Alternate<F, ST> {
    const DYN: DynPinMode = DynPinMode::Alternate(F::DYN, ST::DYN);
}

/// Type-level enum for alternate peripheral function configurations
pub trait AltFunc: private::Sealed {
    /// Corresponding [`DynAlternate`]
    const DYN: DynAltFunc;
}

/// Type-level enum for alternate peripheral function signal types
pub trait SignalType: private::Sealed {
    const DYN: DynSignalType;
}

/// Type-level variant of [`SignalType`] for digital functions.
pub enum Digital {}
impl SignalType for Digital {
    const DYN: DynSignalType = DynSignalType::Digital;
}
impl private::Sealed for Digital {}

/// Type-level variant of [`SignalType`] for analog functions.
pub enum Analog {}
impl SignalType for Analog {
    const DYN: DynSignalType = DynSignalType::Analog;
}
impl private::Sealed for Analog {}
