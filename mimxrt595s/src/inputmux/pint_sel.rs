#[doc = "Register `PINT_SEL[%s]` reader"]
pub struct R(crate::R<PINT_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINT_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINT_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINT_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINT_SEL[%s]` writer"]
pub struct W(crate::W<PINT_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINT_SEL_SPEC>;
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
impl From<crate::W<PINT_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINT_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PINT_SEL` reader - Interrupt select"]
pub type PINT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PINT_SEL` writer - Interrupt select"]
pub type PINT_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PINT_SEL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt select"]
    #[inline(always)]
    pub fn pint_sel(&self) -> PINT_SEL_R {
        PINT_SEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt select"]
    #[inline(always)]
    #[must_use]
    pub fn pint_sel(&mut self) -> PINT_SEL_W<0> {
        PINT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Pin Input Multiplexer index\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pint_sel](index.html) module"]
pub struct PINT_SEL_SPEC;
impl crate::RegisterSpec for PINT_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pint_sel::R](R) reader structure"]
impl crate::Readable for PINT_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pint_sel::W](W) writer structure"]
impl crate::Writable for PINT_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINT_SEL[%s]
to value 0x1f"]
impl crate::Resettable for PINT_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
