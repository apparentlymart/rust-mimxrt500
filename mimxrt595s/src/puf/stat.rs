#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSY` reader - Busy"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `SUCCESS` reader - Success"]
pub type SUCCESS_R = crate::BitReader<bool>;
#[doc = "Field `ERROR` reader - Error"]
pub type ERROR_R = crate::BitReader<bool>;
#[doc = "Field `KEYINREQ` reader - Key In Request"]
pub type KEYINREQ_R = crate::BitReader<bool>;
#[doc = "Field `KEYOUTAVAIL` reader - Key Out Available"]
pub type KEYOUTAVAIL_R = crate::BitReader<bool>;
#[doc = "Field `CODEINREQ` reader - Code In Request"]
pub type CODEINREQ_R = crate::BitReader<bool>;
#[doc = "Field `CODEOUTAVAIL` reader - Code Out Available"]
pub type CODEOUTAVAIL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Success"]
    #[inline(always)]
    pub fn success(&self) -> SUCCESS_R {
        SUCCESS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Key In Request"]
    #[inline(always)]
    pub fn keyinreq(&self) -> KEYINREQ_R {
        KEYINREQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Key Out Available"]
    #[inline(always)]
    pub fn keyoutavail(&self) -> KEYOUTAVAIL_R {
        KEYOUTAVAIL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Code In Request"]
    #[inline(always)]
    pub fn codeinreq(&self) -> CODEINREQ_R {
        CODEINREQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Code Out Available"]
    #[inline(always)]
    pub fn codeoutavail(&self) -> CODEOUTAVAIL_R {
        CODEOUTAVAIL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "PUF Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAT to value 0x01"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
