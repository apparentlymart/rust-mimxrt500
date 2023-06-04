#[doc = "Register `DSI_HOST_CFG_STATUS_OUT` reader"]
pub struct R(crate::R<DSI_HOST_CFG_STATUS_OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_HOST_CFG_STATUS_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_HOST_CFG_STATUS_OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_HOST_CFG_STATUS_OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `dsi_host_cfg_status_out` reader - Status Register"]
pub type DSI_HOST_CFG_STATUS_OUT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Status Register"]
    #[inline(always)]
    pub fn dsi_host_cfg_status_out(&self) -> DSI_HOST_CFG_STATUS_OUT_R {
        DSI_HOST_CFG_STATUS_OUT_R::new(self.bits)
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_host_cfg_status_out](index.html) module"]
pub struct DSI_HOST_CFG_STATUS_OUT_SPEC;
impl crate::RegisterSpec for DSI_HOST_CFG_STATUS_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_host_cfg_status_out::R](R) reader structure"]
impl crate::Readable for DSI_HOST_CFG_STATUS_OUT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DSI_HOST_CFG_STATUS_OUT to value 0"]
impl crate::Resettable for DSI_HOST_CFG_STATUS_OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
