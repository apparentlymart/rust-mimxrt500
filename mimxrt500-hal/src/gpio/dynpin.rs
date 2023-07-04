use super::pin;

/// Value-level `enum` for disabled configurations
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DynDisabled {
    Floating,
    PullDown,
    PullUp,
}

/// Value-level `enum` for input configurations
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DynInput {
    Floating,
    PullDown,
    PullUp,
}

/// Value-level `enum` for output configurations
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct DynOutput {
    pub open_drain: bool,
    pub full_drive: bool,
    pub slow_slew: bool,
}

/// Value-level `enum` for alternate peripheral function configurations
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DynAltFunc {
    Func1 = 1,
    Func2 = 2,
    Func3 = 3,
    Func4 = 4,
    Func5 = 5,
    Func6 = 6,
    Func7 = 7,
    Func8 = 8,
    Func9 = 9,
    Func10 = 10,
    Func11 = 11,
    Func12 = 12,
    Func13 = 13,
    Func14 = 14,
    Func15 = 15,
}

impl DynAltFunc {
    #[inline(always)]
    pub(super) const fn fsel(self) -> u32 {
        self as u32
    }
}

/// Value-level `enum` for alternate peripheral function signal types.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DynSignalType {
    Digital,
    Analog,
}

/// Value-level `enum` representing pin modes
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DynPinMode {
    Unknown,
    Input(DynInput),
    Output(DynOutput),
    Alternate(DynAltFunc, DynSignalType),
}

impl DynPinMode {
    #[inline(always)]
    pub(super) const fn iopctl_value(self) -> u32 {
        match self {
            DynPinMode::Unknown => 0,
            DynPinMode::Input(ic) => {
                let pulls = match ic {
                    DynInput::Floating => 0b00,
                    DynInput::PullDown => 0b01,
                    DynInput::PullUp => 0b11,
                } << 4;
                let ibena = 1 << 6; // input buffer always enabled in input mode
                ibena | pulls
            }
            DynPinMode::Output(oc) => {
                let odena = if oc.open_drain { 1 << 10 } else { 0 };
                let fulldrive = if oc.full_drive { 1 << 8 } else { 0 };
                let slewrate = if oc.slow_slew { 1 << 7 } else { 0 };
                // If we're using simulated open drain then we also need
                // to enable pull-up because the pin will only drive low
                // in this mode.
                let pulls = if odena != 0 { 0b11 << 4 } else { 0 };
                odena | fulldrive | slewrate | pulls
            }
            DynPinMode::Alternate(f, st) => {
                let fsel = f.fsel();
                let amena_ibena = match st {
                    DynSignalType::Digital => 1 << 6, // IBENA
                    DynSignalType::Analog => 1 << 9,  // AMENA
                };
                fsel | amena_ibena
            }
        }
    }
}

/// Value-level `struct` representing pin IDs
#[derive(PartialEq, Clone, Copy)]
pub struct DynPinId {
    pub group: DynGroup,
    pub num: u8,
}

/// Value-level `enum` for pin groups
#[derive(PartialEq, Clone, Copy)]
pub enum DynGroup {
    Group0 = 0,
    Group1 = 1,
    Group2 = 2,
    Group3 = 3,
    Group4 = 4,
    Group5 = 5,
    Group6 = 6,
}

impl DynGroup {
    #[inline(always)]
    pub(super) const fn index(self) -> usize {
        self as usize
    }
}

/// A value-level representation of a pin, with pin ID and mode decided at
/// runtime.
pub struct DynPin {
    id: DynPinId,
    mode: DynPinMode,
}

impl DynPin {
    // Safety: Caller must ensure that `id` describes a valid pin and
    // that `mode` accurately describes the current mode of that pin.
    #[inline(always)]
    pub(super) const unsafe fn new(id: DynPinId, mode: DynPinMode) -> Self {
        Self { id, mode }
    }

    /// Attempt to convert the dynamic pin back into a type-based pin.
    ///
    /// If the `Id` and `Mode` generic parameters match the current dynamic
    /// state of the pin then this returns `Ok` with a type-based `Pin`
    /// object. If not then this returns `Err` to echo back the
    /// originally-provided dynamic pin.
    #[inline(always)]
    pub fn into_type_based<Id: pin::PinId, Mode: pin::PinMode>(
        self,
    ) -> Result<pin::Pin<Id, Mode>, Self> {
        let dest_id = Id::DYN;
        let dest_mode = Mode::DYN;

        if dest_id == self.id && dest_mode == self.mode {
            Ok(unsafe { pin::Pin::new() })
        } else {
            Err(self)
        }
    }
}
