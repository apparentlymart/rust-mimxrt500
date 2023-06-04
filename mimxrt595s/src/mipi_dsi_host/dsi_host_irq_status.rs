#[doc = "Register `DSI_HOST_IRQ_STATUS` reader"]
pub struct R(crate::R<DSI_HOST_IRQ_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_HOST_IRQ_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_HOST_IRQ_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_HOST_IRQ_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `dsi_host_irq_status` reader - Status of APB to packet interface \\[0\\]
- state machine not idle \\[1\\]
- Tx packet done \\[2\\]
- dphy direction 0 - tx had control, 1 - rx has control \\[3\\]
- tx fifo overflow \\[4\\]
- tx fifo underflow \\[5\\]
- rx fifo overflow \\[6\\]
- rx fifo underflow \\[7\\]
- rx packet header has been received \\[8\\]
- all rx packet payload data has been received \\[28:9\\]
- map directory to dsi host controller status_out port bit descriptions \\[29\\]
- high speed tx timeout, host controller hs_tx_timeout port \\[30\\]
- low power rx timeout, host controller lp_rx_timeout port \\[31\\]
- host bta timeout, host controller host_bta_timeout port"]
pub type DSI_HOST_IRQ_STATUS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Status of APB to packet interface \\[0\\]
- state machine not idle \\[1\\]
- Tx packet done \\[2\\]
- dphy direction 0 - tx had control, 1 - rx has control \\[3\\]
- tx fifo overflow \\[4\\]
- tx fifo underflow \\[5\\]
- rx fifo overflow \\[6\\]
- rx fifo underflow \\[7\\]
- rx packet header has been received \\[8\\]
- all rx packet payload data has been received \\[28:9\\]
- map directory to dsi host controller status_out port bit descriptions \\[29\\]
- high speed tx timeout, host controller hs_tx_timeout port \\[30\\]
- low power rx timeout, host controller lp_rx_timeout port \\[31\\]
- host bta timeout, host controller host_bta_timeout port"]
    #[inline(always)]
    pub fn dsi_host_irq_status(&self) -> DSI_HOST_IRQ_STATUS_R {
        DSI_HOST_IRQ_STATUS_R::new(self.bits)
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_host_irq_status](index.html) module"]
pub struct DSI_HOST_IRQ_STATUS_SPEC;
impl crate::RegisterSpec for DSI_HOST_IRQ_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_host_irq_status::R](R) reader structure"]
impl crate::Readable for DSI_HOST_IRQ_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DSI_HOST_IRQ_STATUS to value 0"]
impl crate::Resettable for DSI_HOST_IRQ_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
