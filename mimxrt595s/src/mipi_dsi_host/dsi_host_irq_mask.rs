#[doc = "Register `DSI_HOST_IRQ_MASK` reader"]
pub struct R(crate::R<DSI_HOST_IRQ_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_HOST_IRQ_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_HOST_IRQ_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_HOST_IRQ_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSI_HOST_IRQ_MASK` writer"]
pub struct W(crate::W<DSI_HOST_IRQ_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSI_HOST_IRQ_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DSI_HOST_IRQ_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSI_HOST_IRQ_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dsi_host_irq_mask` reader - irq mask \\[0\\]
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
pub type DSI_HOST_IRQ_MASK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `dsi_host_irq_mask` writer - irq mask \\[0\\]
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
pub type DSI_HOST_IRQ_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DSI_HOST_IRQ_MASK_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - irq mask \\[0\\]
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
    pub fn dsi_host_irq_mask(&self) -> DSI_HOST_IRQ_MASK_R {
        DSI_HOST_IRQ_MASK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - irq mask \\[0\\]
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
    #[must_use]
    pub fn dsi_host_irq_mask(&mut self) -> DSI_HOST_IRQ_MASK_W<0> {
        DSI_HOST_IRQ_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_host_irq_mask](index.html) module"]
pub struct DSI_HOST_IRQ_MASK_SPEC;
impl crate::RegisterSpec for DSI_HOST_IRQ_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_host_irq_mask::R](R) reader structure"]
impl crate::Readable for DSI_HOST_IRQ_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsi_host_irq_mask::W](W) writer structure"]
impl crate::Writable for DSI_HOST_IRQ_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_HOST_IRQ_MASK to value 0"]
impl crate::Resettable for DSI_HOST_IRQ_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
