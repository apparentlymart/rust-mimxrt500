#[doc = "Register `DSI_HOST_IRQ_STATUS2` reader"]
pub struct R(crate::R<DSI_HOST_IRQ_STATUS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_HOST_IRQ_STATUS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_HOST_IRQ_STATUS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_HOST_IRQ_STATUS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `dsi_host_irq_status2` reader - Status of APB to packet interface part 2, read part 2 first then dsi_host_irq_status."]
pub type DSI_HOST_IRQ_STATUS2_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Status of APB to packet interface part 2, read part 2 first then dsi_host_irq_status."]
    #[inline(always)]
    pub fn dsi_host_irq_status2(&self) -> DSI_HOST_IRQ_STATUS2_R {
        DSI_HOST_IRQ_STATUS2_R::new((self.bits & 7) as u8)
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_host_irq_status2](index.html) module"]
pub struct DSI_HOST_IRQ_STATUS2_SPEC;
impl crate::RegisterSpec for DSI_HOST_IRQ_STATUS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_host_irq_status2::R](R) reader structure"]
impl crate::Readable for DSI_HOST_IRQ_STATUS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DSI_HOST_IRQ_STATUS2 to value 0"]
impl crate::Resettable for DSI_HOST_IRQ_STATUS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
