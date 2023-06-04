#[doc = "Register `AUDIOPLL0LOCKTIMEDIV2` reader"]
pub struct R(crate::R<AUDIOPLL0LOCKTIMEDIV2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUDIOPLL0LOCKTIMEDIV2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUDIOPLL0LOCKTIMEDIV2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUDIOPLL0LOCKTIMEDIV2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUDIOPLL0LOCKTIMEDIV2` writer"]
pub struct W(crate::W<AUDIOPLL0LOCKTIMEDIV2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUDIOPLL0LOCKTIMEDIV2_SPEC>;
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
impl From<crate::W<AUDIOPLL0LOCKTIMEDIV2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUDIOPLL0LOCKTIMEDIV2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKTIMEDIV2` reader - AUDIOPLL0 Lock Time Divide-by-2"]
pub type LOCKTIMEDIV2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LOCKTIMEDIV2` writer - AUDIOPLL0 Lock Time Divide-by-2"]
pub type LOCKTIMEDIV2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AUDIOPLL0LOCKTIMEDIV2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - AUDIOPLL0 Lock Time Divide-by-2"]
    #[inline(always)]
    pub fn locktimediv2(&self) -> LOCKTIMEDIV2_R {
        LOCKTIMEDIV2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - AUDIOPLL0 Lock Time Divide-by-2"]
    #[inline(always)]
    #[must_use]
    pub fn locktimediv2(&mut self) -> LOCKTIMEDIV2_W<0> {
        LOCKTIMEDIV2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Audio PLL0 Lock Time Divide-by-2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audiopll0locktimediv2](index.html) module"]
pub struct AUDIOPLL0LOCKTIMEDIV2_SPEC;
impl crate::RegisterSpec for AUDIOPLL0LOCKTIMEDIV2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [audiopll0locktimediv2::R](R) reader structure"]
impl crate::Readable for AUDIOPLL0LOCKTIMEDIV2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [audiopll0locktimediv2::W](W) writer structure"]
impl crate::Writable for AUDIOPLL0LOCKTIMEDIV2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AUDIOPLL0LOCKTIMEDIV2 to value 0xcafe"]
impl crate::Resettable for AUDIOPLL0LOCKTIMEDIV2_SPEC {
    const RESET_VALUE: Self::Ux = 0xcafe;
}
