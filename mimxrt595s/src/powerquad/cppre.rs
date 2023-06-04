#[doc = "Register `CPPRE` reader"]
pub struct R(crate::R<CPPRE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPPRE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPPRE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPPRE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPPRE` writer"]
pub struct W(crate::W<CPPRE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPPRE_SPEC>;
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
impl From<crate::W<CPPRE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPPRE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPPRE_IN` reader - Input"]
pub type CPPRE_IN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPPRE_IN` writer - Input"]
pub type CPPRE_IN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPPRE_SPEC, u8, u8, 8, O>;
#[doc = "Field `CPPRE_OUT` reader - Output"]
pub type CPPRE_OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPPRE_OUT` writer - Output"]
pub type CPPRE_OUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPPRE_SPEC, u8, u8, 8, O>;
#[doc = "Field `CPPRE_SAT` reader - Saturation"]
pub type CPPRE_SAT_R = crate::BitReader<CPPRE_SAT_A>;
#[doc = "Saturation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPPRE_SAT_A {
    #[doc = "0: No saturation"]
    DISABLE = 0,
    #[doc = "1: Forces sub-32 bit saturation"]
    ENABLE = 1,
}
impl From<CPPRE_SAT_A> for bool {
    #[inline(always)]
    fn from(variant: CPPRE_SAT_A) -> Self {
        variant as u8 != 0
    }
}
impl CPPRE_SAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPPRE_SAT_A {
        match self.bits {
            false => CPPRE_SAT_A::DISABLE,
            true => CPPRE_SAT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CPPRE_SAT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CPPRE_SAT_A::ENABLE
    }
}
#[doc = "Field `CPPRE_SAT` writer - Saturation"]
pub type CPPRE_SAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPPRE_SPEC, CPPRE_SAT_A, O>;
impl<'a, const O: u8> CPPRE_SAT_W<'a, O> {
    #[doc = "No saturation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CPPRE_SAT_A::DISABLE)
    }
    #[doc = "Forces sub-32 bit saturation"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CPPRE_SAT_A::ENABLE)
    }
}
#[doc = "Field `CPPRE_SAT8` reader - Saturation 8"]
pub type CPPRE_SAT8_R = crate::BitReader<CPPRE_SAT8_A>;
#[doc = "Saturation 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPPRE_SAT8_A {
    #[doc = "0: 8 bits"]
    SAT_8_BITS = 0,
    #[doc = "1: 16 bits"]
    SAT_16_BITS = 1,
}
impl From<CPPRE_SAT8_A> for bool {
    #[inline(always)]
    fn from(variant: CPPRE_SAT8_A) -> Self {
        variant as u8 != 0
    }
}
impl CPPRE_SAT8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPPRE_SAT8_A {
        match self.bits {
            false => CPPRE_SAT8_A::SAT_8_BITS,
            true => CPPRE_SAT8_A::SAT_16_BITS,
        }
    }
    #[doc = "Checks if the value of the field is `SAT_8_BITS`"]
    #[inline(always)]
    pub fn is_sat_8_bits(&self) -> bool {
        *self == CPPRE_SAT8_A::SAT_8_BITS
    }
    #[doc = "Checks if the value of the field is `SAT_16_BITS`"]
    #[inline(always)]
    pub fn is_sat_16_bits(&self) -> bool {
        *self == CPPRE_SAT8_A::SAT_16_BITS
    }
}
#[doc = "Field `CPPRE_SAT8` writer - Saturation 8"]
pub type CPPRE_SAT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPPRE_SPEC, CPPRE_SAT8_A, O>;
impl<'a, const O: u8> CPPRE_SAT8_W<'a, O> {
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn sat_8_bits(self) -> &'a mut W {
        self.variant(CPPRE_SAT8_A::SAT_8_BITS)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn sat_16_bits(self) -> &'a mut W {
        self.variant(CPPRE_SAT8_A::SAT_16_BITS)
    }
}
impl R {
    #[doc = "Bits 0:7 - Input"]
    #[inline(always)]
    pub fn cppre_in(&self) -> CPPRE_IN_R {
        CPPRE_IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Output"]
    #[inline(always)]
    pub fn cppre_out(&self) -> CPPRE_OUT_R {
        CPPRE_OUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Saturation"]
    #[inline(always)]
    pub fn cppre_sat(&self) -> CPPRE_SAT_R {
        CPPRE_SAT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Saturation 8"]
    #[inline(always)]
    pub fn cppre_sat8(&self) -> CPPRE_SAT8_R {
        CPPRE_SAT8_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Input"]
    #[inline(always)]
    #[must_use]
    pub fn cppre_in(&mut self) -> CPPRE_IN_W<0> {
        CPPRE_IN_W::new(self)
    }
    #[doc = "Bits 8:15 - Output"]
    #[inline(always)]
    #[must_use]
    pub fn cppre_out(&mut self) -> CPPRE_OUT_W<8> {
        CPPRE_OUT_W::new(self)
    }
    #[doc = "Bit 16 - Saturation"]
    #[inline(always)]
    #[must_use]
    pub fn cppre_sat(&mut self) -> CPPRE_SAT_W<16> {
        CPPRE_SAT_W::new(self)
    }
    #[doc = "Bit 17 - Saturation 8"]
    #[inline(always)]
    #[must_use]
    pub fn cppre_sat8(&mut self) -> CPPRE_SAT8_W<17> {
        CPPRE_SAT8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Coprocessor Pre-scale\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cppre](index.html) module"]
pub struct CPPRE_SPEC;
impl crate::RegisterSpec for CPPRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cppre::R](R) reader structure"]
impl crate::Readable for CPPRE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cppre::W](W) writer structure"]
impl crate::Writable for CPPRE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPPRE to value 0"]
impl crate::Resettable for CPPRE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
