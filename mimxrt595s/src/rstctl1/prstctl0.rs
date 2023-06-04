#[doc = "Register `PRSTCTL0` reader"]
pub struct R(crate::R<PRSTCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRSTCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRSTCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRSTCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRSTCTL0` writer"]
pub struct W(crate::W<PRSTCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRSTCTL0_SPEC>;
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
impl From<crate::W<PRSTCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRSTCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLEXCOMM0` reader - Flexcomm0 reset control"]
pub type FLEXCOMM0_R = crate::BitReader<FLEXCOMM0_A>;
#[doc = "Flexcomm0 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM0_A {
    #[doc = "0: Clear Reset"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Set Reset"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM0_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM0_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM0_A {
        match self.bits {
            false => FLEXCOMM0_A::FLEXCOMM_CLR,
            true => FLEXCOMM0_A::FLEXCOMM_SET,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_CLR`"]
    #[inline(always)]
    pub fn is_flexcomm_clr(&self) -> bool {
        *self == FLEXCOMM0_A::FLEXCOMM_CLR
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_SET`"]
    #[inline(always)]
    pub fn is_flexcomm_set(&self) -> bool {
        *self == FLEXCOMM0_A::FLEXCOMM_SET
    }
}
#[doc = "Field `FLEXCOMM0` writer - Flexcomm0 reset control"]
pub type FLEXCOMM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, FLEXCOMM0_A, O>;
impl<'a, const O: u8> FLEXCOMM0_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM0_A::FLEXCOMM_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM0_A::FLEXCOMM_SET)
    }
}
#[doc = "Field `FLEXCOMM1` reader - Flexcomm1 reset control"]
pub type FLEXCOMM1_R = crate::BitReader<FLEXCOMM1_A>;
#[doc = "Flexcomm1 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM1_A {
    #[doc = "0: Clear Reset"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Set Reset"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM1_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM1_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM1_A {
        match self.bits {
            false => FLEXCOMM1_A::FLEXCOMM_CLR,
            true => FLEXCOMM1_A::FLEXCOMM_SET,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_CLR`"]
    #[inline(always)]
    pub fn is_flexcomm_clr(&self) -> bool {
        *self == FLEXCOMM1_A::FLEXCOMM_CLR
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_SET`"]
    #[inline(always)]
    pub fn is_flexcomm_set(&self) -> bool {
        *self == FLEXCOMM1_A::FLEXCOMM_SET
    }
}
#[doc = "Field `FLEXCOMM1` writer - Flexcomm1 reset control"]
pub type FLEXCOMM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, FLEXCOMM1_A, O>;
impl<'a, const O: u8> FLEXCOMM1_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM1_A::FLEXCOMM_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM1_A::FLEXCOMM_SET)
    }
}
#[doc = "Field `FLEXCOMM2` reader - Flexcomm2 reset control"]
pub type FLEXCOMM2_R = crate::BitReader<FLEXCOMM2_A>;
#[doc = "Flexcomm2 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM2_A {
    #[doc = "0: Clear Reset"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Set Reset"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM2_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM2_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM2_A {
        match self.bits {
            false => FLEXCOMM2_A::FLEXCOMM_CLR,
            true => FLEXCOMM2_A::FLEXCOMM_SET,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_CLR`"]
    #[inline(always)]
    pub fn is_flexcomm_clr(&self) -> bool {
        *self == FLEXCOMM2_A::FLEXCOMM_CLR
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_SET`"]
    #[inline(always)]
    pub fn is_flexcomm_set(&self) -> bool {
        *self == FLEXCOMM2_A::FLEXCOMM_SET
    }
}
#[doc = "Field `FLEXCOMM2` writer - Flexcomm2 reset control"]
pub type FLEXCOMM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, FLEXCOMM2_A, O>;
impl<'a, const O: u8> FLEXCOMM2_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM2_A::FLEXCOMM_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM2_A::FLEXCOMM_SET)
    }
}
#[doc = "Field `FLEXCOMM3` reader - Flexcomm3 reset control"]
pub type FLEXCOMM3_R = crate::BitReader<FLEXCOMM3_A>;
#[doc = "Flexcomm3 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM3_A {
    #[doc = "0: Clear Reset"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Set Reset"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM3_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM3_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM3_A {
        match self.bits {
            false => FLEXCOMM3_A::FLEXCOMM_CLR,
            true => FLEXCOMM3_A::FLEXCOMM_SET,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_CLR`"]
    #[inline(always)]
    pub fn is_flexcomm_clr(&self) -> bool {
        *self == FLEXCOMM3_A::FLEXCOMM_CLR
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_SET`"]
    #[inline(always)]
    pub fn is_flexcomm_set(&self) -> bool {
        *self == FLEXCOMM3_A::FLEXCOMM_SET
    }
}
#[doc = "Field `FLEXCOMM3` writer - Flexcomm3 reset control"]
pub type FLEXCOMM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, FLEXCOMM3_A, O>;
impl<'a, const O: u8> FLEXCOMM3_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM3_A::FLEXCOMM_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM3_A::FLEXCOMM_SET)
    }
}
#[doc = "Field `FLEXCOMM4` reader - Flexcomm4 reset control"]
pub type FLEXCOMM4_R = crate::BitReader<FLEXCOMM4_A>;
#[doc = "Flexcomm4 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM4_A {
    #[doc = "0: Clear Reset"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Set Reset"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM4_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM4_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM4_A {
        match self.bits {
            false => FLEXCOMM4_A::FLEXCOMM_CLR,
            true => FLEXCOMM4_A::FLEXCOMM_SET,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_CLR`"]
    #[inline(always)]
    pub fn is_flexcomm_clr(&self) -> bool {
        *self == FLEXCOMM4_A::FLEXCOMM_CLR
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_SET`"]
    #[inline(always)]
    pub fn is_flexcomm_set(&self) -> bool {
        *self == FLEXCOMM4_A::FLEXCOMM_SET
    }
}
#[doc = "Field `FLEXCOMM4` writer - Flexcomm4 reset control"]
pub type FLEXCOMM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, FLEXCOMM4_A, O>;
impl<'a, const O: u8> FLEXCOMM4_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM4_A::FLEXCOMM_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM4_A::FLEXCOMM_SET)
    }
}
#[doc = "Field `FLEXCOMM5` reader - Flexcomm5 reset control"]
pub type FLEXCOMM5_R = crate::BitReader<FLEXCOMM5_A>;
#[doc = "Flexcomm5 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM5_A {
    #[doc = "0: Clear Reset"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Set Reset"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM5_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM5_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM5_A {
        match self.bits {
            false => FLEXCOMM5_A::FLEXCOMM_CLR,
            true => FLEXCOMM5_A::FLEXCOMM_SET,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_CLR`"]
    #[inline(always)]
    pub fn is_flexcomm_clr(&self) -> bool {
        *self == FLEXCOMM5_A::FLEXCOMM_CLR
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_SET`"]
    #[inline(always)]
    pub fn is_flexcomm_set(&self) -> bool {
        *self == FLEXCOMM5_A::FLEXCOMM_SET
    }
}
#[doc = "Field `FLEXCOMM5` writer - Flexcomm5 reset control"]
pub type FLEXCOMM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, FLEXCOMM5_A, O>;
impl<'a, const O: u8> FLEXCOMM5_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM5_A::FLEXCOMM_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM5_A::FLEXCOMM_SET)
    }
}
#[doc = "Field `FLEXCOMM6` reader - Flexcomm6 reset control"]
pub type FLEXCOMM6_R = crate::BitReader<FLEXCOMM6_A>;
#[doc = "Flexcomm6 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM6_A {
    #[doc = "0: Clear Reset"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Set Reset"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM6_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM6_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM6_A {
        match self.bits {
            false => FLEXCOMM6_A::FLEXCOMM_CLR,
            true => FLEXCOMM6_A::FLEXCOMM_SET,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_CLR`"]
    #[inline(always)]
    pub fn is_flexcomm_clr(&self) -> bool {
        *self == FLEXCOMM6_A::FLEXCOMM_CLR
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_SET`"]
    #[inline(always)]
    pub fn is_flexcomm_set(&self) -> bool {
        *self == FLEXCOMM6_A::FLEXCOMM_SET
    }
}
#[doc = "Field `FLEXCOMM6` writer - Flexcomm6 reset control"]
pub type FLEXCOMM6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, FLEXCOMM6_A, O>;
impl<'a, const O: u8> FLEXCOMM6_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM6_A::FLEXCOMM_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM6_A::FLEXCOMM_SET)
    }
}
#[doc = "Field `FLEXCOMM7` reader - Flexcomm7 reset control"]
pub type FLEXCOMM7_R = crate::BitReader<FLEXCOMM7_A>;
#[doc = "Flexcomm7 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM7_A {
    #[doc = "0: Clear Reset"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Set Reset"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM7_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM7_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM7_A {
        match self.bits {
            false => FLEXCOMM7_A::FLEXCOMM_CLR,
            true => FLEXCOMM7_A::FLEXCOMM_SET,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_CLR`"]
    #[inline(always)]
    pub fn is_flexcomm_clr(&self) -> bool {
        *self == FLEXCOMM7_A::FLEXCOMM_CLR
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_SET`"]
    #[inline(always)]
    pub fn is_flexcomm_set(&self) -> bool {
        *self == FLEXCOMM7_A::FLEXCOMM_SET
    }
}
#[doc = "Field `FLEXCOMM7` writer - Flexcomm7 reset control"]
pub type FLEXCOMM7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, FLEXCOMM7_A, O>;
impl<'a, const O: u8> FLEXCOMM7_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM7_A::FLEXCOMM_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM7_A::FLEXCOMM_SET)
    }
}
#[doc = "Field `FLEXCOMM8` reader - Flexcomm8 reset control"]
pub type FLEXCOMM8_R = crate::BitReader<FLEXCOMM8_A>;
#[doc = "Flexcomm8 reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM8_A {
    #[doc = "0: Clear Reset"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Set Reset"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM8_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM8_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM8_A {
        match self.bits {
            false => FLEXCOMM8_A::FLEXCOMM_CLR,
            true => FLEXCOMM8_A::FLEXCOMM_SET,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_CLR`"]
    #[inline(always)]
    pub fn is_flexcomm_clr(&self) -> bool {
        *self == FLEXCOMM8_A::FLEXCOMM_CLR
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_SET`"]
    #[inline(always)]
    pub fn is_flexcomm_set(&self) -> bool {
        *self == FLEXCOMM8_A::FLEXCOMM_SET
    }
}
#[doc = "Field `FLEXCOMM8` writer - Flexcomm8 reset control"]
pub type FLEXCOMM8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, FLEXCOMM8_A, O>;
impl<'a, const O: u8> FLEXCOMM8_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM8_A::FLEXCOMM_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM8_A::FLEXCOMM_SET)
    }
}
#[doc = "Field `FLEXCOMM9` reader - Flexcomm9 reset control"]
pub type FLEXCOMM9_R = crate::BitReader<FLEXCOMM9_A>;
#[doc = "Flexcomm9 reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM9_A {
    #[doc = "0: Clear Reset"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Set Reset"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM9_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM9_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM9_A {
        match self.bits {
            false => FLEXCOMM9_A::FLEXCOMM_CLR,
            true => FLEXCOMM9_A::FLEXCOMM_SET,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_CLR`"]
    #[inline(always)]
    pub fn is_flexcomm_clr(&self) -> bool {
        *self == FLEXCOMM9_A::FLEXCOMM_CLR
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_SET`"]
    #[inline(always)]
    pub fn is_flexcomm_set(&self) -> bool {
        *self == FLEXCOMM9_A::FLEXCOMM_SET
    }
}
#[doc = "Field `FLEXCOMM9` writer - Flexcomm9 reset control"]
pub type FLEXCOMM9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, FLEXCOMM9_A, O>;
impl<'a, const O: u8> FLEXCOMM9_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM9_A::FLEXCOMM_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM9_A::FLEXCOMM_SET)
    }
}
#[doc = "Field `FLEXCOMM10` reader - Flexcomm10 reset control"]
pub type FLEXCOMM10_R = crate::BitReader<FLEXCOMM10_A>;
#[doc = "Flexcomm10 reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM10_A {
    #[doc = "0: Clear Reset"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Set Reset"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM10_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM10_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM10_A {
        match self.bits {
            false => FLEXCOMM10_A::FLEXCOMM_CLR,
            true => FLEXCOMM10_A::FLEXCOMM_SET,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_CLR`"]
    #[inline(always)]
    pub fn is_flexcomm_clr(&self) -> bool {
        *self == FLEXCOMM10_A::FLEXCOMM_CLR
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_SET`"]
    #[inline(always)]
    pub fn is_flexcomm_set(&self) -> bool {
        *self == FLEXCOMM10_A::FLEXCOMM_SET
    }
}
#[doc = "Field `FLEXCOMM10` writer - Flexcomm10 reset control"]
pub type FLEXCOMM10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, FLEXCOMM10_A, O>;
impl<'a, const O: u8> FLEXCOMM10_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM10_A::FLEXCOMM_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM10_A::FLEXCOMM_SET)
    }
}
#[doc = "Field `FLEXCOMM11` reader - Flexcomm11 reset control"]
pub type FLEXCOMM11_R = crate::BitReader<FLEXCOMM11_A>;
#[doc = "Flexcomm11 reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM11_A {
    #[doc = "0: Clear Reset"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Set Reset"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM11_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM11_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM11_A {
        match self.bits {
            false => FLEXCOMM11_A::FLEXCOMM_CLR,
            true => FLEXCOMM11_A::FLEXCOMM_SET,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_CLR`"]
    #[inline(always)]
    pub fn is_flexcomm_clr(&self) -> bool {
        *self == FLEXCOMM11_A::FLEXCOMM_CLR
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_SET`"]
    #[inline(always)]
    pub fn is_flexcomm_set(&self) -> bool {
        *self == FLEXCOMM11_A::FLEXCOMM_SET
    }
}
#[doc = "Field `FLEXCOMM11` writer - Flexcomm11 reset control"]
pub type FLEXCOMM11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, FLEXCOMM11_A, O>;
impl<'a, const O: u8> FLEXCOMM11_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM11_A::FLEXCOMM_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM11_A::FLEXCOMM_SET)
    }
}
#[doc = "Field `FLEXCOMM12` reader - Flexcomm12 reset control"]
pub type FLEXCOMM12_R = crate::BitReader<FLEXCOMM12_A>;
#[doc = "Flexcomm12 reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM12_A {
    #[doc = "0: Clear Reset"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Set Reset"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM12_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM12_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM12_A {
        match self.bits {
            false => FLEXCOMM12_A::FLEXCOMM_CLR,
            true => FLEXCOMM12_A::FLEXCOMM_SET,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_CLR`"]
    #[inline(always)]
    pub fn is_flexcomm_clr(&self) -> bool {
        *self == FLEXCOMM12_A::FLEXCOMM_CLR
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_SET`"]
    #[inline(always)]
    pub fn is_flexcomm_set(&self) -> bool {
        *self == FLEXCOMM12_A::FLEXCOMM_SET
    }
}
#[doc = "Field `FLEXCOMM12` writer - Flexcomm12 reset control"]
pub type FLEXCOMM12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, FLEXCOMM12_A, O>;
impl<'a, const O: u8> FLEXCOMM12_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM12_A::FLEXCOMM_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM12_A::FLEXCOMM_SET)
    }
}
#[doc = "Field `FLEXCOMM13` reader - Flexcomm13 reset control"]
pub type FLEXCOMM13_R = crate::BitReader<FLEXCOMM13_A>;
#[doc = "Flexcomm13 reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM13_A {
    #[doc = "0: Clear Reset"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Set Reset"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM13_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM13_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM13_A {
        match self.bits {
            false => FLEXCOMM13_A::FLEXCOMM_CLR,
            true => FLEXCOMM13_A::FLEXCOMM_SET,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_CLR`"]
    #[inline(always)]
    pub fn is_flexcomm_clr(&self) -> bool {
        *self == FLEXCOMM13_A::FLEXCOMM_CLR
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_SET`"]
    #[inline(always)]
    pub fn is_flexcomm_set(&self) -> bool {
        *self == FLEXCOMM13_A::FLEXCOMM_SET
    }
}
#[doc = "Field `FLEXCOMM13` writer - Flexcomm13 reset control"]
pub type FLEXCOMM13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, FLEXCOMM13_A, O>;
impl<'a, const O: u8> FLEXCOMM13_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM13_A::FLEXCOMM_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM13_A::FLEXCOMM_SET)
    }
}
#[doc = "Field `FLEXCOMM14` reader - Flexcomm14 SPI0 reset control"]
pub type FLEXCOMM14_R = crate::BitReader<FLEXCOMM14_A>;
#[doc = "Flexcomm14 SPI0 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM14_A {
    #[doc = "0: Clear Reset"]
    FLEXCOMM_CLR = 0,
    #[doc = "1: Set Reset"]
    FLEXCOMM_SET = 1,
}
impl From<FLEXCOMM14_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM14_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM14_A {
        match self.bits {
            false => FLEXCOMM14_A::FLEXCOMM_CLR,
            true => FLEXCOMM14_A::FLEXCOMM_SET,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_CLR`"]
    #[inline(always)]
    pub fn is_flexcomm_clr(&self) -> bool {
        *self == FLEXCOMM14_A::FLEXCOMM_CLR
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM_SET`"]
    #[inline(always)]
    pub fn is_flexcomm_set(&self) -> bool {
        *self == FLEXCOMM14_A::FLEXCOMM_SET
    }
}
#[doc = "Field `FLEXCOMM14` writer - Flexcomm14 SPI0 reset control"]
pub type FLEXCOMM14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, FLEXCOMM14_A, O>;
impl<'a, const O: u8> FLEXCOMM14_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn flexcomm_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM14_A::FLEXCOMM_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn flexcomm_set(self) -> &'a mut W {
        self.variant(FLEXCOMM14_A::FLEXCOMM_SET)
    }
}
#[doc = "Field `FLEXCOMM15_I2C` reader - Flexcomm15 I2C reset control"]
pub type FLEXCOMM15_I2C_R = crate::BitReader<FLEXCOMM15_I2C_A>;
#[doc = "Flexcomm15 I2C reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM15_I2C_A {
    #[doc = "0: Clear Reset"]
    FLEXCOMM15_I2C_CLR = 0,
    #[doc = "1: Set Reset"]
    FLEXCOMM15_I2C_SET = 1,
}
impl From<FLEXCOMM15_I2C_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM15_I2C_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM15_I2C_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM15_I2C_A {
        match self.bits {
            false => FLEXCOMM15_I2C_A::FLEXCOMM15_I2C_CLR,
            true => FLEXCOMM15_I2C_A::FLEXCOMM15_I2C_SET,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM15_I2C_CLR`"]
    #[inline(always)]
    pub fn is_flexcomm15_i2c_clr(&self) -> bool {
        *self == FLEXCOMM15_I2C_A::FLEXCOMM15_I2C_CLR
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM15_I2C_SET`"]
    #[inline(always)]
    pub fn is_flexcomm15_i2c_set(&self) -> bool {
        *self == FLEXCOMM15_I2C_A::FLEXCOMM15_I2C_SET
    }
}
#[doc = "Field `FLEXCOMM15_I2C` writer - Flexcomm15 I2C reset control"]
pub type FLEXCOMM15_I2C_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_SPEC, FLEXCOMM15_I2C_A, O>;
impl<'a, const O: u8> FLEXCOMM15_I2C_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn flexcomm15_i2c_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM15_I2C_A::FLEXCOMM15_I2C_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn flexcomm15_i2c_set(self) -> &'a mut W {
        self.variant(FLEXCOMM15_I2C_A::FLEXCOMM15_I2C_SET)
    }
}
#[doc = "Field `DMIC0` reader - DMIC0 reset control"]
pub type DMIC0_R = crate::BitReader<DMIC0_A>;
#[doc = "DMIC0 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMIC0_A {
    #[doc = "0: Clear Reset"]
    DMIC0_CLR = 0,
    #[doc = "1: Set Reset"]
    DMIC0_SET = 1,
}
impl From<DMIC0_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC0_A) -> Self {
        variant as u8 != 0
    }
}
impl DMIC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC0_A {
        match self.bits {
            false => DMIC0_A::DMIC0_CLR,
            true => DMIC0_A::DMIC0_SET,
        }
    }
    #[doc = "Checks if the value of the field is `DMIC0_CLR`"]
    #[inline(always)]
    pub fn is_dmic0_clr(&self) -> bool {
        *self == DMIC0_A::DMIC0_CLR
    }
    #[doc = "Checks if the value of the field is `DMIC0_SET`"]
    #[inline(always)]
    pub fn is_dmic0_set(&self) -> bool {
        *self == DMIC0_A::DMIC0_SET
    }
}
#[doc = "Field `DMIC0` writer - DMIC0 reset control"]
pub type DMIC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, DMIC0_A, O>;
impl<'a, const O: u8> DMIC0_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn dmic0_clr(self) -> &'a mut W {
        self.variant(DMIC0_A::DMIC0_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn dmic0_set(self) -> &'a mut W {
        self.variant(DMIC0_A::DMIC0_SET)
    }
}
#[doc = "Field `FLEXCOMM16` reader - Flexcomm SPI reset control"]
pub type FLEXCOMM16_R = crate::BitReader<FLEXCOMM16_A>;
#[doc = "Flexcomm SPI reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM16_A {
    #[doc = "0: Clear Reset"]
    FLEXCOMM16_SPI1_CLR = 0,
    #[doc = "1: Set Reset"]
    FLEXCOMM16_SPI1_SET = 1,
}
impl From<FLEXCOMM16_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM16_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM16_A {
        match self.bits {
            false => FLEXCOMM16_A::FLEXCOMM16_SPI1_CLR,
            true => FLEXCOMM16_A::FLEXCOMM16_SPI1_SET,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM16_SPI1_CLR`"]
    #[inline(always)]
    pub fn is_flexcomm16_spi1_clr(&self) -> bool {
        *self == FLEXCOMM16_A::FLEXCOMM16_SPI1_CLR
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM16_SPI1_SET`"]
    #[inline(always)]
    pub fn is_flexcomm16_spi1_set(&self) -> bool {
        *self == FLEXCOMM16_A::FLEXCOMM16_SPI1_SET
    }
}
#[doc = "Field `FLEXCOMM16` writer - Flexcomm SPI reset control"]
pub type FLEXCOMM16_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, FLEXCOMM16_A, O>;
impl<'a, const O: u8> FLEXCOMM16_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn flexcomm16_spi1_clr(self) -> &'a mut W {
        self.variant(FLEXCOMM16_A::FLEXCOMM16_SPI1_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn flexcomm16_spi1_set(self) -> &'a mut W {
        self.variant(FLEXCOMM16_A::FLEXCOMM16_SPI1_SET)
    }
}
#[doc = "Field `OSEVENT_TIMER` reader - OSEVENT Timer reset control"]
pub type OSEVENT_TIMER_R = crate::BitReader<OSEVENT_TIMER_A>;
#[doc = "OSEVENT Timer reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSEVENT_TIMER_A {
    #[doc = "0: Clear Reset"]
    OSEVENT_TIMER_CLR = 0,
    #[doc = "1: Set Reset"]
    OSEVENT_TIMER_SET = 1,
}
impl From<OSEVENT_TIMER_A> for bool {
    #[inline(always)]
    fn from(variant: OSEVENT_TIMER_A) -> Self {
        variant as u8 != 0
    }
}
impl OSEVENT_TIMER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSEVENT_TIMER_A {
        match self.bits {
            false => OSEVENT_TIMER_A::OSEVENT_TIMER_CLR,
            true => OSEVENT_TIMER_A::OSEVENT_TIMER_SET,
        }
    }
    #[doc = "Checks if the value of the field is `OSEVENT_TIMER_CLR`"]
    #[inline(always)]
    pub fn is_osevent_timer_clr(&self) -> bool {
        *self == OSEVENT_TIMER_A::OSEVENT_TIMER_CLR
    }
    #[doc = "Checks if the value of the field is `OSEVENT_TIMER_SET`"]
    #[inline(always)]
    pub fn is_osevent_timer_set(&self) -> bool {
        *self == OSEVENT_TIMER_A::OSEVENT_TIMER_SET
    }
}
#[doc = "Field `OSEVENT_TIMER` writer - OSEVENT Timer reset control"]
pub type OSEVENT_TIMER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_SPEC, OSEVENT_TIMER_A, O>;
impl<'a, const O: u8> OSEVENT_TIMER_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn osevent_timer_clr(self) -> &'a mut W {
        self.variant(OSEVENT_TIMER_A::OSEVENT_TIMER_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn osevent_timer_set(self) -> &'a mut W {
        self.variant(OSEVENT_TIMER_A::OSEVENT_TIMER_SET)
    }
}
#[doc = "Field `FLEXIO` reader - FLEXIO reset control"]
pub type FLEXIO_R = crate::BitReader<FLEXIO_A>;
#[doc = "FLEXIO reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXIO_A {
    #[doc = "0: Clear Reset"]
    FLEXIO_CLR = 0,
    #[doc = "1: Set Reset"]
    FLEXIO_SET = 1,
}
impl From<FLEXIO_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXIO_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXIO_A {
        match self.bits {
            false => FLEXIO_A::FLEXIO_CLR,
            true => FLEXIO_A::FLEXIO_SET,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO_CLR`"]
    #[inline(always)]
    pub fn is_flexio_clr(&self) -> bool {
        *self == FLEXIO_A::FLEXIO_CLR
    }
    #[doc = "Checks if the value of the field is `FLEXIO_SET`"]
    #[inline(always)]
    pub fn is_flexio_set(&self) -> bool {
        *self == FLEXIO_A::FLEXIO_SET
    }
}
#[doc = "Field `FLEXIO` writer - FLEXIO reset control"]
pub type FLEXIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, FLEXIO_A, O>;
impl<'a, const O: u8> FLEXIO_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn flexio_clr(self) -> &'a mut W {
        self.variant(FLEXIO_A::FLEXIO_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn flexio_set(self) -> &'a mut W {
        self.variant(FLEXIO_A::FLEXIO_SET)
    }
}
impl R {
    #[doc = "Bit 8 - Flexcomm0 reset control"]
    #[inline(always)]
    pub fn flexcomm0(&self) -> FLEXCOMM0_R {
        FLEXCOMM0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Flexcomm1 reset control"]
    #[inline(always)]
    pub fn flexcomm1(&self) -> FLEXCOMM1_R {
        FLEXCOMM1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Flexcomm2 reset control"]
    #[inline(always)]
    pub fn flexcomm2(&self) -> FLEXCOMM2_R {
        FLEXCOMM2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Flexcomm3 reset control"]
    #[inline(always)]
    pub fn flexcomm3(&self) -> FLEXCOMM3_R {
        FLEXCOMM3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Flexcomm4 reset control"]
    #[inline(always)]
    pub fn flexcomm4(&self) -> FLEXCOMM4_R {
        FLEXCOMM4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Flexcomm5 reset control"]
    #[inline(always)]
    pub fn flexcomm5(&self) -> FLEXCOMM5_R {
        FLEXCOMM5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Flexcomm6 reset control"]
    #[inline(always)]
    pub fn flexcomm6(&self) -> FLEXCOMM6_R {
        FLEXCOMM6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Flexcomm7 reset control"]
    #[inline(always)]
    pub fn flexcomm7(&self) -> FLEXCOMM7_R {
        FLEXCOMM7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Flexcomm8 reset control"]
    #[inline(always)]
    pub fn flexcomm8(&self) -> FLEXCOMM8_R {
        FLEXCOMM8_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Flexcomm9 reset control"]
    #[inline(always)]
    pub fn flexcomm9(&self) -> FLEXCOMM9_R {
        FLEXCOMM9_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Flexcomm10 reset control"]
    #[inline(always)]
    pub fn flexcomm10(&self) -> FLEXCOMM10_R {
        FLEXCOMM10_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Flexcomm11 reset control"]
    #[inline(always)]
    pub fn flexcomm11(&self) -> FLEXCOMM11_R {
        FLEXCOMM11_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Flexcomm12 reset control"]
    #[inline(always)]
    pub fn flexcomm12(&self) -> FLEXCOMM12_R {
        FLEXCOMM12_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Flexcomm13 reset control"]
    #[inline(always)]
    pub fn flexcomm13(&self) -> FLEXCOMM13_R {
        FLEXCOMM13_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Flexcomm14 SPI0 reset control"]
    #[inline(always)]
    pub fn flexcomm14(&self) -> FLEXCOMM14_R {
        FLEXCOMM14_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Flexcomm15 I2C reset control"]
    #[inline(always)]
    pub fn flexcomm15_i2c(&self) -> FLEXCOMM15_I2C_R {
        FLEXCOMM15_I2C_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DMIC0 reset control"]
    #[inline(always)]
    pub fn dmic0(&self) -> DMIC0_R {
        DMIC0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Flexcomm SPI reset control"]
    #[inline(always)]
    pub fn flexcomm16(&self) -> FLEXCOMM16_R {
        FLEXCOMM16_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - OSEVENT Timer reset control"]
    #[inline(always)]
    pub fn osevent_timer(&self) -> OSEVENT_TIMER_R {
        OSEVENT_TIMER_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - FLEXIO reset control"]
    #[inline(always)]
    pub fn flexio(&self) -> FLEXIO_R {
        FLEXIO_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Flexcomm0 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm0(&mut self) -> FLEXCOMM0_W<8> {
        FLEXCOMM0_W::new(self)
    }
    #[doc = "Bit 9 - Flexcomm1 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm1(&mut self) -> FLEXCOMM1_W<9> {
        FLEXCOMM1_W::new(self)
    }
    #[doc = "Bit 10 - Flexcomm2 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm2(&mut self) -> FLEXCOMM2_W<10> {
        FLEXCOMM2_W::new(self)
    }
    #[doc = "Bit 11 - Flexcomm3 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm3(&mut self) -> FLEXCOMM3_W<11> {
        FLEXCOMM3_W::new(self)
    }
    #[doc = "Bit 12 - Flexcomm4 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm4(&mut self) -> FLEXCOMM4_W<12> {
        FLEXCOMM4_W::new(self)
    }
    #[doc = "Bit 13 - Flexcomm5 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm5(&mut self) -> FLEXCOMM5_W<13> {
        FLEXCOMM5_W::new(self)
    }
    #[doc = "Bit 14 - Flexcomm6 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm6(&mut self) -> FLEXCOMM6_W<14> {
        FLEXCOMM6_W::new(self)
    }
    #[doc = "Bit 15 - Flexcomm7 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm7(&mut self) -> FLEXCOMM7_W<15> {
        FLEXCOMM7_W::new(self)
    }
    #[doc = "Bit 16 - Flexcomm8 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm8(&mut self) -> FLEXCOMM8_W<16> {
        FLEXCOMM8_W::new(self)
    }
    #[doc = "Bit 17 - Flexcomm9 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm9(&mut self) -> FLEXCOMM9_W<17> {
        FLEXCOMM9_W::new(self)
    }
    #[doc = "Bit 18 - Flexcomm10 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm10(&mut self) -> FLEXCOMM10_W<18> {
        FLEXCOMM10_W::new(self)
    }
    #[doc = "Bit 19 - Flexcomm11 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm11(&mut self) -> FLEXCOMM11_W<19> {
        FLEXCOMM11_W::new(self)
    }
    #[doc = "Bit 20 - Flexcomm12 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm12(&mut self) -> FLEXCOMM12_W<20> {
        FLEXCOMM12_W::new(self)
    }
    #[doc = "Bit 21 - Flexcomm13 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm13(&mut self) -> FLEXCOMM13_W<21> {
        FLEXCOMM13_W::new(self)
    }
    #[doc = "Bit 22 - Flexcomm14 SPI0 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm14(&mut self) -> FLEXCOMM14_W<22> {
        FLEXCOMM14_W::new(self)
    }
    #[doc = "Bit 23 - Flexcomm15 I2C reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm15_i2c(&mut self) -> FLEXCOMM15_I2C_W<23> {
        FLEXCOMM15_I2C_W::new(self)
    }
    #[doc = "Bit 24 - DMIC0 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn dmic0(&mut self) -> DMIC0_W<24> {
        DMIC0_W::new(self)
    }
    #[doc = "Bit 25 - Flexcomm SPI reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm16(&mut self) -> FLEXCOMM16_W<25> {
        FLEXCOMM16_W::new(self)
    }
    #[doc = "Bit 27 - OSEVENT Timer reset control"]
    #[inline(always)]
    #[must_use]
    pub fn osevent_timer(&mut self) -> OSEVENT_TIMER_W<27> {
        OSEVENT_TIMER_W::new(self)
    }
    #[doc = "Bit 29 - FLEXIO reset control"]
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
#[doc = "Peripheral Reset Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prstctl0](index.html) module"]
pub struct PRSTCTL0_SPEC;
impl crate::RegisterSpec for PRSTCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prstctl0::R](R) reader structure"]
impl crate::Readable for PRSTCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prstctl0::W](W) writer structure"]
impl crate::Writable for PRSTCTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRSTCTL0 to value 0x01c0_ff00"]
impl crate::Resettable for PRSTCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x01c0_ff00;
}
