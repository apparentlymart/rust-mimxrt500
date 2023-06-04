#[doc = "Register `DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP` reader"]
pub struct R(crate::R<DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP` writer"]
pub struct W(crate::W<DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP_SPEC>;
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
impl From<crate::W<DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dsi_host_cfg_dpi_use_null_pkt_bllp` reader - Selects type of blanking packet to be sent during bllp region 0 - Blanking packet used in bllp region 1 - Null packet used in bllp region"]
pub type DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP_R = crate::BitReader<bool>;
#[doc = "Field `dsi_host_cfg_dpi_use_null_pkt_bllp` writer - Selects type of blanking packet to be sent during bllp region 0 - Blanking packet used in bllp region 1 - Null packet used in bllp region"]
pub type DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Selects type of blanking packet to be sent during bllp region 0 - Blanking packet used in bllp region 1 - Null packet used in bllp region"]
    #[inline(always)]
    pub fn dsi_host_cfg_dpi_use_null_pkt_bllp(&self) -> DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP_R {
        DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects type of blanking packet to be sent during bllp region 0 - Blanking packet used in bllp region 1 - Null packet used in bllp region"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_host_cfg_dpi_use_null_pkt_bllp(
        &mut self,
    ) -> DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP_W<0> {
        DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_host_cfg_dpi_use_null_pkt_bllp](index.html) module"]
pub struct DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP_SPEC;
impl crate::RegisterSpec for DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_host_cfg_dpi_use_null_pkt_bllp::R](R) reader structure"]
impl crate::Readable for DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsi_host_cfg_dpi_use_null_pkt_bllp::W](W) writer structure"]
impl crate::Writable for DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP to value 0"]
impl crate::Resettable for DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
