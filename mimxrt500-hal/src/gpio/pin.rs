use super::dynpin::{DynPin, DynPinId, DynPinMode};
use core::marker::PhantomData;
use embedded_hal::digital as ehal;
use paste::paste;

/// Types for representing alternate function modes.
pub mod alternate;

/// Types for representing input modes.
pub mod input;

/// Types for representing output modes.
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

/// A type-level GPIO pin, parameterized by [`PinId`] and [`PinMode`] types.
///
/// This is the main interesting type for pin configuration and GPIO
/// functionality. Use `.into()` to switch a pin into a different mode and
/// therefore obtain a new `Pin` with different parameters.
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

    /// Reconfigures the pin into a push-pull output and returns the new pin.
    #[inline(always)]
    pub fn into_push_pull_output(self) -> Pin<I, output::OutputPushPull> {
        self.into_new_mode()
    }

    /// Reconfigures the pin into a floating input and returns the new pin.
    #[inline(always)]
    pub fn into_input(self) -> Pin<I, input::InputFloating> {
        self.into_new_mode()
    }

    /// Reconfigures the pin into an input with pull-up and returns the new pin.
    #[inline(always)]
    pub fn into_input_pullup(self) -> Pin<I, input::InputPullUp> {
        self.into_new_mode()
    }

    /// Reconfigures the pin into an input with pull-down and returns the new pin.
    #[inline(always)]
    pub fn into_input_pulldown(self) -> Pin<I, input::InputPullDown> {
        self.into_new_mode()
    }

    /// Reinterprets the pin as a value-based [`DynPin`], which tracks the
    /// pin ID and mode as runtime data rather than as type parameters.
    #[inline(always)]
    pub const fn into_value_based(self) -> DynPin {
        // Safety: this is safe as long as we've correctly guaranteed that it's
        // impossible to construct a Pin with invalid ID and mode.
        unsafe { DynPin::new(I::DYN, M::DYN) }
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

macro_rules! pin_id_group {
    ($Group:literal) => {
        paste! {
            pin_id!([<PIO $Group _0>], [<Group $Group>], 0);
            pin_id!([<PIO $Group _1>], [<Group $Group>], 1);
            pin_id!([<PIO $Group _2>], [<Group $Group>], 2);
            pin_id!([<PIO $Group _3>], [<Group $Group>], 3);
            pin_id!([<PIO $Group _4>], [<Group $Group>], 4);
            pin_id!([<PIO $Group _5>], [<Group $Group>], 5);
            pin_id!([<PIO $Group _6>], [<Group $Group>], 6);
            pin_id!([<PIO $Group _7>], [<Group $Group>], 7);
            pin_id!([<PIO $Group _8>], [<Group $Group>], 8);
            pin_id!([<PIO $Group _9>], [<Group $Group>], 9);
            pin_id!([<PIO $Group _10>], [<Group $Group>], 10);
            pin_id!([<PIO $Group _11>], [<Group $Group>], 11);
            pin_id!([<PIO $Group _12>], [<Group $Group>], 12);
            pin_id!([<PIO $Group _13>], [<Group $Group>], 13);
            pin_id!([<PIO $Group _14>], [<Group $Group>], 14);
            pin_id!([<PIO $Group _15>], [<Group $Group>], 15);
            pin_id!([<PIO $Group _16>], [<Group $Group>], 16);
            pin_id!([<PIO $Group _17>], [<Group $Group>], 17);
            pin_id!([<PIO $Group _18>], [<Group $Group>], 18);
            pin_id!([<PIO $Group _19>], [<Group $Group>], 19);
            pin_id!([<PIO $Group _20>], [<Group $Group>], 20);
            pin_id!([<PIO $Group _21>], [<Group $Group>], 21);
            pin_id!([<PIO $Group _22>], [<Group $Group>], 22);
            pin_id!([<PIO $Group _23>], [<Group $Group>], 23);
            pin_id!([<PIO $Group _24>], [<Group $Group>], 24);
            pin_id!([<PIO $Group _25>], [<Group $Group>], 25);
            pin_id!([<PIO $Group _26>], [<Group $Group>], 26);
            pin_id!([<PIO $Group _27>], [<Group $Group>], 27);
            pin_id!([<PIO $Group _28>], [<Group $Group>], 28);
            pin_id!([<PIO $Group _29>], [<Group $Group>], 29);
            pin_id!([<PIO $Group _30>], [<Group $Group>], 31);
            pin_id!([<PIO $Group _31>], [<Group $Group>], 31);
        }
    };
}

pin_id_group!(0);
pin_id_group!(1);
pin_id_group!(2);
pin_id_group!(3);
pin_id_group!(4);
pin_id_group!(5);
// Group 6 only has pins 0-27.
pin_id!(PIO6_0, Group6, 0);
pin_id!(PIO6_1, Group6, 1);
pin_id!(PIO6_2, Group6, 2);
pin_id!(PIO6_3, Group6, 3);
pin_id!(PIO6_4, Group6, 4);
pin_id!(PIO6_5, Group6, 5);
pin_id!(PIO6_6, Group6, 6);
pin_id!(PIO6_7, Group6, 7);
pin_id!(PIO6_8, Group6, 8);
pin_id!(PIO6_9, Group6, 9);
pin_id!(PIO6_10, Group6, 10);
pin_id!(PIO6_11, Group6, 11);
pin_id!(PIO6_12, Group6, 12);
pin_id!(PIO6_13, Group6, 13);
pin_id!(PIO6_14, Group6, 14);
pin_id!(PIO6_15, Group6, 15);
pin_id!(PIO6_16, Group6, 16);
pin_id!(PIO6_17, Group6, 17);
pin_id!(PIO6_18, Group6, 18);
pin_id!(PIO6_19, Group6, 19);
pin_id!(PIO6_20, Group6, 20);
pin_id!(PIO6_21, Group6, 21);
pin_id!(PIO6_22, Group6, 22);
pin_id!(PIO6_23, Group6, 23);
pin_id!(PIO6_24, Group6, 24);
pin_id!(PIO6_25, Group6, 25);
pin_id!(PIO6_26, Group6, 26);
pin_id!(PIO6_27, Group6, 27);

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

impl<Id: PinId, const OD: bool, const FD: bool, const SS: bool> From<Pin<Id, Unknown>>
    for Pin<Id, output::Output<OD, FD, SS>>
{
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

impl<Id: PinId, const OOD: bool, const OFD: bool, const OSS: bool, IC: input::InputConfig>
    From<Pin<Id, output::Output<OOD, OFD, OSS>>> for Pin<Id, input::Input<IC>>
{
    #[inline(always)]
    fn from(value: Pin<Id, output::Output<OOD, OFD, OSS>>) -> Self {
        value.into_new_mode()
    }
}

impl<Id: PinId, IC: input::InputConfig, const OOD: bool, const OFD: bool, const OSS: bool>
    From<Pin<Id, input::Input<IC>>> for Pin<Id, output::Output<OOD, OFD, OSS>>
{
    #[inline(always)]
    fn from(value: Pin<Id, input::Input<IC>>) -> Self {
        value.into_new_mode()
    }
}

/// Pins in input mode implement [`ehal::InputPin`].
///
/// This implementation does not take into account the possibility that the
/// port might be configured to invert its input. If that flag is set then
/// the two methods will return opposite values.
impl<Id: PinId, C: input::InputConfig> ehal::InputPin for Pin<Id, input::Input<C>> {
    #[inline(always)]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.regs.input_is_high())
    }

    #[inline(always)]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(!self.regs.input_is_high())
    }
}

/// Pins in output mode implement [`ehal::OutputPin`].
impl<Id: PinId, const OD: bool, const FD: bool, const SS: bool> ehal::OutputPin
    for Pin<Id, output::Output<OD, FD, SS>>
{
    #[inline(always)]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.regs.set_output_value(false);
        Ok(())
    }

    #[inline(always)]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.regs.set_output_value(true);
        Ok(())
    }
}

/// Pins in output mode implement [`ehal::ToggleableOutputPin`].
impl<Id: PinId, const OD: bool, const FD: bool, const SS: bool> ehal::ToggleableOutputPin
    for Pin<Id, output::Output<OD, FD, SS>>
{
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.regs.toggle_output_value();
        Ok(())
    }
}

impl<Id: PinId, M: PinMode> ehal::ErrorType for Pin<Id, M> {
    type Error = core::convert::Infallible;
}
