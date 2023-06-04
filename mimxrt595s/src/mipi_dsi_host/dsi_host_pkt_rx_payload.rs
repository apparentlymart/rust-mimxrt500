#[doc = "Register `DSI_HOST_PKT_RX_PAYLOAD` reader"]
pub struct R(crate::R<DSI_HOST_PKT_RX_PAYLOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_HOST_PKT_RX_PAYLOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_HOST_PKT_RX_PAYLOAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_HOST_PKT_RX_PAYLOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `dsi_host_pkt_rx_payload` reader - APB to pkt interface rx payload read"]
pub type DSI_HOST_PKT_RX_PAYLOAD_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - APB to pkt interface rx payload read"]
    #[inline(always)]
    pub fn dsi_host_pkt_rx_payload(&self) -> DSI_HOST_PKT_RX_PAYLOAD_R {
        DSI_HOST_PKT_RX_PAYLOAD_R::new(self.bits)
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_host_pkt_rx_payload](index.html) module"]
pub struct DSI_HOST_PKT_RX_PAYLOAD_SPEC;
impl crate::RegisterSpec for DSI_HOST_PKT_RX_PAYLOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_host_pkt_rx_payload::R](R) reader structure"]
impl crate::Readable for DSI_HOST_PKT_RX_PAYLOAD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DSI_HOST_PKT_RX_PAYLOAD to value 0"]
impl crate::Resettable for DSI_HOST_PKT_RX_PAYLOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
