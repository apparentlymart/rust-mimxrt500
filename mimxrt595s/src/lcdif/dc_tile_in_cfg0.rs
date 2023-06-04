#[doc = "Register `DcTileInCfg0` reader"]
pub struct R(crate::R<DC_TILE_IN_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_TILE_IN_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_TILE_IN_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_TILE_IN_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DcTileInCfg0` writer"]
pub struct W(crate::W<DC_TILE_IN_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DC_TILE_IN_CFG0_SPEC>;
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
impl From<crate::W<DC_TILE_IN_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DC_TILE_IN_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TILE_FORMAT` reader - Tile Format"]
pub type TILE_FORMAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TILE_FORMAT` writer - Tile Format"]
pub type TILE_FORMAT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DC_TILE_IN_CFG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `YUV_STANDARD` reader - YUV Standard Select"]
pub type YUV_STANDARD_R = crate::FieldReader<u8, YUV_STANDARD_A>;
#[doc = "YUV Standard Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum YUV_STANDARD_A {
    #[doc = "0: BT601"]
    DISABLE = 0,
    #[doc = "1: BT709"]
    ENABLE = 1,
}
impl From<YUV_STANDARD_A> for u8 {
    #[inline(always)]
    fn from(variant: YUV_STANDARD_A) -> Self {
        variant as _
    }
}
impl YUV_STANDARD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<YUV_STANDARD_A> {
        match self.bits {
            0 => Some(YUV_STANDARD_A::DISABLE),
            1 => Some(YUV_STANDARD_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == YUV_STANDARD_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == YUV_STANDARD_A::ENABLE
    }
}
#[doc = "Field `YUV_STANDARD` writer - YUV Standard Select"]
pub type YUV_STANDARD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DC_TILE_IN_CFG0_SPEC, u8, YUV_STANDARD_A, 2, O>;
impl<'a, const O: u8> YUV_STANDARD_W<'a, O> {
    #[doc = "BT601"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(YUV_STANDARD_A::DISABLE)
    }
    #[doc = "BT709"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(YUV_STANDARD_A::ENABLE)
    }
}
#[doc = "Field `YUV2_RGB_EN` reader - YUV2RGB Module Enable"]
pub type YUV2_RGB_EN_R = crate::BitReader<YUV2_RGB_EN_A>;
#[doc = "YUV2RGB Module Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum YUV2_RGB_EN_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<YUV2_RGB_EN_A> for bool {
    #[inline(always)]
    fn from(variant: YUV2_RGB_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl YUV2_RGB_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> YUV2_RGB_EN_A {
        match self.bits {
            false => YUV2_RGB_EN_A::DISABLE,
            true => YUV2_RGB_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == YUV2_RGB_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == YUV2_RGB_EN_A::ENABLE
    }
}
#[doc = "Field `YUV2_RGB_EN` writer - YUV2RGB Module Enable"]
pub type YUV2_RGB_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DC_TILE_IN_CFG0_SPEC, YUV2_RGB_EN_A, O>;
impl<'a, const O: u8> YUV2_RGB_EN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(YUV2_RGB_EN_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(YUV2_RGB_EN_A::ENABLE)
    }
}
#[doc = "Field `CFG_MODE_EN` reader - Configuration Mode Enable."]
pub type CFG_MODE_EN_R = crate::BitReader<CFG_MODE_EN_A>;
#[doc = "Configuration Mode Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFG_MODE_EN_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<CFG_MODE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CFG_MODE_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CFG_MODE_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG_MODE_EN_A {
        match self.bits {
            false => CFG_MODE_EN_A::DISABLE,
            true => CFG_MODE_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CFG_MODE_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CFG_MODE_EN_A::ENABLE
    }
}
#[doc = "Field `CFG_MODE_EN` writer - Configuration Mode Enable."]
pub type CFG_MODE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DC_TILE_IN_CFG0_SPEC, CFG_MODE_EN_A, O>;
impl<'a, const O: u8> CFG_MODE_EN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CFG_MODE_EN_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CFG_MODE_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Tile Format"]
    #[inline(always)]
    pub fn tile_format(&self) -> TILE_FORMAT_R {
        TILE_FORMAT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - YUV Standard Select"]
    #[inline(always)]
    pub fn yuv_standard(&self) -> YUV_STANDARD_R {
        YUV_STANDARD_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - YUV2RGB Module Enable"]
    #[inline(always)]
    pub fn yuv2_rgb_en(&self) -> YUV2_RGB_EN_R {
        YUV2_RGB_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configuration Mode Enable."]
    #[inline(always)]
    pub fn cfg_mode_en(&self) -> CFG_MODE_EN_R {
        CFG_MODE_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Tile Format"]
    #[inline(always)]
    #[must_use]
    pub fn tile_format(&mut self) -> TILE_FORMAT_W<0> {
        TILE_FORMAT_W::new(self)
    }
    #[doc = "Bits 2:3 - YUV Standard Select"]
    #[inline(always)]
    #[must_use]
    pub fn yuv_standard(&mut self) -> YUV_STANDARD_W<2> {
        YUV_STANDARD_W::new(self)
    }
    #[doc = "Bit 4 - YUV2RGB Module Enable"]
    #[inline(always)]
    #[must_use]
    pub fn yuv2_rgb_en(&mut self) -> YUV2_RGB_EN_W<4> {
        YUV2_RGB_EN_W::new(self)
    }
    #[doc = "Bit 5 - Configuration Mode Enable."]
    #[inline(always)]
    #[must_use]
    pub fn cfg_mode_en(&mut self) -> CFG_MODE_EN_W<5> {
        CFG_MODE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tile Input Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_tile_in_cfg0](index.html) module"]
pub struct DC_TILE_IN_CFG0_SPEC;
impl crate::RegisterSpec for DC_TILE_IN_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dc_tile_in_cfg0::R](R) reader structure"]
impl crate::Readable for DC_TILE_IN_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dc_tile_in_cfg0::W](W) writer structure"]
impl crate::Writable for DC_TILE_IN_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DcTileInCfg0 to value 0"]
impl crate::Resettable for DC_TILE_IN_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
