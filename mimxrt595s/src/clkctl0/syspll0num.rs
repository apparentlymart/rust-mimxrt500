#[doc = "Register `SYSPLL0NUM` reader"]
pub struct R(crate::R<SYSPLL0NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSPLL0NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSPLL0NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSPLL0NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSPLL0NUM` writer"]
pub struct W(crate::W<SYSPLL0NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSPLL0NUM_SPEC>;
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
impl From<crate::W<SYSPLL0NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSPLL0NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NUM` reader - Numerator of the SYSPLL0 fractional loop divider"]
pub type NUM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `NUM` writer - Numerator of the SYSPLL0 fractional loop divider"]
pub type NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSPLL0NUM_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:29 - Numerator of the SYSPLL0 fractional loop divider"]
    #[inline(always)]
    pub fn num(&self) -> NUM_R {
        NUM_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Numerator of the SYSPLL0 fractional loop divider"]
    #[inline(always)]
    #[must_use]
    pub fn num(&mut self) -> NUM_W<0> {
        NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System PLL0 Numerator\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspll0num](index.html) module"]
pub struct SYSPLL0NUM_SPEC;
impl crate::RegisterSpec for SYSPLL0NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syspll0num::R](R) reader structure"]
impl crate::Readable for SYSPLL0NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syspll0num::W](W) writer structure"]
impl crate::Writable for SYSPLL0NUM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSPLL0NUM to value 0x04dd_2f15"]
impl crate::Resettable for SYSPLL0NUM_SPEC {
    const RESET_VALUE: Self::Ux = 0x04dd_2f15;
}
