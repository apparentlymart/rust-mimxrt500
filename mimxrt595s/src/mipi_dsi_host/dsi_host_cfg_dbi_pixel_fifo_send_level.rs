#[doc = "Register `DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL` reader"]
pub struct R(crate::R<DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL` writer"]
pub struct W(crate::W<DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL_SPEC>;
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
impl From<crate::W<DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dsi_host_cfg_dbi_pixel_fifo_send_level` reader - In order to optimize DSI utility, the DBI bridge buffers a cerntain number of DBI pixels before initiating a DSI packet."]
pub type DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `dsi_host_cfg_dbi_pixel_fifo_send_level` writer - In order to optimize DSI utility, the DBI bridge buffers a cerntain number of DBI pixels before initiating a DSI packet."]
pub type DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - In order to optimize DSI utility, the DBI bridge buffers a cerntain number of DBI pixels before initiating a DSI packet."]
    #[inline(always)]
    pub fn dsi_host_cfg_dbi_pixel_fifo_send_level(
        &self,
    ) -> DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL_R {
        DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - In order to optimize DSI utility, the DBI bridge buffers a cerntain number of DBI pixels before initiating a DSI packet."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_host_cfg_dbi_pixel_fifo_send_level(
        &mut self,
    ) -> DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL_W<0> {
        DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_host_cfg_dbi_pixel_fifo_send_level](index.html) module"]
pub struct DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL_SPEC;
impl crate::RegisterSpec for DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_host_cfg_dbi_pixel_fifo_send_level::R](R) reader structure"]
impl crate::Readable for DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsi_host_cfg_dbi_pixel_fifo_send_level::W](W) writer structure"]
impl crate::Writable for DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL to value 0"]
impl crate::Resettable for DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
