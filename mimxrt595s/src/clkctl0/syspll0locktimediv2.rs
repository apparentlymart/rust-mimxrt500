#[doc = "Register `SYSPLL0LOCKTIMEDIV2` reader"]
pub struct R(crate::R<SYSPLL0LOCKTIMEDIV2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSPLL0LOCKTIMEDIV2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSPLL0LOCKTIMEDIV2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSPLL0LOCKTIMEDIV2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSPLL0LOCKTIMEDIV2` writer"]
pub struct W(crate::W<SYSPLL0LOCKTIMEDIV2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSPLL0LOCKTIMEDIV2_SPEC>;
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
impl From<crate::W<SYSPLL0LOCKTIMEDIV2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSPLL0LOCKTIMEDIV2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKTIMEDIV2` reader - SYSPLL0 Lock Time Divide-by-2"]
pub type LOCKTIMEDIV2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LOCKTIMEDIV2` writer - SYSPLL0 Lock Time Divide-by-2"]
pub type LOCKTIMEDIV2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYSPLL0LOCKTIMEDIV2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - SYSPLL0 Lock Time Divide-by-2"]
    #[inline(always)]
    pub fn locktimediv2(&self) -> LOCKTIMEDIV2_R {
        LOCKTIMEDIV2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SYSPLL0 Lock Time Divide-by-2"]
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
#[doc = "System PLL0 Lock Time Div2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspll0locktimediv2](index.html) module"]
pub struct SYSPLL0LOCKTIMEDIV2_SPEC;
impl crate::RegisterSpec for SYSPLL0LOCKTIMEDIV2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syspll0locktimediv2::R](R) reader structure"]
impl crate::Readable for SYSPLL0LOCKTIMEDIV2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syspll0locktimediv2::W](W) writer structure"]
impl crate::Writable for SYSPLL0LOCKTIMEDIV2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSPLL0LOCKTIMEDIV2 to value 0xcafe"]
impl crate::Resettable for SYSPLL0LOCKTIMEDIV2_SPEC {
    const RESET_VALUE: Self::Ux = 0xcafe;
}
