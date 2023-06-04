#[doc = "Register `DSI_HOST_PKT_FIFO_RD_LEVEL` reader"]
pub struct R(crate::R<DSI_HOST_PKT_FIFO_RD_LEVEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_HOST_PKT_FIFO_RD_LEVEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_HOST_PKT_FIFO_RD_LEVEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_HOST_PKT_FIFO_RD_LEVEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `dsi_host_pkt_fifo_rd_level` reader - Read level of APB to pkt interface fifo"]
pub type DSI_HOST_PKT_FIFO_RD_LEVEL_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Read level of APB to pkt interface fifo"]
    #[inline(always)]
    pub fn dsi_host_pkt_fifo_rd_level(&self) -> DSI_HOST_PKT_FIFO_RD_LEVEL_R {
        DSI_HOST_PKT_FIFO_RD_LEVEL_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_host_pkt_fifo_rd_level](index.html) module"]
pub struct DSI_HOST_PKT_FIFO_RD_LEVEL_SPEC;
impl crate::RegisterSpec for DSI_HOST_PKT_FIFO_RD_LEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_host_pkt_fifo_rd_level::R](R) reader structure"]
impl crate::Readable for DSI_HOST_PKT_FIFO_RD_LEVEL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DSI_HOST_PKT_FIFO_RD_LEVEL to value 0"]
impl crate::Resettable for DSI_HOST_PKT_FIFO_RD_LEVEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
