#[doc = "Register `PINOUTSET` reader"]
pub struct R(crate::R<PINOUTSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINOUTSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINOUTSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINOUTSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINOUTSET` writer"]
pub struct W(crate::W<PINOUTSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINOUTSET_SPEC>;
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
impl From<crate::W<PINOUTSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINOUTSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTSET` reader - Output Set"]
pub type OUTSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OUTSET` writer - Output Set"]
pub type OUTSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PINOUTSET_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Output Set"]
    #[inline(always)]
    pub fn outset(&self) -> OUTSET_R {
        OUTSET_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn outset(&mut self) -> OUTSET_W<0> {
        OUTSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Output Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinoutset](index.html) module"]
pub struct PINOUTSET_SPEC;
impl crate::RegisterSpec for PINOUTSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinoutset::R](R) reader structure"]
impl crate::Readable for PINOUTSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinoutset::W](W) writer structure"]
impl crate::Writable for PINOUTSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINOUTSET to value 0"]
impl crate::Resettable for PINOUTSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
