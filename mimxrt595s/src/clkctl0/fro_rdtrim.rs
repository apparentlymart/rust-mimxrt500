#[doc = "Register `FRO_RDTRIM` reader"]
pub struct R(crate::R<FRO_RDTRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRO_RDTRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRO_RDTRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRO_RDTRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRO_RDTRIM` writer"]
pub struct W(crate::W<FRO_RDTRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRO_RDTRIM_SPEC>;
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
impl From<crate::W<FRO_RDTRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRO_RDTRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIM` reader - It is the trim value supplied to the oscillator"]
pub type TRIM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TRIM` writer - It is the trim value supplied to the oscillator"]
pub type TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRO_RDTRIM_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - It is the trim value supplied to the oscillator"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - It is the trim value supplied to the oscillator"]
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
#[doc = "Free Running Oscillator Trim\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fro_rdtrim](index.html) module"]
pub struct FRO_RDTRIM_SPEC;
impl crate::RegisterSpec for FRO_RDTRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fro_rdtrim::R](R) reader structure"]
impl crate::Readable for FRO_RDTRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fro_rdtrim::W](W) writer structure"]
impl crate::Writable for FRO_RDTRIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRO_RDTRIM to value 0x03bf"]
impl crate::Resettable for FRO_RDTRIM_SPEC {
    const RESET_VALUE: Self::Ux = 0x03bf;
}
