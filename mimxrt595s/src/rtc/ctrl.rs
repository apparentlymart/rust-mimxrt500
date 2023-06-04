#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRESET` reader - Software Reset Control"]
pub type SWRESET_R = crate::BitReader<SWRESET_A>;
#[doc = "Software Reset Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRESET_A {
    #[doc = "0: Not in reset. The RTC is not held in reset. Clear SWRESET before configuring or initiating any operation of the RTC."]
    NOT_IN_RESET = 0,
    #[doc = "1: In reset"]
    IN_RESET = 1,
}
impl From<SWRESET_A> for bool {
    #[inline(always)]
    fn from(variant: SWRESET_A) -> Self {
        variant as u8 != 0
    }
}
impl SWRESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRESET_A {
        match self.bits {
            false => SWRESET_A::NOT_IN_RESET,
            true => SWRESET_A::IN_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_IN_RESET`"]
    #[inline(always)]
    pub fn is_not_in_reset(&self) -> bool {
        *self == SWRESET_A::NOT_IN_RESET
    }
    #[doc = "Checks if the value of the field is `IN_RESET`"]
    #[inline(always)]
    pub fn is_in_reset(&self) -> bool {
        *self == SWRESET_A::IN_RESET
    }
}
#[doc = "Field `SWRESET` writer - Software Reset Control"]
pub type SWRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, SWRESET_A, O>;
impl<'a, const O: u8> SWRESET_W<'a, O> {
    #[doc = "Not in reset. The RTC is not held in reset. Clear SWRESET before configuring or initiating any operation of the RTC."]
    #[inline(always)]
    pub fn not_in_reset(self) -> &'a mut W {
        self.variant(SWRESET_A::NOT_IN_RESET)
    }
    #[doc = "In reset"]
    #[inline(always)]
    pub fn in_reset(self) -> &'a mut W {
        self.variant(SWRESET_A::IN_RESET)
    }
}
#[doc = "Field `ALARM1HZ` reader - RTC 1 Hz Timer Alarm Flag Status"]
pub type ALARM1HZ_R = crate::BitReader<ALARM1HZ_A>;
#[doc = "RTC 1 Hz Timer Alarm Flag Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALARM1HZ_A {
    #[doc = "0: No match. No match condition has occurred on the 1 Hz RTC timer. Writing a 0 has no effect."]
    NO_MATCH = 0,
    #[doc = "1: Match"]
    MATCH = 1,
}
impl From<ALARM1HZ_A> for bool {
    #[inline(always)]
    fn from(variant: ALARM1HZ_A) -> Self {
        variant as u8 != 0
    }
}
impl ALARM1HZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALARM1HZ_A {
        match self.bits {
            false => ALARM1HZ_A::NO_MATCH,
            true => ALARM1HZ_A::MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `NO_MATCH`"]
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == ALARM1HZ_A::NO_MATCH
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ALARM1HZ_A::MATCH
    }
}
#[doc = "Field `ALARM1HZ` writer - RTC 1 Hz Timer Alarm Flag Status"]
pub type ALARM1HZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, ALARM1HZ_A, O>;
impl<'a, const O: u8> ALARM1HZ_W<'a, O> {
    #[doc = "No match. No match condition has occurred on the 1 Hz RTC timer. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn no_match(self) -> &'a mut W {
        self.variant(ALARM1HZ_A::NO_MATCH)
    }
    #[doc = "Match"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(ALARM1HZ_A::MATCH)
    }
}
#[doc = "Field `WAKE1KHZ` reader - RTC 1 kHz Timer Wake-up Flag Status"]
pub type WAKE1KHZ_R = crate::BitReader<WAKE1KHZ_A>;
#[doc = "RTC 1 kHz Timer Wake-up Flag Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKE1KHZ_A {
    #[doc = "0: Run. The RTC 1 kHz timer is running. Writing a 0 has no effect."]
    RUN = 0,
    #[doc = "1: Time-out. The 1 kHz high-resolution/wake-up timer has timed out. This flag generates an RTC wake-up interrupt request RTC-WAKE which can also wake up the device from any low power mode. Write 1 to clear."]
    TIME_OUT = 1,
}
impl From<WAKE1KHZ_A> for bool {
    #[inline(always)]
    fn from(variant: WAKE1KHZ_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKE1KHZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKE1KHZ_A {
        match self.bits {
            false => WAKE1KHZ_A::RUN,
            true => WAKE1KHZ_A::TIME_OUT,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == WAKE1KHZ_A::RUN
    }
    #[doc = "Checks if the value of the field is `TIME_OUT`"]
    #[inline(always)]
    pub fn is_time_out(&self) -> bool {
        *self == WAKE1KHZ_A::TIME_OUT
    }
}
#[doc = "Field `WAKE1KHZ` writer - RTC 1 kHz Timer Wake-up Flag Status"]
pub type WAKE1KHZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, WAKE1KHZ_A, O>;
impl<'a, const O: u8> WAKE1KHZ_W<'a, O> {
    #[doc = "Run. The RTC 1 kHz timer is running. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(WAKE1KHZ_A::RUN)
    }
    #[doc = "Time-out. The 1 kHz high-resolution/wake-up timer has timed out. This flag generates an RTC wake-up interrupt request RTC-WAKE which can also wake up the device from any low power mode. Write 1 to clear."]
    #[inline(always)]
    pub fn time_out(self) -> &'a mut W {
        self.variant(WAKE1KHZ_A::TIME_OUT)
    }
}
#[doc = "Field `ALARMDPD_EN` reader - RTC 1 Hz Timer Alarm Enable for Deep Power-down"]
pub type ALARMDPD_EN_R = crate::BitReader<ALARMDPD_EN_A>;
#[doc = "RTC 1 Hz Timer Alarm Enable for Deep Power-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALARMDPD_EN_A {
    #[doc = "0: Disable. A match on the 1 Hz RTC timer does not bring the device out of Deep Power-down mode."]
    DISABLE = 0,
    #[doc = "1: Enable. A match on the 1 Hz RTC timer brings the device out of Deep Power-down mode."]
    ENABLE = 1,
}
impl From<ALARMDPD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ALARMDPD_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ALARMDPD_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALARMDPD_EN_A {
        match self.bits {
            false => ALARMDPD_EN_A::DISABLE,
            true => ALARMDPD_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ALARMDPD_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ALARMDPD_EN_A::ENABLE
    }
}
#[doc = "Field `ALARMDPD_EN` writer - RTC 1 Hz Timer Alarm Enable for Deep Power-down"]
pub type ALARMDPD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, ALARMDPD_EN_A, O>;
impl<'a, const O: u8> ALARMDPD_EN_W<'a, O> {
    #[doc = "Disable. A match on the 1 Hz RTC timer does not bring the device out of Deep Power-down mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ALARMDPD_EN_A::DISABLE)
    }
    #[doc = "Enable. A match on the 1 Hz RTC timer brings the device out of Deep Power-down mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ALARMDPD_EN_A::ENABLE)
    }
}
#[doc = "Field `WAKEDPD_EN` reader - RTC 1 kHz Timer Wake-up Enable for Deep Power-down"]
pub type WAKEDPD_EN_R = crate::BitReader<WAKEDPD_EN_A>;
#[doc = "RTC 1 kHz Timer Wake-up Enable for Deep Power-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKEDPD_EN_A {
    #[doc = "0: Disable. A match on the 1 kHz RTC timer does not bring the device out of Deep Power-down mode."]
    DISABLE = 0,
    #[doc = "1: Enable. A match on the 1 kHz RTC timer brings the device out of Deep Power-down mode."]
    ENABLE = 1,
}
impl From<WAKEDPD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEDPD_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKEDPD_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEDPD_EN_A {
        match self.bits {
            false => WAKEDPD_EN_A::DISABLE,
            true => WAKEDPD_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WAKEDPD_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WAKEDPD_EN_A::ENABLE
    }
}
#[doc = "Field `WAKEDPD_EN` writer - RTC 1 kHz Timer Wake-up Enable for Deep Power-down"]
pub type WAKEDPD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, WAKEDPD_EN_A, O>;
impl<'a, const O: u8> WAKEDPD_EN_W<'a, O> {
    #[doc = "Disable. A match on the 1 kHz RTC timer does not bring the device out of Deep Power-down mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WAKEDPD_EN_A::DISABLE)
    }
    #[doc = "Enable. A match on the 1 kHz RTC timer brings the device out of Deep Power-down mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WAKEDPD_EN_A::ENABLE)
    }
}
#[doc = "Field `RTC1KHZ_EN` reader - RTC 1 kHz Clock Enable"]
pub type RTC1KHZ_EN_R = crate::BitReader<RTC1KHZ_EN_A>;
#[doc = "RTC 1 kHz Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC1KHZ_EN_A {
    #[doc = "0: Disable. A match on the 1 kHz RTC timer does not bring the device out of Deep Power-down mode."]
    DISABLE = 0,
    #[doc = "1: Enable. The 1 kHz RTC timer is enabled."]
    ENABLE = 1,
}
impl From<RTC1KHZ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RTC1KHZ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC1KHZ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC1KHZ_EN_A {
        match self.bits {
            false => RTC1KHZ_EN_A::DISABLE,
            true => RTC1KHZ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RTC1KHZ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTC1KHZ_EN_A::ENABLE
    }
}
#[doc = "Field `RTC1KHZ_EN` writer - RTC 1 kHz Clock Enable"]
pub type RTC1KHZ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, RTC1KHZ_EN_A, O>;
impl<'a, const O: u8> RTC1KHZ_EN_W<'a, O> {
    #[doc = "Disable. A match on the 1 kHz RTC timer does not bring the device out of Deep Power-down mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTC1KHZ_EN_A::DISABLE)
    }
    #[doc = "Enable. The 1 kHz RTC timer is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTC1KHZ_EN_A::ENABLE)
    }
}
#[doc = "Field `RTC_EN` reader - RTC enable"]
pub type RTC_EN_R = crate::BitReader<RTC_EN_A>;
#[doc = "RTC enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_EN_A {
    #[doc = "0: Disable. The RTC 1 Hz and 1 kHz clocks are shut down and the RTC operation is disabled. RTC_EN should be 0 when writing to load a value in the RTC counter (COUNT) register ."]
    DISABLE = 0,
    #[doc = "1: Enable. The 1 Hz RTC clock is running and RTC operation is enabled. The first clock to the RTC counter occurs 1 s after RTC_EN is set. To also enable the high-resolution, 1 kHz clock, set CTRL\\[RTC1KHZ_EN\\]
= 1."]
    ENABLE = 1,
}
impl From<RTC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_EN_A {
        match self.bits {
            false => RTC_EN_A::DISABLE,
            true => RTC_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RTC_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTC_EN_A::ENABLE
    }
}
#[doc = "Field `RTC_EN` writer - RTC enable"]
pub type RTC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, RTC_EN_A, O>;
impl<'a, const O: u8> RTC_EN_W<'a, O> {
    #[doc = "Disable. The RTC 1 Hz and 1 kHz clocks are shut down and the RTC operation is disabled. RTC_EN should be 0 when writing to load a value in the RTC counter (COUNT) register ."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTC_EN_A::DISABLE)
    }
    #[doc = "Enable. The 1 Hz RTC clock is running and RTC operation is enabled. The first clock to the RTC counter occurs 1 s after RTC_EN is set. To also enable the high-resolution, 1 kHz clock, set CTRL\\[RTC1KHZ_EN\\]
= 1."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTC_EN_A::ENABLE)
    }
}
#[doc = "Field `RTC_OSC_PD` reader - The RTC Oscillator Enable"]
pub type RTC_OSC_PD_R = crate::BitReader<RTC_OSC_PD_A>;
#[doc = "The RTC Oscillator Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_OSC_PD_A {
    #[doc = "0: Enable. The RTC oscillator is enabled. RTC_OSC_PD must be cleared for the RTC module to function."]
    ENABLE = 0,
    #[doc = "1: Shut Off. The RTC operation is disabled. The RTC oscillator is shut-off to limit power consumption."]
    SHUT_OFF = 1,
}
impl From<RTC_OSC_PD_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_OSC_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC_OSC_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_OSC_PD_A {
        match self.bits {
            false => RTC_OSC_PD_A::ENABLE,
            true => RTC_OSC_PD_A::SHUT_OFF,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTC_OSC_PD_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `SHUT_OFF`"]
    #[inline(always)]
    pub fn is_shut_off(&self) -> bool {
        *self == RTC_OSC_PD_A::SHUT_OFF
    }
}
#[doc = "Field `RTC_OSC_PD` writer - The RTC Oscillator Enable"]
pub type RTC_OSC_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, RTC_OSC_PD_A, O>;
impl<'a, const O: u8> RTC_OSC_PD_W<'a, O> {
    #[doc = "Enable. The RTC oscillator is enabled. RTC_OSC_PD must be cleared for the RTC module to function."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTC_OSC_PD_A::ENABLE)
    }
    #[doc = "Shut Off. The RTC operation is disabled. The RTC oscillator is shut-off to limit power consumption."]
    #[inline(always)]
    pub fn shut_off(self) -> &'a mut W {
        self.variant(RTC_OSC_PD_A::SHUT_OFF)
    }
}
#[doc = "Field `RTC_SUBSEC_ENA` reader - 32-KHz Sub-second Counter Enable"]
pub type RTC_SUBSEC_ENA_R = crate::BitReader<RTC_SUBSEC_ENA_A>;
#[doc = "32-KHz Sub-second Counter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_SUBSEC_ENA_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<RTC_SUBSEC_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_SUBSEC_ENA_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC_SUBSEC_ENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_SUBSEC_ENA_A {
        match self.bits {
            false => RTC_SUBSEC_ENA_A::DISABLE,
            true => RTC_SUBSEC_ENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RTC_SUBSEC_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTC_SUBSEC_ENA_A::ENABLE
    }
}
#[doc = "Field `RTC_SUBSEC_ENA` writer - 32-KHz Sub-second Counter Enable"]
pub type RTC_SUBSEC_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_SPEC, RTC_SUBSEC_ENA_A, O>;
impl<'a, const O: u8> RTC_SUBSEC_ENA_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTC_SUBSEC_ENA_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTC_SUBSEC_ENA_A::ENABLE)
    }
}
#[doc = "Field `RTC_OSC_loadcap` reader - Capacitive Load Selection"]
pub type RTC_OSC_LOADCAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_OSC_loadcap` writer - Capacitive Load Selection"]
pub type RTC_OSC_LOADCAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Software Reset Control"]
    #[inline(always)]
    pub fn swreset(&self) -> SWRESET_R {
        SWRESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - RTC 1 Hz Timer Alarm Flag Status"]
    #[inline(always)]
    pub fn alarm1hz(&self) -> ALARM1HZ_R {
        ALARM1HZ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC 1 kHz Timer Wake-up Flag Status"]
    #[inline(always)]
    pub fn wake1khz(&self) -> WAKE1KHZ_R {
        WAKE1KHZ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC 1 Hz Timer Alarm Enable for Deep Power-down"]
    #[inline(always)]
    pub fn alarmdpd_en(&self) -> ALARMDPD_EN_R {
        ALARMDPD_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC 1 kHz Timer Wake-up Enable for Deep Power-down"]
    #[inline(always)]
    pub fn wakedpd_en(&self) -> WAKEDPD_EN_R {
        WAKEDPD_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTC 1 kHz Clock Enable"]
    #[inline(always)]
    pub fn rtc1khz_en(&self) -> RTC1KHZ_EN_R {
        RTC1KHZ_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RTC enable"]
    #[inline(always)]
    pub fn rtc_en(&self) -> RTC_EN_R {
        RTC_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The RTC Oscillator Enable"]
    #[inline(always)]
    pub fn rtc_osc_pd(&self) -> RTC_OSC_PD_R {
        RTC_OSC_PD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - 32-KHz Sub-second Counter Enable"]
    #[inline(always)]
    pub fn rtc_subsec_ena(&self) -> RTC_SUBSEC_ENA_R {
        RTC_SUBSEC_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Capacitive Load Selection"]
    #[inline(always)]
    pub fn rtc_osc_loadcap(&self) -> RTC_OSC_LOADCAP_R {
        RTC_OSC_LOADCAP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset Control"]
    #[inline(always)]
    #[must_use]
    pub fn swreset(&mut self) -> SWRESET_W<0> {
        SWRESET_W::new(self)
    }
    #[doc = "Bit 2 - RTC 1 Hz Timer Alarm Flag Status"]
    #[inline(always)]
    #[must_use]
    pub fn alarm1hz(&mut self) -> ALARM1HZ_W<2> {
        ALARM1HZ_W::new(self)
    }
    #[doc = "Bit 3 - RTC 1 kHz Timer Wake-up Flag Status"]
    #[inline(always)]
    #[must_use]
    pub fn wake1khz(&mut self) -> WAKE1KHZ_W<3> {
        WAKE1KHZ_W::new(self)
    }
    #[doc = "Bit 4 - RTC 1 Hz Timer Alarm Enable for Deep Power-down"]
    #[inline(always)]
    #[must_use]
    pub fn alarmdpd_en(&mut self) -> ALARMDPD_EN_W<4> {
        ALARMDPD_EN_W::new(self)
    }
    #[doc = "Bit 5 - RTC 1 kHz Timer Wake-up Enable for Deep Power-down"]
    #[inline(always)]
    #[must_use]
    pub fn wakedpd_en(&mut self) -> WAKEDPD_EN_W<5> {
        WAKEDPD_EN_W::new(self)
    }
    #[doc = "Bit 6 - RTC 1 kHz Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtc1khz_en(&mut self) -> RTC1KHZ_EN_W<6> {
        RTC1KHZ_EN_W::new(self)
    }
    #[doc = "Bit 7 - RTC enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_en(&mut self) -> RTC_EN_W<7> {
        RTC_EN_W::new(self)
    }
    #[doc = "Bit 8 - The RTC Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_osc_pd(&mut self) -> RTC_OSC_PD_W<8> {
        RTC_OSC_PD_W::new(self)
    }
    #[doc = "Bit 10 - 32-KHz Sub-second Counter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_subsec_ena(&mut self) -> RTC_SUBSEC_ENA_W<10> {
        RTC_SUBSEC_ENA_W::new(self)
    }
    #[doc = "Bits 28:31 - Capacitive Load Selection"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_osc_loadcap(&mut self) -> RTC_OSC_LOADCAP_W<28> {
        RTC_OSC_LOADCAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x1000_0103"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000_0103;
}
