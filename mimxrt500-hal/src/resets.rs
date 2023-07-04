use crate::pac;

pub struct Dependencies {
    pub rstctl0: pac::RSTCTL0,
    pub rstctl1: pac::RSTCTL1,
}

pub struct Resets {
    deps: Dependencies,

    pub gpio: GpioReset<true>,
}

impl Resets {
    #[inline(always)]
    pub fn new(rstctl0: pac::RSTCTL0, rstctl1: pac::RSTCTL1) -> Self {
        Self {
            deps: Dependencies { rstctl0, rstctl1 },

            gpio: unsafe { Reset::new() },
        }
    }

    /// Consume the [`Resets`] object and recover its dependencies.
    ///
    /// Safety: The peripherals are returned in an undefined state, and
    /// so should both be reset to defaults before using them for anything that
    /// expects them to be in a known state, which includes creating a new
    /// instance of [`Resets`].
    #[inline(always)]
    pub const unsafe fn free(self) -> Dependencies {
        self.deps
    }
}

pub struct Reset<P, const INDEX: usize, const MASK: u32, const IN_RESET: bool>
where
    P: ResetControlPeripheral,
{
    _phantom: core::marker::PhantomData<P>,
}

impl<P, const INDEX: usize, const MASK: u32, const IN_RESET: bool> Reset<P, INDEX, MASK, IN_RESET>
where
    P: ResetControlPeripheral,
{
    // Safety: Caller must ensure that all of the type parameters are sensible.
    #[inline(always)]
    unsafe fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<P, const INDEX: usize, const MASK: u32> Reset<P, INDEX, MASK, true>
where
    P: ResetControlPeripheral,
{
    pub fn unassert_reset(self) -> Reset<P, INDEX, MASK, false> {
        // Safety: Callers must only construct Clock with valid type params
        unsafe { P::deactivate_resets(INDEX, MASK) };
        // Safety: Memory layout of Clock is identical regardless of parameters
        unsafe { core::mem::transmute(self) }
    }
}

impl<P, const INDEX: usize, const MASK: u32> Reset<P, INDEX, MASK, false>
where
    P: ResetControlPeripheral,
{
    pub fn assert_reset(self) -> Reset<P, INDEX, MASK, true> {
        // Safety: Callers must only construct Clock with valid type params
        unsafe { P::activate_resets(INDEX, MASK) };
        // Safety: Memory layout of Clock is identical regardless of parameters
        unsafe { core::mem::transmute(self) }
    }
}

impl<P, const INDEX: usize, const MASK: u32> From<Reset<P, INDEX, MASK, true>>
    for Reset<P, INDEX, MASK, false>
where
    P: ResetControlPeripheral,
{
    #[inline(always)]
    fn from(value: Reset<P, INDEX, MASK, true>) -> Self {
        value.unassert_reset()
    }
}

impl<P, const INDEX: usize, const MASK: u32> From<Reset<P, INDEX, MASK, false>>
    for Reset<P, INDEX, MASK, true>
where
    P: ResetControlPeripheral,
{
    #[inline(always)]
    fn from(value: Reset<P, INDEX, MASK, false>) -> Self {
        value.assert_reset()
    }
}

pub unsafe trait ResetControlPeripheral {
    unsafe fn activate_resets(idx: usize, mask: u32);
    unsafe fn deactivate_resets(idx: usize, mask: u32);
}

unsafe impl ResetControlPeripheral for pac::RSTCTL0 {
    #[inline(always)]
    unsafe fn activate_resets(idx: usize, mask: u32) {
        let set_base: *mut u8 = (Self::PTR as *mut u8).add(0x40);
        let set_ptr = (set_base as *mut u32).add(idx);
        core::ptr::write_volatile(set_ptr, mask);
    }

    #[inline(always)]
    unsafe fn deactivate_resets(idx: usize, mask: u32) {
        let clr_base: *mut u8 = (Self::PTR as *mut u8).add(0x70);
        let clr_ptr = (clr_base as *mut u32).add(idx);
        core::ptr::write_volatile(clr_ptr, mask);
    }
}

unsafe impl ResetControlPeripheral for pac::RSTCTL1 {
    #[inline(always)]
    unsafe fn activate_resets(idx: usize, mask: u32) {
        let set_base: *mut u8 = (Self::PTR as *mut u8).add(0x40);
        let set_ptr = (set_base as *mut u32).add(idx);
        core::ptr::write_volatile(set_ptr, mask);
    }

    #[inline(always)]
    unsafe fn deactivate_resets(idx: usize, mask: u32) {
        let clr_base: *mut u8 = (Self::PTR as *mut u8).add(0x70);
        let clr_ptr = (clr_base as *mut u32).add(idx);
        core::ptr::write_volatile(clr_ptr, mask);
    }
}

macro_rules! named_reset {
    ($name:ident, $periph:ident, $idx:expr, $bit:expr) => {
        pub type $name<const ACTIVE: bool> = Reset<pac::$periph, $idx, { 1 << $bit }, ACTIVE>;
    };
}

named_reset!(GpioReset, RSTCTL1, 1, 0);
