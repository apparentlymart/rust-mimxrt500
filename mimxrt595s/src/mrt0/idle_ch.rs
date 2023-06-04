#[doc = "Register `IDLE_CH` reader"]
pub struct R(crate::R<IDLE_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDLE_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDLE_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDLE_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHAN` reader - Idle Channel"]
pub type CHAN_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 4:7 - Idle Channel"]
    #[inline(always)]
    pub fn chan(&self) -> CHAN_R {
        CHAN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Idle Channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idle_ch](index.html) module"]
pub struct IDLE_CH_SPEC;
impl crate::RegisterSpec for IDLE_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idle_ch::R](R) reader structure"]
impl crate::Readable for IDLE_CH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IDLE_CH to value 0"]
impl crate::Resettable for IDLE_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
