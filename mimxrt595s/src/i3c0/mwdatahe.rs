#[doc = "Register `MWDATAHE` writer"]
pub struct W(crate::W<MWDATAHE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MWDATAHE_SPEC>;
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
impl From<crate::W<MWDATAHE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MWDATAHE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA0` writer - DATA 0"]
pub type DATA0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MWDATAHE_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA1` writer - DATA 1"]
pub type DATA1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MWDATAHE_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - DATA 0"]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> DATA0_W<0> {
        DATA0_W::new(self)
    }
    #[doc = "Bits 8:15 - DATA 1"]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> DATA1_W<8> {
        DATA1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Write Data Byte End Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mwdatahe](index.html) module"]
pub struct MWDATAHE_SPEC;
impl crate::RegisterSpec for MWDATAHE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [mwdatahe::W](W) writer structure"]
impl crate::Writable for MWDATAHE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MWDATAHE to value 0"]
impl crate::Resettable for MWDATAHE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
