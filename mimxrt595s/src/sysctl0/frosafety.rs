#[doc = "Register `FROSAFETY` reader"]
pub struct R(crate::R<FROSAFETY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FROSAFETY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FROSAFETY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FROSAFETY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FROSAFETY` writer"]
pub struct W(crate::W<FROSAFETY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FROSAFETY_SPEC>;
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
impl From<crate::W<FROSAFETY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FROSAFETY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FROSAFETY` reader - FRO Safety"]
pub type FROSAFETY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FROSAFETY` writer - FRO Safety"]
pub type FROSAFETY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FROSAFETY_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - FRO Safety"]
    #[inline(always)]
    pub fn frosafety(&self) -> FROSAFETY_R {
        FROSAFETY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - FRO Safety"]
    #[inline(always)]
    #[must_use]
    pub fn frosafety(&mut self) -> FROSAFETY_W<0> {
        FROSAFETY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FRO Safety\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frosafety](index.html) module"]
pub struct FROSAFETY_SPEC;
impl crate::RegisterSpec for FROSAFETY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frosafety::R](R) reader structure"]
impl crate::Readable for FROSAFETY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frosafety::W](W) writer structure"]
impl crate::Writable for FROSAFETY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff_ffff;
}
#[doc = "`reset()` method sets FROSAFETY to value 0"]
impl crate::Resettable for FROSAFETY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
