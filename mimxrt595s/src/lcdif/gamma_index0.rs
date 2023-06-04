#[doc = "Register `GammaIndex0` reader"]
pub struct R(crate::R<GAMMA_INDEX0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAMMA_INDEX0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAMMA_INDEX0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAMMA_INDEX0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GammaIndex0` writer"]
pub struct W(crate::W<GAMMA_INDEX0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAMMA_INDEX0_SPEC>;
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
impl From<crate::W<GAMMA_INDEX0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAMMA_INDEX0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INDEX` reader - Index into Gamma Table."]
pub type INDEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INDEX` writer - Index into Gamma Table."]
pub type INDEX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAMMA_INDEX0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Index into Gamma Table."]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Index into Gamma Table."]
    #[inline(always)]
    #[must_use]
    pub fn index(&mut self) -> INDEX_W<0> {
        INDEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Index into Gamma Table\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gamma_index0](index.html) module"]
pub struct GAMMA_INDEX0_SPEC;
impl crate::RegisterSpec for GAMMA_INDEX0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gamma_index0::R](R) reader structure"]
impl crate::Readable for GAMMA_INDEX0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gamma_index0::W](W) writer structure"]
impl crate::Writable for GAMMA_INDEX0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GammaIndex0 to value 0"]
impl crate::Resettable for GAMMA_INDEX0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
