#[doc = "Register `DSI_HOST_CFG_DPI_PIXEL_FORMAT` reader"]
pub struct R(crate::R<DSI_HOST_CFG_DPI_PIXEL_FORMAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_HOST_CFG_DPI_PIXEL_FORMAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_HOST_CFG_DPI_PIXEL_FORMAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_HOST_CFG_DPI_PIXEL_FORMAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSI_HOST_CFG_DPI_PIXEL_FORMAT` writer"]
pub struct W(crate::W<DSI_HOST_CFG_DPI_PIXEL_FORMAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSI_HOST_CFG_DPI_PIXEL_FORMAT_SPEC>;
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
impl From<crate::W<DSI_HOST_CFG_DPI_PIXEL_FORMAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSI_HOST_CFG_DPI_PIXEL_FORMAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dsi_host_cfg_dpi_pixel_format` reader - Sets the DSI packet type of the pixels."]
pub type DSI_HOST_CFG_DPI_PIXEL_FORMAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dsi_host_cfg_dpi_pixel_format` writer - Sets the DSI packet type of the pixels."]
pub type DSI_HOST_CFG_DPI_PIXEL_FORMAT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DSI_HOST_CFG_DPI_PIXEL_FORMAT_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Sets the DSI packet type of the pixels."]
    #[inline(always)]
    pub fn dsi_host_cfg_dpi_pixel_format(&self) -> DSI_HOST_CFG_DPI_PIXEL_FORMAT_R {
        DSI_HOST_CFG_DPI_PIXEL_FORMAT_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets the DSI packet type of the pixels."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_host_cfg_dpi_pixel_format(&mut self) -> DSI_HOST_CFG_DPI_PIXEL_FORMAT_W<0> {
        DSI_HOST_CFG_DPI_PIXEL_FORMAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_host_cfg_dpi_pixel_format](index.html) module"]
pub struct DSI_HOST_CFG_DPI_PIXEL_FORMAT_SPEC;
impl crate::RegisterSpec for DSI_HOST_CFG_DPI_PIXEL_FORMAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_host_cfg_dpi_pixel_format::R](R) reader structure"]
impl crate::Readable for DSI_HOST_CFG_DPI_PIXEL_FORMAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsi_host_cfg_dpi_pixel_format::W](W) writer structure"]
impl crate::Writable for DSI_HOST_CFG_DPI_PIXEL_FORMAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_HOST_CFG_DPI_PIXEL_FORMAT to value 0"]
impl crate::Resettable for DSI_HOST_CFG_DPI_PIXEL_FORMAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
