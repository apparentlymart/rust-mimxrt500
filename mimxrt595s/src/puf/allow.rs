#[doc = "Register `ALLOW` reader"]
pub struct R(crate::R<ALLOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALLOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALLOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALLOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ALLOWENROLL` reader - Allow Enroll"]
pub type ALLOWENROLL_R = crate::BitReader<bool>;
#[doc = "Field `ALLOWSTART` reader - Allow Start"]
pub type ALLOWSTART_R = crate::BitReader<bool>;
#[doc = "Field `ALLOWSETKEY` reader - Allow Set Key"]
pub type ALLOWSETKEY_R = crate::BitReader<bool>;
#[doc = "Field `ALLOWGETKEY` reader - Allow Get Key"]
pub type ALLOWGETKEY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Allow Enroll"]
    #[inline(always)]
    pub fn allowenroll(&self) -> ALLOWENROLL_R {
        ALLOWENROLL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Allow Start"]
    #[inline(always)]
    pub fn allowstart(&self) -> ALLOWSTART_R {
        ALLOWSTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Allow Set Key"]
    #[inline(always)]
    pub fn allowsetkey(&self) -> ALLOWSETKEY_R {
        ALLOWSETKEY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Allow Get Key"]
    #[inline(always)]
    pub fn allowgetkey(&self) -> ALLOWGETKEY_R {
        ALLOWGETKEY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "PUF Allow\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [allow](index.html) module"]
pub struct ALLOW_SPEC;
impl crate::RegisterSpec for ALLOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [allow::R](R) reader structure"]
impl crate::Readable for ALLOW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ALLOW to value 0"]
impl crate::Resettable for ALLOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
