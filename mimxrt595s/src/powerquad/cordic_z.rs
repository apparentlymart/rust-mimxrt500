#[doc = "Register `CORDIC_Z` reader"]
pub struct R(crate::R<CORDIC_Z_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORDIC_Z_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORDIC_Z_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORDIC_Z_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORDIC_Z` writer"]
pub struct W(crate::W<CORDIC_Z_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORDIC_Z_SPEC>;
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
impl From<crate::W<CORDIC_Z_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORDIC_Z_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORDIC_Z` reader - CORDIC input z"]
pub type CORDIC_Z_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CORDIC_Z` writer - CORDIC input z"]
pub type CORDIC_Z_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CORDIC_Z_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CORDIC input z"]
    #[inline(always)]
    pub fn cordic_z(&self) -> CORDIC_Z_R {
        CORDIC_Z_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CORDIC input z"]
    #[inline(always)]
    #[must_use]
    pub fn cordic_z(&mut self) -> CORDIC_Z_W<0> {
        CORDIC_Z_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CORDIC input Z\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cordic_z](index.html) module"]
pub struct CORDIC_Z_SPEC;
impl crate::RegisterSpec for CORDIC_Z_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cordic_z::R](R) reader structure"]
impl crate::Readable for CORDIC_Z_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cordic_z::W](W) writer structure"]
impl crate::Writable for CORDIC_Z_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORDIC_Z to value 0"]
impl crate::Resettable for CORDIC_Z_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
