#[doc = "Register `PSCCTL1_SET` reader"]
pub struct R(crate::R<PSCCTL1_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSCCTL1_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSCCTL1_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSCCTL1_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSCCTL1_SET` writer"]
pub struct W(crate::W<PSCCTL1_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSCCTL1_SET_SPEC>;
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
impl From<crate::W<PSCCTL1_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSCCTL1_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSGPIO0_CLK` reader - Non-secure GPIO0 clock control set"]
pub type HSGPIO0_CLK_R = crate::BitReader<HSGPIO0_CLK_A>;
#[doc = "Non-secure GPIO0 clock control set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSGPIO0_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL1 bit"]
    ENABLE = 1,
}
impl From<HSGPIO0_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: HSGPIO0_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl HSGPIO0_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSGPIO0_CLK_A {
        match self.bits {
            false => HSGPIO0_CLK_A::DISABLE,
            true => HSGPIO0_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HSGPIO0_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HSGPIO0_CLK_A::ENABLE
    }
}
#[doc = "Field `HSGPIO0_CLK` writer - Non-secure GPIO0 clock control set"]
pub type HSGPIO0_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL1_SET_SPEC, HSGPIO0_CLK_A, O>;
impl<'a, const O: u8> HSGPIO0_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HSGPIO0_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL1 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HSGPIO0_CLK_A::ENABLE)
    }
}
#[doc = "Field `HSGPIO1_CLK` reader - Non-secure GPIO1 clock control set"]
pub type HSGPIO1_CLK_R = crate::BitReader<HSGPIO1_CLK_A>;
#[doc = "Non-secure GPIO1 clock control set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSGPIO1_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL1 bit"]
    ENABLE = 1,
}
impl From<HSGPIO1_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: HSGPIO1_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl HSGPIO1_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSGPIO1_CLK_A {
        match self.bits {
            false => HSGPIO1_CLK_A::DISABLE,
            true => HSGPIO1_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HSGPIO1_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HSGPIO1_CLK_A::ENABLE
    }
}
#[doc = "Field `HSGPIO1_CLK` writer - Non-secure GPIO1 clock control set"]
pub type HSGPIO1_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL1_SET_SPEC, HSGPIO1_CLK_A, O>;
impl<'a, const O: u8> HSGPIO1_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HSGPIO1_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL1 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HSGPIO1_CLK_A::ENABLE)
    }
}
#[doc = "Field `HSGPIO2_CLK` reader - Non-secure GPIO2 clock control set"]
pub type HSGPIO2_CLK_R = crate::BitReader<HSGPIO2_CLK_A>;
#[doc = "Non-secure GPIO2 clock control set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSGPIO2_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL1 bit"]
    ENABLE = 1,
}
impl From<HSGPIO2_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: HSGPIO2_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl HSGPIO2_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSGPIO2_CLK_A {
        match self.bits {
            false => HSGPIO2_CLK_A::DISABLE,
            true => HSGPIO2_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HSGPIO2_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HSGPIO2_CLK_A::ENABLE
    }
}
#[doc = "Field `HSGPIO2_CLK` writer - Non-secure GPIO2 clock control set"]
pub type HSGPIO2_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL1_SET_SPEC, HSGPIO2_CLK_A, O>;
impl<'a, const O: u8> HSGPIO2_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HSGPIO2_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL1 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HSGPIO2_CLK_A::ENABLE)
    }
}
#[doc = "Field `HSGPIO3_CLK` reader - Non-secure GPIO3 clock control set"]
pub type HSGPIO3_CLK_R = crate::BitReader<HSGPIO3_CLK_A>;
#[doc = "Non-secure GPIO3 clock control set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSGPIO3_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL1 bit"]
    ENABLE = 1,
}
impl From<HSGPIO3_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: HSGPIO3_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl HSGPIO3_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSGPIO3_CLK_A {
        match self.bits {
            false => HSGPIO3_CLK_A::DISABLE,
            true => HSGPIO3_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HSGPIO3_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HSGPIO3_CLK_A::ENABLE
    }
}
#[doc = "Field `HSGPIO3_CLK` writer - Non-secure GPIO3 clock control set"]
pub type HSGPIO3_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL1_SET_SPEC, HSGPIO3_CLK_A, O>;
impl<'a, const O: u8> HSGPIO3_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HSGPIO3_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL1 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HSGPIO3_CLK_A::ENABLE)
    }
}
#[doc = "Field `HSGPIO4_CLK` reader - Non-secure GPIO4 clock control set"]
pub type HSGPIO4_CLK_R = crate::BitReader<HSGPIO4_CLK_A>;
#[doc = "Non-secure GPIO4 clock control set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSGPIO4_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL1 bit"]
    ENABLE = 1,
}
impl From<HSGPIO4_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: HSGPIO4_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl HSGPIO4_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSGPIO4_CLK_A {
        match self.bits {
            false => HSGPIO4_CLK_A::DISABLE,
            true => HSGPIO4_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HSGPIO4_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HSGPIO4_CLK_A::ENABLE
    }
}
#[doc = "Field `HSGPIO4_CLK` writer - Non-secure GPIO4 clock control set"]
pub type HSGPIO4_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL1_SET_SPEC, HSGPIO4_CLK_A, O>;
impl<'a, const O: u8> HSGPIO4_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HSGPIO4_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL1 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HSGPIO4_CLK_A::ENABLE)
    }
}
#[doc = "Field `HSGPIO5_CLK` reader - Non-secure GPIO5 clock control set"]
pub type HSGPIO5_CLK_R = crate::BitReader<HSGPIO5_CLK_A>;
#[doc = "Non-secure GPIO5 clock control set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSGPIO5_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL1 bit"]
    ENABLE = 1,
}
impl From<HSGPIO5_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: HSGPIO5_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl HSGPIO5_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSGPIO5_CLK_A {
        match self.bits {
            false => HSGPIO5_CLK_A::DISABLE,
            true => HSGPIO5_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HSGPIO5_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HSGPIO5_CLK_A::ENABLE
    }
}
#[doc = "Field `HSGPIO5_CLK` writer - Non-secure GPIO5 clock control set"]
pub type HSGPIO5_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL1_SET_SPEC, HSGPIO5_CLK_A, O>;
impl<'a, const O: u8> HSGPIO5_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HSGPIO5_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL1 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HSGPIO5_CLK_A::ENABLE)
    }
}
#[doc = "Field `HSGPIO6_CLK` reader - Non-secure GPIO6 clock control set"]
pub type HSGPIO6_CLK_R = crate::BitReader<HSGPIO6_CLK_A>;
#[doc = "Non-secure GPIO6 clock control set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSGPIO6_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL1 bit"]
    ENABLE = 1,
}
impl From<HSGPIO6_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: HSGPIO6_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl HSGPIO6_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSGPIO6_CLK_A {
        match self.bits {
            false => HSGPIO6_CLK_A::DISABLE,
            true => HSGPIO6_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HSGPIO6_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HSGPIO6_CLK_A::ENABLE
    }
}
#[doc = "Field `HSGPIO6_CLK` writer - Non-secure GPIO6 clock control set"]
pub type HSGPIO6_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL1_SET_SPEC, HSGPIO6_CLK_A, O>;
impl<'a, const O: u8> HSGPIO6_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HSGPIO6_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL1 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HSGPIO6_CLK_A::ENABLE)
    }
}
#[doc = "Field `HSGPIO7_CLK` reader - Non-secure GPIO7 clock control set"]
pub type HSGPIO7_CLK_R = crate::BitReader<HSGPIO7_CLK_A>;
#[doc = "Non-secure GPIO7 clock control set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSGPIO7_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL1 bit"]
    ENABLE = 1,
}
impl From<HSGPIO7_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: HSGPIO7_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl HSGPIO7_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSGPIO7_CLK_A {
        match self.bits {
            false => HSGPIO7_CLK_A::DISABLE,
            true => HSGPIO7_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HSGPIO7_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HSGPIO7_CLK_A::ENABLE
    }
}
#[doc = "Field `HSGPIO7_CLK` writer - Non-secure GPIO7 clock control set"]
pub type HSGPIO7_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL1_SET_SPEC, HSGPIO7_CLK_A, O>;
impl<'a, const O: u8> HSGPIO7_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HSGPIO7_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL1 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HSGPIO7_CLK_A::ENABLE)
    }
}
#[doc = "Field `CRC_CLK` reader - CRC clock control set"]
pub type CRC_CLK_R = crate::BitReader<CRC_CLK_A>;
#[doc = "CRC clock control set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRC_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL1 bit"]
    ENABLE = 1,
}
impl From<CRC_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: CRC_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl CRC_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_CLK_A {
        match self.bits {
            false => CRC_CLK_A::DISABLE,
            true => CRC_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CRC_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CRC_CLK_A::ENABLE
    }
}
#[doc = "Field `CRC_CLK` writer - CRC clock control set"]
pub type CRC_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL1_SET_SPEC, CRC_CLK_A, O>;
impl<'a, const O: u8> CRC_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CRC_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL1 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CRC_CLK_A::ENABLE)
    }
}
#[doc = "Field `DMAC0_CLK` reader - DMAC0 clock control set"]
pub type DMAC0_CLK_R = crate::BitReader<DMAC0_CLK_A>;
#[doc = "DMAC0 clock control set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAC0_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL1 bit"]
    ENABLE = 1,
}
impl From<DMAC0_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: DMAC0_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAC0_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAC0_CLK_A {
        match self.bits {
            false => DMAC0_CLK_A::DISABLE,
            true => DMAC0_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMAC0_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMAC0_CLK_A::ENABLE
    }
}
#[doc = "Field `DMAC0_CLK` writer - DMAC0 clock control set"]
pub type DMAC0_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL1_SET_SPEC, DMAC0_CLK_A, O>;
impl<'a, const O: u8> DMAC0_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMAC0_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL1 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMAC0_CLK_A::ENABLE)
    }
}
#[doc = "Field `DMAC1_CLK` reader - DMAC1 clock control set"]
pub type DMAC1_CLK_R = crate::BitReader<DMAC1_CLK_A>;
#[doc = "DMAC1 clock control set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAC1_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL1 bit"]
    ENABLE = 1,
}
impl From<DMAC1_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: DMAC1_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAC1_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAC1_CLK_A {
        match self.bits {
            false => DMAC1_CLK_A::DISABLE,
            true => DMAC1_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMAC1_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMAC1_CLK_A::ENABLE
    }
}
#[doc = "Field `DMAC1_CLK` writer - DMAC1 clock control set"]
pub type DMAC1_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL1_SET_SPEC, DMAC1_CLK_A, O>;
impl<'a, const O: u8> DMAC1_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMAC1_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL1 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMAC1_CLK_A::ENABLE)
    }
}
#[doc = "Field `MU_CLK` reader - Messaging Unit clock control set"]
pub type MU_CLK_R = crate::BitReader<MU_CLK_A>;
#[doc = "Messaging Unit clock control set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MU_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL1 bit"]
    ENABLE = 1,
}
impl From<MU_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: MU_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl MU_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MU_CLK_A {
        match self.bits {
            false => MU_CLK_A::DISABLE,
            true => MU_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MU_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MU_CLK_A::ENABLE
    }
}
#[doc = "Field `MU_CLK` writer - Messaging Unit clock control set"]
pub type MU_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL1_SET_SPEC, MU_CLK_A, O>;
impl<'a, const O: u8> MU_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MU_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL1 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MU_CLK_A::ENABLE)
    }
}
#[doc = "Field `SEMA_CLK` reader - Semaphore clock control set"]
pub type SEMA_CLK_R = crate::BitReader<SEMA_CLK_A>;
#[doc = "Semaphore clock control set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEMA_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL1 bit"]
    ENABLE = 1,
}
impl From<SEMA_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: SEMA_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl SEMA_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEMA_CLK_A {
        match self.bits {
            false => SEMA_CLK_A::DISABLE,
            true => SEMA_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SEMA_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SEMA_CLK_A::ENABLE
    }
}
#[doc = "Field `SEMA_CLK` writer - Semaphore clock control set"]
pub type SEMA_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL1_SET_SPEC, SEMA_CLK_A, O>;
impl<'a, const O: u8> SEMA_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SEMA_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL1 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SEMA_CLK_A::ENABLE)
    }
}
#[doc = "Field `FREQME_CLK` reader - Frequency Measurement clock control set"]
pub type FREQME_CLK_R = crate::BitReader<FREQME_CLK_A>;
#[doc = "Frequency Measurement clock control set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FREQME_CLK_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PSCCTL1 bit"]
    ENABLE = 1,
}
impl From<FREQME_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: FREQME_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl FREQME_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREQME_CLK_A {
        match self.bits {
            false => FREQME_CLK_A::DISABLE,
            true => FREQME_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FREQME_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FREQME_CLK_A::ENABLE
    }
}
#[doc = "Field `FREQME_CLK` writer - Frequency Measurement clock control set"]
pub type FREQME_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL1_SET_SPEC, FREQME_CLK_A, O>;
impl<'a, const O: u8> FREQME_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FREQME_CLK_A::DISABLE)
    }
    #[doc = "Sets the PSCCTL1 bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FREQME_CLK_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Non-secure GPIO0 clock control set"]
    #[inline(always)]
    pub fn hsgpio0_clk(&self) -> HSGPIO0_CLK_R {
        HSGPIO0_CLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Non-secure GPIO1 clock control set"]
    #[inline(always)]
    pub fn hsgpio1_clk(&self) -> HSGPIO1_CLK_R {
        HSGPIO1_CLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Non-secure GPIO2 clock control set"]
    #[inline(always)]
    pub fn hsgpio2_clk(&self) -> HSGPIO2_CLK_R {
        HSGPIO2_CLK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Non-secure GPIO3 clock control set"]
    #[inline(always)]
    pub fn hsgpio3_clk(&self) -> HSGPIO3_CLK_R {
        HSGPIO3_CLK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Non-secure GPIO4 clock control set"]
    #[inline(always)]
    pub fn hsgpio4_clk(&self) -> HSGPIO4_CLK_R {
        HSGPIO4_CLK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Non-secure GPIO5 clock control set"]
    #[inline(always)]
    pub fn hsgpio5_clk(&self) -> HSGPIO5_CLK_R {
        HSGPIO5_CLK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Non-secure GPIO6 clock control set"]
    #[inline(always)]
    pub fn hsgpio6_clk(&self) -> HSGPIO6_CLK_R {
        HSGPIO6_CLK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Non-secure GPIO7 clock control set"]
    #[inline(always)]
    pub fn hsgpio7_clk(&self) -> HSGPIO7_CLK_R {
        HSGPIO7_CLK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - CRC clock control set"]
    #[inline(always)]
    pub fn crc_clk(&self) -> CRC_CLK_R {
        CRC_CLK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 23 - DMAC0 clock control set"]
    #[inline(always)]
    pub fn dmac0_clk(&self) -> DMAC0_CLK_R {
        DMAC0_CLK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DMAC1 clock control set"]
    #[inline(always)]
    pub fn dmac1_clk(&self) -> DMAC1_CLK_R {
        DMAC1_CLK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Messaging Unit clock control set"]
    #[inline(always)]
    pub fn mu_clk(&self) -> MU_CLK_R {
        MU_CLK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Semaphore clock control set"]
    #[inline(always)]
    pub fn sema_clk(&self) -> SEMA_CLK_R {
        SEMA_CLK_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Frequency Measurement clock control set"]
    #[inline(always)]
    pub fn freqme_clk(&self) -> FREQME_CLK_R {
        FREQME_CLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non-secure GPIO0 clock control set"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio0_clk(&mut self) -> HSGPIO0_CLK_W<0> {
        HSGPIO0_CLK_W::new(self)
    }
    #[doc = "Bit 1 - Non-secure GPIO1 clock control set"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio1_clk(&mut self) -> HSGPIO1_CLK_W<1> {
        HSGPIO1_CLK_W::new(self)
    }
    #[doc = "Bit 2 - Non-secure GPIO2 clock control set"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio2_clk(&mut self) -> HSGPIO2_CLK_W<2> {
        HSGPIO2_CLK_W::new(self)
    }
    #[doc = "Bit 3 - Non-secure GPIO3 clock control set"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio3_clk(&mut self) -> HSGPIO3_CLK_W<3> {
        HSGPIO3_CLK_W::new(self)
    }
    #[doc = "Bit 4 - Non-secure GPIO4 clock control set"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio4_clk(&mut self) -> HSGPIO4_CLK_W<4> {
        HSGPIO4_CLK_W::new(self)
    }
    #[doc = "Bit 5 - Non-secure GPIO5 clock control set"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio5_clk(&mut self) -> HSGPIO5_CLK_W<5> {
        HSGPIO5_CLK_W::new(self)
    }
    #[doc = "Bit 6 - Non-secure GPIO6 clock control set"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio6_clk(&mut self) -> HSGPIO6_CLK_W<6> {
        HSGPIO6_CLK_W::new(self)
    }
    #[doc = "Bit 7 - Non-secure GPIO7 clock control set"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio7_clk(&mut self) -> HSGPIO7_CLK_W<7> {
        HSGPIO7_CLK_W::new(self)
    }
    #[doc = "Bit 16 - CRC clock control set"]
    #[inline(always)]
    #[must_use]
    pub fn crc_clk(&mut self) -> CRC_CLK_W<16> {
        CRC_CLK_W::new(self)
    }
    #[doc = "Bit 23 - DMAC0 clock control set"]
    #[inline(always)]
    #[must_use]
    pub fn dmac0_clk(&mut self) -> DMAC0_CLK_W<23> {
        DMAC0_CLK_W::new(self)
    }
    #[doc = "Bit 24 - DMAC1 clock control set"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_clk(&mut self) -> DMAC1_CLK_W<24> {
        DMAC1_CLK_W::new(self)
    }
    #[doc = "Bit 28 - Messaging Unit clock control set"]
    #[inline(always)]
    #[must_use]
    pub fn mu_clk(&mut self) -> MU_CLK_W<28> {
        MU_CLK_W::new(self)
    }
    #[doc = "Bit 29 - Semaphore clock control set"]
    #[inline(always)]
    #[must_use]
    pub fn sema_clk(&mut self) -> SEMA_CLK_W<29> {
        SEMA_CLK_W::new(self)
    }
    #[doc = "Bit 31 - Frequency Measurement clock control set"]
    #[inline(always)]
    #[must_use]
    pub fn freqme_clk(&mut self) -> FREQME_CLK_W<31> {
        FREQME_CLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Set 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pscctl1_set](index.html) module"]
pub struct PSCCTL1_SET_SPEC;
impl crate::RegisterSpec for PSCCTL1_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pscctl1_set::R](R) reader structure"]
impl crate::Readable for PSCCTL1_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pscctl1_set::W](W) writer structure"]
impl crate::Writable for PSCCTL1_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSCCTL1_SET to value 0"]
impl crate::Resettable for PSCCTL1_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
