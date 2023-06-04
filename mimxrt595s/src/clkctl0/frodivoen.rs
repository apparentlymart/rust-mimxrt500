#[doc = "Register `FRODIVOEN` reader"]
pub struct R(crate::R<FRODIVOEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRODIVOEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRODIVOEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRODIVOEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRODIVOEN` writer"]
pub struct W(crate::W<FRODIVOEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRODIVOEN_SPEC>;
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
impl From<crate::W<FRODIVOEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRODIVOEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRO_DIV1_O_EN` reader - FRO Divided-by-1 Clock Enable"]
pub type FRO_DIV1_O_EN_R = crate::BitReader<FRO_DIV1_O_EN_A>;
#[doc = "FRO Divided-by-1 Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRO_DIV1_O_EN_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<FRO_DIV1_O_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FRO_DIV1_O_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FRO_DIV1_O_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRO_DIV1_O_EN_A {
        match self.bits {
            false => FRO_DIV1_O_EN_A::CLK_DISABLE,
            true => FRO_DIV1_O_EN_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == FRO_DIV1_O_EN_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == FRO_DIV1_O_EN_A::CLK_ENABLE
    }
}
#[doc = "Field `FRO_DIV1_O_EN` writer - FRO Divided-by-1 Clock Enable"]
pub type FRO_DIV1_O_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FRODIVOEN_SPEC, FRO_DIV1_O_EN_A, O>;
impl<'a, const O: u8> FRO_DIV1_O_EN_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(FRO_DIV1_O_EN_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(FRO_DIV1_O_EN_A::CLK_ENABLE)
    }
}
#[doc = "Field `FRO_DIV2_O_EN` reader - FRO Divided-by-2 Clock Enable"]
pub type FRO_DIV2_O_EN_R = crate::BitReader<FRO_DIV2_O_EN_A>;
#[doc = "FRO Divided-by-2 Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRO_DIV2_O_EN_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<FRO_DIV2_O_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FRO_DIV2_O_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FRO_DIV2_O_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRO_DIV2_O_EN_A {
        match self.bits {
            false => FRO_DIV2_O_EN_A::CLK_DISABLE,
            true => FRO_DIV2_O_EN_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == FRO_DIV2_O_EN_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == FRO_DIV2_O_EN_A::CLK_ENABLE
    }
}
#[doc = "Field `FRO_DIV2_O_EN` writer - FRO Divided-by-2 Clock Enable"]
pub type FRO_DIV2_O_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FRODIVOEN_SPEC, FRO_DIV2_O_EN_A, O>;
impl<'a, const O: u8> FRO_DIV2_O_EN_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(FRO_DIV2_O_EN_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(FRO_DIV2_O_EN_A::CLK_ENABLE)
    }
}
#[doc = "Field `FRO_DIV4_O_EN` reader - FRO Divided-by-4 Clock Enable"]
pub type FRO_DIV4_O_EN_R = crate::BitReader<FRO_DIV4_O_EN_A>;
#[doc = "FRO Divided-by-4 Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRO_DIV4_O_EN_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<FRO_DIV4_O_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FRO_DIV4_O_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FRO_DIV4_O_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRO_DIV4_O_EN_A {
        match self.bits {
            false => FRO_DIV4_O_EN_A::CLK_DISABLE,
            true => FRO_DIV4_O_EN_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == FRO_DIV4_O_EN_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == FRO_DIV4_O_EN_A::CLK_ENABLE
    }
}
#[doc = "Field `FRO_DIV4_O_EN` writer - FRO Divided-by-4 Clock Enable"]
pub type FRO_DIV4_O_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FRODIVOEN_SPEC, FRO_DIV4_O_EN_A, O>;
impl<'a, const O: u8> FRO_DIV4_O_EN_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(FRO_DIV4_O_EN_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(FRO_DIV4_O_EN_A::CLK_ENABLE)
    }
}
#[doc = "Field `FRO_DIV8_O_EN` reader - FRO Divided-by-8 Clock Enable"]
pub type FRO_DIV8_O_EN_R = crate::BitReader<FRO_DIV8_O_EN_A>;
#[doc = "FRO Divided-by-8 Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRO_DIV8_O_EN_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<FRO_DIV8_O_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FRO_DIV8_O_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FRO_DIV8_O_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRO_DIV8_O_EN_A {
        match self.bits {
            false => FRO_DIV8_O_EN_A::CLK_DISABLE,
            true => FRO_DIV8_O_EN_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == FRO_DIV8_O_EN_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == FRO_DIV8_O_EN_A::CLK_ENABLE
    }
}
#[doc = "Field `FRO_DIV8_O_EN` writer - FRO Divided-by-8 Clock Enable"]
pub type FRO_DIV8_O_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FRODIVOEN_SPEC, FRO_DIV8_O_EN_A, O>;
impl<'a, const O: u8> FRO_DIV8_O_EN_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(FRO_DIV8_O_EN_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(FRO_DIV8_O_EN_A::CLK_ENABLE)
    }
}
#[doc = "Field `FRO_DIV16_O_EN` reader - FRO Divided-by-16 Clock Enable"]
pub type FRO_DIV16_O_EN_R = crate::BitReader<FRO_DIV16_O_EN_A>;
#[doc = "FRO Divided-by-16 Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRO_DIV16_O_EN_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<FRO_DIV16_O_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FRO_DIV16_O_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FRO_DIV16_O_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRO_DIV16_O_EN_A {
        match self.bits {
            false => FRO_DIV16_O_EN_A::CLK_DISABLE,
            true => FRO_DIV16_O_EN_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == FRO_DIV16_O_EN_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == FRO_DIV16_O_EN_A::CLK_ENABLE
    }
}
#[doc = "Field `FRO_DIV16_O_EN` writer - FRO Divided-by-16 Clock Enable"]
pub type FRO_DIV16_O_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FRODIVOEN_SPEC, FRO_DIV16_O_EN_A, O>;
impl<'a, const O: u8> FRO_DIV16_O_EN_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(FRO_DIV16_O_EN_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(FRO_DIV16_O_EN_A::CLK_ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - FRO Divided-by-1 Clock Enable"]
    #[inline(always)]
    pub fn fro_div1_o_en(&self) -> FRO_DIV1_O_EN_R {
        FRO_DIV1_O_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FRO Divided-by-2 Clock Enable"]
    #[inline(always)]
    pub fn fro_div2_o_en(&self) -> FRO_DIV2_O_EN_R {
        FRO_DIV2_O_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FRO Divided-by-4 Clock Enable"]
    #[inline(always)]
    pub fn fro_div4_o_en(&self) -> FRO_DIV4_O_EN_R {
        FRO_DIV4_O_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FRO Divided-by-8 Clock Enable"]
    #[inline(always)]
    pub fn fro_div8_o_en(&self) -> FRO_DIV8_O_EN_R {
        FRO_DIV8_O_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FRO Divided-by-16 Clock Enable"]
    #[inline(always)]
    pub fn fro_div16_o_en(&self) -> FRO_DIV16_O_EN_R {
        FRO_DIV16_O_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FRO Divided-by-1 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fro_div1_o_en(&mut self) -> FRO_DIV1_O_EN_W<0> {
        FRO_DIV1_O_EN_W::new(self)
    }
    #[doc = "Bit 1 - FRO Divided-by-2 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fro_div2_o_en(&mut self) -> FRO_DIV2_O_EN_W<1> {
        FRO_DIV2_O_EN_W::new(self)
    }
    #[doc = "Bit 2 - FRO Divided-by-4 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fro_div4_o_en(&mut self) -> FRO_DIV4_O_EN_W<2> {
        FRO_DIV4_O_EN_W::new(self)
    }
    #[doc = "Bit 3 - FRO Divided-by-8 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fro_div8_o_en(&mut self) -> FRO_DIV8_O_EN_W<3> {
        FRO_DIV8_O_EN_W::new(self)
    }
    #[doc = "Bit 4 - FRO Divided-by-16 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fro_div16_o_en(&mut self) -> FRO_DIV16_O_EN_W<4> {
        FRO_DIV16_O_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FRO Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frodivoen](index.html) module"]
pub struct FRODIVOEN_SPEC;
impl crate::RegisterSpec for FRODIVOEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frodivoen::R](R) reader structure"]
impl crate::Readable for FRODIVOEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frodivoen::W](W) writer structure"]
impl crate::Writable for FRODIVOEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRODIVOEN to value 0"]
impl crate::Resettable for FRODIVOEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
