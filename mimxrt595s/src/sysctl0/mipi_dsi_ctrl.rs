#[doc = "Register `MIPI_DSI_CTRL` reader"]
pub struct R(crate::R<MIPI_DSI_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIPI_DSI_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIPI_DSI_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIPI_DSI_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIPI_DSI_CTRL` writer"]
pub struct W(crate::W<MIPI_DSI_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIPI_DSI_CTRL_SPEC>;
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
impl From<crate::W<MIPI_DSI_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIPI_DSI_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSI_SD` reader - DSI Shutdown Control."]
pub type DSI_SD_R = crate::BitReader<DSI_SD_A>;
#[doc = "DSI Shutdown Control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSI_SD_A {
    #[doc = "0: Shutdown command not to be sent to the Type-4 display (default)."]
    DISABLE = 0,
    #[doc = "1: Shutdown command to be sent to the Type-4 display"]
    ENABLE = 1,
}
impl From<DSI_SD_A> for bool {
    #[inline(always)]
    fn from(variant: DSI_SD_A) -> Self {
        variant as u8 != 0
    }
}
impl DSI_SD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSI_SD_A {
        match self.bits {
            false => DSI_SD_A::DISABLE,
            true => DSI_SD_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DSI_SD_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DSI_SD_A::ENABLE
    }
}
#[doc = "Field `DSI_SD` writer - DSI Shutdown Control."]
pub type DSI_SD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIPI_DSI_CTRL_SPEC, DSI_SD_A, O>;
impl<'a, const O: u8> DSI_SD_W<'a, O> {
    #[doc = "Shutdown command not to be sent to the Type-4 display (default)."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DSI_SD_A::DISABLE)
    }
    #[doc = "Shutdown command to be sent to the Type-4 display"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DSI_SD_A::ENABLE)
    }
}
#[doc = "Field `DSI_CM` reader - DSI Color Mode Control."]
pub type DSI_CM_R = crate::BitReader<DSI_CM_A>;
#[doc = "DSI Color Mode Control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSI_CM_A {
    #[doc = "0: Normal mode (full color) (default)"]
    ENABLE = 0,
    #[doc = "1: Low color mode (8-bit)"]
    DISABLE = 1,
}
impl From<DSI_CM_A> for bool {
    #[inline(always)]
    fn from(variant: DSI_CM_A) -> Self {
        variant as u8 != 0
    }
}
impl DSI_CM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSI_CM_A {
        match self.bits {
            false => DSI_CM_A::ENABLE,
            true => DSI_CM_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DSI_CM_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DSI_CM_A::DISABLE
    }
}
#[doc = "Field `DSI_CM` writer - DSI Color Mode Control."]
pub type DSI_CM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIPI_DSI_CTRL_SPEC, DSI_CM_A, O>;
impl<'a, const O: u8> DSI_CM_W<'a, O> {
    #[doc = "Normal mode (full color) (default)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DSI_CM_A::ENABLE)
    }
    #[doc = "Low color mode (8-bit)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DSI_CM_A::DISABLE)
    }
}
#[doc = "Field `DSI_TX_ACTIVE` reader - DSI TX ACTIVE"]
pub type DSI_TX_ACTIVE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - DSI Shutdown Control."]
    #[inline(always)]
    pub fn dsi_sd(&self) -> DSI_SD_R {
        DSI_SD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSI Color Mode Control."]
    #[inline(always)]
    pub fn dsi_cm(&self) -> DSI_CM_R {
        DSI_CM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DSI TX ACTIVE"]
    #[inline(always)]
    pub fn dsi_tx_active(&self) -> DSI_TX_ACTIVE_R {
        DSI_TX_ACTIVE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DSI Shutdown Control."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_sd(&mut self) -> DSI_SD_W<0> {
        DSI_SD_W::new(self)
    }
    #[doc = "Bit 1 - DSI Color Mode Control."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_cm(&mut self) -> DSI_CM_W<1> {
        DSI_CM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIPI DSI Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mipi_dsi_ctrl](index.html) module"]
pub struct MIPI_DSI_CTRL_SPEC;
impl crate::RegisterSpec for MIPI_DSI_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mipi_dsi_ctrl::R](R) reader structure"]
impl crate::Readable for MIPI_DSI_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mipi_dsi_ctrl::W](W) writer structure"]
impl crate::Writable for MIPI_DSI_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIPI_DSI_CTRL to value 0"]
impl crate::Resettable for MIPI_DSI_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
