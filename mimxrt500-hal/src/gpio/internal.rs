use super::dynpin::DynPinId;
use crate::pac;

pub(super) unsafe trait SingletonPinId {
    fn id(&self) -> DynPinId;
}

// The PAC considers the entire GPIO peripheral and the entire IOPCTL peripheral
// each as a single ownable value, which is not granular enough for the API
// we need to present to implement the embedded-hal GPIO traits.
//
// This type allows us to instead treat each pin as a separate ownable object,
// by directly calculating the register offsets relative to the PAC's idea
// of the base address of each peripheral.
//
// This is still safe as long as the caller makes sure that:
// - They have exclusive ownership of both peripherals and will not try to
//   use any of the individual-pin-related registers except via values of this
//   type.
// - They make sure that there can only be one value of this type for each
//   distinct pin ID.
pub(super) struct SingletonPin<Id: SingletonPinId> {
    id: Id,
}

impl<Id: SingletonPinId> SingletonPin<Id> {
    // Safety: Caller must have exclusive control over the pin identified
    // by the given pin ID, and ensure that all access to that pin's
    // registers is through the returned object.
    #[inline(always)]
    pub(super) const unsafe fn new(id: Id) -> Self {
        Self { id }
    }

    #[inline(always)]
    fn bitmask(&self) -> u32 {
        let dyn_id = self.id.id();
        1 << dyn_id.num
    }

    #[inline(always)]
    fn value_ptr(&self) -> *mut u8 {
        // The byte-sized registers for all of the pins are right at the
        // start of the GPIO peripheral's memory block, at (a * 0x20) + b
        // where "a" is the group number and "b" is the pin number within
        // the group, as in PIOa_b.
        let dyn_id = self.id.id();
        let base = pac::GPIO::ptr() as *mut u8;
        let offset = (dyn_id.group.index() * 0x20) + (dyn_id.num as usize);
        unsafe { base.add(offset) }
    }

    #[inline(always)]
    unsafe fn gpio_port_bitmask_ptr(&self, offset: usize) -> (*mut u32, u32) {
        let dyn_id = self.id.id();
        let base = unsafe { (pac::GPIO::ptr() as *mut u8).add(offset) as *mut u32 };
        let offset_words = dyn_id.group.index();
        (unsafe { base.add(offset_words) }, self.bitmask())
    }

    #[inline(always)]
    unsafe fn iopctl_ptr(&self) -> *mut u32 {
        // IOPCTL port configuration registers are at offset
        // (a * 0x80) + (b * 4) from the IOPCTL peripheral, as in
        // PIOa_b.
        let dyn_id = self.id.id();
        let base =  pac::IOPCTL::ptr() as *mut u8;
        let offset_bytes = (dyn_id.group.index() * 0x80) + (dyn_id.num as usize * 4);
        unsafe { base.add(offset_bytes) as *mut u32 }
    }

    #[inline(always)]
    pub(super) fn input_is_high(&self) -> bool {
        let ptr = self.value_ptr();
        return unsafe { core::ptr::read_volatile(ptr) } != 0;
    }

    #[inline(always)]
    pub(super) fn set_output_value(&self, is_high: bool) {
        let v: u8 = if is_high { 1 } else { 0 };
        let ptr = self.value_ptr();
        unsafe { core::ptr::write_volatile(ptr, v) }
    }

    #[inline(always)]
    pub(super) fn toggle_output_value(&self) {
        let (ptr, mask) = unsafe { self.gpio_port_bitmask_ptr(0x2300) };
        unsafe { core::ptr::write_volatile(ptr, mask) }
    }

    #[inline(always)]
    pub(super) fn set_output_mode(&self, is_output: bool) {
        let reg_offset = if is_output {
            0x2380 // DIRSETa
        } else {
            0x2400 // DIRCLRa
        };
        let (ptr, mask) = unsafe { self.gpio_port_bitmask_ptr(reg_offset) };
        unsafe { core::ptr::write_volatile(ptr, mask) }
    }

    /// Set the IOPCTL configuration register for this pin.
    ///
    /// Safety: Caller is responsible for ensuring that the given value is
    /// valid to write to an IOPCTL configuration register.
    #[inline(always)]
    pub(super) unsafe fn set_iopctl(&self, v: u32) {
        let ptr = self.iopctl_ptr();
        unsafe { core::ptr::write_volatile(ptr, v) }
    }
}
