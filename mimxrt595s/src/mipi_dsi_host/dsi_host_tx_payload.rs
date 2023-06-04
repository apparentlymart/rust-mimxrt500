#[doc = "Register `DSI_HOST_TX_PAYLOAD` reader"]
pub struct R(crate::R<DSI_HOST_TX_PAYLOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_HOST_TX_PAYLOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_HOST_TX_PAYLOAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_HOST_TX_PAYLOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSI_HOST_TX_PAYLOAD` writer"]
pub struct W(crate::W<DSI_HOST_TX_PAYLOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSI_HOST_TX_PAYLOAD_SPEC>;
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
impl From<crate::W<DSI_HOST_TX_PAYLOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSI_HOST_TX_PAYLOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dsi_host_tx_payload` reader - Tx Payload data write register."]
pub type DSI_HOST_TX_PAYLOAD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `dsi_host_tx_payload` writer - Tx Payload data write register."]
pub type DSI_HOST_TX_PAYLOAD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DSI_HOST_TX_PAYLOAD_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Tx Payload data write register."]
    #[inline(always)]
    pub fn dsi_host_tx_payload(&self) -> DSI_HOST_TX_PAYLOAD_R {
        DSI_HOST_TX_PAYLOAD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Tx Payload data write register."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_host_tx_payload(&mut self) -> DSI_HOST_TX_PAYLOAD_W<0> {
        DSI_HOST_TX_PAYLOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_host_tx_payload](index.html) module"]
pub struct DSI_HOST_TX_PAYLOAD_SPEC;
impl crate::RegisterSpec for DSI_HOST_TX_PAYLOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_host_tx_payload::R](R) reader structure"]
impl crate::Readable for DSI_HOST_TX_PAYLOAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsi_host_tx_payload::W](W) writer structure"]
impl crate::Writable for DSI_HOST_TX_PAYLOAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_HOST_TX_PAYLOAD to value 0"]
impl crate::Resettable for DSI_HOST_TX_PAYLOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
