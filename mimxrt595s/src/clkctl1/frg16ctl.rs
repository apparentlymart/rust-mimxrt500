#[doc = "Register `FRG16CTL` reader"]
pub struct R(crate::R<FRG16CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRG16CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRG16CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRG16CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRG16CTL` writer"]
pub struct W(crate::W<FRG16CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRG16CTL_SPEC>;
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
impl From<crate::W<FRG16CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRG16CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - Denominator of the fractional divider: DIV is equal to the programmed value +1"]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - Denominator of the fractional divider: DIV is equal to the programmed value +1"]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRG16CTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `MULT` reader - Numerator of the fractional divider: MULT is equal to the programmed value."]
pub type MULT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MULT` writer - Numerator of the fractional divider: MULT is equal to the programmed value."]
pub type MULT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRG16CTL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Denominator of the fractional divider: DIV is equal to the programmed value +1"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Numerator of the fractional divider: MULT is equal to the programmed value."]
    #[inline(always)]
    pub fn mult(&self) -> MULT_R {
        MULT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Denominator of the fractional divider: DIV is equal to the programmed value +1"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<0> {
        DIV_W::new(self)
    }
    #[doc = "Bits 8:15 - Numerator of the fractional divider: MULT is equal to the programmed value."]
    #[inline(always)]
    #[must_use]
    pub fn mult(&mut self) -> MULT_W<8> {
        MULT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fractional Rate Generator 16 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frg16ctl](index.html) module"]
pub struct FRG16CTL_SPEC;
impl crate::RegisterSpec for FRG16CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frg16ctl::R](R) reader structure"]
impl crate::Readable for FRG16CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frg16ctl::W](W) writer structure"]
impl crate::Writable for FRG16CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRG16CTL to value 0xff"]
impl crate::Resettable for FRG16CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
