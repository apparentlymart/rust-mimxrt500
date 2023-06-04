#[doc = "Register `MWDATABE` writer"]
pub struct W(crate::W<MWDATABE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MWDATABE_SPEC>;
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
impl From<crate::W<MWDATABE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MWDATABE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` writer - Data"]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MWDATABE_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Data"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<0> {
        VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Write Data Byte End Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mwdatabe](index.html) module"]
pub struct MWDATABE_SPEC;
impl crate::RegisterSpec for MWDATABE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [mwdatabe::W](W) writer structure"]
impl crate::Writable for MWDATABE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MWDATABE to value 0"]
impl crate::Resettable for MWDATABE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
