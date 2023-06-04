#[doc = "Register `FRO_SCTRIM` reader"]
pub struct R(crate::R<FRO_SCTRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRO_SCTRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRO_SCTRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRO_SCTRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRO_SCTRIM` writer"]
pub struct W(crate::W<FRO_SCTRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRO_SCTRIM_SPEC>;
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
impl From<crate::W<FRO_SCTRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRO_SCTRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIM` reader - sc_trim value for the oscillator."]
pub type TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM` writer - sc_trim value for the oscillator."]
pub type TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRO_SCTRIM_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - sc_trim value for the oscillator."]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - sc_trim value for the oscillator."]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TRIM_W<0> {
        TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Free Running OscillatorSC Trim\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fro_sctrim](index.html) module"]
pub struct FRO_SCTRIM_SPEC;
impl crate::RegisterSpec for FRO_SCTRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fro_sctrim::R](R) reader structure"]
impl crate::Readable for FRO_SCTRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fro_sctrim::W](W) writer structure"]
impl crate::Writable for FRO_SCTRIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRO_SCTRIM to value 0x20"]
impl crate::Resettable for FRO_SCTRIM_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
