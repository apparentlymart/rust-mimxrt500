#[doc = "Register `AHBBRIDGEBUFFER[%s]` reader"]
pub struct R(crate::R<AHBBRIDGEBUFFER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBBRIDGEBUFFER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBBRIDGEBUFFER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBBRIDGEBUFFER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBBRIDGEBUFFER[%s]` writer"]
pub struct W(crate::W<AHBBRIDGEBUFFER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBBRIDGEBUFFER_SPEC>;
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
impl From<crate::W<AHBBRIDGEBUFFER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBBRIDGEBUFFER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLAVE0` reader - SLAVE0 buffering"]
pub type SLAVE0_R = crate::BitReader<SLAVE0_A>;
#[doc = "SLAVE0 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE0_A {
    #[doc = "0: No Buffering"]
    SLAVE0_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE0_1 = 1,
}
impl From<SLAVE0_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE0_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE0_A {
        match self.bits {
            false => SLAVE0_A::SLAVE0_0,
            true => SLAVE0_A::SLAVE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE0_0`"]
    #[inline(always)]
    pub fn is_slave0_0(&self) -> bool {
        *self == SLAVE0_A::SLAVE0_0
    }
    #[doc = "Checks if the value of the field is `SLAVE0_1`"]
    #[inline(always)]
    pub fn is_slave0_1(&self) -> bool {
        *self == SLAVE0_A::SLAVE0_1
    }
}
#[doc = "Field `SLAVE0` writer - SLAVE0 buffering"]
pub type SLAVE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE0_A, O>;
impl<'a, const O: u8> SLAVE0_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave0_0(self) -> &'a mut W {
        self.variant(SLAVE0_A::SLAVE0_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave0_1(self) -> &'a mut W {
        self.variant(SLAVE0_A::SLAVE0_1)
    }
}
#[doc = "Field `SLAVE1` reader - SLAVE1 buffering"]
pub type SLAVE1_R = crate::BitReader<SLAVE1_A>;
#[doc = "SLAVE1 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE1_A {
    #[doc = "0: No Buffering"]
    SLAVE1_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE1_1 = 1,
}
impl From<SLAVE1_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE1_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE1_A {
        match self.bits {
            false => SLAVE1_A::SLAVE1_0,
            true => SLAVE1_A::SLAVE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE1_0`"]
    #[inline(always)]
    pub fn is_slave1_0(&self) -> bool {
        *self == SLAVE1_A::SLAVE1_0
    }
    #[doc = "Checks if the value of the field is `SLAVE1_1`"]
    #[inline(always)]
    pub fn is_slave1_1(&self) -> bool {
        *self == SLAVE1_A::SLAVE1_1
    }
}
#[doc = "Field `SLAVE1` writer - SLAVE1 buffering"]
pub type SLAVE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE1_A, O>;
impl<'a, const O: u8> SLAVE1_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave1_0(self) -> &'a mut W {
        self.variant(SLAVE1_A::SLAVE1_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave1_1(self) -> &'a mut W {
        self.variant(SLAVE1_A::SLAVE1_1)
    }
}
#[doc = "Field `SLAVE2` reader - SLAVE2 buffering"]
pub type SLAVE2_R = crate::BitReader<SLAVE2_A>;
#[doc = "SLAVE2 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE2_A {
    #[doc = "0: No Buffering"]
    SLAVE2_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE2_1 = 1,
}
impl From<SLAVE2_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE2_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE2_A {
        match self.bits {
            false => SLAVE2_A::SLAVE2_0,
            true => SLAVE2_A::SLAVE2_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE2_0`"]
    #[inline(always)]
    pub fn is_slave2_0(&self) -> bool {
        *self == SLAVE2_A::SLAVE2_0
    }
    #[doc = "Checks if the value of the field is `SLAVE2_1`"]
    #[inline(always)]
    pub fn is_slave2_1(&self) -> bool {
        *self == SLAVE2_A::SLAVE2_1
    }
}
#[doc = "Field `SLAVE2` writer - SLAVE2 buffering"]
pub type SLAVE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE2_A, O>;
impl<'a, const O: u8> SLAVE2_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave2_0(self) -> &'a mut W {
        self.variant(SLAVE2_A::SLAVE2_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave2_1(self) -> &'a mut W {
        self.variant(SLAVE2_A::SLAVE2_1)
    }
}
#[doc = "Field `SLAVE3` reader - SLAVE3 buffering"]
pub type SLAVE3_R = crate::BitReader<SLAVE3_A>;
#[doc = "SLAVE3 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE3_A {
    #[doc = "0: No Buffering"]
    SLAVE3_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE3_1 = 1,
}
impl From<SLAVE3_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE3_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE3_A {
        match self.bits {
            false => SLAVE3_A::SLAVE3_0,
            true => SLAVE3_A::SLAVE3_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE3_0`"]
    #[inline(always)]
    pub fn is_slave3_0(&self) -> bool {
        *self == SLAVE3_A::SLAVE3_0
    }
    #[doc = "Checks if the value of the field is `SLAVE3_1`"]
    #[inline(always)]
    pub fn is_slave3_1(&self) -> bool {
        *self == SLAVE3_A::SLAVE3_1
    }
}
#[doc = "Field `SLAVE3` writer - SLAVE3 buffering"]
pub type SLAVE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE3_A, O>;
impl<'a, const O: u8> SLAVE3_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave3_0(self) -> &'a mut W {
        self.variant(SLAVE3_A::SLAVE3_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave3_1(self) -> &'a mut W {
        self.variant(SLAVE3_A::SLAVE3_1)
    }
}
#[doc = "Field `SLAVE4` reader - SLAVE4 buffering"]
pub type SLAVE4_R = crate::BitReader<SLAVE4_A>;
#[doc = "SLAVE4 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE4_A {
    #[doc = "0: No Buffering"]
    SLAVE4_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE4_1 = 1,
}
impl From<SLAVE4_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE4_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE4_A {
        match self.bits {
            false => SLAVE4_A::SLAVE4_0,
            true => SLAVE4_A::SLAVE4_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE4_0`"]
    #[inline(always)]
    pub fn is_slave4_0(&self) -> bool {
        *self == SLAVE4_A::SLAVE4_0
    }
    #[doc = "Checks if the value of the field is `SLAVE4_1`"]
    #[inline(always)]
    pub fn is_slave4_1(&self) -> bool {
        *self == SLAVE4_A::SLAVE4_1
    }
}
#[doc = "Field `SLAVE4` writer - SLAVE4 buffering"]
pub type SLAVE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE4_A, O>;
impl<'a, const O: u8> SLAVE4_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave4_0(self) -> &'a mut W {
        self.variant(SLAVE4_A::SLAVE4_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave4_1(self) -> &'a mut W {
        self.variant(SLAVE4_A::SLAVE4_1)
    }
}
#[doc = "Field `SLAVE5` reader - SLAVE5 buffering"]
pub type SLAVE5_R = crate::BitReader<SLAVE5_A>;
#[doc = "SLAVE5 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE5_A {
    #[doc = "0: No Buffering"]
    SLAVE5_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE5_1 = 1,
}
impl From<SLAVE5_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE5_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE5_A {
        match self.bits {
            false => SLAVE5_A::SLAVE5_0,
            true => SLAVE5_A::SLAVE5_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE5_0`"]
    #[inline(always)]
    pub fn is_slave5_0(&self) -> bool {
        *self == SLAVE5_A::SLAVE5_0
    }
    #[doc = "Checks if the value of the field is `SLAVE5_1`"]
    #[inline(always)]
    pub fn is_slave5_1(&self) -> bool {
        *self == SLAVE5_A::SLAVE5_1
    }
}
#[doc = "Field `SLAVE5` writer - SLAVE5 buffering"]
pub type SLAVE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE5_A, O>;
impl<'a, const O: u8> SLAVE5_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave5_0(self) -> &'a mut W {
        self.variant(SLAVE5_A::SLAVE5_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave5_1(self) -> &'a mut W {
        self.variant(SLAVE5_A::SLAVE5_1)
    }
}
#[doc = "Field `SLAVE6` reader - SLAVE6 buffering"]
pub type SLAVE6_R = crate::BitReader<SLAVE6_A>;
#[doc = "SLAVE6 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE6_A {
    #[doc = "0: No Buffering"]
    SLAVE6_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE6_1 = 1,
}
impl From<SLAVE6_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE6_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE6_A {
        match self.bits {
            false => SLAVE6_A::SLAVE6_0,
            true => SLAVE6_A::SLAVE6_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE6_0`"]
    #[inline(always)]
    pub fn is_slave6_0(&self) -> bool {
        *self == SLAVE6_A::SLAVE6_0
    }
    #[doc = "Checks if the value of the field is `SLAVE6_1`"]
    #[inline(always)]
    pub fn is_slave6_1(&self) -> bool {
        *self == SLAVE6_A::SLAVE6_1
    }
}
#[doc = "Field `SLAVE6` writer - SLAVE6 buffering"]
pub type SLAVE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE6_A, O>;
impl<'a, const O: u8> SLAVE6_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave6_0(self) -> &'a mut W {
        self.variant(SLAVE6_A::SLAVE6_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave6_1(self) -> &'a mut W {
        self.variant(SLAVE6_A::SLAVE6_1)
    }
}
#[doc = "Field `SLAVE7` reader - SLAVE7 buffering"]
pub type SLAVE7_R = crate::BitReader<SLAVE7_A>;
#[doc = "SLAVE7 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE7_A {
    #[doc = "0: No Buffering"]
    SLAVE7_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE7_1 = 1,
}
impl From<SLAVE7_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE7_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE7_A {
        match self.bits {
            false => SLAVE7_A::SLAVE7_0,
            true => SLAVE7_A::SLAVE7_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE7_0`"]
    #[inline(always)]
    pub fn is_slave7_0(&self) -> bool {
        *self == SLAVE7_A::SLAVE7_0
    }
    #[doc = "Checks if the value of the field is `SLAVE7_1`"]
    #[inline(always)]
    pub fn is_slave7_1(&self) -> bool {
        *self == SLAVE7_A::SLAVE7_1
    }
}
#[doc = "Field `SLAVE7` writer - SLAVE7 buffering"]
pub type SLAVE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE7_A, O>;
impl<'a, const O: u8> SLAVE7_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave7_0(self) -> &'a mut W {
        self.variant(SLAVE7_A::SLAVE7_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave7_1(self) -> &'a mut W {
        self.variant(SLAVE7_A::SLAVE7_1)
    }
}
#[doc = "Field `SLAVE8` reader - SLAVE8 buffering"]
pub type SLAVE8_R = crate::BitReader<SLAVE8_A>;
#[doc = "SLAVE8 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE8_A {
    #[doc = "0: No Buffering"]
    SLAVE8_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE8_1 = 1,
}
impl From<SLAVE8_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE8_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE8_A {
        match self.bits {
            false => SLAVE8_A::SLAVE8_0,
            true => SLAVE8_A::SLAVE8_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE8_0`"]
    #[inline(always)]
    pub fn is_slave8_0(&self) -> bool {
        *self == SLAVE8_A::SLAVE8_0
    }
    #[doc = "Checks if the value of the field is `SLAVE8_1`"]
    #[inline(always)]
    pub fn is_slave8_1(&self) -> bool {
        *self == SLAVE8_A::SLAVE8_1
    }
}
#[doc = "Field `SLAVE8` writer - SLAVE8 buffering"]
pub type SLAVE8_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE8_A, O>;
impl<'a, const O: u8> SLAVE8_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave8_0(self) -> &'a mut W {
        self.variant(SLAVE8_A::SLAVE8_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave8_1(self) -> &'a mut W {
        self.variant(SLAVE8_A::SLAVE8_1)
    }
}
#[doc = "Field `SLAVE9` reader - SLAVE9 buffering"]
pub type SLAVE9_R = crate::BitReader<SLAVE9_A>;
#[doc = "SLAVE9 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE9_A {
    #[doc = "0: No Buffering"]
    SLAVE9_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE9_1 = 1,
}
impl From<SLAVE9_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE9_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE9_A {
        match self.bits {
            false => SLAVE9_A::SLAVE9_0,
            true => SLAVE9_A::SLAVE9_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE9_0`"]
    #[inline(always)]
    pub fn is_slave9_0(&self) -> bool {
        *self == SLAVE9_A::SLAVE9_0
    }
    #[doc = "Checks if the value of the field is `SLAVE9_1`"]
    #[inline(always)]
    pub fn is_slave9_1(&self) -> bool {
        *self == SLAVE9_A::SLAVE9_1
    }
}
#[doc = "Field `SLAVE9` writer - SLAVE9 buffering"]
pub type SLAVE9_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE9_A, O>;
impl<'a, const O: u8> SLAVE9_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave9_0(self) -> &'a mut W {
        self.variant(SLAVE9_A::SLAVE9_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave9_1(self) -> &'a mut W {
        self.variant(SLAVE9_A::SLAVE9_1)
    }
}
#[doc = "Field `SLAVE10` reader - SLAVE10 buffering"]
pub type SLAVE10_R = crate::BitReader<SLAVE10_A>;
#[doc = "SLAVE10 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE10_A {
    #[doc = "0: No Buffering"]
    SLAVE10_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE10_1 = 1,
}
impl From<SLAVE10_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE10_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE10_A {
        match self.bits {
            false => SLAVE10_A::SLAVE10_0,
            true => SLAVE10_A::SLAVE10_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE10_0`"]
    #[inline(always)]
    pub fn is_slave10_0(&self) -> bool {
        *self == SLAVE10_A::SLAVE10_0
    }
    #[doc = "Checks if the value of the field is `SLAVE10_1`"]
    #[inline(always)]
    pub fn is_slave10_1(&self) -> bool {
        *self == SLAVE10_A::SLAVE10_1
    }
}
#[doc = "Field `SLAVE10` writer - SLAVE10 buffering"]
pub type SLAVE10_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE10_A, O>;
impl<'a, const O: u8> SLAVE10_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave10_0(self) -> &'a mut W {
        self.variant(SLAVE10_A::SLAVE10_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave10_1(self) -> &'a mut W {
        self.variant(SLAVE10_A::SLAVE10_1)
    }
}
#[doc = "Field `SLAVE11` reader - SLAVE11 buffering"]
pub type SLAVE11_R = crate::BitReader<SLAVE11_A>;
#[doc = "SLAVE11 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE11_A {
    #[doc = "0: No Buffering"]
    SLAVE11_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE11_1 = 1,
}
impl From<SLAVE11_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE11_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE11_A {
        match self.bits {
            false => SLAVE11_A::SLAVE11_0,
            true => SLAVE11_A::SLAVE11_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE11_0`"]
    #[inline(always)]
    pub fn is_slave11_0(&self) -> bool {
        *self == SLAVE11_A::SLAVE11_0
    }
    #[doc = "Checks if the value of the field is `SLAVE11_1`"]
    #[inline(always)]
    pub fn is_slave11_1(&self) -> bool {
        *self == SLAVE11_A::SLAVE11_1
    }
}
#[doc = "Field `SLAVE11` writer - SLAVE11 buffering"]
pub type SLAVE11_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE11_A, O>;
impl<'a, const O: u8> SLAVE11_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave11_0(self) -> &'a mut W {
        self.variant(SLAVE11_A::SLAVE11_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave11_1(self) -> &'a mut W {
        self.variant(SLAVE11_A::SLAVE11_1)
    }
}
#[doc = "Field `SLAVE12` reader - SLAVE12 buffering"]
pub type SLAVE12_R = crate::BitReader<SLAVE12_A>;
#[doc = "SLAVE12 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE12_A {
    #[doc = "0: No Buffering"]
    SLAVE12_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE12_1 = 1,
}
impl From<SLAVE12_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE12_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE12_A {
        match self.bits {
            false => SLAVE12_A::SLAVE12_0,
            true => SLAVE12_A::SLAVE12_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE12_0`"]
    #[inline(always)]
    pub fn is_slave12_0(&self) -> bool {
        *self == SLAVE12_A::SLAVE12_0
    }
    #[doc = "Checks if the value of the field is `SLAVE12_1`"]
    #[inline(always)]
    pub fn is_slave12_1(&self) -> bool {
        *self == SLAVE12_A::SLAVE12_1
    }
}
#[doc = "Field `SLAVE12` writer - SLAVE12 buffering"]
pub type SLAVE12_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE12_A, O>;
impl<'a, const O: u8> SLAVE12_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave12_0(self) -> &'a mut W {
        self.variant(SLAVE12_A::SLAVE12_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave12_1(self) -> &'a mut W {
        self.variant(SLAVE12_A::SLAVE12_1)
    }
}
#[doc = "Field `SLAVE13` reader - SLAVE13 buffering"]
pub type SLAVE13_R = crate::BitReader<SLAVE13_A>;
#[doc = "SLAVE13 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE13_A {
    #[doc = "0: No Buffering"]
    SLAVE13_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE13_1 = 1,
}
impl From<SLAVE13_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE13_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE13_A {
        match self.bits {
            false => SLAVE13_A::SLAVE13_0,
            true => SLAVE13_A::SLAVE13_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE13_0`"]
    #[inline(always)]
    pub fn is_slave13_0(&self) -> bool {
        *self == SLAVE13_A::SLAVE13_0
    }
    #[doc = "Checks if the value of the field is `SLAVE13_1`"]
    #[inline(always)]
    pub fn is_slave13_1(&self) -> bool {
        *self == SLAVE13_A::SLAVE13_1
    }
}
#[doc = "Field `SLAVE13` writer - SLAVE13 buffering"]
pub type SLAVE13_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE13_A, O>;
impl<'a, const O: u8> SLAVE13_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave13_0(self) -> &'a mut W {
        self.variant(SLAVE13_A::SLAVE13_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave13_1(self) -> &'a mut W {
        self.variant(SLAVE13_A::SLAVE13_1)
    }
}
#[doc = "Field `SLAVE14` reader - SLAVE14 buffering"]
pub type SLAVE14_R = crate::BitReader<SLAVE14_A>;
#[doc = "SLAVE14 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE14_A {
    #[doc = "0: No Buffering"]
    SLAVE14_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE14_1 = 1,
}
impl From<SLAVE14_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE14_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE14_A {
        match self.bits {
            false => SLAVE14_A::SLAVE14_0,
            true => SLAVE14_A::SLAVE14_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE14_0`"]
    #[inline(always)]
    pub fn is_slave14_0(&self) -> bool {
        *self == SLAVE14_A::SLAVE14_0
    }
    #[doc = "Checks if the value of the field is `SLAVE14_1`"]
    #[inline(always)]
    pub fn is_slave14_1(&self) -> bool {
        *self == SLAVE14_A::SLAVE14_1
    }
}
#[doc = "Field `SLAVE14` writer - SLAVE14 buffering"]
pub type SLAVE14_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE14_A, O>;
impl<'a, const O: u8> SLAVE14_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave14_0(self) -> &'a mut W {
        self.variant(SLAVE14_A::SLAVE14_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave14_1(self) -> &'a mut W {
        self.variant(SLAVE14_A::SLAVE14_1)
    }
}
#[doc = "Field `SLAVE15` reader - SLAVE15 buffering"]
pub type SLAVE15_R = crate::BitReader<SLAVE15_A>;
#[doc = "SLAVE15 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE15_A {
    #[doc = "0: No Buffering"]
    SLAVE15_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE15_1 = 1,
}
impl From<SLAVE15_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE15_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE15_A {
        match self.bits {
            false => SLAVE15_A::SLAVE15_0,
            true => SLAVE15_A::SLAVE15_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE15_0`"]
    #[inline(always)]
    pub fn is_slave15_0(&self) -> bool {
        *self == SLAVE15_A::SLAVE15_0
    }
    #[doc = "Checks if the value of the field is `SLAVE15_1`"]
    #[inline(always)]
    pub fn is_slave15_1(&self) -> bool {
        *self == SLAVE15_A::SLAVE15_1
    }
}
#[doc = "Field `SLAVE15` writer - SLAVE15 buffering"]
pub type SLAVE15_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE15_A, O>;
impl<'a, const O: u8> SLAVE15_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave15_0(self) -> &'a mut W {
        self.variant(SLAVE15_A::SLAVE15_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave15_1(self) -> &'a mut W {
        self.variant(SLAVE15_A::SLAVE15_1)
    }
}
#[doc = "Field `SLAVE16` reader - SLAVE16 buffering"]
pub type SLAVE16_R = crate::BitReader<SLAVE16_A>;
#[doc = "SLAVE16 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE16_A {
    #[doc = "0: No Buffering"]
    SLAVE16_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE16_1 = 1,
}
impl From<SLAVE16_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE16_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE16_A {
        match self.bits {
            false => SLAVE16_A::SLAVE16_0,
            true => SLAVE16_A::SLAVE16_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE16_0`"]
    #[inline(always)]
    pub fn is_slave16_0(&self) -> bool {
        *self == SLAVE16_A::SLAVE16_0
    }
    #[doc = "Checks if the value of the field is `SLAVE16_1`"]
    #[inline(always)]
    pub fn is_slave16_1(&self) -> bool {
        *self == SLAVE16_A::SLAVE16_1
    }
}
#[doc = "Field `SLAVE16` writer - SLAVE16 buffering"]
pub type SLAVE16_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE16_A, O>;
impl<'a, const O: u8> SLAVE16_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave16_0(self) -> &'a mut W {
        self.variant(SLAVE16_A::SLAVE16_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave16_1(self) -> &'a mut W {
        self.variant(SLAVE16_A::SLAVE16_1)
    }
}
#[doc = "Field `SLAVE17` reader - SLAVE17 buffering"]
pub type SLAVE17_R = crate::BitReader<SLAVE17_A>;
#[doc = "SLAVE17 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE17_A {
    #[doc = "0: No Buffering"]
    SLAVE17_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE17_1 = 1,
}
impl From<SLAVE17_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE17_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE17_A {
        match self.bits {
            false => SLAVE17_A::SLAVE17_0,
            true => SLAVE17_A::SLAVE17_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE17_0`"]
    #[inline(always)]
    pub fn is_slave17_0(&self) -> bool {
        *self == SLAVE17_A::SLAVE17_0
    }
    #[doc = "Checks if the value of the field is `SLAVE17_1`"]
    #[inline(always)]
    pub fn is_slave17_1(&self) -> bool {
        *self == SLAVE17_A::SLAVE17_1
    }
}
#[doc = "Field `SLAVE17` writer - SLAVE17 buffering"]
pub type SLAVE17_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE17_A, O>;
impl<'a, const O: u8> SLAVE17_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave17_0(self) -> &'a mut W {
        self.variant(SLAVE17_A::SLAVE17_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave17_1(self) -> &'a mut W {
        self.variant(SLAVE17_A::SLAVE17_1)
    }
}
#[doc = "Field `SLAVE18` reader - SLAVE18 buffering"]
pub type SLAVE18_R = crate::BitReader<SLAVE18_A>;
#[doc = "SLAVE18 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE18_A {
    #[doc = "0: No Buffering"]
    SLAVE18_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE18_1 = 1,
}
impl From<SLAVE18_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE18_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE18_A {
        match self.bits {
            false => SLAVE18_A::SLAVE18_0,
            true => SLAVE18_A::SLAVE18_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE18_0`"]
    #[inline(always)]
    pub fn is_slave18_0(&self) -> bool {
        *self == SLAVE18_A::SLAVE18_0
    }
    #[doc = "Checks if the value of the field is `SLAVE18_1`"]
    #[inline(always)]
    pub fn is_slave18_1(&self) -> bool {
        *self == SLAVE18_A::SLAVE18_1
    }
}
#[doc = "Field `SLAVE18` writer - SLAVE18 buffering"]
pub type SLAVE18_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE18_A, O>;
impl<'a, const O: u8> SLAVE18_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave18_0(self) -> &'a mut W {
        self.variant(SLAVE18_A::SLAVE18_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave18_1(self) -> &'a mut W {
        self.variant(SLAVE18_A::SLAVE18_1)
    }
}
#[doc = "Field `SLAVE19` reader - SLAVE19 buffering"]
pub type SLAVE19_R = crate::BitReader<SLAVE19_A>;
#[doc = "SLAVE19 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE19_A {
    #[doc = "0: No Buffering"]
    SLAVE19_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE19_1 = 1,
}
impl From<SLAVE19_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE19_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE19_A {
        match self.bits {
            false => SLAVE19_A::SLAVE19_0,
            true => SLAVE19_A::SLAVE19_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE19_0`"]
    #[inline(always)]
    pub fn is_slave19_0(&self) -> bool {
        *self == SLAVE19_A::SLAVE19_0
    }
    #[doc = "Checks if the value of the field is `SLAVE19_1`"]
    #[inline(always)]
    pub fn is_slave19_1(&self) -> bool {
        *self == SLAVE19_A::SLAVE19_1
    }
}
#[doc = "Field `SLAVE19` writer - SLAVE19 buffering"]
pub type SLAVE19_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE19_A, O>;
impl<'a, const O: u8> SLAVE19_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave19_0(self) -> &'a mut W {
        self.variant(SLAVE19_A::SLAVE19_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave19_1(self) -> &'a mut W {
        self.variant(SLAVE19_A::SLAVE19_1)
    }
}
#[doc = "Field `SLAVE20` reader - SLAVE20 buffering"]
pub type SLAVE20_R = crate::BitReader<SLAVE20_A>;
#[doc = "SLAVE20 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE20_A {
    #[doc = "0: No Buffering"]
    SLAVE20_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE20_1 = 1,
}
impl From<SLAVE20_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE20_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE20_A {
        match self.bits {
            false => SLAVE20_A::SLAVE20_0,
            true => SLAVE20_A::SLAVE20_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE20_0`"]
    #[inline(always)]
    pub fn is_slave20_0(&self) -> bool {
        *self == SLAVE20_A::SLAVE20_0
    }
    #[doc = "Checks if the value of the field is `SLAVE20_1`"]
    #[inline(always)]
    pub fn is_slave20_1(&self) -> bool {
        *self == SLAVE20_A::SLAVE20_1
    }
}
#[doc = "Field `SLAVE20` writer - SLAVE20 buffering"]
pub type SLAVE20_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE20_A, O>;
impl<'a, const O: u8> SLAVE20_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave20_0(self) -> &'a mut W {
        self.variant(SLAVE20_A::SLAVE20_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave20_1(self) -> &'a mut W {
        self.variant(SLAVE20_A::SLAVE20_1)
    }
}
#[doc = "Field `SLAVE21` reader - SLAVE21 buffering"]
pub type SLAVE21_R = crate::BitReader<SLAVE21_A>;
#[doc = "SLAVE21 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE21_A {
    #[doc = "0: No Buffering"]
    SLAVE21_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE21_1 = 1,
}
impl From<SLAVE21_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE21_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE21_A {
        match self.bits {
            false => SLAVE21_A::SLAVE21_0,
            true => SLAVE21_A::SLAVE21_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE21_0`"]
    #[inline(always)]
    pub fn is_slave21_0(&self) -> bool {
        *self == SLAVE21_A::SLAVE21_0
    }
    #[doc = "Checks if the value of the field is `SLAVE21_1`"]
    #[inline(always)]
    pub fn is_slave21_1(&self) -> bool {
        *self == SLAVE21_A::SLAVE21_1
    }
}
#[doc = "Field `SLAVE21` writer - SLAVE21 buffering"]
pub type SLAVE21_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE21_A, O>;
impl<'a, const O: u8> SLAVE21_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave21_0(self) -> &'a mut W {
        self.variant(SLAVE21_A::SLAVE21_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave21_1(self) -> &'a mut W {
        self.variant(SLAVE21_A::SLAVE21_1)
    }
}
#[doc = "Field `SLAVE22` reader - SLAVE22 buffering"]
pub type SLAVE22_R = crate::BitReader<SLAVE22_A>;
#[doc = "SLAVE22 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE22_A {
    #[doc = "0: No Buffering"]
    SLAVE22_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE22_1 = 1,
}
impl From<SLAVE22_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE22_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE22_A {
        match self.bits {
            false => SLAVE22_A::SLAVE22_0,
            true => SLAVE22_A::SLAVE22_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE22_0`"]
    #[inline(always)]
    pub fn is_slave22_0(&self) -> bool {
        *self == SLAVE22_A::SLAVE22_0
    }
    #[doc = "Checks if the value of the field is `SLAVE22_1`"]
    #[inline(always)]
    pub fn is_slave22_1(&self) -> bool {
        *self == SLAVE22_A::SLAVE22_1
    }
}
#[doc = "Field `SLAVE22` writer - SLAVE22 buffering"]
pub type SLAVE22_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE22_A, O>;
impl<'a, const O: u8> SLAVE22_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave22_0(self) -> &'a mut W {
        self.variant(SLAVE22_A::SLAVE22_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave22_1(self) -> &'a mut W {
        self.variant(SLAVE22_A::SLAVE22_1)
    }
}
#[doc = "Field `SLAVE23` reader - SLAVE23 buffering"]
pub type SLAVE23_R = crate::BitReader<SLAVE23_A>;
#[doc = "SLAVE23 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE23_A {
    #[doc = "0: No Buffering"]
    SLAVE23_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE23_1 = 1,
}
impl From<SLAVE23_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE23_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE23_A {
        match self.bits {
            false => SLAVE23_A::SLAVE23_0,
            true => SLAVE23_A::SLAVE23_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE23_0`"]
    #[inline(always)]
    pub fn is_slave23_0(&self) -> bool {
        *self == SLAVE23_A::SLAVE23_0
    }
    #[doc = "Checks if the value of the field is `SLAVE23_1`"]
    #[inline(always)]
    pub fn is_slave23_1(&self) -> bool {
        *self == SLAVE23_A::SLAVE23_1
    }
}
#[doc = "Field `SLAVE23` writer - SLAVE23 buffering"]
pub type SLAVE23_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE23_A, O>;
impl<'a, const O: u8> SLAVE23_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave23_0(self) -> &'a mut W {
        self.variant(SLAVE23_A::SLAVE23_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave23_1(self) -> &'a mut W {
        self.variant(SLAVE23_A::SLAVE23_1)
    }
}
#[doc = "Field `SLAVE24` reader - SLAVE24 buffering"]
pub type SLAVE24_R = crate::BitReader<SLAVE24_A>;
#[doc = "SLAVE24 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE24_A {
    #[doc = "0: No Buffering"]
    SLAVE24_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE24_1 = 1,
}
impl From<SLAVE24_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE24_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE24_A {
        match self.bits {
            false => SLAVE24_A::SLAVE24_0,
            true => SLAVE24_A::SLAVE24_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE24_0`"]
    #[inline(always)]
    pub fn is_slave24_0(&self) -> bool {
        *self == SLAVE24_A::SLAVE24_0
    }
    #[doc = "Checks if the value of the field is `SLAVE24_1`"]
    #[inline(always)]
    pub fn is_slave24_1(&self) -> bool {
        *self == SLAVE24_A::SLAVE24_1
    }
}
#[doc = "Field `SLAVE24` writer - SLAVE24 buffering"]
pub type SLAVE24_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE24_A, O>;
impl<'a, const O: u8> SLAVE24_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave24_0(self) -> &'a mut W {
        self.variant(SLAVE24_A::SLAVE24_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave24_1(self) -> &'a mut W {
        self.variant(SLAVE24_A::SLAVE24_1)
    }
}
#[doc = "Field `SLAVE25` reader - SLAVE25 buffering"]
pub type SLAVE25_R = crate::BitReader<SLAVE25_A>;
#[doc = "SLAVE25 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE25_A {
    #[doc = "0: No Buffering"]
    SLAVE25_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE25_1 = 1,
}
impl From<SLAVE25_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE25_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE25_A {
        match self.bits {
            false => SLAVE25_A::SLAVE25_0,
            true => SLAVE25_A::SLAVE25_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE25_0`"]
    #[inline(always)]
    pub fn is_slave25_0(&self) -> bool {
        *self == SLAVE25_A::SLAVE25_0
    }
    #[doc = "Checks if the value of the field is `SLAVE25_1`"]
    #[inline(always)]
    pub fn is_slave25_1(&self) -> bool {
        *self == SLAVE25_A::SLAVE25_1
    }
}
#[doc = "Field `SLAVE25` writer - SLAVE25 buffering"]
pub type SLAVE25_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE25_A, O>;
impl<'a, const O: u8> SLAVE25_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave25_0(self) -> &'a mut W {
        self.variant(SLAVE25_A::SLAVE25_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave25_1(self) -> &'a mut W {
        self.variant(SLAVE25_A::SLAVE25_1)
    }
}
#[doc = "Field `SLAVE26` reader - SLAVE26 buffering"]
pub type SLAVE26_R = crate::BitReader<SLAVE26_A>;
#[doc = "SLAVE26 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE26_A {
    #[doc = "0: No Buffering"]
    SLAVE26_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE26_1 = 1,
}
impl From<SLAVE26_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE26_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE26_A {
        match self.bits {
            false => SLAVE26_A::SLAVE26_0,
            true => SLAVE26_A::SLAVE26_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE26_0`"]
    #[inline(always)]
    pub fn is_slave26_0(&self) -> bool {
        *self == SLAVE26_A::SLAVE26_0
    }
    #[doc = "Checks if the value of the field is `SLAVE26_1`"]
    #[inline(always)]
    pub fn is_slave26_1(&self) -> bool {
        *self == SLAVE26_A::SLAVE26_1
    }
}
#[doc = "Field `SLAVE26` writer - SLAVE26 buffering"]
pub type SLAVE26_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE26_A, O>;
impl<'a, const O: u8> SLAVE26_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave26_0(self) -> &'a mut W {
        self.variant(SLAVE26_A::SLAVE26_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave26_1(self) -> &'a mut W {
        self.variant(SLAVE26_A::SLAVE26_1)
    }
}
#[doc = "Field `SLAVE27` reader - SLAVE27 buffering"]
pub type SLAVE27_R = crate::BitReader<SLAVE27_A>;
#[doc = "SLAVE27 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE27_A {
    #[doc = "0: No Buffering"]
    SLAVE27_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE27_1 = 1,
}
impl From<SLAVE27_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE27_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE27_A {
        match self.bits {
            false => SLAVE27_A::SLAVE27_0,
            true => SLAVE27_A::SLAVE27_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE27_0`"]
    #[inline(always)]
    pub fn is_slave27_0(&self) -> bool {
        *self == SLAVE27_A::SLAVE27_0
    }
    #[doc = "Checks if the value of the field is `SLAVE27_1`"]
    #[inline(always)]
    pub fn is_slave27_1(&self) -> bool {
        *self == SLAVE27_A::SLAVE27_1
    }
}
#[doc = "Field `SLAVE27` writer - SLAVE27 buffering"]
pub type SLAVE27_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE27_A, O>;
impl<'a, const O: u8> SLAVE27_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave27_0(self) -> &'a mut W {
        self.variant(SLAVE27_A::SLAVE27_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave27_1(self) -> &'a mut W {
        self.variant(SLAVE27_A::SLAVE27_1)
    }
}
#[doc = "Field `SLAVE28` reader - SLAVE28 buffering"]
pub type SLAVE28_R = crate::BitReader<SLAVE28_A>;
#[doc = "SLAVE28 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE28_A {
    #[doc = "0: No Buffering"]
    SLAVE28_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE28_1 = 1,
}
impl From<SLAVE28_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE28_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE28_A {
        match self.bits {
            false => SLAVE28_A::SLAVE28_0,
            true => SLAVE28_A::SLAVE28_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE28_0`"]
    #[inline(always)]
    pub fn is_slave28_0(&self) -> bool {
        *self == SLAVE28_A::SLAVE28_0
    }
    #[doc = "Checks if the value of the field is `SLAVE28_1`"]
    #[inline(always)]
    pub fn is_slave28_1(&self) -> bool {
        *self == SLAVE28_A::SLAVE28_1
    }
}
#[doc = "Field `SLAVE28` writer - SLAVE28 buffering"]
pub type SLAVE28_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE28_A, O>;
impl<'a, const O: u8> SLAVE28_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave28_0(self) -> &'a mut W {
        self.variant(SLAVE28_A::SLAVE28_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave28_1(self) -> &'a mut W {
        self.variant(SLAVE28_A::SLAVE28_1)
    }
}
#[doc = "Field `SLAVE29` reader - SLAVE29 buffering"]
pub type SLAVE29_R = crate::BitReader<SLAVE29_A>;
#[doc = "SLAVE29 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE29_A {
    #[doc = "0: No Buffering"]
    SLAVE29_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE29_1 = 1,
}
impl From<SLAVE29_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE29_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE29_A {
        match self.bits {
            false => SLAVE29_A::SLAVE29_0,
            true => SLAVE29_A::SLAVE29_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE29_0`"]
    #[inline(always)]
    pub fn is_slave29_0(&self) -> bool {
        *self == SLAVE29_A::SLAVE29_0
    }
    #[doc = "Checks if the value of the field is `SLAVE29_1`"]
    #[inline(always)]
    pub fn is_slave29_1(&self) -> bool {
        *self == SLAVE29_A::SLAVE29_1
    }
}
#[doc = "Field `SLAVE29` writer - SLAVE29 buffering"]
pub type SLAVE29_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE29_A, O>;
impl<'a, const O: u8> SLAVE29_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave29_0(self) -> &'a mut W {
        self.variant(SLAVE29_A::SLAVE29_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave29_1(self) -> &'a mut W {
        self.variant(SLAVE29_A::SLAVE29_1)
    }
}
#[doc = "Field `SLAVE30` reader - SLAVE30 buffering"]
pub type SLAVE30_R = crate::BitReader<SLAVE30_A>;
#[doc = "SLAVE30 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE30_A {
    #[doc = "0: No Buffering"]
    SLAVE30_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE30_1 = 1,
}
impl From<SLAVE30_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE30_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE30_A {
        match self.bits {
            false => SLAVE30_A::SLAVE30_0,
            true => SLAVE30_A::SLAVE30_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE30_0`"]
    #[inline(always)]
    pub fn is_slave30_0(&self) -> bool {
        *self == SLAVE30_A::SLAVE30_0
    }
    #[doc = "Checks if the value of the field is `SLAVE30_1`"]
    #[inline(always)]
    pub fn is_slave30_1(&self) -> bool {
        *self == SLAVE30_A::SLAVE30_1
    }
}
#[doc = "Field `SLAVE30` writer - SLAVE30 buffering"]
pub type SLAVE30_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE30_A, O>;
impl<'a, const O: u8> SLAVE30_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave30_0(self) -> &'a mut W {
        self.variant(SLAVE30_A::SLAVE30_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave30_1(self) -> &'a mut W {
        self.variant(SLAVE30_A::SLAVE30_1)
    }
}
#[doc = "Field `SLAVE31` reader - SLAVE31 buffering"]
pub type SLAVE31_R = crate::BitReader<SLAVE31_A>;
#[doc = "SLAVE31 buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE31_A {
    #[doc = "0: No Buffering"]
    SLAVE31_0 = 0,
    #[doc = "1: Buffering"]
    SLAVE31_1 = 1,
}
impl From<SLAVE31_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE31_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE31_A {
        match self.bits {
            false => SLAVE31_A::SLAVE31_0,
            true => SLAVE31_A::SLAVE31_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE31_0`"]
    #[inline(always)]
    pub fn is_slave31_0(&self) -> bool {
        *self == SLAVE31_A::SLAVE31_0
    }
    #[doc = "Checks if the value of the field is `SLAVE31_1`"]
    #[inline(always)]
    pub fn is_slave31_1(&self) -> bool {
        *self == SLAVE31_A::SLAVE31_1
    }
}
#[doc = "Field `SLAVE31` writer - SLAVE31 buffering"]
pub type SLAVE31_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBBRIDGEBUFFER_SPEC, SLAVE31_A, O>;
impl<'a, const O: u8> SLAVE31_W<'a, O> {
    #[doc = "No Buffering"]
    #[inline(always)]
    pub fn slave31_0(self) -> &'a mut W {
        self.variant(SLAVE31_A::SLAVE31_0)
    }
    #[doc = "Buffering"]
    #[inline(always)]
    pub fn slave31_1(self) -> &'a mut W {
        self.variant(SLAVE31_A::SLAVE31_1)
    }
}
impl R {
    #[doc = "Bit 0 - SLAVE0 buffering"]
    #[inline(always)]
    pub fn slave0(&self) -> SLAVE0_R {
        SLAVE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SLAVE1 buffering"]
    #[inline(always)]
    pub fn slave1(&self) -> SLAVE1_R {
        SLAVE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SLAVE2 buffering"]
    #[inline(always)]
    pub fn slave2(&self) -> SLAVE2_R {
        SLAVE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SLAVE3 buffering"]
    #[inline(always)]
    pub fn slave3(&self) -> SLAVE3_R {
        SLAVE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SLAVE4 buffering"]
    #[inline(always)]
    pub fn slave4(&self) -> SLAVE4_R {
        SLAVE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SLAVE5 buffering"]
    #[inline(always)]
    pub fn slave5(&self) -> SLAVE5_R {
        SLAVE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SLAVE6 buffering"]
    #[inline(always)]
    pub fn slave6(&self) -> SLAVE6_R {
        SLAVE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SLAVE7 buffering"]
    #[inline(always)]
    pub fn slave7(&self) -> SLAVE7_R {
        SLAVE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SLAVE8 buffering"]
    #[inline(always)]
    pub fn slave8(&self) -> SLAVE8_R {
        SLAVE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SLAVE9 buffering"]
    #[inline(always)]
    pub fn slave9(&self) -> SLAVE9_R {
        SLAVE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SLAVE10 buffering"]
    #[inline(always)]
    pub fn slave10(&self) -> SLAVE10_R {
        SLAVE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SLAVE11 buffering"]
    #[inline(always)]
    pub fn slave11(&self) -> SLAVE11_R {
        SLAVE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SLAVE12 buffering"]
    #[inline(always)]
    pub fn slave12(&self) -> SLAVE12_R {
        SLAVE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SLAVE13 buffering"]
    #[inline(always)]
    pub fn slave13(&self) -> SLAVE13_R {
        SLAVE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SLAVE14 buffering"]
    #[inline(always)]
    pub fn slave14(&self) -> SLAVE14_R {
        SLAVE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SLAVE15 buffering"]
    #[inline(always)]
    pub fn slave15(&self) -> SLAVE15_R {
        SLAVE15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SLAVE16 buffering"]
    #[inline(always)]
    pub fn slave16(&self) -> SLAVE16_R {
        SLAVE16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SLAVE17 buffering"]
    #[inline(always)]
    pub fn slave17(&self) -> SLAVE17_R {
        SLAVE17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SLAVE18 buffering"]
    #[inline(always)]
    pub fn slave18(&self) -> SLAVE18_R {
        SLAVE18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SLAVE19 buffering"]
    #[inline(always)]
    pub fn slave19(&self) -> SLAVE19_R {
        SLAVE19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SLAVE20 buffering"]
    #[inline(always)]
    pub fn slave20(&self) -> SLAVE20_R {
        SLAVE20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SLAVE21 buffering"]
    #[inline(always)]
    pub fn slave21(&self) -> SLAVE21_R {
        SLAVE21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SLAVE22 buffering"]
    #[inline(always)]
    pub fn slave22(&self) -> SLAVE22_R {
        SLAVE22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SLAVE23 buffering"]
    #[inline(always)]
    pub fn slave23(&self) -> SLAVE23_R {
        SLAVE23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SLAVE24 buffering"]
    #[inline(always)]
    pub fn slave24(&self) -> SLAVE24_R {
        SLAVE24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SLAVE25 buffering"]
    #[inline(always)]
    pub fn slave25(&self) -> SLAVE25_R {
        SLAVE25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SLAVE26 buffering"]
    #[inline(always)]
    pub fn slave26(&self) -> SLAVE26_R {
        SLAVE26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - SLAVE27 buffering"]
    #[inline(always)]
    pub fn slave27(&self) -> SLAVE27_R {
        SLAVE27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - SLAVE28 buffering"]
    #[inline(always)]
    pub fn slave28(&self) -> SLAVE28_R {
        SLAVE28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SLAVE29 buffering"]
    #[inline(always)]
    pub fn slave29(&self) -> SLAVE29_R {
        SLAVE29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SLAVE30 buffering"]
    #[inline(always)]
    pub fn slave30(&self) -> SLAVE30_R {
        SLAVE30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SLAVE31 buffering"]
    #[inline(always)]
    pub fn slave31(&self) -> SLAVE31_R {
        SLAVE31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SLAVE0 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave0(&mut self) -> SLAVE0_W<0> {
        SLAVE0_W::new(self)
    }
    #[doc = "Bit 1 - SLAVE1 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave1(&mut self) -> SLAVE1_W<1> {
        SLAVE1_W::new(self)
    }
    #[doc = "Bit 2 - SLAVE2 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave2(&mut self) -> SLAVE2_W<2> {
        SLAVE2_W::new(self)
    }
    #[doc = "Bit 3 - SLAVE3 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave3(&mut self) -> SLAVE3_W<3> {
        SLAVE3_W::new(self)
    }
    #[doc = "Bit 4 - SLAVE4 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave4(&mut self) -> SLAVE4_W<4> {
        SLAVE4_W::new(self)
    }
    #[doc = "Bit 5 - SLAVE5 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave5(&mut self) -> SLAVE5_W<5> {
        SLAVE5_W::new(self)
    }
    #[doc = "Bit 6 - SLAVE6 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave6(&mut self) -> SLAVE6_W<6> {
        SLAVE6_W::new(self)
    }
    #[doc = "Bit 7 - SLAVE7 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave7(&mut self) -> SLAVE7_W<7> {
        SLAVE7_W::new(self)
    }
    #[doc = "Bit 8 - SLAVE8 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave8(&mut self) -> SLAVE8_W<8> {
        SLAVE8_W::new(self)
    }
    #[doc = "Bit 9 - SLAVE9 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave9(&mut self) -> SLAVE9_W<9> {
        SLAVE9_W::new(self)
    }
    #[doc = "Bit 10 - SLAVE10 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave10(&mut self) -> SLAVE10_W<10> {
        SLAVE10_W::new(self)
    }
    #[doc = "Bit 11 - SLAVE11 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave11(&mut self) -> SLAVE11_W<11> {
        SLAVE11_W::new(self)
    }
    #[doc = "Bit 12 - SLAVE12 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave12(&mut self) -> SLAVE12_W<12> {
        SLAVE12_W::new(self)
    }
    #[doc = "Bit 13 - SLAVE13 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave13(&mut self) -> SLAVE13_W<13> {
        SLAVE13_W::new(self)
    }
    #[doc = "Bit 14 - SLAVE14 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave14(&mut self) -> SLAVE14_W<14> {
        SLAVE14_W::new(self)
    }
    #[doc = "Bit 15 - SLAVE15 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave15(&mut self) -> SLAVE15_W<15> {
        SLAVE15_W::new(self)
    }
    #[doc = "Bit 16 - SLAVE16 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave16(&mut self) -> SLAVE16_W<16> {
        SLAVE16_W::new(self)
    }
    #[doc = "Bit 17 - SLAVE17 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave17(&mut self) -> SLAVE17_W<17> {
        SLAVE17_W::new(self)
    }
    #[doc = "Bit 18 - SLAVE18 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave18(&mut self) -> SLAVE18_W<18> {
        SLAVE18_W::new(self)
    }
    #[doc = "Bit 19 - SLAVE19 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave19(&mut self) -> SLAVE19_W<19> {
        SLAVE19_W::new(self)
    }
    #[doc = "Bit 20 - SLAVE20 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave20(&mut self) -> SLAVE20_W<20> {
        SLAVE20_W::new(self)
    }
    #[doc = "Bit 21 - SLAVE21 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave21(&mut self) -> SLAVE21_W<21> {
        SLAVE21_W::new(self)
    }
    #[doc = "Bit 22 - SLAVE22 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave22(&mut self) -> SLAVE22_W<22> {
        SLAVE22_W::new(self)
    }
    #[doc = "Bit 23 - SLAVE23 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave23(&mut self) -> SLAVE23_W<23> {
        SLAVE23_W::new(self)
    }
    #[doc = "Bit 24 - SLAVE24 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave24(&mut self) -> SLAVE24_W<24> {
        SLAVE24_W::new(self)
    }
    #[doc = "Bit 25 - SLAVE25 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave25(&mut self) -> SLAVE25_W<25> {
        SLAVE25_W::new(self)
    }
    #[doc = "Bit 26 - SLAVE26 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave26(&mut self) -> SLAVE26_W<26> {
        SLAVE26_W::new(self)
    }
    #[doc = "Bit 27 - SLAVE27 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave27(&mut self) -> SLAVE27_W<27> {
        SLAVE27_W::new(self)
    }
    #[doc = "Bit 28 - SLAVE28 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave28(&mut self) -> SLAVE28_W<28> {
        SLAVE28_W::new(self)
    }
    #[doc = "Bit 29 - SLAVE29 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave29(&mut self) -> SLAVE29_W<29> {
        SLAVE29_W::new(self)
    }
    #[doc = "Bit 30 - SLAVE30 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave30(&mut self) -> SLAVE30_W<30> {
        SLAVE30_W::new(self)
    }
    #[doc = "Bit 31 - SLAVE31 buffering"]
    #[inline(always)]
    #[must_use]
    pub fn slave31(&mut self) -> SLAVE31_W<31> {
        SLAVE31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbbridgebuffer](index.html) module"]
pub struct AHBBRIDGEBUFFER_SPEC;
impl crate::RegisterSpec for AHBBRIDGEBUFFER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbbridgebuffer::R](R) reader structure"]
impl crate::Readable for AHBBRIDGEBUFFER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbbridgebuffer::W](W) writer structure"]
impl crate::Writable for AHBBRIDGEBUFFER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBBRIDGEBUFFER[%s]
to value 0"]
impl crate::Resettable for AHBBRIDGEBUFFER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
