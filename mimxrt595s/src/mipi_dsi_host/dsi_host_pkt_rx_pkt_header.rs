#[doc = "Register `DSI_HOST_PKT_RX_PKT_HEADER` reader"]
pub struct R(crate::R<DSI_HOST_PKT_RX_PKT_HEADER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_HOST_PKT_RX_PKT_HEADER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_HOST_PKT_RX_PKT_HEADER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_HOST_PKT_RX_PKT_HEADER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `dsi_host_pkt_rx_pkt_header` reader - APB to pkt interface rx packet header \\[15:0\\]
word count \\[21:16\\]
data type \\[23:22\\]
Virtual Channel"]
pub type DSI_HOST_PKT_RX_PKT_HEADER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - APB to pkt interface rx packet header \\[15:0\\]
word count \\[21:16\\]
data type \\[23:22\\]
Virtual Channel"]
    #[inline(always)]
    pub fn dsi_host_pkt_rx_pkt_header(&self) -> DSI_HOST_PKT_RX_PKT_HEADER_R {
        DSI_HOST_PKT_RX_PKT_HEADER_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_host_pkt_rx_pkt_header](index.html) module"]
pub struct DSI_HOST_PKT_RX_PKT_HEADER_SPEC;
impl crate::RegisterSpec for DSI_HOST_PKT_RX_PKT_HEADER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_host_pkt_rx_pkt_header::R](R) reader structure"]
impl crate::Readable for DSI_HOST_PKT_RX_PKT_HEADER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DSI_HOST_PKT_RX_PKT_HEADER to value 0"]
impl crate::Resettable for DSI_HOST_PKT_RX_PKT_HEADER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
