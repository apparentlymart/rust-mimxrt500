#[doc = "Register `MWDATAB1` writer"]
pub struct W(crate::W<MWDATAB1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MWDATAB1_SPEC>;
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
impl From<crate::W<MWDATAB1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MWDATAB1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` writer - Value"]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MWDATAB1_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Value"]
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
#[doc = "Write Byte Data 1 (to bus)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mwdatab1](index.html) module"]
pub struct MWDATAB1_SPEC;
impl crate::RegisterSpec for MWDATAB1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [mwdatab1::W](W) writer structure"]
impl crate::Writable for MWDATAB1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MWDATAB1 to value 0"]
impl crate::Resettable for MWDATAB1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
