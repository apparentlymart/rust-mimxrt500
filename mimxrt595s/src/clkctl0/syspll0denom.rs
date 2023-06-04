#[doc = "Register `SYSPLL0DENOM` reader"]
pub struct R(crate::R<SYSPLL0DENOM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSPLL0DENOM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSPLL0DENOM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSPLL0DENOM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSPLL0DENOM` writer"]
pub struct W(crate::W<SYSPLL0DENOM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSPLL0DENOM_SPEC>;
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
impl From<crate::W<SYSPLL0DENOM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSPLL0DENOM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DENOM` reader - Denominator of the SYSPLL0 fractional loop divider"]
pub type DENOM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DENOM` writer - Denominator of the SYSPLL0 fractional loop divider"]
pub type DENOM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSPLL0DENOM_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:29 - Denominator of the SYSPLL0 fractional loop divider"]
    #[inline(always)]
    pub fn denom(&self) -> DENOM_R {
        DENOM_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Denominator of the SYSPLL0 fractional loop divider"]
    #[inline(always)]
    #[must_use]
    pub fn denom(&mut self) -> DENOM_W<0> {
        DENOM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System PLL0 Denominator\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspll0denom](index.html) module"]
pub struct SYSPLL0DENOM_SPEC;
impl crate::RegisterSpec for SYSPLL0DENOM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syspll0denom::R](R) reader structure"]
impl crate::Readable for SYSPLL0DENOM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syspll0denom::W](W) writer structure"]
impl crate::Writable for SYSPLL0DENOM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSPLL0DENOM to value 0x1fff_ffdb"]
impl crate::Resettable for SYSPLL0DENOM_SPEC {
    const RESET_VALUE: Self::Ux = 0x1fff_ffdb;
}
