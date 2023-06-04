#[doc = "Register `DSI_HOST_PKT_STATUS` reader"]
pub struct R(crate::R<DSI_HOST_PKT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_HOST_PKT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_HOST_PKT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_HOST_PKT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `dsi_host_pkt_status` reader - Status of APB to packet interface \\[0\\]
- state machine not idle \\[1\\]
- Tx packet done \\[2\\]
- dphy direction 0 - tx had control, 1 - rx has control \\[3\\]
- tx fifo overflow \\[4\\]
- tx fifo underflow \\[5\\]
- rx fifo overflow \\[6\\]
- rx fifo underflow \\[7\\]
- rx packet header has been received \\[8\\]
- all rx packet payload data has been received"]
pub type DSI_HOST_PKT_STATUS_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:8 - Status of APB to packet interface \\[0\\]
- state machine not idle \\[1\\]
- Tx packet done \\[2\\]
- dphy direction 0 - tx had control, 1 - rx has control \\[3\\]
- tx fifo overflow \\[4\\]
- tx fifo underflow \\[5\\]
- rx fifo overflow \\[6\\]
- rx fifo underflow \\[7\\]
- rx packet header has been received \\[8\\]
- all rx packet payload data has been received"]
    #[inline(always)]
    pub fn dsi_host_pkt_status(&self) -> DSI_HOST_PKT_STATUS_R {
        DSI_HOST_PKT_STATUS_R::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_host_pkt_status](index.html) module"]
pub struct DSI_HOST_PKT_STATUS_SPEC;
impl crate::RegisterSpec for DSI_HOST_PKT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_host_pkt_status::R](R) reader structure"]
impl crate::Readable for DSI_HOST_PKT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DSI_HOST_PKT_STATUS to value 0"]
impl crate::Resettable for DSI_HOST_PKT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
