#[doc = "Register `DisplayDitherConfig0` reader"]
pub struct R(crate::R<DISPLAY_DITHER_CONFIG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DISPLAY_DITHER_CONFIG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DISPLAY_DITHER_CONFIG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DISPLAY_DITHER_CONFIG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DisplayDitherConfig0` writer"]
pub struct W(crate::W<DISPLAY_DITHER_CONFIG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DISPLAY_DITHER_CONFIG0_SPEC>;
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
impl From<crate::W<DISPLAY_DITHER_CONFIG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DISPLAY_DITHER_CONFIG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLUE_SIZE` reader - Blue Size"]
pub type BLUE_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BLUE_SIZE` writer - Blue Size"]
pub type BLUE_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DISPLAY_DITHER_CONFIG0_SPEC, u8, u8, 4, O>;
#[doc = "Field `GREEN_SIZE` reader - Green Size"]
pub type GREEN_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GREEN_SIZE` writer - Green Size"]
pub type GREEN_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DISPLAY_DITHER_CONFIG0_SPEC, u8, u8, 4, O>;
#[doc = "Field `RED_SIZE` reader - Red Size"]
pub type RED_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RED_SIZE` writer - Red Size"]
pub type RED_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DISPLAY_DITHER_CONFIG0_SPEC, u8, u8, 4, O>;
#[doc = "Field `ENABLE` reader - Enable Dithering"]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "Enable Dithering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLE,
            true => ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_A::ENABLE
    }
}
#[doc = "Field `ENABLE` writer - Enable Dithering"]
pub type ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DISPLAY_DITHER_CONFIG0_SPEC, ENABLE_A, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:3 - Blue Size"]
    #[inline(always)]
    pub fn blue_size(&self) -> BLUE_SIZE_R {
        BLUE_SIZE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Green Size"]
    #[inline(always)]
    pub fn green_size(&self) -> GREEN_SIZE_R {
        GREEN_SIZE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Red Size"]
    #[inline(always)]
    pub fn red_size(&self) -> RED_SIZE_R {
        RED_SIZE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Enable Dithering"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Blue Size"]
    #[inline(always)]
    #[must_use]
    pub fn blue_size(&mut self) -> BLUE_SIZE_W<0> {
        BLUE_SIZE_W::new(self)
    }
    #[doc = "Bits 8:11 - Green Size"]
    #[inline(always)]
    #[must_use]
    pub fn green_size(&mut self) -> GREEN_SIZE_W<8> {
        GREEN_SIZE_W::new(self)
    }
    #[doc = "Bits 16:19 - Red Size"]
    #[inline(always)]
    #[must_use]
    pub fn red_size(&mut self) -> RED_SIZE_W<16> {
        RED_SIZE_W::new(self)
    }
    #[doc = "Bit 31 - Enable Dithering"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<31> {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration for Dithering\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [display_dither_config0](index.html) module"]
pub struct DISPLAY_DITHER_CONFIG0_SPEC;
impl crate::RegisterSpec for DISPLAY_DITHER_CONFIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [display_dither_config0::R](R) reader structure"]
impl crate::Readable for DISPLAY_DITHER_CONFIG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [display_dither_config0::W](W) writer structure"]
impl crate::Writable for DISPLAY_DITHER_CONFIG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DisplayDitherConfig0 to value 0"]
impl crate::Resettable for DISPLAY_DITHER_CONFIG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
