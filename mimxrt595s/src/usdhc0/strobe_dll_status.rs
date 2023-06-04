#[doc = "Register `STROBE_DLL_STATUS` reader"]
pub struct R(crate::R<STROBE_DLL_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STROBE_DLL_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STROBE_DLL_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STROBE_DLL_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STROBE_DLL_STS_SLV_LOCK` reader - Strobe DLL status slave lock"]
pub type STROBE_DLL_STS_SLV_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `STROBE_DLL_STS_REF_LOCK` reader - Strobe DLL status reference lock"]
pub type STROBE_DLL_STS_REF_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `STROBE_DLL_STS_SLV_SEL` reader - Strobe DLL status slave select"]
pub type STROBE_DLL_STS_SLV_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STROBE_DLL_STS_REF_SEL` reader - Strobe DLL status reference select"]
pub type STROBE_DLL_STS_REF_SEL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Strobe DLL status slave lock"]
    #[inline(always)]
    pub fn strobe_dll_sts_slv_lock(&self) -> STROBE_DLL_STS_SLV_LOCK_R {
        STROBE_DLL_STS_SLV_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Strobe DLL status reference lock"]
    #[inline(always)]
    pub fn strobe_dll_sts_ref_lock(&self) -> STROBE_DLL_STS_REF_LOCK_R {
        STROBE_DLL_STS_REF_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:8 - Strobe DLL status slave select"]
    #[inline(always)]
    pub fn strobe_dll_sts_slv_sel(&self) -> STROBE_DLL_STS_SLV_SEL_R {
        STROBE_DLL_STS_SLV_SEL_R::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bits 9:15 - Strobe DLL status reference select"]
    #[inline(always)]
    pub fn strobe_dll_sts_ref_sel(&self) -> STROBE_DLL_STS_REF_SEL_R {
        STROBE_DLL_STS_REF_SEL_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
#[doc = "Strobe DLL status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [strobe_dll_status](index.html) module"]
pub struct STROBE_DLL_STATUS_SPEC;
impl crate::RegisterSpec for STROBE_DLL_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [strobe_dll_status::R](R) reader structure"]
impl crate::Readable for STROBE_DLL_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STROBE_DLL_STATUS to value 0x0200"]
impl crate::Resettable for STROBE_DLL_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
