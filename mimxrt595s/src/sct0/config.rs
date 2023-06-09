#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UNIFY` reader - SCT Operation"]
pub type UNIFY_R = crate::BitReader<UNIFY_A>;
#[doc = "SCT Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNIFY_A {
    #[doc = "0: Dual counter. The SCT operates as two 16-bit counters named COUNTER_L and COUNTER_H."]
    DUAL_COUNTER = 0,
    #[doc = "1: Unified counter. The SCT operates as a unified 32-bit counter."]
    UNIFIED_COUNTER = 1,
}
impl From<UNIFY_A> for bool {
    #[inline(always)]
    fn from(variant: UNIFY_A) -> Self {
        variant as u8 != 0
    }
}
impl UNIFY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNIFY_A {
        match self.bits {
            false => UNIFY_A::DUAL_COUNTER,
            true => UNIFY_A::UNIFIED_COUNTER,
        }
    }
    #[doc = "Checks if the value of the field is `DUAL_COUNTER`"]
    #[inline(always)]
    pub fn is_dual_counter(&self) -> bool {
        *self == UNIFY_A::DUAL_COUNTER
    }
    #[doc = "Checks if the value of the field is `UNIFIED_COUNTER`"]
    #[inline(always)]
    pub fn is_unified_counter(&self) -> bool {
        *self == UNIFY_A::UNIFIED_COUNTER
    }
}
#[doc = "Field `UNIFY` writer - SCT Operation"]
pub type UNIFY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, UNIFY_A, O>;
impl<'a, const O: u8> UNIFY_W<'a, O> {
    #[doc = "Dual counter. The SCT operates as two 16-bit counters named COUNTER_L and COUNTER_H."]
    #[inline(always)]
    pub fn dual_counter(self) -> &'a mut W {
        self.variant(UNIFY_A::DUAL_COUNTER)
    }
    #[doc = "Unified counter. The SCT operates as a unified 32-bit counter."]
    #[inline(always)]
    pub fn unified_counter(self) -> &'a mut W {
        self.variant(UNIFY_A::UNIFIED_COUNTER)
    }
}
#[doc = "Field `CLKMODE` reader - SCT Clock Mode"]
pub type CLKMODE_R = crate::FieldReader<u8, CLKMODE_A>;
#[doc = "SCT Clock Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKMODE_A {
    #[doc = "0: System Clock Mode. The system clock clocks the entire SCT module including all counters and counter prescalers."]
    SYSTEM_CLOCK_MODE = 0,
    #[doc = "1: Sampled System Clock Mode. The system clock clocks the SCT module, but the counter and prescalers are only enabled to count when the designated edge is detected on the input selected by the CKSEL field. The minimum pulse width on the selected clock-gate input is 1 bus clock period. This mode is the high-performance, sampled-clock mode."]
    SAMPLED_SYSTEM_CLOCK_MODE = 1,
    #[doc = "2: SCT Input Clock Mode. The input/edge selected by the CKSEL field clocks the SCT module, including all counters and prescalers, after first being synchronized to the system clock. The minimum pulse width on the clock input is 1 bus clock period. This mode is the low-power, sampled-clock mode."]
    SCT_INPUT_CLOCK_MODE = 2,
    #[doc = "3: Asynchronous Mode. The entire SCT module is clocked directly by the input/edge selected by the CKSEL field. In this mode, the SCT outputs are switched synchronously to the SCT input clock - not the system clock. The input clock rate must be at least half the system clock rate and can be the same or faster than the system clock."]
    ASYNCHRONOUS_MODE = 3,
}
impl From<CLKMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKMODE_A) -> Self {
        variant as _
    }
}
impl CLKMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKMODE_A {
        match self.bits {
            0 => CLKMODE_A::SYSTEM_CLOCK_MODE,
            1 => CLKMODE_A::SAMPLED_SYSTEM_CLOCK_MODE,
            2 => CLKMODE_A::SCT_INPUT_CLOCK_MODE,
            3 => CLKMODE_A::ASYNCHRONOUS_MODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYSTEM_CLOCK_MODE`"]
    #[inline(always)]
    pub fn is_system_clock_mode(&self) -> bool {
        *self == CLKMODE_A::SYSTEM_CLOCK_MODE
    }
    #[doc = "Checks if the value of the field is `SAMPLED_SYSTEM_CLOCK_MODE`"]
    #[inline(always)]
    pub fn is_sampled_system_clock_mode(&self) -> bool {
        *self == CLKMODE_A::SAMPLED_SYSTEM_CLOCK_MODE
    }
    #[doc = "Checks if the value of the field is `SCT_INPUT_CLOCK_MODE`"]
    #[inline(always)]
    pub fn is_sct_input_clock_mode(&self) -> bool {
        *self == CLKMODE_A::SCT_INPUT_CLOCK_MODE
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS_MODE`"]
    #[inline(always)]
    pub fn is_asynchronous_mode(&self) -> bool {
        *self == CLKMODE_A::ASYNCHRONOUS_MODE
    }
}
#[doc = "Field `CLKMODE` writer - SCT Clock Mode"]
pub type CLKMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CONFIG_SPEC, u8, CLKMODE_A, 2, O>;
impl<'a, const O: u8> CLKMODE_W<'a, O> {
    #[doc = "System Clock Mode. The system clock clocks the entire SCT module including all counters and counter prescalers."]
    #[inline(always)]
    pub fn system_clock_mode(self) -> &'a mut W {
        self.variant(CLKMODE_A::SYSTEM_CLOCK_MODE)
    }
    #[doc = "Sampled System Clock Mode. The system clock clocks the SCT module, but the counter and prescalers are only enabled to count when the designated edge is detected on the input selected by the CKSEL field. The minimum pulse width on the selected clock-gate input is 1 bus clock period. This mode is the high-performance, sampled-clock mode."]
    #[inline(always)]
    pub fn sampled_system_clock_mode(self) -> &'a mut W {
        self.variant(CLKMODE_A::SAMPLED_SYSTEM_CLOCK_MODE)
    }
    #[doc = "SCT Input Clock Mode. The input/edge selected by the CKSEL field clocks the SCT module, including all counters and prescalers, after first being synchronized to the system clock. The minimum pulse width on the clock input is 1 bus clock period. This mode is the low-power, sampled-clock mode."]
    #[inline(always)]
    pub fn sct_input_clock_mode(self) -> &'a mut W {
        self.variant(CLKMODE_A::SCT_INPUT_CLOCK_MODE)
    }
    #[doc = "Asynchronous Mode. The entire SCT module is clocked directly by the input/edge selected by the CKSEL field. In this mode, the SCT outputs are switched synchronously to the SCT input clock - not the system clock. The input clock rate must be at least half the system clock rate and can be the same or faster than the system clock."]
    #[inline(always)]
    pub fn asynchronous_mode(self) -> &'a mut W {
        self.variant(CLKMODE_A::ASYNCHRONOUS_MODE)
    }
}
#[doc = "Field `CKSEL` reader - SCT Clock Select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register."]
pub type CKSEL_R = crate::FieldReader<u8, CKSEL_A>;
#[doc = "SCT Clock Select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKSEL_A {
    #[doc = "0: Rising edges on input 0"]
    INPUT_0_RISING_EDGES = 0,
    #[doc = "1: Falling edges on input 0"]
    INPUT_0_FALLING_EDGES = 1,
    #[doc = "2: Rising edges on input 1"]
    INPUT_1_RISING_EDGES = 2,
    #[doc = "3: Falling edges on input 1"]
    INPUT_1_FALLING_EDGES = 3,
    #[doc = "4: Rising edges on input 2"]
    INPUT_2_RISING_EDGES = 4,
    #[doc = "5: Falling edges on input 2"]
    INPUT_2_FALLING_EDGES = 5,
    #[doc = "6: Rising edges on input 3"]
    INPUT_3_RISING_EDGES = 6,
    #[doc = "7: Falling edges on input 3"]
    INPUT_3_FALLING_EDGES = 7,
    #[doc = "8: Rising edges on input 4"]
    INPUT_4_RISING_EDGES = 8,
    #[doc = "9: Falling edges on input 4"]
    INPUT_4_FALLING_EDGES = 9,
    #[doc = "10: Rising edges on input 5"]
    INPUT_5_RISING_EDGES = 10,
    #[doc = "11: Falling edges on input 5"]
    INPUT_5_FALLING_EDGES = 11,
    #[doc = "12: Rising edges on input 6"]
    INPUT_6_RISING_EDGES = 12,
    #[doc = "13: Falling edges on input 6"]
    INPUT_6_FALLING_EDGES = 13,
    #[doc = "14: Rising edges on input 7"]
    INPUT_7_RISING_EDGES = 14,
    #[doc = "15: Falling edges on input 7"]
    INPUT_7_FALLING_EDGES = 15,
}
impl From<CKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CKSEL_A) -> Self {
        variant as _
    }
}
impl CKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKSEL_A {
        match self.bits {
            0 => CKSEL_A::INPUT_0_RISING_EDGES,
            1 => CKSEL_A::INPUT_0_FALLING_EDGES,
            2 => CKSEL_A::INPUT_1_RISING_EDGES,
            3 => CKSEL_A::INPUT_1_FALLING_EDGES,
            4 => CKSEL_A::INPUT_2_RISING_EDGES,
            5 => CKSEL_A::INPUT_2_FALLING_EDGES,
            6 => CKSEL_A::INPUT_3_RISING_EDGES,
            7 => CKSEL_A::INPUT_3_FALLING_EDGES,
            8 => CKSEL_A::INPUT_4_RISING_EDGES,
            9 => CKSEL_A::INPUT_4_FALLING_EDGES,
            10 => CKSEL_A::INPUT_5_RISING_EDGES,
            11 => CKSEL_A::INPUT_5_FALLING_EDGES,
            12 => CKSEL_A::INPUT_6_RISING_EDGES,
            13 => CKSEL_A::INPUT_6_FALLING_EDGES,
            14 => CKSEL_A::INPUT_7_RISING_EDGES,
            15 => CKSEL_A::INPUT_7_FALLING_EDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT_0_RISING_EDGES`"]
    #[inline(always)]
    pub fn is_input_0_rising_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_0_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_0_FALLING_EDGES`"]
    #[inline(always)]
    pub fn is_input_0_falling_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_0_FALLING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_1_RISING_EDGES`"]
    #[inline(always)]
    pub fn is_input_1_rising_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_1_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_1_FALLING_EDGES`"]
    #[inline(always)]
    pub fn is_input_1_falling_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_1_FALLING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_2_RISING_EDGES`"]
    #[inline(always)]
    pub fn is_input_2_rising_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_2_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_2_FALLING_EDGES`"]
    #[inline(always)]
    pub fn is_input_2_falling_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_2_FALLING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_3_RISING_EDGES`"]
    #[inline(always)]
    pub fn is_input_3_rising_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_3_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_3_FALLING_EDGES`"]
    #[inline(always)]
    pub fn is_input_3_falling_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_3_FALLING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_4_RISING_EDGES`"]
    #[inline(always)]
    pub fn is_input_4_rising_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_4_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_4_FALLING_EDGES`"]
    #[inline(always)]
    pub fn is_input_4_falling_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_4_FALLING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_5_RISING_EDGES`"]
    #[inline(always)]
    pub fn is_input_5_rising_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_5_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_5_FALLING_EDGES`"]
    #[inline(always)]
    pub fn is_input_5_falling_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_5_FALLING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_6_RISING_EDGES`"]
    #[inline(always)]
    pub fn is_input_6_rising_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_6_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_6_FALLING_EDGES`"]
    #[inline(always)]
    pub fn is_input_6_falling_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_6_FALLING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_7_RISING_EDGES`"]
    #[inline(always)]
    pub fn is_input_7_rising_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_7_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_7_FALLING_EDGES`"]
    #[inline(always)]
    pub fn is_input_7_falling_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_7_FALLING_EDGES
    }
}
#[doc = "Field `CKSEL` writer - SCT Clock Select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register."]
pub type CKSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CONFIG_SPEC, u8, CKSEL_A, 4, O>;
impl<'a, const O: u8> CKSEL_W<'a, O> {
    #[doc = "Rising edges on input 0"]
    #[inline(always)]
    pub fn input_0_rising_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_0_RISING_EDGES)
    }
    #[doc = "Falling edges on input 0"]
    #[inline(always)]
    pub fn input_0_falling_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_0_FALLING_EDGES)
    }
    #[doc = "Rising edges on input 1"]
    #[inline(always)]
    pub fn input_1_rising_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_1_RISING_EDGES)
    }
    #[doc = "Falling edges on input 1"]
    #[inline(always)]
    pub fn input_1_falling_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_1_FALLING_EDGES)
    }
    #[doc = "Rising edges on input 2"]
    #[inline(always)]
    pub fn input_2_rising_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_2_RISING_EDGES)
    }
    #[doc = "Falling edges on input 2"]
    #[inline(always)]
    pub fn input_2_falling_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_2_FALLING_EDGES)
    }
    #[doc = "Rising edges on input 3"]
    #[inline(always)]
    pub fn input_3_rising_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_3_RISING_EDGES)
    }
    #[doc = "Falling edges on input 3"]
    #[inline(always)]
    pub fn input_3_falling_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_3_FALLING_EDGES)
    }
    #[doc = "Rising edges on input 4"]
    #[inline(always)]
    pub fn input_4_rising_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_4_RISING_EDGES)
    }
    #[doc = "Falling edges on input 4"]
    #[inline(always)]
    pub fn input_4_falling_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_4_FALLING_EDGES)
    }
    #[doc = "Rising edges on input 5"]
    #[inline(always)]
    pub fn input_5_rising_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_5_RISING_EDGES)
    }
    #[doc = "Falling edges on input 5"]
    #[inline(always)]
    pub fn input_5_falling_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_5_FALLING_EDGES)
    }
    #[doc = "Rising edges on input 6"]
    #[inline(always)]
    pub fn input_6_rising_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_6_RISING_EDGES)
    }
    #[doc = "Falling edges on input 6"]
    #[inline(always)]
    pub fn input_6_falling_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_6_FALLING_EDGES)
    }
    #[doc = "Rising edges on input 7"]
    #[inline(always)]
    pub fn input_7_rising_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_7_RISING_EDGES)
    }
    #[doc = "Falling edges on input 7"]
    #[inline(always)]
    pub fn input_7_falling_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_7_FALLING_EDGES)
    }
}
#[doc = "Field `NORELOAD_L` reader - No Reload Lower Match"]
pub type NORELOAD_L_R = crate::BitReader<NORELOAD_L_A>;
#[doc = "No Reload Lower Match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NORELOAD_L_A {
    #[doc = "0: Reload. The default setting."]
    RELOAD = 0,
    #[doc = "1: No Reload. Prevents the lower match registers from being reloaded from their respective reload registers."]
    NO_RELOAD = 1,
}
impl From<NORELOAD_L_A> for bool {
    #[inline(always)]
    fn from(variant: NORELOAD_L_A) -> Self {
        variant as u8 != 0
    }
}
impl NORELOAD_L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NORELOAD_L_A {
        match self.bits {
            false => NORELOAD_L_A::RELOAD,
            true => NORELOAD_L_A::NO_RELOAD,
        }
    }
    #[doc = "Checks if the value of the field is `RELOAD`"]
    #[inline(always)]
    pub fn is_reload(&self) -> bool {
        *self == NORELOAD_L_A::RELOAD
    }
    #[doc = "Checks if the value of the field is `NO_RELOAD`"]
    #[inline(always)]
    pub fn is_no_reload(&self) -> bool {
        *self == NORELOAD_L_A::NO_RELOAD
    }
}
#[doc = "Field `NORELOAD_L` writer - No Reload Lower Match"]
pub type NORELOAD_L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, NORELOAD_L_A, O>;
impl<'a, const O: u8> NORELOAD_L_W<'a, O> {
    #[doc = "Reload. The default setting."]
    #[inline(always)]
    pub fn reload(self) -> &'a mut W {
        self.variant(NORELOAD_L_A::RELOAD)
    }
    #[doc = "No Reload. Prevents the lower match registers from being reloaded from their respective reload registers."]
    #[inline(always)]
    pub fn no_reload(self) -> &'a mut W {
        self.variant(NORELOAD_L_A::NO_RELOAD)
    }
}
#[doc = "Field `NORELOAD_H` reader - No Reload Higher Match"]
pub type NORELOAD_H_R = crate::BitReader<NORELOAD_H_A>;
#[doc = "No Reload Higher Match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NORELOAD_H_A {
    #[doc = "0: Reload. The default setting."]
    RELOAD_H = 0,
    #[doc = "1: No Reload. Prevents the higher match registers from being reloaded from their respective reload registers."]
    NO_RELOAD_H = 1,
}
impl From<NORELOAD_H_A> for bool {
    #[inline(always)]
    fn from(variant: NORELOAD_H_A) -> Self {
        variant as u8 != 0
    }
}
impl NORELOAD_H_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NORELOAD_H_A {
        match self.bits {
            false => NORELOAD_H_A::RELOAD_H,
            true => NORELOAD_H_A::NO_RELOAD_H,
        }
    }
    #[doc = "Checks if the value of the field is `RELOAD_H`"]
    #[inline(always)]
    pub fn is_reload_h(&self) -> bool {
        *self == NORELOAD_H_A::RELOAD_H
    }
    #[doc = "Checks if the value of the field is `NO_RELOAD_H`"]
    #[inline(always)]
    pub fn is_no_reload_h(&self) -> bool {
        *self == NORELOAD_H_A::NO_RELOAD_H
    }
}
#[doc = "Field `NORELOAD_H` writer - No Reload Higher Match"]
pub type NORELOAD_H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, NORELOAD_H_A, O>;
impl<'a, const O: u8> NORELOAD_H_W<'a, O> {
    #[doc = "Reload. The default setting."]
    #[inline(always)]
    pub fn reload_h(self) -> &'a mut W {
        self.variant(NORELOAD_H_A::RELOAD_H)
    }
    #[doc = "No Reload. Prevents the higher match registers from being reloaded from their respective reload registers."]
    #[inline(always)]
    pub fn no_reload_h(self) -> &'a mut W {
        self.variant(NORELOAD_H_A::NO_RELOAD_H)
    }
}
#[doc = "Field `INSYNC` reader - Input Synchronization"]
pub type INSYNC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSYNC` writer - Input Synchronization"]
pub type INSYNC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `AUTOLIMIT_L` reader - Auto Limit Lower"]
pub type AUTOLIMIT_L_R = crate::BitReader<AUTOLIMIT_L_A>;
#[doc = "Auto Limit Lower\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOLIMIT_L_A {
    #[doc = "0: Disable."]
    DISABLE = 0,
    #[doc = "1: Enable. A match on match register 0 is the LIMIT condition. No need to define an associated event."]
    ENABLE = 1,
}
impl From<AUTOLIMIT_L_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOLIMIT_L_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTOLIMIT_L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOLIMIT_L_A {
        match self.bits {
            false => AUTOLIMIT_L_A::DISABLE,
            true => AUTOLIMIT_L_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AUTOLIMIT_L_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AUTOLIMIT_L_A::ENABLE
    }
}
#[doc = "Field `AUTOLIMIT_L` writer - Auto Limit Lower"]
pub type AUTOLIMIT_L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, AUTOLIMIT_L_A, O>;
impl<'a, const O: u8> AUTOLIMIT_L_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AUTOLIMIT_L_A::DISABLE)
    }
    #[doc = "Enable. A match on match register 0 is the LIMIT condition. No need to define an associated event."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AUTOLIMIT_L_A::ENABLE)
    }
}
#[doc = "Field `AUTOLIMIT_H` reader - Auto Limit Higher"]
pub type AUTOLIMIT_H_R = crate::BitReader<AUTOLIMIT_H_A>;
#[doc = "Auto Limit Higher\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOLIMIT_H_A {
    #[doc = "0: Disable."]
    DISABLE = 0,
    #[doc = "1: Enable. A match on match register 0 is the LIMIT condition. No need to define an associated event."]
    ENABLE = 1,
}
impl From<AUTOLIMIT_H_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOLIMIT_H_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTOLIMIT_H_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOLIMIT_H_A {
        match self.bits {
            false => AUTOLIMIT_H_A::DISABLE,
            true => AUTOLIMIT_H_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AUTOLIMIT_H_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AUTOLIMIT_H_A::ENABLE
    }
}
#[doc = "Field `AUTOLIMIT_H` writer - Auto Limit Higher"]
pub type AUTOLIMIT_H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, AUTOLIMIT_H_A, O>;
impl<'a, const O: u8> AUTOLIMIT_H_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AUTOLIMIT_H_A::DISABLE)
    }
    #[doc = "Enable. A match on match register 0 is the LIMIT condition. No need to define an associated event."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AUTOLIMIT_H_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - SCT Operation"]
    #[inline(always)]
    pub fn unify(&self) -> UNIFY_R {
        UNIFY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - SCT Clock Mode"]
    #[inline(always)]
    pub fn clkmode(&self) -> CLKMODE_R {
        CLKMODE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:6 - SCT Clock Select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register."]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - No Reload Lower Match"]
    #[inline(always)]
    pub fn noreload_l(&self) -> NORELOAD_L_R {
        NORELOAD_L_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - No Reload Higher Match"]
    #[inline(always)]
    pub fn noreload_h(&self) -> NORELOAD_H_R {
        NORELOAD_H_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:16 - Input Synchronization"]
    #[inline(always)]
    pub fn insync(&self) -> INSYNC_R {
        INSYNC_R::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bit 17 - Auto Limit Lower"]
    #[inline(always)]
    pub fn autolimit_l(&self) -> AUTOLIMIT_L_R {
        AUTOLIMIT_L_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Auto Limit Higher"]
    #[inline(always)]
    pub fn autolimit_h(&self) -> AUTOLIMIT_H_R {
        AUTOLIMIT_H_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCT Operation"]
    #[inline(always)]
    #[must_use]
    pub fn unify(&mut self) -> UNIFY_W<0> {
        UNIFY_W::new(self)
    }
    #[doc = "Bits 1:2 - SCT Clock Mode"]
    #[inline(always)]
    #[must_use]
    pub fn clkmode(&mut self) -> CLKMODE_W<1> {
        CLKMODE_W::new(self)
    }
    #[doc = "Bits 3:6 - SCT Clock Select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register."]
    #[inline(always)]
    #[must_use]
    pub fn cksel(&mut self) -> CKSEL_W<3> {
        CKSEL_W::new(self)
    }
    #[doc = "Bit 7 - No Reload Lower Match"]
    #[inline(always)]
    #[must_use]
    pub fn noreload_l(&mut self) -> NORELOAD_L_W<7> {
        NORELOAD_L_W::new(self)
    }
    #[doc = "Bit 8 - No Reload Higher Match"]
    #[inline(always)]
    #[must_use]
    pub fn noreload_h(&mut self) -> NORELOAD_H_W<8> {
        NORELOAD_H_W::new(self)
    }
    #[doc = "Bits 9:16 - Input Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn insync(&mut self) -> INSYNC_W<9> {
        INSYNC_W::new(self)
    }
    #[doc = "Bit 17 - Auto Limit Lower"]
    #[inline(always)]
    #[must_use]
    pub fn autolimit_l(&mut self) -> AUTOLIMIT_L_W<17> {
        AUTOLIMIT_L_W::new(self)
    }
    #[doc = "Bit 18 - Auto Limit Higher"]
    #[inline(always)]
    #[must_use]
    pub fn autolimit_h(&mut self) -> AUTOLIMIT_H_W<18> {
        AUTOLIMIT_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCTimer Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0x0001_fe00"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_fe00;
}
