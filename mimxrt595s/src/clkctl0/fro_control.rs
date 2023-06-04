#[doc = "Register `FRO_CONTROL` reader"]
pub struct R(crate::R<FRO_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRO_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRO_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRO_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRO_CONTROL` writer"]
pub struct W(crate::W<FRO_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRO_CONTROL_SPEC>;
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
impl From<crate::W<FRO_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRO_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXP_COUNT` reader - Expected Count"]
pub type EXP_COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EXP_COUNT` writer - Expected Count"]
pub type EXP_COUNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRO_CONTROL_SPEC, u16, u16, 16, O>;
#[doc = "Field `THRESH_RANGE_UP` reader - Threshold Range Upper Limit"]
pub type THRESH_RANGE_UP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THRESH_RANGE_UP` writer - Threshold Range Upper Limit"]
pub type THRESH_RANGE_UP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRO_CONTROL_SPEC, u8, u8, 5, O>;
#[doc = "Field `THRESH_RANGE_LOW` reader - Threshold Range Lower Limit"]
pub type THRESH_RANGE_LOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THRESH_RANGE_LOW` writer - Threshold Range Lower Limit"]
pub type THRESH_RANGE_LOW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRO_CONTROL_SPEC, u8, u8, 5, O>;
#[doc = "Field `ENA_TUNE` reader - Enable Tuning"]
pub type ENA_TUNE_R = crate::BitReader<ENA_TUNE_A>;
#[doc = "Enable Tuning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENA_TUNE_A {
    #[doc = "0: Stop tuning"]
    ENA_TUNE_CLEAR = 0,
    #[doc = "1: Start tuning"]
    ENA_TUNE_START = 1,
}
impl From<ENA_TUNE_A> for bool {
    #[inline(always)]
    fn from(variant: ENA_TUNE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENA_TUNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA_TUNE_A {
        match self.bits {
            false => ENA_TUNE_A::ENA_TUNE_CLEAR,
            true => ENA_TUNE_A::ENA_TUNE_START,
        }
    }
    #[doc = "Checks if the value of the field is `ENA_TUNE_CLEAR`"]
    #[inline(always)]
    pub fn is_ena_tune_clear(&self) -> bool {
        *self == ENA_TUNE_A::ENA_TUNE_CLEAR
    }
    #[doc = "Checks if the value of the field is `ENA_TUNE_START`"]
    #[inline(always)]
    pub fn is_ena_tune_start(&self) -> bool {
        *self == ENA_TUNE_A::ENA_TUNE_START
    }
}
#[doc = "Field `ENA_TUNE` writer - Enable Tuning"]
pub type ENA_TUNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRO_CONTROL_SPEC, ENA_TUNE_A, O>;
impl<'a, const O: u8> ENA_TUNE_W<'a, O> {
    #[doc = "Stop tuning"]
    #[inline(always)]
    pub fn ena_tune_clear(self) -> &'a mut W {
        self.variant(ENA_TUNE_A::ENA_TUNE_CLEAR)
    }
    #[doc = "Start tuning"]
    #[inline(always)]
    pub fn ena_tune_start(self) -> &'a mut W {
        self.variant(ENA_TUNE_A::ENA_TUNE_START)
    }
}
impl R {
    #[doc = "Bits 0:15 - Expected Count"]
    #[inline(always)]
    pub fn exp_count(&self) -> EXP_COUNT_R {
        EXP_COUNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - Threshold Range Upper Limit"]
    #[inline(always)]
    pub fn thresh_range_up(&self) -> THRESH_RANGE_UP_R {
        THRESH_RANGE_UP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Threshold Range Lower Limit"]
    #[inline(always)]
    pub fn thresh_range_low(&self) -> THRESH_RANGE_LOW_R {
        THRESH_RANGE_LOW_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Enable Tuning"]
    #[inline(always)]
    pub fn ena_tune(&self) -> ENA_TUNE_R {
        ENA_TUNE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Expected Count"]
    #[inline(always)]
    #[must_use]
    pub fn exp_count(&mut self) -> EXP_COUNT_W<0> {
        EXP_COUNT_W::new(self)
    }
    #[doc = "Bits 16:20 - Threshold Range Upper Limit"]
    #[inline(always)]
    #[must_use]
    pub fn thresh_range_up(&mut self) -> THRESH_RANGE_UP_W<16> {
        THRESH_RANGE_UP_W::new(self)
    }
    #[doc = "Bits 21:25 - Threshold Range Lower Limit"]
    #[inline(always)]
    #[must_use]
    pub fn thresh_range_low(&mut self) -> THRESH_RANGE_LOW_W<21> {
        THRESH_RANGE_LOW_W::new(self)
    }
    #[doc = "Bit 31 - Enable Tuning"]
    #[inline(always)]
    #[must_use]
    pub fn ena_tune(&mut self) -> ENA_TUNE_W<31> {
        ENA_TUNE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Free Running Oscillator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fro_control](index.html) module"]
pub struct FRO_CONTROL_SPEC;
impl crate::RegisterSpec for FRO_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fro_control::R](R) reader structure"]
impl crate::Readable for FRO_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fro_control::W](W) writer structure"]
impl crate::Writable for FRO_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRO_CONTROL to value 0"]
impl crate::Resettable for FRO_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
