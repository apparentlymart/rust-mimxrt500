#[doc = "Register `DLLCR%s` reader"]
pub struct R(crate::R<DLLCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLLCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLLCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLLCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLLCR%s` writer"]
pub struct W(crate::W<DLLCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLLCR_SPEC>;
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
impl From<crate::W<DLLCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLLCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLLEN` reader - DLL calibration enable."]
pub type DLLEN_R = crate::BitReader<DLLEN_A>;
#[doc = "DLL calibration enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLLEN_A {
    #[doc = "0: DLL calibration is disabled"]
    VALUE0 = 0,
    #[doc = "1: DLL calibration is enabled"]
    VALUE1 = 1,
}
impl From<DLLEN_A> for bool {
    #[inline(always)]
    fn from(variant: DLLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DLLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLLEN_A {
        match self.bits {
            false => DLLEN_A::VALUE0,
            true => DLLEN_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == DLLEN_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DLLEN_A::VALUE1
    }
}
#[doc = "Field `DLLEN` writer - DLL calibration enable."]
pub type DLLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLLCR_SPEC, DLLEN_A, O>;
impl<'a, const O: u8> DLLEN_W<'a, O> {
    #[doc = "DLL calibration is disabled"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(DLLEN_A::VALUE0)
    }
    #[doc = "DLL calibration is enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DLLEN_A::VALUE1)
    }
}
#[doc = "Field `DLLRESET` reader - DLL reset"]
pub type DLLRESET_R = crate::BitReader<DLLRESET_A>;
#[doc = "DLL reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLLRESET_A {
    #[doc = "0: No function."]
    VALUE0 = 0,
    #[doc = "1: Software could force a reset on DLL by setting this field to 0x1."]
    VALUE1 = 1,
}
impl From<DLLRESET_A> for bool {
    #[inline(always)]
    fn from(variant: DLLRESET_A) -> Self {
        variant as u8 != 0
    }
}
impl DLLRESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLLRESET_A {
        match self.bits {
            false => DLLRESET_A::VALUE0,
            true => DLLRESET_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == DLLRESET_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DLLRESET_A::VALUE1
    }
}
#[doc = "Field `DLLRESET` writer - DLL reset"]
pub type DLLRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLLCR_SPEC, DLLRESET_A, O>;
impl<'a, const O: u8> DLLRESET_W<'a, O> {
    #[doc = "No function."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(DLLRESET_A::VALUE0)
    }
    #[doc = "Software could force a reset on DLL by setting this field to 0x1."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DLLRESET_A::VALUE1)
    }
}
#[doc = "Field `SLVDLYTARGET` reader - The delay target for slave delay line is: ((SLVDLYTARGET+1) * 1/32 * clock cycle of reference clock (serial root clock). If serial root clock is >= 100 MHz, DLLEN set to 0x1, OVRDEN set to =0x0, then SLVDLYTARGET setting of 0 is recommended."]
pub type SLVDLYTARGET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLVDLYTARGET` writer - The delay target for slave delay line is: ((SLVDLYTARGET+1) * 1/32 * clock cycle of reference clock (serial root clock). If serial root clock is >= 100 MHz, DLLEN set to 0x1, OVRDEN set to =0x0, then SLVDLYTARGET setting of 0 is recommended."]
pub type SLVDLYTARGET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLLCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `OVRDEN` reader - Slave clock delay line delay cell number selection override enable."]
pub type OVRDEN_R = crate::BitReader<OVRDEN_A>;
#[doc = "Slave clock delay line delay cell number selection override enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRDEN_A {
    #[doc = "0: Slave clock delay line delay cell number selection override is disabled."]
    VALUE0 = 0,
    #[doc = "1: Slave clock delay line delay cell number selection override is enabled."]
    VALUE1 = 1,
}
impl From<OVRDEN_A> for bool {
    #[inline(always)]
    fn from(variant: OVRDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRDEN_A {
        match self.bits {
            false => OVRDEN_A::VALUE0,
            true => OVRDEN_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == OVRDEN_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OVRDEN_A::VALUE1
    }
}
#[doc = "Field `OVRDEN` writer - Slave clock delay line delay cell number selection override enable."]
pub type OVRDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLLCR_SPEC, OVRDEN_A, O>;
impl<'a, const O: u8> OVRDEN_W<'a, O> {
    #[doc = "Slave clock delay line delay cell number selection override is disabled."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(OVRDEN_A::VALUE0)
    }
    #[doc = "Slave clock delay line delay cell number selection override is enabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OVRDEN_A::VALUE1)
    }
}
#[doc = "Field `OVRDVAL` reader - Slave clock delay line delay cell number selection override value."]
pub type OVRDVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OVRDVAL` writer - Slave clock delay line delay cell number selection override value."]
pub type OVRDVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLLCR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0 - DLL calibration enable."]
    #[inline(always)]
    pub fn dllen(&self) -> DLLEN_R {
        DLLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DLL reset"]
    #[inline(always)]
    pub fn dllreset(&self) -> DLLRESET_R {
        DLLRESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:6 - The delay target for slave delay line is: ((SLVDLYTARGET+1) * 1/32 * clock cycle of reference clock (serial root clock). If serial root clock is >= 100 MHz, DLLEN set to 0x1, OVRDEN set to =0x0, then SLVDLYTARGET setting of 0 is recommended."]
    #[inline(always)]
    pub fn slvdlytarget(&self) -> SLVDLYTARGET_R {
        SLVDLYTARGET_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Slave clock delay line delay cell number selection override enable."]
    #[inline(always)]
    pub fn ovrden(&self) -> OVRDEN_R {
        OVRDEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:14 - Slave clock delay line delay cell number selection override value."]
    #[inline(always)]
    pub fn ovrdval(&self) -> OVRDVAL_R {
        OVRDVAL_R::new(((self.bits >> 9) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DLL calibration enable."]
    #[inline(always)]
    #[must_use]
    pub fn dllen(&mut self) -> DLLEN_W<0> {
        DLLEN_W::new(self)
    }
    #[doc = "Bit 1 - DLL reset"]
    #[inline(always)]
    #[must_use]
    pub fn dllreset(&mut self) -> DLLRESET_W<1> {
        DLLRESET_W::new(self)
    }
    #[doc = "Bits 3:6 - The delay target for slave delay line is: ((SLVDLYTARGET+1) * 1/32 * clock cycle of reference clock (serial root clock). If serial root clock is >= 100 MHz, DLLEN set to 0x1, OVRDEN set to =0x0, then SLVDLYTARGET setting of 0 is recommended."]
    #[inline(always)]
    #[must_use]
    pub fn slvdlytarget(&mut self) -> SLVDLYTARGET_W<3> {
        SLVDLYTARGET_W::new(self)
    }
    #[doc = "Bit 8 - Slave clock delay line delay cell number selection override enable."]
    #[inline(always)]
    #[must_use]
    pub fn ovrden(&mut self) -> OVRDEN_W<8> {
        OVRDEN_W::new(self)
    }
    #[doc = "Bits 9:14 - Slave clock delay line delay cell number selection override value."]
    #[inline(always)]
    #[must_use]
    pub fn ovrdval(&mut self) -> OVRDVAL_W<9> {
        OVRDVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DLL Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dllcr](index.html) module"]
pub struct DLLCR_SPEC;
impl crate::RegisterSpec for DLLCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dllcr::R](R) reader structure"]
impl crate::Readable for DLLCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dllcr::W](W) writer structure"]
impl crate::Writable for DLLCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DLLCR%s to value 0x0100"]
impl crate::Resettable for DLLCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
