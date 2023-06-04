#[doc = "Register `DSI_HOST_RX_ERROR_STATUS` reader"]
pub struct R(crate::R<DSI_HOST_RX_ERROR_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_HOST_RX_ERROR_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_HOST_RX_ERROR_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_HOST_RX_ERROR_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `dsi_host_rx_error_status` reader - Status Register for Host receive error detection, ECC errors, CRC errors and for timeout indicators \\[0\\]
ECC single bit error detected \\[1\\]
ECC multi bit error detected \\[6:2\\]
Errored bit position for single bit ECC error \\[7\\]
CRC error detected \\[8\\]
High Speed forward TX timeout detected \\[9\\]
Reverse Low power data receive timeout detected \\[10\\]
BTA timeout detected"]
pub type DSI_HOST_RX_ERROR_STATUS_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - Status Register for Host receive error detection, ECC errors, CRC errors and for timeout indicators \\[0\\]
ECC single bit error detected \\[1\\]
ECC multi bit error detected \\[6:2\\]
Errored bit position for single bit ECC error \\[7\\]
CRC error detected \\[8\\]
High Speed forward TX timeout detected \\[9\\]
Reverse Low power data receive timeout detected \\[10\\]
BTA timeout detected"]
    #[inline(always)]
    pub fn dsi_host_rx_error_status(&self) -> DSI_HOST_RX_ERROR_STATUS_R {
        DSI_HOST_RX_ERROR_STATUS_R::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_host_rx_error_status](index.html) module"]
pub struct DSI_HOST_RX_ERROR_STATUS_SPEC;
impl crate::RegisterSpec for DSI_HOST_RX_ERROR_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_host_rx_error_status::R](R) reader structure"]
impl crate::Readable for DSI_HOST_RX_ERROR_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DSI_HOST_RX_ERROR_STATUS to value 0"]
impl crate::Resettable for DSI_HOST_RX_ERROR_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
