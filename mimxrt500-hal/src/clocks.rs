use crate::pac;

pub struct Dependencies {
    pub clkctl0: pac::CLKCTL0,
    pub clkctl1: pac::CLKCTL1,
}

pub struct Clocks {
    deps: Dependencies,

    pub gpio: GpioClock<false>,
}

impl Clocks {
    #[inline(always)]
    pub fn new(clkctl0: pac::CLKCTL0, clkctl1: pac::CLKCTL1) -> Self {
        Self {
            deps: Dependencies { clkctl0, clkctl1 },

            gpio: unsafe { Clock::new() },
        }
    }
}

pub struct Clock<P, const INDEX: usize, const MASK: u32, const ACTIVE: bool>
where
    P: ClockControlPeripheral,
{
    _phantom: core::marker::PhantomData<P>,
}

impl<P, const INDEX: usize, const MASK: u32, const ACTIVE: bool> Clock<P, INDEX, MASK, ACTIVE>
where
    P: ClockControlPeripheral,
{
    // Safety: Caller must ensure that all of the type parameters are sensible.
    #[inline(always)]
    unsafe fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<P, const INDEX: usize, const MASK: u32> Clock<P, INDEX, MASK, false>
where
    P: ClockControlPeripheral,
{
    #[inline(always)]
    pub fn activate(self) -> Clock<P, INDEX, MASK, true> {
        // Safety: Callers must only construct Clock with valid type params
        unsafe { P::activate_clocks(INDEX, MASK) };
        // Safety: Memory layout of Clock is identical regardless of parameters
        unsafe { core::mem::transmute(self) }
    }
}

impl<P, const INDEX: usize, const MASK: u32> Clock<P, INDEX, MASK, true>
where
    P: ClockControlPeripheral,
{
    #[inline(always)]
    pub fn deactivate(self) -> Clock<P, INDEX, MASK, false> {
        // Safety: Callers must only construct Clock with valid type params
        unsafe { P::deactivate_clocks(INDEX, MASK) };
        // Safety: Memory layout of Clock is identical regardless of parameters
        unsafe { core::mem::transmute(self) }
    }
}

impl<P, const INDEX: usize, const MASK: u32> From<Clock<P, INDEX, MASK, false>>
    for Clock<P, INDEX, MASK, true>
where
    P: ClockControlPeripheral,
{
    #[inline(always)]
    fn from(value: Clock<P, INDEX, MASK, false>) -> Self {
        value.activate()
    }
}

impl<P, const INDEX: usize, const MASK: u32> From<Clock<P, INDEX, MASK, true>>
    for Clock<P, INDEX, MASK, false>
where
    P: ClockControlPeripheral,
{
    #[inline(always)]
    fn from(value: Clock<P, INDEX, MASK, true>) -> Self {
        value.deactivate()
    }
}

pub unsafe trait ClockControlPeripheral {
    unsafe fn activate_clocks(idx: usize, mask: u32);
    unsafe fn deactivate_clocks(idx: usize, mask: u32);
}

unsafe impl ClockControlPeripheral for pac::CLKCTL0 {
    #[inline(always)]
    unsafe fn activate_clocks(idx: usize, mask: u32) {
        let set_base: *mut u8 = (Self::PTR as *mut u8).add(0x40);
        let set_ptr = (set_base as *mut u32).add(idx);
        core::ptr::write_volatile(set_ptr, mask);
    }

    #[inline(always)]
    unsafe fn deactivate_clocks(idx: usize, mask: u32) {
        let clr_base: *mut u8 = (Self::PTR as *mut u8).add(0x40);
        let clr_ptr = (clr_base as *mut u32).add(idx);
        core::ptr::write_volatile(clr_ptr, mask);
    }
}

unsafe impl ClockControlPeripheral for pac::CLKCTL1 {
    #[inline(always)]
    unsafe fn activate_clocks(idx: usize, mask: u32) {
        let set_base: *mut u8 = (Self::PTR as *mut u8).add(0x40);
        let set_ptr = (set_base as *mut u32).add(idx);
        core::ptr::write_volatile(set_ptr, mask);
    }

    #[inline(always)]
    unsafe fn deactivate_clocks(idx: usize, mask: u32) {
        let clr_base: *mut u8 = (Self::PTR as *mut u8).add(0x40);
        let clr_ptr = (clr_base as *mut u32).add(idx);
        core::ptr::write_volatile(clr_ptr, mask);
    }
}

mod private {
    pub trait Sealed {}
}

macro_rules! named_clock {
    ($name:ident, $periph:ident, $idx:expr, $bit:expr) => {
        pub type $name<const ACTIVE: bool> = Clock<pac::$periph, $idx, { 1 << $bit }, ACTIVE>;
    };
}

named_clock!(GpioClock, CLKCTL1, 1, 0);
