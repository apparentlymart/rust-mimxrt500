#[doc = "Register `CODEOUTPUT` reader"]
pub struct R(crate::R<CODEOUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CODEOUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CODEOUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CODEOUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CODEOUT` reader - AC/KC Output Data"]
pub type CODEOUT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - AC/KC Output Data"]
    #[inline(always)]
    pub fn codeout(&self) -> CODEOUT_R {
        CODEOUT_R::new(self.bits)
    }
}
#[doc = "PUF Code Output\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [codeoutput](index.html) module"]
pub struct CODEOUTPUT_SPEC;
impl crate::RegisterSpec for CODEOUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [codeoutput::R](R) reader structure"]
impl crate::Readable for CODEOUTPUT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CODEOUTPUT to value 0"]
impl crate::Resettable for CODEOUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
