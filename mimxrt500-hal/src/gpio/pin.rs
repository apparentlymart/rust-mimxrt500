use super::dynpin::{DynAltFunc, DynInput, DynOutput, DynPinId, DynPinMode};
use core::marker::PhantomData;
use paste::paste;

pub mod alternate;
pub mod input;
pub mod output;

pub trait PinMode: private::Sealed {
    const DYN: DynPinMode;
}

/// A [`PinMode`] used for pins that have not yet been explicitly configured
/// and which are therefore not known to be any specific mode.
pub enum Unknown {}

impl PinMode for Unknown {
    const DYN: DynPinMode = DynPinMode::Unknown;
}
impl private::Sealed for Unknown {}

/// Type-level enum for pin IDs
///
/// Valid options take the form `PIOX_YY`, where `X` is a number in `0`-`5` and
/// `YY` is a number between 00-31.
pub trait PinId: private::Sealed {
    /// Corresponding [`DynPinId`](DynPinId)
    const DYN: DynPinId;
}

/// A type-level GPIO pin, parameterized by [`PinId`] and [`PinMode`] types
pub struct Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    regs: super::internal::SingletonPin<InternalPinId<I>>,
    mode: PhantomData<M>,
}

impl<I, M> Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    #[inline(always)]
    pub(super) const unsafe fn new() -> Self {
        Self {
            regs: super::internal::SingletonPin::new(InternalPinId(PhantomData)),
            mode: PhantomData,
        }
    }

    #[inline(always)]
    pub(super) fn into_new_mode<NewM: PinMode>(self) -> Pin<I, NewM> {
        // This function is intended to optimize away to just some inline
        // register writes, since old_mode and new_mode should always be
        // entirely known at compile time.
        let old_mode = M::DYN;
        let new_mode = NewM::DYN;

        if new_mode == old_mode {
            // This function should compile to nothing at all if nothing
            // is actually changing.

            // Safety: pin layout is identical (zero-length) regardless of mode
            return unsafe { core::mem::transmute(self) };
        }

        match new_mode {
            DynPinMode::Input(_) => {
                self.regs.set_output_mode(false);
                unsafe { self.regs.set_iopctl(new_mode.iopctl_value()) };
            }
            DynPinMode::Output(_) => {
                let v = new_mode.iopctl_value();
                unsafe { self.regs.set_iopctl(v) };
                self.regs.set_output_mode(true);
            }
            DynPinMode::Alternate(_, _) => {
                unsafe { self.regs.set_iopctl(new_mode.iopctl_value()) };
                self.regs.set_output_mode(false);
            }
            DynPinMode::Unknown => {
                // We don't offer any type conversions towards unknown, so
                // it should not be possible to get here.
                unreachable!();
            }
        }

        // Safety: pin layout is identical (zero-length) regardless of mode
        unsafe { core::mem::transmute(self) }
    }

    /// Sets the output value for the pin.
    ///
    /// This method is always available, but will have no immediate effect
    /// if the pin is not in output mode. Use `set_output` before converting
    /// to an output to specify the initial value once output mode is enabled.
    #[inline(always)]
    pub fn set_output(&self, high: bool) {
        self.regs.set_output_value(high)
    }

    /// Toggles the output state for the pin.
    ///
    /// This method is always available, but will have no immediate effect
    /// if the pin is not in output mode.
    #[inline(always)]
    pub fn toggle_output(&self) {
        self.regs.toggle_output_value()
    }

}

struct InternalPinId<I: PinId>(PhantomData<I>);

unsafe impl<I: PinId> super::internal::SingletonPinId for InternalPinId<I> {
    #[inline(always)]
    fn id(&self) -> DynPinId {
        I::DYN
    }
}

macro_rules! pin_id {
    (
        $(
            $Name:ident,
            $Group:ident,
            $Num:expr
        ),+
    ) => {
        paste! {
            $(
                #[
                    doc = "Type-level variant of [`PinId`] for \
                    GPIO pin " $Name
                ]
                pub enum $Name {}
                impl private::Sealed for $Name {}
                impl PinId for $Name {
                    const DYN: DynPinId = DynPinId {
                        group: super::dynpin::DynGroup::$Group,
                        num: $Num,
                    };
                }
            )+
        }
    };
}

pin_id!(PIO0_0, Group0, 0);
pin_id!(PIO0_1, Group0, 1);
pin_id!(PIO0_2, Group0, 2);
pin_id!(PIO0_3, Group0, 3);
pin_id!(PIO0_4, Group0, 4);
pin_id!(PIO0_5, Group0, 5);
pin_id!(PIO0_6, Group0, 6);
pin_id!(PIO0_7, Group0, 7);
pin_id!(PIO0_8, Group0, 8);
pin_id!(PIO0_9, Group0, 9);
pin_id!(PIO0_10, Group0, 10);
pin_id!(PIO0_11, Group0, 11);
pin_id!(PIO0_12, Group0, 12);
pin_id!(PIO0_13, Group0, 13);
pin_id!(PIO0_14, Group0, 14);
pin_id!(PIO0_15, Group0, 15);

pin_id!(PIO1_0, Group1, 0);
pin_id!(PIO1_1, Group1, 1);
pin_id!(PIO1_2, Group1, 2);
pin_id!(PIO1_3, Group1, 3);
pin_id!(PIO1_4, Group1, 4);
pin_id!(PIO1_5, Group1, 5);
pin_id!(PIO1_6, Group1, 6);
pin_id!(PIO1_7, Group1, 7);
pin_id!(PIO1_8, Group1, 8);
pin_id!(PIO1_9, Group1, 9);
pin_id!(PIO1_10, Group1, 10);
pin_id!(PIO1_11, Group1, 11);
pin_id!(PIO1_12, Group1, 12);
pin_id!(PIO1_13, Group1, 13);
pin_id!(PIO1_14, Group1, 14);
pin_id!(PIO1_15, Group1, 15);

mod private {
    /// Super trait used to mark traits with an exhaustive set of
    /// implementations
    pub trait Sealed {}
}

impl<Id: PinId, C: input::InputConfig> From<Pin<Id, Unknown>> for Pin<Id, input::Input<C>> {
    #[inline(always)]
    fn from(value: Pin<Id, Unknown>) -> Self {
        value.into_new_mode()
    }
}

impl<Id: PinId, const OD: bool, const FD: bool, const SS: bool> From<Pin<Id, Unknown>> for Pin<Id, output::Output<OD, FD, SS>> {
    #[inline(always)]
    fn from(value: Pin<Id, Unknown>) -> Self {
        value.into_new_mode()
    }
}

impl<Id: PinId, F: alternate::AltFunc, ST: alternate::SignalType> From<Pin<Id, Unknown>>
    for Pin<Id, alternate::Alternate<F, ST>>
{
    #[inline(always)]
    fn from(value: Pin<Id, Unknown>) -> Self {
        value.into_new_mode()
    }
}

impl<Id: PinId, const OOD: bool, const OFD: bool, const OSS: bool, IC: input::InputConfig> From<Pin<Id, output::Output<OOD, OFD, OSS>>>
    for Pin<Id, input::Input<IC>>
{
    #[inline(always)]
    fn from(value: Pin<Id, output::Output<OOD, OFD, OSS>>) -> Self {
        value.into_new_mode()
    }
}
