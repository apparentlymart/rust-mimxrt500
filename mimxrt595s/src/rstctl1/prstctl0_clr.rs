#[doc = "Register `PRSTCTL0_CLR` writer"]
pub struct W(crate::W<PRSTCTL0_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRSTCTL0_CLR_SPEC>;
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
impl From<crate::W<PRSTCTL0_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRSTCTL0_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Flexcomm0 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM0_AW {
    #[doc = "0: No effect"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM0_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM0` writer - Flexcomm0 reset clear"]
pub type FLEXCOMM0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_CLR_SPEC, FLEXCOMM0_AW, O>;
impl<'a, const O: u8> FLEXCOMM0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM0_AW::FLEXCOMM_CLR)
    }
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM0_AW::FLEXCOMM_SET)
    }
}
#[doc = "Flexcomm1 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM1_AW {
    #[doc = "0: No effect"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM1_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM1` writer - Flexcomm1 reset clear"]
pub type FLEXCOMM1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_CLR_SPEC, FLEXCOMM1_AW, O>;
impl<'a, const O: u8> FLEXCOMM1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM1_AW::FLEXCOMM_CLR)
    }
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM1_AW::FLEXCOMM_SET)
    }
}
#[doc = "Flexcomm2 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM2_AW {
    #[doc = "0: No effect"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM2_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM2` writer - Flexcomm2 reset clear"]
pub type FLEXCOMM2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_CLR_SPEC, FLEXCOMM2_AW, O>;
impl<'a, const O: u8> FLEXCOMM2_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM2_AW::FLEXCOMM_CLR)
    }
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM2_AW::FLEXCOMM_SET)
    }
}
#[doc = "Flexcomm3 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM3_AW {
    #[doc = "0: No effect"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM3_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM3` writer - Flexcomm3 reset clear"]
pub type FLEXCOMM3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_CLR_SPEC, FLEXCOMM3_AW, O>;
impl<'a, const O: u8> FLEXCOMM3_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM3_AW::FLEXCOMM_CLR)
    }
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM3_AW::FLEXCOMM_SET)
    }
}
#[doc = "Flexcomm4 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM4_AW {
    #[doc = "0: No effect"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM4_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM4` writer - Flexcomm4 reset clear"]
pub type FLEXCOMM4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_CLR_SPEC, FLEXCOMM4_AW, O>;
impl<'a, const O: u8> FLEXCOMM4_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM4_AW::FLEXCOMM_CLR)
    }
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM4_AW::FLEXCOMM_SET)
    }
}
#[doc = "Flexcomm5 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM5_AW {
    #[doc = "0: No effect"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM5_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM5` writer - Flexcomm5 reset clear"]
pub type FLEXCOMM5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_CLR_SPEC, FLEXCOMM5_AW, O>;
impl<'a, const O: u8> FLEXCOMM5_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM5_AW::FLEXCOMM_CLR)
    }
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM5_AW::FLEXCOMM_SET)
    }
}
#[doc = "Flexcomm6 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM6_AW {
    #[doc = "0: No effect"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM6_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM6` writer - Flexcomm6 reset clear"]
pub type FLEXCOMM6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_CLR_SPEC, FLEXCOMM6_AW, O>;
impl<'a, const O: u8> FLEXCOMM6_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM6_AW::FLEXCOMM_CLR)
    }
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM6_AW::FLEXCOMM_SET)
    }
}
#[doc = "Flexcomm7 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM7_AW {
    #[doc = "0: No effect"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM7_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM7` writer - Flexcomm7 reset clear"]
pub type FLEXCOMM7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_CLR_SPEC, FLEXCOMM7_AW, O>;
impl<'a, const O: u8> FLEXCOMM7_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM7_AW::FLEXCOMM_CLR)
    }
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM7_AW::FLEXCOMM_SET)
    }
}
#[doc = "Flexcomm8 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM8_AW {
    #[doc = "0: No effect"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM8_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM8` writer - Flexcomm8 reset clear"]
pub type FLEXCOMM8_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_CLR_SPEC, FLEXCOMM8_AW, O>;
impl<'a, const O: u8> FLEXCOMM8_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM8_AW::FLEXCOMM_CLR)
    }
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM8_AW::FLEXCOMM_SET)
    }
}
#[doc = "Flexcomm9 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM9_AW {
    #[doc = "0: No effect"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM9_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM9` writer - Flexcomm9 reset clear"]
pub type FLEXCOMM9_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_CLR_SPEC, FLEXCOMM9_AW, O>;
impl<'a, const O: u8> FLEXCOMM9_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM9_AW::FLEXCOMM_CLR)
    }
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM9_AW::FLEXCOMM_SET)
    }
}
#[doc = "Flexcomm10 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM10_AW {
    #[doc = "0: No effect"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM10_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM10` writer - Flexcomm10 reset clear"]
pub type FLEXCOMM10_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_CLR_SPEC, FLEXCOMM10_AW, O>;
impl<'a, const O: u8> FLEXCOMM10_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM10_AW::FLEXCOMM_CLR)
    }
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM10_AW::FLEXCOMM_SET)
    }
}
#[doc = "Flexcomm11 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM11_AW {
    #[doc = "0: No effect"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM11_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM11` writer - Flexcomm11 reset clear"]
pub type FLEXCOMM11_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_CLR_SPEC, FLEXCOMM11_AW, O>;
impl<'a, const O: u8> FLEXCOMM11_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM11_AW::FLEXCOMM_CLR)
    }
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM11_AW::FLEXCOMM_SET)
    }
}
#[doc = "Flexcomm12 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM12_AW {
    #[doc = "0: No effect"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM12_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM12` writer - Flexcomm12 reset clear"]
pub type FLEXCOMM12_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_CLR_SPEC, FLEXCOMM12_AW, O>;
impl<'a, const O: u8> FLEXCOMM12_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM12_AW::FLEXCOMM_CLR)
    }
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM12_AW::FLEXCOMM_SET)
    }
}
#[doc = "Flexcomm13 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM13_AW {
    #[doc = "0: No effect"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM13_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM13` writer - Flexcomm13 reset clear"]
pub type FLEXCOMM13_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_CLR_SPEC, FLEXCOMM13_AW, O>;
impl<'a, const O: u8> FLEXCOMM13_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM13_AW::FLEXCOMM_CLR)
    }
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM13_AW::FLEXCOMM_SET)
    }
}
#[doc = "FLexcomm SPI0 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM14_AW {
    #[doc = "0: No effect"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM14_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM14` writer - FLexcomm SPI0 reset clear"]
pub type FLEXCOMM14_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_CLR_SPEC, FLEXCOMM14_AW, O>;
impl<'a, const O: u8> FLEXCOMM14_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM14_AW::FLEXCOMM_CLR)
    }
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM14_AW::FLEXCOMM_SET)
    }
}
#[doc = "Flexcomm I2C reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM15_I2C_AW {
    #[doc = "0: No effect"]
    FLEXCOMM15_I2C_CLR = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    FLEXCOMM15_I2C_SET = 1,
}
impl From<FLEXCOMM15_I2C_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM15_I2C_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM15_I2C` writer - Flexcomm I2C reset clear"]
pub type FLEXCOMM15_I2C_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_CLR_SPEC, FLEXCOMM15_I2C_AW, O>;
impl<'a, const O: u8> FLEXCOMM15_I2C_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn flexcomm15_i2c_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM15_I2C_AW::FLEXCOMM15_I2C_CLR)
    }
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn flexcomm15_i2c_set(self) -> &'a mut W {
        self.variant(FLEXCOMM15_I2C_AW::FLEXCOMM15_I2C_SET)
    }
}
#[doc = "DMIC0 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMIC0_AW {
    #[doc = "0: No effect"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    FLEXCOMM_SET = 1,
}
impl From<DMIC0_AW> for bool {
    #[inline(always)]
    fn from(variant: DMIC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIC0` writer - DMIC0 reset clear"]
pub type DMIC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_CLR_SPEC, DMIC0_AW, O>;
impl<'a, const O: u8> DMIC0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(DMIC0_AW::FLEXCOMM_CLR)
    }
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(DMIC0_AW::FLEXCOMM_SET)
    }
}
#[doc = "Flexcomm SPI1 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM16_AW {
    #[doc = "0: No effect"]
    FLEXCOMM16_SPI1_CLR = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    FLEXCOMM16_SPI1_SET = 1,
}
impl From<FLEXCOMM16_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM16_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM16` writer - Flexcomm SPI1 reset clear"]
pub type FLEXCOMM16_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_CLR_SPEC, FLEXCOMM16_AW, O>;
impl<'a, const O: u8> FLEXCOMM16_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn flexcomm16_spi1_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM16_AW::FLEXCOMM16_SPI1_CLR)
    }
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn flexcomm16_spi1_set(self) -> &'a mut W {
        self.variant(FLEXCOMM16_AW::FLEXCOMM16_SPI1_SET)
    }
}
#[doc = "OSEVENT Timer reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSEVENT_TIMER_AW {
    #[doc = "0: No effect"]
    OSEVENT_TIMER_CLR = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    OSEVENT_TIMER_SET = 1,
}
impl From<OSEVENT_TIMER_AW> for bool {
    #[inline(always)]
    fn from(variant: OSEVENT_TIMER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSEVENT_TIMER` writer - OSEVENT Timer reset clear"]
pub type OSEVENT_TIMER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_CLR_SPEC, OSEVENT_TIMER_AW, O>;
impl<'a, const O: u8> OSEVENT_TIMER_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn osevent_timer_clr(self) -> &'a mut W {
        self.variant(OSEVENT_TIMER_AW::OSEVENT_TIMER_CLR)
    }
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn osevent_timer_set(self) -> &'a mut W {
        self.variant(OSEVENT_TIMER_AW::OSEVENT_TIMER_SET)
    }
}
#[doc = "FLEXIO reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXIO_AW {
    #[doc = "0: No effect"]
    FLEXIO_CLR = 0,
    #[doc = "1: Clears the PRSTCTL0 Bit"]
    FLEXIO_SET = 1,
}
impl From<FLEXIO_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXIO_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXIO` writer - FLEXIO reset clear"]
pub type FLEXIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_CLR_SPEC, FLEXIO_AW, O>;
impl<'a, const O: u8> FLEXIO_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn flexio_clr(self) -> &'a mut W {
        self.variant(FLEXIO_AW::FLEXIO_CLR)
    }
    #[doc = "Clears the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn flexio_set(self) -> &'a mut W {
        self.variant(FLEXIO_AW::FLEXIO_SET)
    }
}
impl W {
    #[doc = "Bit 8 - Flexcomm0 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm0(&mut self) -> FLEXCOMM0_W<8> {
        FLEXCOMM0_W::new(self)
    }
    #[doc = "Bit 9 - Flexcomm1 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm1(&mut self) -> FLEXCOMM1_W<9> {
        FLEXCOMM1_W::new(self)
    }
    #[doc = "Bit 10 - Flexcomm2 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm2(&mut self) -> FLEXCOMM2_W<10> {
        FLEXCOMM2_W::new(self)
    }
    #[doc = "Bit 11 - Flexcomm3 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm3(&mut self) -> FLEXCOMM3_W<11> {
        FLEXCOMM3_W::new(self)
    }
    #[doc = "Bit 12 - Flexcomm4 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm4(&mut self) -> FLEXCOMM4_W<12> {
        FLEXCOMM4_W::new(self)
    }
    #[doc = "Bit 13 - Flexcomm5 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm5(&mut self) -> FLEXCOMM5_W<13> {
        FLEXCOMM5_W::new(self)
    }
    #[doc = "Bit 14 - Flexcomm6 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm6(&mut self) -> FLEXCOMM6_W<14> {
        FLEXCOMM6_W::new(self)
    }
    #[doc = "Bit 15 - Flexcomm7 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm7(&mut self) -> FLEXCOMM7_W<15> {
        FLEXCOMM7_W::new(self)
    }
    #[doc = "Bit 16 - Flexcomm8 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm8(&mut self) -> FLEXCOMM8_W<16> {
        FLEXCOMM8_W::new(self)
    }
    #[doc = "Bit 17 - Flexcomm9 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm9(&mut self) -> FLEXCOMM9_W<17> {
        FLEXCOMM9_W::new(self)
    }
    #[doc = "Bit 18 - Flexcomm10 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm10(&mut self) -> FLEXCOMM10_W<18> {
        FLEXCOMM10_W::new(self)
    }
    #[doc = "Bit 19 - Flexcomm11 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm11(&mut self) -> FLEXCOMM11_W<19> {
        FLEXCOMM11_W::new(self)
    }
    #[doc = "Bit 20 - Flexcomm12 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm12(&mut self) -> FLEXCOMM12_W<20> {
        FLEXCOMM12_W::new(self)
    }
    #[doc = "Bit 21 - Flexcomm13 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm13(&mut self) -> FLEXCOMM13_W<21> {
        FLEXCOMM13_W::new(self)
    }
    #[doc = "Bit 22 - FLexcomm SPI0 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm14(&mut self) -> FLEXCOMM14_W<22> {
        FLEXCOMM14_W::new(self)
    }
    #[doc = "Bit 23 - Flexcomm I2C reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm15_i2c(&mut self) -> FLEXCOMM15_I2C_W<23> {
        FLEXCOMM15_I2C_W::new(self)
    }
    #[doc = "Bit 24 - DMIC0 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn dmic0(&mut self) -> DMIC0_W<24> {
        DMIC0_W::new(self)
    }
    #[doc = "Bit 25 - Flexcomm SPI1 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm16(&mut self) -> FLEXCOMM16_W<25> {
        FLEXCOMM16_W::new(self)
    }
    #[doc = "Bit 27 - OSEVENT Timer reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn osevent_timer(&mut self) -> OSEVENT_TIMER_W<27> {
        OSEVENT_TIMER_W::new(self)
    }
    #[doc = "Bit 29 - FLEXIO reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn flexio(&mut self) -> FLEXIO_W<29> {
        FLEXIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Reset Control Register 0 CLR\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prstctl0_clr](index.html) module"]
pub struct PRSTCTL0_CLR_SPEC;
impl crate::RegisterSpec for PRSTCTL0_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [prstctl0_clr::W](W) writer structure"]
impl crate::Writable for PRSTCTL0_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRSTCTL0_CLR to value 0"]
impl crate::Resettable for PRSTCTL0_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
