#[doc = "Register `USBPHYPLL0LOCKTIMEDIV2` reader"]
pub struct R(crate::R<USBPHYPLL0LOCKTIMEDIV2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPHYPLL0LOCKTIMEDIV2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPHYPLL0LOCKTIMEDIV2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPHYPLL0LOCKTIMEDIV2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPHYPLL0LOCKTIMEDIV2` writer"]
pub struct W(crate::W<USBPHYPLL0LOCKTIMEDIV2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPHYPLL0LOCKTIMEDIV2_SPEC>;
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
impl From<crate::W<USBPHYPLL0LOCKTIMEDIV2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPHYPLL0LOCKTIMEDIV2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKTIMEDIV2` reader - USBPHYPLL0 Lock Time"]
pub type LOCKTIMEDIV2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LOCKTIMEDIV2` writer - USBPHYPLL0 Lock Time"]
pub type LOCKTIMEDIV2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USBPHYPLL0LOCKTIMEDIV2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - USBPHYPLL0 Lock Time"]
    #[inline(always)]
    pub fn locktimediv2(&self) -> LOCKTIMEDIV2_R {
        LOCKTIMEDIV2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - USBPHYPLL0 Lock Time"]
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
#[doc = "USB PHY PLL0 lock time division\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbphypll0locktimediv2](index.html) module"]
pub struct USBPHYPLL0LOCKTIMEDIV2_SPEC;
impl crate::RegisterSpec for USBPHYPLL0LOCKTIMEDIV2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbphypll0locktimediv2::R](R) reader structure"]
impl crate::Readable for USBPHYPLL0LOCKTIMEDIV2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbphypll0locktimediv2::W](W) writer structure"]
impl crate::Writable for USBPHYPLL0LOCKTIMEDIV2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBPHYPLL0LOCKTIMEDIV2 to value 0xcafe"]
impl crate::Resettable for USBPHYPLL0LOCKTIMEDIV2_SPEC {
    const RESET_VALUE: Self::Ux = 0xcafe;
}
