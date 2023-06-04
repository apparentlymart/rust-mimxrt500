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
#[doc = "Field `ADCEN` reader - ADC Enable"]
pub type ADCEN_R = crate::BitReader<ADCEN_A>;
#[doc = "ADC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCEN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADCEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCEN_A {
        match self.bits {
            false => ADCEN_A::DISABLED,
            true => ADCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADCEN_A::ENABLED
    }
}
#[doc = "Field `ADCEN` writer - ADC Enable"]
pub type ADCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, ADCEN_A, O>;
impl<'a, const O: u8> ADCEN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCEN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADCEN_A::ENABLED)
    }
}
#[doc = "Field `RST` reader - Software Reset"]
pub type RST_R = crate::BitReader<RST_A>;
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RST_A {
    #[doc = "0: ADC logic is not reset."]
    RELEASED_FROM_RESET = 0,
    #[doc = "1: ADC logic is reset."]
    HELD_IN_RESET = 1,
}
impl From<RST_A> for bool {
    #[inline(always)]
    fn from(variant: RST_A) -> Self {
        variant as u8 != 0
    }
}
impl RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RST_A {
        match self.bits {
            false => RST_A::RELEASED_FROM_RESET,
            true => RST_A::HELD_IN_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED_FROM_RESET`"]
    #[inline(always)]
    pub fn is_released_from_reset(&self) -> bool {
        *self == RST_A::RELEASED_FROM_RESET
    }
    #[doc = "Checks if the value of the field is `HELD_IN_RESET`"]
    #[inline(always)]
    pub fn is_held_in_reset(&self) -> bool {
        *self == RST_A::HELD_IN_RESET
    }
}
#[doc = "Field `RST` writer - Software Reset"]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, RST_A, O>;
impl<'a, const O: u8> RST_W<'a, O> {
    #[doc = "ADC logic is not reset."]
    #[inline(always)]
    pub fn released_from_reset(self) -> &'a mut W {
        self.variant(RST_A::RELEASED_FROM_RESET)
    }
    #[doc = "ADC logic is reset."]
    #[inline(always)]
    pub fn held_in_reset(self) -> &'a mut W {
        self.variant(RST_A::HELD_IN_RESET)
    }
}
#[doc = "Field `DOZEN` reader - Doze Enable"]
pub type DOZEN_R = crate::BitReader<DOZEN_A>;
#[doc = "Doze Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOZEN_A {
    #[doc = "0: ADC is enabled in low-power mode."]
    ENABLED = 0,
    #[doc = "1: ADC is disabled in low-power mode."]
    DISABLED = 1,
}
impl From<DOZEN_A> for bool {
    #[inline(always)]
    fn from(variant: DOZEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DOZEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOZEN_A {
        match self.bits {
            false => DOZEN_A::ENABLED,
            true => DOZEN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DOZEN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DOZEN_A::DISABLED
    }
}
#[doc = "Field `DOZEN` writer - Doze Enable"]
pub type DOZEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, DOZEN_A, O>;
impl<'a, const O: u8> DOZEN_W<'a, O> {
    #[doc = "ADC is enabled in low-power mode."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DOZEN_A::ENABLED)
    }
    #[doc = "ADC is disabled in low-power mode."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DOZEN_A::DISABLED)
    }
}
#[doc = "Field `RSTFIFO0` reader - Reset FIFO 0"]
pub type RSTFIFO0_R = crate::BitReader<RSTFIFO0_A>;
#[doc = "Reset FIFO 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTFIFO0_A {
    #[doc = "0: No effect."]
    NO_ACTION = 0,
    #[doc = "1: FIFO 0 is reset."]
    TRIGGER_RESET = 1,
}
impl From<RSTFIFO0_A> for bool {
    #[inline(always)]
    fn from(variant: RSTFIFO0_A) -> Self {
        variant as u8 != 0
    }
}
impl RSTFIFO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTFIFO0_A {
        match self.bits {
            false => RSTFIFO0_A::NO_ACTION,
            true => RSTFIFO0_A::TRIGGER_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACTION`"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == RSTFIFO0_A::NO_ACTION
    }
    #[doc = "Checks if the value of the field is `TRIGGER_RESET`"]
    #[inline(always)]
    pub fn is_trigger_reset(&self) -> bool {
        *self == RSTFIFO0_A::TRIGGER_RESET
    }
}
#[doc = "Field `RSTFIFO0` writer - Reset FIFO 0"]
pub type RSTFIFO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, RSTFIFO0_A, O>;
impl<'a, const O: u8> RSTFIFO0_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(RSTFIFO0_A::NO_ACTION)
    }
    #[doc = "FIFO 0 is reset."]
    #[inline(always)]
    pub fn trigger_reset(self) -> &'a mut W {
        self.variant(RSTFIFO0_A::TRIGGER_RESET)
    }
}
#[doc = "Field `RSTFIFO1` reader - Reset FIFO 1"]
pub type RSTFIFO1_R = crate::BitReader<RSTFIFO1_A>;
#[doc = "Reset FIFO 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTFIFO1_A {
    #[doc = "0: No effect."]
    NO_ACTION = 0,
    #[doc = "1: FIFO 1 is reset."]
    TRIGGER_RESET = 1,
}
impl From<RSTFIFO1_A> for bool {
    #[inline(always)]
    fn from(variant: RSTFIFO1_A) -> Self {
        variant as u8 != 0
    }
}
impl RSTFIFO1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTFIFO1_A {
        match self.bits {
            false => RSTFIFO1_A::NO_ACTION,
            true => RSTFIFO1_A::TRIGGER_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACTION`"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == RSTFIFO1_A::NO_ACTION
    }
    #[doc = "Checks if the value of the field is `TRIGGER_RESET`"]
    #[inline(always)]
    pub fn is_trigger_reset(&self) -> bool {
        *self == RSTFIFO1_A::TRIGGER_RESET
    }
}
#[doc = "Field `RSTFIFO1` writer - Reset FIFO 1"]
pub type RSTFIFO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, RSTFIFO1_A, O>;
impl<'a, const O: u8> RSTFIFO1_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(RSTFIFO1_A::NO_ACTION)
    }
    #[doc = "FIFO 1 is reset."]
    #[inline(always)]
    pub fn trigger_reset(self) -> &'a mut W {
        self.variant(RSTFIFO1_A::TRIGGER_RESET)
    }
}
impl R {
    #[doc = "Bit 0 - ADC Enable"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Doze Enable"]
    #[inline(always)]
    pub fn dozen(&self) -> DOZEN_R {
        DOZEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Reset FIFO 0"]
    #[inline(always)]
    pub fn rstfifo0(&self) -> RSTFIFO0_R {
        RSTFIFO0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reset FIFO 1"]
    #[inline(always)]
    pub fn rstfifo1(&self) -> RSTFIFO1_R {
        RSTFIFO1_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> ADCEN_W<0> {
        ADCEN_W::new(self)
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<1> {
        RST_W::new(self)
    }
    #[doc = "Bit 2 - Doze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dozen(&mut self) -> DOZEN_W<2> {
        DOZEN_W::new(self)
    }
    #[doc = "Bit 8 - Reset FIFO 0"]
    #[inline(always)]
    #[must_use]
    pub fn rstfifo0(&mut self) -> RSTFIFO0_W<8> {
        RSTFIFO0_W::new(self)
    }
    #[doc = "Bit 9 - Reset FIFO 1"]
    #[inline(always)]
    #[must_use]
    pub fn rstfifo1(&mut self) -> RSTFIFO1_W<9> {
        RSTFIFO1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
