#[doc = "Register `PRSTCTL1` reader"]
pub struct R(crate::R<PRSTCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRSTCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRSTCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRSTCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRSTCTL1` writer"]
pub struct W(crate::W<PRSTCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRSTCTL1_SPEC>;
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
impl From<crate::W<PRSTCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRSTCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSGPIO0` reader - HSGPIO\\[7:0\\]
reset control"]
pub type HSGPIO0_R = crate::BitReader<HSGPIO0_A>;
#[doc = "HSGPIO\\[7:0\\]
reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSGPIO0_A {
    #[doc = "0: Clear Reset"]
    HSGPIO_CLR = 0,
    #[doc = "1: Set Reset"]
    HSGPIO_SET = 1,
}
impl From<HSGPIO0_A> for bool {
    #[inline(always)]
    fn from(variant: HSGPIO0_A) -> Self {
        variant as u8 != 0
    }
}
impl HSGPIO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSGPIO0_A {
        match self.bits {
            false => HSGPIO0_A::HSGPIO_CLR,
            true => HSGPIO0_A::HSGPIO_SET,
        }
    }
    #[doc = "Checks if the value of the field is `HSGPIO_CLR`"]
    #[inline(always)]
    pub fn is_hsgpio_clr(&self) -> bool {
        *self == HSGPIO0_A::HSGPIO_CLR
    }
    #[doc = "Checks if the value of the field is `HSGPIO_SET`"]
    #[inline(always)]
    pub fn is_hsgpio_set(&self) -> bool {
        *self == HSGPIO0_A::HSGPIO_SET
    }
}
#[doc = "Field `HSGPIO0` writer - HSGPIO\\[7:0\\]
reset control"]
pub type HSGPIO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_SPEC, HSGPIO0_A, O>;
impl<'a, const O: u8> HSGPIO0_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn hsgpio_clr(self) -> &'a mut W {
        self.variant(HSGPIO0_A::HSGPIO_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn hsgpio_set(self) -> &'a mut W {
        self.variant(HSGPIO0_A::HSGPIO_SET)
    }
}
#[doc = "Field `HSGPIO1` reader - HSGPIO\\[7:0\\]
reset control"]
pub type HSGPIO1_R = crate::BitReader<HSGPIO1_A>;
#[doc = "HSGPIO\\[7:0\\]
reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSGPIO1_A {
    #[doc = "0: Clear Reset"]
    HSGPIO_CLR = 0,
    #[doc = "1: Set Reset"]
    HSGPIO_SET = 1,
}
impl From<HSGPIO1_A> for bool {
    #[inline(always)]
    fn from(variant: HSGPIO1_A) -> Self {
        variant as u8 != 0
    }
}
impl HSGPIO1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSGPIO1_A {
        match self.bits {
            false => HSGPIO1_A::HSGPIO_CLR,
            true => HSGPIO1_A::HSGPIO_SET,
        }
    }
    #[doc = "Checks if the value of the field is `HSGPIO_CLR`"]
    #[inline(always)]
    pub fn is_hsgpio_clr(&self) -> bool {
        *self == HSGPIO1_A::HSGPIO_CLR
    }
    #[doc = "Checks if the value of the field is `HSGPIO_SET`"]
    #[inline(always)]
    pub fn is_hsgpio_set(&self) -> bool {
        *self == HSGPIO1_A::HSGPIO_SET
    }
}
#[doc = "Field `HSGPIO1` writer - HSGPIO\\[7:0\\]
reset control"]
pub type HSGPIO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_SPEC, HSGPIO1_A, O>;
impl<'a, const O: u8> HSGPIO1_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn hsgpio_clr(self) -> &'a mut W {
        self.variant(HSGPIO1_A::HSGPIO_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn hsgpio_set(self) -> &'a mut W {
        self.variant(HSGPIO1_A::HSGPIO_SET)
    }
}
#[doc = "Field `HSGPIO2` reader - HSGPIO\\[7:0\\]
reset control"]
pub type HSGPIO2_R = crate::BitReader<HSGPIO2_A>;
#[doc = "HSGPIO\\[7:0\\]
reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSGPIO2_A {
    #[doc = "0: Clear Reset"]
    HSGPIO_CLR = 0,
    #[doc = "1: Set Reset"]
    HSGPIO_SET = 1,
}
impl From<HSGPIO2_A> for bool {
    #[inline(always)]
    fn from(variant: HSGPIO2_A) -> Self {
        variant as u8 != 0
    }
}
impl HSGPIO2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSGPIO2_A {
        match self.bits {
            false => HSGPIO2_A::HSGPIO_CLR,
            true => HSGPIO2_A::HSGPIO_SET,
        }
    }
    #[doc = "Checks if the value of the field is `HSGPIO_CLR`"]
    #[inline(always)]
    pub fn is_hsgpio_clr(&self) -> bool {
        *self == HSGPIO2_A::HSGPIO_CLR
    }
    #[doc = "Checks if the value of the field is `HSGPIO_SET`"]
    #[inline(always)]
    pub fn is_hsgpio_set(&self) -> bool {
        *self == HSGPIO2_A::HSGPIO_SET
    }
}
#[doc = "Field `HSGPIO2` writer - HSGPIO\\[7:0\\]
reset control"]
pub type HSGPIO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_SPEC, HSGPIO2_A, O>;
impl<'a, const O: u8> HSGPIO2_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn hsgpio_clr(self) -> &'a mut W {
        self.variant(HSGPIO2_A::HSGPIO_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn hsgpio_set(self) -> &'a mut W {
        self.variant(HSGPIO2_A::HSGPIO_SET)
    }
}
#[doc = "Field `HSGPIO3` reader - HSGPIO\\[7:0\\]
reset control"]
pub type HSGPIO3_R = crate::BitReader<HSGPIO3_A>;
#[doc = "HSGPIO\\[7:0\\]
reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSGPIO3_A {
    #[doc = "0: Clear Reset"]
    HSGPIO_CLR = 0,
    #[doc = "1: Set Reset"]
    HSGPIO_SET = 1,
}
impl From<HSGPIO3_A> for bool {
    #[inline(always)]
    fn from(variant: HSGPIO3_A) -> Self {
        variant as u8 != 0
    }
}
impl HSGPIO3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSGPIO3_A {
        match self.bits {
            false => HSGPIO3_A::HSGPIO_CLR,
            true => HSGPIO3_A::HSGPIO_SET,
        }
    }
    #[doc = "Checks if the value of the field is `HSGPIO_CLR`"]
    #[inline(always)]
    pub fn is_hsgpio_clr(&self) -> bool {
        *self == HSGPIO3_A::HSGPIO_CLR
    }
    #[doc = "Checks if the value of the field is `HSGPIO_SET`"]
    #[inline(always)]
    pub fn is_hsgpio_set(&self) -> bool {
        *self == HSGPIO3_A::HSGPIO_SET
    }
}
#[doc = "Field `HSGPIO3` writer - HSGPIO\\[7:0\\]
reset control"]
pub type HSGPIO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_SPEC, HSGPIO3_A, O>;
impl<'a, const O: u8> HSGPIO3_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn hsgpio_clr(self) -> &'a mut W {
        self.variant(HSGPIO3_A::HSGPIO_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn hsgpio_set(self) -> &'a mut W {
        self.variant(HSGPIO3_A::HSGPIO_SET)
    }
}
#[doc = "Field `HSGPIO4` reader - HSGPIO\\[7:0\\]
reset control"]
pub type HSGPIO4_R = crate::BitReader<HSGPIO4_A>;
#[doc = "HSGPIO\\[7:0\\]
reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSGPIO4_A {
    #[doc = "0: Clear Reset"]
    HSGPIO_CLR = 0,
    #[doc = "1: Set Reset"]
    HSGPIO_SET = 1,
}
impl From<HSGPIO4_A> for bool {
    #[inline(always)]
    fn from(variant: HSGPIO4_A) -> Self {
        variant as u8 != 0
    }
}
impl HSGPIO4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSGPIO4_A {
        match self.bits {
            false => HSGPIO4_A::HSGPIO_CLR,
            true => HSGPIO4_A::HSGPIO_SET,
        }
    }
    #[doc = "Checks if the value of the field is `HSGPIO_CLR`"]
    #[inline(always)]
    pub fn is_hsgpio_clr(&self) -> bool {
        *self == HSGPIO4_A::HSGPIO_CLR
    }
    #[doc = "Checks if the value of the field is `HSGPIO_SET`"]
    #[inline(always)]
    pub fn is_hsgpio_set(&self) -> bool {
        *self == HSGPIO4_A::HSGPIO_SET
    }
}
#[doc = "Field `HSGPIO4` writer - HSGPIO\\[7:0\\]
reset control"]
pub type HSGPIO4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_SPEC, HSGPIO4_A, O>;
impl<'a, const O: u8> HSGPIO4_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn hsgpio_clr(self) -> &'a mut W {
        self.variant(HSGPIO4_A::HSGPIO_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn hsgpio_set(self) -> &'a mut W {
        self.variant(HSGPIO4_A::HSGPIO_SET)
    }
}
#[doc = "Field `HSGPIO5` reader - HSGPIO\\[7:0\\]
reset control"]
pub type HSGPIO5_R = crate::BitReader<HSGPIO5_A>;
#[doc = "HSGPIO\\[7:0\\]
reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSGPIO5_A {
    #[doc = "0: Clear Reset"]
    HSGPIO_CLR = 0,
    #[doc = "1: Set Reset"]
    HSGPIO_SET = 1,
}
impl From<HSGPIO5_A> for bool {
    #[inline(always)]
    fn from(variant: HSGPIO5_A) -> Self {
        variant as u8 != 0
    }
}
impl HSGPIO5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSGPIO5_A {
        match self.bits {
            false => HSGPIO5_A::HSGPIO_CLR,
            true => HSGPIO5_A::HSGPIO_SET,
        }
    }
    #[doc = "Checks if the value of the field is `HSGPIO_CLR`"]
    #[inline(always)]
    pub fn is_hsgpio_clr(&self) -> bool {
        *self == HSGPIO5_A::HSGPIO_CLR
    }
    #[doc = "Checks if the value of the field is `HSGPIO_SET`"]
    #[inline(always)]
    pub fn is_hsgpio_set(&self) -> bool {
        *self == HSGPIO5_A::HSGPIO_SET
    }
}
#[doc = "Field `HSGPIO5` writer - HSGPIO\\[7:0\\]
reset control"]
pub type HSGPIO5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_SPEC, HSGPIO5_A, O>;
impl<'a, const O: u8> HSGPIO5_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn hsgpio_clr(self) -> &'a mut W {
        self.variant(HSGPIO5_A::HSGPIO_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn hsgpio_set(self) -> &'a mut W {
        self.variant(HSGPIO5_A::HSGPIO_SET)
    }
}
#[doc = "Field `HSGPIO6` reader - HSGPIO\\[7:0\\]
reset control"]
pub type HSGPIO6_R = crate::BitReader<HSGPIO6_A>;
#[doc = "HSGPIO\\[7:0\\]
reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSGPIO6_A {
    #[doc = "0: Clear Reset"]
    HSGPIO_CLR = 0,
    #[doc = "1: Set Reset"]
    HSGPIO_SET = 1,
}
impl From<HSGPIO6_A> for bool {
    #[inline(always)]
    fn from(variant: HSGPIO6_A) -> Self {
        variant as u8 != 0
    }
}
impl HSGPIO6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSGPIO6_A {
        match self.bits {
            false => HSGPIO6_A::HSGPIO_CLR,
            true => HSGPIO6_A::HSGPIO_SET,
        }
    }
    #[doc = "Checks if the value of the field is `HSGPIO_CLR`"]
    #[inline(always)]
    pub fn is_hsgpio_clr(&self) -> bool {
        *self == HSGPIO6_A::HSGPIO_CLR
    }
    #[doc = "Checks if the value of the field is `HSGPIO_SET`"]
    #[inline(always)]
    pub fn is_hsgpio_set(&self) -> bool {
        *self == HSGPIO6_A::HSGPIO_SET
    }
}
#[doc = "Field `HSGPIO6` writer - HSGPIO\\[7:0\\]
reset control"]
pub type HSGPIO6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_SPEC, HSGPIO6_A, O>;
impl<'a, const O: u8> HSGPIO6_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn hsgpio_clr(self) -> &'a mut W {
        self.variant(HSGPIO6_A::HSGPIO_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn hsgpio_set(self) -> &'a mut W {
        self.variant(HSGPIO6_A::HSGPIO_SET)
    }
}
#[doc = "Field `HSGPIO7` reader - HSGPIO\\[7:0\\]
reset control"]
pub type HSGPIO7_R = crate::BitReader<HSGPIO7_A>;
#[doc = "HSGPIO\\[7:0\\]
reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSGPIO7_A {
    #[doc = "0: Clear Reset"]
    HSGPIO_CLR = 0,
    #[doc = "1: Set Reset"]
    HSGPIO_SET = 1,
}
impl From<HSGPIO7_A> for bool {
    #[inline(always)]
    fn from(variant: HSGPIO7_A) -> Self {
        variant as u8 != 0
    }
}
impl HSGPIO7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSGPIO7_A {
        match self.bits {
            false => HSGPIO7_A::HSGPIO_CLR,
            true => HSGPIO7_A::HSGPIO_SET,
        }
    }
    #[doc = "Checks if the value of the field is `HSGPIO_CLR`"]
    #[inline(always)]
    pub fn is_hsgpio_clr(&self) -> bool {
        *self == HSGPIO7_A::HSGPIO_CLR
    }
    #[doc = "Checks if the value of the field is `HSGPIO_SET`"]
    #[inline(always)]
    pub fn is_hsgpio_set(&self) -> bool {
        *self == HSGPIO7_A::HSGPIO_SET
    }
}
#[doc = "Field `HSGPIO7` writer - HSGPIO\\[7:0\\]
reset control"]
pub type HSGPIO7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_SPEC, HSGPIO7_A, O>;
impl<'a, const O: u8> HSGPIO7_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn hsgpio_clr(self) -> &'a mut W {
        self.variant(HSGPIO7_A::HSGPIO_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn hsgpio_set(self) -> &'a mut W {
        self.variant(HSGPIO7_A::HSGPIO_SET)
    }
}
#[doc = "Field `CRC` reader - CRC reset control"]
pub type CRC_R = crate::BitReader<CRC_A>;
#[doc = "CRC reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRC_A {
    #[doc = "0: Clear Reset"]
    CRC_CLR = 0,
    #[doc = "1: Set Reset"]
    CRC_SET = 1,
}
impl From<CRC_A> for bool {
    #[inline(always)]
    fn from(variant: CRC_A) -> Self {
        variant as u8 != 0
    }
}
impl CRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_A {
        match self.bits {
            false => CRC_A::CRC_CLR,
            true => CRC_A::CRC_SET,
        }
    }
    #[doc = "Checks if the value of the field is `CRC_CLR`"]
    #[inline(always)]
    pub fn is_crc_clr(&self) -> bool {
        *self == CRC_A::CRC_CLR
    }
    #[doc = "Checks if the value of the field is `CRC_SET`"]
    #[inline(always)]
    pub fn is_crc_set(&self) -> bool {
        *self == CRC_A::CRC_SET
    }
}
#[doc = "Field `CRC` writer - CRC reset control"]
pub type CRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_SPEC, CRC_A, O>;
impl<'a, const O: u8> CRC_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn crc_clr(self) -> &'a mut W {
        self.variant(CRC_A::CRC_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn crc_set(self) -> &'a mut W {
        self.variant(CRC_A::CRC_SET)
    }
}
#[doc = "Field `DMAC0` reader - DMAC reset control"]
pub type DMAC0_R = crate::BitReader<DMAC0_A>;
#[doc = "DMAC reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAC0_A {
    #[doc = "0: Clear Reset"]
    DMAC_CLR = 0,
    #[doc = "1: Set Reset"]
    DMAC_SET = 1,
}
impl From<DMAC0_A> for bool {
    #[inline(always)]
    fn from(variant: DMAC0_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAC0_A {
        match self.bits {
            false => DMAC0_A::DMAC_CLR,
            true => DMAC0_A::DMAC_SET,
        }
    }
    #[doc = "Checks if the value of the field is `DMAC_CLR`"]
    #[inline(always)]
    pub fn is_dmac_clr(&self) -> bool {
        *self == DMAC0_A::DMAC_CLR
    }
    #[doc = "Checks if the value of the field is `DMAC_SET`"]
    #[inline(always)]
    pub fn is_dmac_set(&self) -> bool {
        *self == DMAC0_A::DMAC_SET
    }
}
#[doc = "Field `DMAC0` writer - DMAC reset control"]
pub type DMAC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_SPEC, DMAC0_A, O>;
impl<'a, const O: u8> DMAC0_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn dmac_clr(self) -> &'a mut W {
        self.variant(DMAC0_A::DMAC_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn dmac_set(self) -> &'a mut W {
        self.variant(DMAC0_A::DMAC_SET)
    }
}
#[doc = "Field `DMAC1` reader - DMAC reset control"]
pub type DMAC1_R = crate::BitReader<DMAC1_A>;
#[doc = "DMAC reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAC1_A {
    #[doc = "0: Clear Reset"]
    DMAC_CLR = 0,
    #[doc = "1: Set Reset"]
    DMAC_SET = 1,
}
impl From<DMAC1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAC1_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAC1_A {
        match self.bits {
            false => DMAC1_A::DMAC_CLR,
            true => DMAC1_A::DMAC_SET,
        }
    }
    #[doc = "Checks if the value of the field is `DMAC_CLR`"]
    #[inline(always)]
    pub fn is_dmac_clr(&self) -> bool {
        *self == DMAC1_A::DMAC_CLR
    }
    #[doc = "Checks if the value of the field is `DMAC_SET`"]
    #[inline(always)]
    pub fn is_dmac_set(&self) -> bool {
        *self == DMAC1_A::DMAC_SET
    }
}
#[doc = "Field `DMAC1` writer - DMAC reset control"]
pub type DMAC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_SPEC, DMAC1_A, O>;
impl<'a, const O: u8> DMAC1_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn dmac_clr(self) -> &'a mut W {
        self.variant(DMAC1_A::DMAC_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn dmac_set(self) -> &'a mut W {
        self.variant(DMAC1_A::DMAC_SET)
    }
}
#[doc = "Field `MU` reader - MU reset control"]
pub type MU_R = crate::BitReader<MU_A>;
#[doc = "MU reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MU_A {
    #[doc = "0: Clear Reset"]
    MU_CLR = 0,
    #[doc = "1: Set Reset"]
    MU_SET = 1,
}
impl From<MU_A> for bool {
    #[inline(always)]
    fn from(variant: MU_A) -> Self {
        variant as u8 != 0
    }
}
impl MU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MU_A {
        match self.bits {
            false => MU_A::MU_CLR,
            true => MU_A::MU_SET,
        }
    }
    #[doc = "Checks if the value of the field is `MU_CLR`"]
    #[inline(always)]
    pub fn is_mu_clr(&self) -> bool {
        *self == MU_A::MU_CLR
    }
    #[doc = "Checks if the value of the field is `MU_SET`"]
    #[inline(always)]
    pub fn is_mu_set(&self) -> bool {
        *self == MU_A::MU_SET
    }
}
#[doc = "Field `MU` writer - MU reset control"]
pub type MU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_SPEC, MU_A, O>;
impl<'a, const O: u8> MU_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn mu_clr(self) -> &'a mut W {
        self.variant(MU_A::MU_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn mu_set(self) -> &'a mut W {
        self.variant(MU_A::MU_SET)
    }
}
#[doc = "Field `SEMA` reader - SEMA reset control"]
pub type SEMA_R = crate::BitReader<SEMA_A>;
#[doc = "SEMA reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEMA_A {
    #[doc = "0: Clear Reset"]
    SEMA_CLR = 0,
    #[doc = "1: Set Reset"]
    SEMA_SET = 1,
}
impl From<SEMA_A> for bool {
    #[inline(always)]
    fn from(variant: SEMA_A) -> Self {
        variant as u8 != 0
    }
}
impl SEMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEMA_A {
        match self.bits {
            false => SEMA_A::SEMA_CLR,
            true => SEMA_A::SEMA_SET,
        }
    }
    #[doc = "Checks if the value of the field is `SEMA_CLR`"]
    #[inline(always)]
    pub fn is_sema_clr(&self) -> bool {
        *self == SEMA_A::SEMA_CLR
    }
    #[doc = "Checks if the value of the field is `SEMA_SET`"]
    #[inline(always)]
    pub fn is_sema_set(&self) -> bool {
        *self == SEMA_A::SEMA_SET
    }
}
#[doc = "Field `SEMA` writer - SEMA reset control"]
pub type SEMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_SPEC, SEMA_A, O>;
impl<'a, const O: u8> SEMA_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn sema_clr(self) -> &'a mut W {
        self.variant(SEMA_A::SEMA_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn sema_set(self) -> &'a mut W {
        self.variant(SEMA_A::SEMA_SET)
    }
}
#[doc = "Field `FREQME` reader - FREQME reset control"]
pub type FREQME_R = crate::BitReader<FREQME_A>;
#[doc = "FREQME reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FREQME_A {
    #[doc = "0: Clear Reset"]
    FREQME_CLR = 0,
    #[doc = "1: Set Reset"]
    FREQME_SET = 1,
}
impl From<FREQME_A> for bool {
    #[inline(always)]
    fn from(variant: FREQME_A) -> Self {
        variant as u8 != 0
    }
}
impl FREQME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREQME_A {
        match self.bits {
            false => FREQME_A::FREQME_CLR,
            true => FREQME_A::FREQME_SET,
        }
    }
    #[doc = "Checks if the value of the field is `FREQME_CLR`"]
    #[inline(always)]
    pub fn is_freqme_clr(&self) -> bool {
        *self == FREQME_A::FREQME_CLR
    }
    #[doc = "Checks if the value of the field is `FREQME_SET`"]
    #[inline(always)]
    pub fn is_freqme_set(&self) -> bool {
        *self == FREQME_A::FREQME_SET
    }
}
#[doc = "Field `FREQME` writer - FREQME reset control"]
pub type FREQME_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_SPEC, FREQME_A, O>;
impl<'a, const O: u8> FREQME_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn freqme_clr(self) -> &'a mut W {
        self.variant(FREQME_A::FREQME_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn freqme_set(self) -> &'a mut W {
        self.variant(FREQME_A::FREQME_SET)
    }
}
impl R {
    #[doc = "Bit 0 - HSGPIO\\[7:0\\]
reset control"]
    #[inline(always)]
    pub fn hsgpio0(&self) -> HSGPIO0_R {
        HSGPIO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSGPIO\\[7:0\\]
reset control"]
    #[inline(always)]
    pub fn hsgpio1(&self) -> HSGPIO1_R {
        HSGPIO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSGPIO\\[7:0\\]
reset control"]
    #[inline(always)]
    pub fn hsgpio2(&self) -> HSGPIO2_R {
        HSGPIO2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSGPIO\\[7:0\\]
reset control"]
    #[inline(always)]
    pub fn hsgpio3(&self) -> HSGPIO3_R {
        HSGPIO3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSGPIO\\[7:0\\]
reset control"]
    #[inline(always)]
    pub fn hsgpio4(&self) -> HSGPIO4_R {
        HSGPIO4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HSGPIO\\[7:0\\]
reset control"]
    #[inline(always)]
    pub fn hsgpio5(&self) -> HSGPIO5_R {
        HSGPIO5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HSGPIO\\[7:0\\]
reset control"]
    #[inline(always)]
    pub fn hsgpio6(&self) -> HSGPIO6_R {
        HSGPIO6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HSGPIO\\[7:0\\]
reset control"]
    #[inline(always)]
    pub fn hsgpio7(&self) -> HSGPIO7_R {
        HSGPIO7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - CRC reset control"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 23 - DMAC reset control"]
    #[inline(always)]
    pub fn dmac0(&self) -> DMAC0_R {
        DMAC0_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DMAC reset control"]
    #[inline(always)]
    pub fn dmac1(&self) -> DMAC1_R {
        DMAC1_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - MU reset control"]
    #[inline(always)]
    pub fn mu(&self) -> MU_R {
        MU_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SEMA reset control"]
    #[inline(always)]
    pub fn sema(&self) -> SEMA_R {
        SEMA_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - FREQME reset control"]
    #[inline(always)]
    pub fn freqme(&self) -> FREQME_R {
        FREQME_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSGPIO\\[7:0\\]
reset control"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio0(&mut self) -> HSGPIO0_W<0> {
        HSGPIO0_W::new(self)
    }
    #[doc = "Bit 1 - HSGPIO\\[7:0\\]
reset control"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio1(&mut self) -> HSGPIO1_W<1> {
        HSGPIO1_W::new(self)
    }
    #[doc = "Bit 2 - HSGPIO\\[7:0\\]
reset control"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio2(&mut self) -> HSGPIO2_W<2> {
        HSGPIO2_W::new(self)
    }
    #[doc = "Bit 3 - HSGPIO\\[7:0\\]
reset control"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio3(&mut self) -> HSGPIO3_W<3> {
        HSGPIO3_W::new(self)
    }
    #[doc = "Bit 4 - HSGPIO\\[7:0\\]
reset control"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio4(&mut self) -> HSGPIO4_W<4> {
        HSGPIO4_W::new(self)
    }
    #[doc = "Bit 5 - HSGPIO\\[7:0\\]
reset control"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio5(&mut self) -> HSGPIO5_W<5> {
        HSGPIO5_W::new(self)
    }
    #[doc = "Bit 6 - HSGPIO\\[7:0\\]
reset control"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio6(&mut self) -> HSGPIO6_W<6> {
        HSGPIO6_W::new(self)
    }
    #[doc = "Bit 7 - HSGPIO\\[7:0\\]
reset control"]
    #[inline(always)]
    #[must_use]
    pub fn hsgpio7(&mut self) -> HSGPIO7_W<7> {
        HSGPIO7_W::new(self)
    }
    #[doc = "Bit 16 - CRC reset control"]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<16> {
        CRC_W::new(self)
    }
    #[doc = "Bit 23 - DMAC reset control"]
    #[inline(always)]
    #[must_use]
    pub fn dmac0(&mut self) -> DMAC0_W<23> {
        DMAC0_W::new(self)
    }
    #[doc = "Bit 24 - DMAC reset control"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1(&mut self) -> DMAC1_W<24> {
        DMAC1_W::new(self)
    }
    #[doc = "Bit 28 - MU reset control"]
    #[inline(always)]
    #[must_use]
    pub fn mu(&mut self) -> MU_W<28> {
        MU_W::new(self)
    }
    #[doc = "Bit 29 - SEMA reset control"]
    #[inline(always)]
    #[must_use]
    pub fn sema(&mut self) -> SEMA_W<29> {
        SEMA_W::new(self)
    }
    #[doc = "Bit 31 - FREQME reset control"]
    #[inline(always)]
    #[must_use]
    pub fn freqme(&mut self) -> FREQME_W<31> {
        FREQME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Reset Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prstctl1](index.html) module"]
pub struct PRSTCTL1_SPEC;
impl crate::RegisterSpec for PRSTCTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prstctl1::R](R) reader structure"]
impl crate::Readable for PRSTCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prstctl1::W](W) writer structure"]
impl crate::Writable for PRSTCTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRSTCTL1 to value 0xb181_00ff"]
impl crate::Resettable for PRSTCTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0xb181_00ff;
}
