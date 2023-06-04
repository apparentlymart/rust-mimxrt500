#[doc = "Register `PDSLEEPCFG2` reader"]
pub struct R(crate::R<PDSLEEPCFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDSLEEPCFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDSLEEPCFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDSLEEPCFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDSLEEPCFG2` writer"]
pub struct W(crate::W<PDSLEEPCFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDSLEEPCFG2_SPEC>;
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
impl From<crate::W<PDSLEEPCFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDSLEEPCFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM_IF0_APD` reader - Array power for RAM interface 0"]
pub type SRAM_IF0_APD_R = crate::BitReader<SRAM_IF0_APD_A>;
#[doc = "Array power for RAM interface 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF0_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF0_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF0_APD_1 = 1,
}
impl From<SRAM_IF0_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF0_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF0_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF0_APD_A {
        match self.bits {
            false => SRAM_IF0_APD_A::PDSLEEPCFG2_SRAM_IF0_APD_0,
            true => SRAM_IF0_APD_A::PDSLEEPCFG2_SRAM_IF0_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF0_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if0_apd_0(&self) -> bool {
        *self == SRAM_IF0_APD_A::PDSLEEPCFG2_SRAM_IF0_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF0_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if0_apd_1(&self) -> bool {
        *self == SRAM_IF0_APD_A::PDSLEEPCFG2_SRAM_IF0_APD_1
    }
}
#[doc = "Field `SRAM_IF0_APD` writer - Array power for RAM interface 0"]
pub type SRAM_IF0_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF0_APD_A, O>;
impl<'a, const O: u8> SRAM_IF0_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if0_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF0_APD_A::PDSLEEPCFG2_SRAM_IF0_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if0_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF0_APD_A::PDSLEEPCFG2_SRAM_IF0_APD_1)
    }
}
#[doc = "Field `SRAM_IF1_APD` reader - Array power for RAM interface 1"]
pub type SRAM_IF1_APD_R = crate::BitReader<SRAM_IF1_APD_A>;
#[doc = "Array power for RAM interface 1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF1_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF1_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF1_APD_1 = 1,
}
impl From<SRAM_IF1_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF1_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF1_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF1_APD_A {
        match self.bits {
            false => SRAM_IF1_APD_A::PDSLEEPCFG2_SRAM_IF1_APD_0,
            true => SRAM_IF1_APD_A::PDSLEEPCFG2_SRAM_IF1_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF1_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if1_apd_0(&self) -> bool {
        *self == SRAM_IF1_APD_A::PDSLEEPCFG2_SRAM_IF1_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF1_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if1_apd_1(&self) -> bool {
        *self == SRAM_IF1_APD_A::PDSLEEPCFG2_SRAM_IF1_APD_1
    }
}
#[doc = "Field `SRAM_IF1_APD` writer - Array power for RAM interface 1"]
pub type SRAM_IF1_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF1_APD_A, O>;
impl<'a, const O: u8> SRAM_IF1_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if1_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF1_APD_A::PDSLEEPCFG2_SRAM_IF1_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if1_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF1_APD_A::PDSLEEPCFG2_SRAM_IF1_APD_1)
    }
}
#[doc = "Field `SRAM_IF2_APD` reader - Array power for RAM interface 2"]
pub type SRAM_IF2_APD_R = crate::BitReader<SRAM_IF2_APD_A>;
#[doc = "Array power for RAM interface 2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF2_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF2_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF2_APD_1 = 1,
}
impl From<SRAM_IF2_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF2_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF2_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF2_APD_A {
        match self.bits {
            false => SRAM_IF2_APD_A::PDSLEEPCFG2_SRAM_IF2_APD_0,
            true => SRAM_IF2_APD_A::PDSLEEPCFG2_SRAM_IF2_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF2_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if2_apd_0(&self) -> bool {
        *self == SRAM_IF2_APD_A::PDSLEEPCFG2_SRAM_IF2_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF2_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if2_apd_1(&self) -> bool {
        *self == SRAM_IF2_APD_A::PDSLEEPCFG2_SRAM_IF2_APD_1
    }
}
#[doc = "Field `SRAM_IF2_APD` writer - Array power for RAM interface 2"]
pub type SRAM_IF2_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF2_APD_A, O>;
impl<'a, const O: u8> SRAM_IF2_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if2_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF2_APD_A::PDSLEEPCFG2_SRAM_IF2_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if2_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF2_APD_A::PDSLEEPCFG2_SRAM_IF2_APD_1)
    }
}
#[doc = "Field `SRAM_IF3_APD` reader - Array power for RAM interface 3"]
pub type SRAM_IF3_APD_R = crate::BitReader<SRAM_IF3_APD_A>;
#[doc = "Array power for RAM interface 3\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF3_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF3_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF3_APD_1 = 1,
}
impl From<SRAM_IF3_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF3_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF3_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF3_APD_A {
        match self.bits {
            false => SRAM_IF3_APD_A::PDSLEEPCFG2_SRAM_IF3_APD_0,
            true => SRAM_IF3_APD_A::PDSLEEPCFG2_SRAM_IF3_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF3_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if3_apd_0(&self) -> bool {
        *self == SRAM_IF3_APD_A::PDSLEEPCFG2_SRAM_IF3_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF3_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if3_apd_1(&self) -> bool {
        *self == SRAM_IF3_APD_A::PDSLEEPCFG2_SRAM_IF3_APD_1
    }
}
#[doc = "Field `SRAM_IF3_APD` writer - Array power for RAM interface 3"]
pub type SRAM_IF3_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF3_APD_A, O>;
impl<'a, const O: u8> SRAM_IF3_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if3_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF3_APD_A::PDSLEEPCFG2_SRAM_IF3_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if3_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF3_APD_A::PDSLEEPCFG2_SRAM_IF3_APD_1)
    }
}
#[doc = "Field `SRAM_IF4_APD` reader - Array power for RAM interface 4"]
pub type SRAM_IF4_APD_R = crate::BitReader<SRAM_IF4_APD_A>;
#[doc = "Array power for RAM interface 4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF4_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF4_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF4_APD_1 = 1,
}
impl From<SRAM_IF4_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF4_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF4_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF4_APD_A {
        match self.bits {
            false => SRAM_IF4_APD_A::PDSLEEPCFG2_SRAM_IF4_APD_0,
            true => SRAM_IF4_APD_A::PDSLEEPCFG2_SRAM_IF4_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF4_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if4_apd_0(&self) -> bool {
        *self == SRAM_IF4_APD_A::PDSLEEPCFG2_SRAM_IF4_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF4_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if4_apd_1(&self) -> bool {
        *self == SRAM_IF4_APD_A::PDSLEEPCFG2_SRAM_IF4_APD_1
    }
}
#[doc = "Field `SRAM_IF4_APD` writer - Array power for RAM interface 4"]
pub type SRAM_IF4_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF4_APD_A, O>;
impl<'a, const O: u8> SRAM_IF4_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if4_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF4_APD_A::PDSLEEPCFG2_SRAM_IF4_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if4_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF4_APD_A::PDSLEEPCFG2_SRAM_IF4_APD_1)
    }
}
#[doc = "Field `SRAM_IF5_APD` reader - Array power for RAM interface 5"]
pub type SRAM_IF5_APD_R = crate::BitReader<SRAM_IF5_APD_A>;
#[doc = "Array power for RAM interface 5\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF5_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF5_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF5_APD_1 = 1,
}
impl From<SRAM_IF5_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF5_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF5_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF5_APD_A {
        match self.bits {
            false => SRAM_IF5_APD_A::PDSLEEPCFG2_SRAM_IF5_APD_0,
            true => SRAM_IF5_APD_A::PDSLEEPCFG2_SRAM_IF5_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF5_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if5_apd_0(&self) -> bool {
        *self == SRAM_IF5_APD_A::PDSLEEPCFG2_SRAM_IF5_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF5_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if5_apd_1(&self) -> bool {
        *self == SRAM_IF5_APD_A::PDSLEEPCFG2_SRAM_IF5_APD_1
    }
}
#[doc = "Field `SRAM_IF5_APD` writer - Array power for RAM interface 5"]
pub type SRAM_IF5_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF5_APD_A, O>;
impl<'a, const O: u8> SRAM_IF5_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if5_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF5_APD_A::PDSLEEPCFG2_SRAM_IF5_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if5_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF5_APD_A::PDSLEEPCFG2_SRAM_IF5_APD_1)
    }
}
#[doc = "Field `SRAM_IF6_APD` reader - Array power for RAM interface 6"]
pub type SRAM_IF6_APD_R = crate::BitReader<SRAM_IF6_APD_A>;
#[doc = "Array power for RAM interface 6\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF6_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF6_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF6_APD_1 = 1,
}
impl From<SRAM_IF6_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF6_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF6_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF6_APD_A {
        match self.bits {
            false => SRAM_IF6_APD_A::PDSLEEPCFG2_SRAM_IF6_APD_0,
            true => SRAM_IF6_APD_A::PDSLEEPCFG2_SRAM_IF6_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF6_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if6_apd_0(&self) -> bool {
        *self == SRAM_IF6_APD_A::PDSLEEPCFG2_SRAM_IF6_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF6_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if6_apd_1(&self) -> bool {
        *self == SRAM_IF6_APD_A::PDSLEEPCFG2_SRAM_IF6_APD_1
    }
}
#[doc = "Field `SRAM_IF6_APD` writer - Array power for RAM interface 6"]
pub type SRAM_IF6_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF6_APD_A, O>;
impl<'a, const O: u8> SRAM_IF6_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if6_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF6_APD_A::PDSLEEPCFG2_SRAM_IF6_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if6_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF6_APD_A::PDSLEEPCFG2_SRAM_IF6_APD_1)
    }
}
#[doc = "Field `SRAM_IF7_APD` reader - Array power for RAM interface 7"]
pub type SRAM_IF7_APD_R = crate::BitReader<SRAM_IF7_APD_A>;
#[doc = "Array power for RAM interface 7\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF7_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF7_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF7_APD_1 = 1,
}
impl From<SRAM_IF7_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF7_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF7_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF7_APD_A {
        match self.bits {
            false => SRAM_IF7_APD_A::PDSLEEPCFG2_SRAM_IF7_APD_0,
            true => SRAM_IF7_APD_A::PDSLEEPCFG2_SRAM_IF7_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF7_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if7_apd_0(&self) -> bool {
        *self == SRAM_IF7_APD_A::PDSLEEPCFG2_SRAM_IF7_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF7_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if7_apd_1(&self) -> bool {
        *self == SRAM_IF7_APD_A::PDSLEEPCFG2_SRAM_IF7_APD_1
    }
}
#[doc = "Field `SRAM_IF7_APD` writer - Array power for RAM interface 7"]
pub type SRAM_IF7_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF7_APD_A, O>;
impl<'a, const O: u8> SRAM_IF7_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if7_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF7_APD_A::PDSLEEPCFG2_SRAM_IF7_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if7_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF7_APD_A::PDSLEEPCFG2_SRAM_IF7_APD_1)
    }
}
#[doc = "Field `SRAM_IF8_APD` reader - Array power for RAM interface 8"]
pub type SRAM_IF8_APD_R = crate::BitReader<SRAM_IF8_APD_A>;
#[doc = "Array power for RAM interface 8\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF8_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF8_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF8_APD_1 = 1,
}
impl From<SRAM_IF8_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF8_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF8_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF8_APD_A {
        match self.bits {
            false => SRAM_IF8_APD_A::PDSLEEPCFG2_SRAM_IF8_APD_0,
            true => SRAM_IF8_APD_A::PDSLEEPCFG2_SRAM_IF8_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF8_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if8_apd_0(&self) -> bool {
        *self == SRAM_IF8_APD_A::PDSLEEPCFG2_SRAM_IF8_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF8_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if8_apd_1(&self) -> bool {
        *self == SRAM_IF8_APD_A::PDSLEEPCFG2_SRAM_IF8_APD_1
    }
}
#[doc = "Field `SRAM_IF8_APD` writer - Array power for RAM interface 8"]
pub type SRAM_IF8_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF8_APD_A, O>;
impl<'a, const O: u8> SRAM_IF8_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if8_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF8_APD_A::PDSLEEPCFG2_SRAM_IF8_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if8_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF8_APD_A::PDSLEEPCFG2_SRAM_IF8_APD_1)
    }
}
#[doc = "Field `SRAM_IF9_APD` reader - Array power for RAM interface 9"]
pub type SRAM_IF9_APD_R = crate::BitReader<SRAM_IF9_APD_A>;
#[doc = "Array power for RAM interface 9\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF9_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF9_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF9_APD_1 = 1,
}
impl From<SRAM_IF9_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF9_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF9_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF9_APD_A {
        match self.bits {
            false => SRAM_IF9_APD_A::PDSLEEPCFG2_SRAM_IF9_APD_0,
            true => SRAM_IF9_APD_A::PDSLEEPCFG2_SRAM_IF9_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF9_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if9_apd_0(&self) -> bool {
        *self == SRAM_IF9_APD_A::PDSLEEPCFG2_SRAM_IF9_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF9_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if9_apd_1(&self) -> bool {
        *self == SRAM_IF9_APD_A::PDSLEEPCFG2_SRAM_IF9_APD_1
    }
}
#[doc = "Field `SRAM_IF9_APD` writer - Array power for RAM interface 9"]
pub type SRAM_IF9_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF9_APD_A, O>;
impl<'a, const O: u8> SRAM_IF9_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if9_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF9_APD_A::PDSLEEPCFG2_SRAM_IF9_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if9_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF9_APD_A::PDSLEEPCFG2_SRAM_IF9_APD_1)
    }
}
#[doc = "Field `SRAM_IF10_APD` reader - Array power for RAM interface 10"]
pub type SRAM_IF10_APD_R = crate::BitReader<SRAM_IF10_APD_A>;
#[doc = "Array power for RAM interface 10\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF10_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF10_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF10_APD_1 = 1,
}
impl From<SRAM_IF10_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF10_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF10_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF10_APD_A {
        match self.bits {
            false => SRAM_IF10_APD_A::PDSLEEPCFG2_SRAM_IF10_APD_0,
            true => SRAM_IF10_APD_A::PDSLEEPCFG2_SRAM_IF10_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF10_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if10_apd_0(&self) -> bool {
        *self == SRAM_IF10_APD_A::PDSLEEPCFG2_SRAM_IF10_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF10_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if10_apd_1(&self) -> bool {
        *self == SRAM_IF10_APD_A::PDSLEEPCFG2_SRAM_IF10_APD_1
    }
}
#[doc = "Field `SRAM_IF10_APD` writer - Array power for RAM interface 10"]
pub type SRAM_IF10_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF10_APD_A, O>;
impl<'a, const O: u8> SRAM_IF10_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if10_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF10_APD_A::PDSLEEPCFG2_SRAM_IF10_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if10_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF10_APD_A::PDSLEEPCFG2_SRAM_IF10_APD_1)
    }
}
#[doc = "Field `SRAM_IF11_APD` reader - Array power for RAM interface 11"]
pub type SRAM_IF11_APD_R = crate::BitReader<SRAM_IF11_APD_A>;
#[doc = "Array power for RAM interface 11\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF11_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF11_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF11_APD_1 = 1,
}
impl From<SRAM_IF11_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF11_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF11_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF11_APD_A {
        match self.bits {
            false => SRAM_IF11_APD_A::PDSLEEPCFG2_SRAM_IF11_APD_0,
            true => SRAM_IF11_APD_A::PDSLEEPCFG2_SRAM_IF11_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF11_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if11_apd_0(&self) -> bool {
        *self == SRAM_IF11_APD_A::PDSLEEPCFG2_SRAM_IF11_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF11_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if11_apd_1(&self) -> bool {
        *self == SRAM_IF11_APD_A::PDSLEEPCFG2_SRAM_IF11_APD_1
    }
}
#[doc = "Field `SRAM_IF11_APD` writer - Array power for RAM interface 11"]
pub type SRAM_IF11_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF11_APD_A, O>;
impl<'a, const O: u8> SRAM_IF11_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if11_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF11_APD_A::PDSLEEPCFG2_SRAM_IF11_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if11_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF11_APD_A::PDSLEEPCFG2_SRAM_IF11_APD_1)
    }
}
#[doc = "Field `SRAM_IF12_APD` reader - Array power for RAM interface 12"]
pub type SRAM_IF12_APD_R = crate::BitReader<SRAM_IF12_APD_A>;
#[doc = "Array power for RAM interface 12\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF12_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF12_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF12_APD_1 = 1,
}
impl From<SRAM_IF12_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF12_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF12_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF12_APD_A {
        match self.bits {
            false => SRAM_IF12_APD_A::PDSLEEPCFG2_SRAM_IF12_APD_0,
            true => SRAM_IF12_APD_A::PDSLEEPCFG2_SRAM_IF12_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF12_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if12_apd_0(&self) -> bool {
        *self == SRAM_IF12_APD_A::PDSLEEPCFG2_SRAM_IF12_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF12_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if12_apd_1(&self) -> bool {
        *self == SRAM_IF12_APD_A::PDSLEEPCFG2_SRAM_IF12_APD_1
    }
}
#[doc = "Field `SRAM_IF12_APD` writer - Array power for RAM interface 12"]
pub type SRAM_IF12_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF12_APD_A, O>;
impl<'a, const O: u8> SRAM_IF12_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if12_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF12_APD_A::PDSLEEPCFG2_SRAM_IF12_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if12_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF12_APD_A::PDSLEEPCFG2_SRAM_IF12_APD_1)
    }
}
#[doc = "Field `SRAM_IF13_APD` reader - Array power for RAM interface 13"]
pub type SRAM_IF13_APD_R = crate::BitReader<SRAM_IF13_APD_A>;
#[doc = "Array power for RAM interface 13\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF13_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF13_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF13_APD_1 = 1,
}
impl From<SRAM_IF13_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF13_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF13_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF13_APD_A {
        match self.bits {
            false => SRAM_IF13_APD_A::PDSLEEPCFG2_SRAM_IF13_APD_0,
            true => SRAM_IF13_APD_A::PDSLEEPCFG2_SRAM_IF13_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF13_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if13_apd_0(&self) -> bool {
        *self == SRAM_IF13_APD_A::PDSLEEPCFG2_SRAM_IF13_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF13_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if13_apd_1(&self) -> bool {
        *self == SRAM_IF13_APD_A::PDSLEEPCFG2_SRAM_IF13_APD_1
    }
}
#[doc = "Field `SRAM_IF13_APD` writer - Array power for RAM interface 13"]
pub type SRAM_IF13_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF13_APD_A, O>;
impl<'a, const O: u8> SRAM_IF13_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if13_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF13_APD_A::PDSLEEPCFG2_SRAM_IF13_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if13_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF13_APD_A::PDSLEEPCFG2_SRAM_IF13_APD_1)
    }
}
#[doc = "Field `SRAM_IF14_APD` reader - Array power for RAM interface 14"]
pub type SRAM_IF14_APD_R = crate::BitReader<SRAM_IF14_APD_A>;
#[doc = "Array power for RAM interface 14\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF14_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF14_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF14_APD_1 = 1,
}
impl From<SRAM_IF14_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF14_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF14_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF14_APD_A {
        match self.bits {
            false => SRAM_IF14_APD_A::PDSLEEPCFG2_SRAM_IF14_APD_0,
            true => SRAM_IF14_APD_A::PDSLEEPCFG2_SRAM_IF14_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF14_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if14_apd_0(&self) -> bool {
        *self == SRAM_IF14_APD_A::PDSLEEPCFG2_SRAM_IF14_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF14_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if14_apd_1(&self) -> bool {
        *self == SRAM_IF14_APD_A::PDSLEEPCFG2_SRAM_IF14_APD_1
    }
}
#[doc = "Field `SRAM_IF14_APD` writer - Array power for RAM interface 14"]
pub type SRAM_IF14_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF14_APD_A, O>;
impl<'a, const O: u8> SRAM_IF14_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if14_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF14_APD_A::PDSLEEPCFG2_SRAM_IF14_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if14_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF14_APD_A::PDSLEEPCFG2_SRAM_IF14_APD_1)
    }
}
#[doc = "Field `SRAM_IF15_APD` reader - Array power for RAM interface 15"]
pub type SRAM_IF15_APD_R = crate::BitReader<SRAM_IF15_APD_A>;
#[doc = "Array power for RAM interface 15\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF15_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF15_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF15_APD_1 = 1,
}
impl From<SRAM_IF15_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF15_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF15_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF15_APD_A {
        match self.bits {
            false => SRAM_IF15_APD_A::PDSLEEPCFG2_SRAM_IF15_APD_0,
            true => SRAM_IF15_APD_A::PDSLEEPCFG2_SRAM_IF15_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF15_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if15_apd_0(&self) -> bool {
        *self == SRAM_IF15_APD_A::PDSLEEPCFG2_SRAM_IF15_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF15_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if15_apd_1(&self) -> bool {
        *self == SRAM_IF15_APD_A::PDSLEEPCFG2_SRAM_IF15_APD_1
    }
}
#[doc = "Field `SRAM_IF15_APD` writer - Array power for RAM interface 15"]
pub type SRAM_IF15_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF15_APD_A, O>;
impl<'a, const O: u8> SRAM_IF15_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if15_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF15_APD_A::PDSLEEPCFG2_SRAM_IF15_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if15_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF15_APD_A::PDSLEEPCFG2_SRAM_IF15_APD_1)
    }
}
#[doc = "Field `SRAM_IF16_APD` reader - Array power for RAM interface 16"]
pub type SRAM_IF16_APD_R = crate::BitReader<SRAM_IF16_APD_A>;
#[doc = "Array power for RAM interface 16\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF16_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF16_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF16_APD_1 = 1,
}
impl From<SRAM_IF16_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF16_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF16_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF16_APD_A {
        match self.bits {
            false => SRAM_IF16_APD_A::PDSLEEPCFG2_SRAM_IF16_APD_0,
            true => SRAM_IF16_APD_A::PDSLEEPCFG2_SRAM_IF16_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF16_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if16_apd_0(&self) -> bool {
        *self == SRAM_IF16_APD_A::PDSLEEPCFG2_SRAM_IF16_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF16_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if16_apd_1(&self) -> bool {
        *self == SRAM_IF16_APD_A::PDSLEEPCFG2_SRAM_IF16_APD_1
    }
}
#[doc = "Field `SRAM_IF16_APD` writer - Array power for RAM interface 16"]
pub type SRAM_IF16_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF16_APD_A, O>;
impl<'a, const O: u8> SRAM_IF16_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if16_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF16_APD_A::PDSLEEPCFG2_SRAM_IF16_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if16_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF16_APD_A::PDSLEEPCFG2_SRAM_IF16_APD_1)
    }
}
#[doc = "Field `SRAM_IF17_APD` reader - Array power for RAM interface 17"]
pub type SRAM_IF17_APD_R = crate::BitReader<SRAM_IF17_APD_A>;
#[doc = "Array power for RAM interface 17\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF17_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF17_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF17_APD_1 = 1,
}
impl From<SRAM_IF17_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF17_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF17_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF17_APD_A {
        match self.bits {
            false => SRAM_IF17_APD_A::PDSLEEPCFG2_SRAM_IF17_APD_0,
            true => SRAM_IF17_APD_A::PDSLEEPCFG2_SRAM_IF17_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF17_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if17_apd_0(&self) -> bool {
        *self == SRAM_IF17_APD_A::PDSLEEPCFG2_SRAM_IF17_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF17_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if17_apd_1(&self) -> bool {
        *self == SRAM_IF17_APD_A::PDSLEEPCFG2_SRAM_IF17_APD_1
    }
}
#[doc = "Field `SRAM_IF17_APD` writer - Array power for RAM interface 17"]
pub type SRAM_IF17_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF17_APD_A, O>;
impl<'a, const O: u8> SRAM_IF17_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if17_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF17_APD_A::PDSLEEPCFG2_SRAM_IF17_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if17_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF17_APD_A::PDSLEEPCFG2_SRAM_IF17_APD_1)
    }
}
#[doc = "Field `SRAM_IF18_APD` reader - Array power for RAM interface 18"]
pub type SRAM_IF18_APD_R = crate::BitReader<SRAM_IF18_APD_A>;
#[doc = "Array power for RAM interface 18\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF18_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF18_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF18_APD_1 = 1,
}
impl From<SRAM_IF18_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF18_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF18_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF18_APD_A {
        match self.bits {
            false => SRAM_IF18_APD_A::PDSLEEPCFG2_SRAM_IF18_APD_0,
            true => SRAM_IF18_APD_A::PDSLEEPCFG2_SRAM_IF18_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF18_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if18_apd_0(&self) -> bool {
        *self == SRAM_IF18_APD_A::PDSLEEPCFG2_SRAM_IF18_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF18_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if18_apd_1(&self) -> bool {
        *self == SRAM_IF18_APD_A::PDSLEEPCFG2_SRAM_IF18_APD_1
    }
}
#[doc = "Field `SRAM_IF18_APD` writer - Array power for RAM interface 18"]
pub type SRAM_IF18_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF18_APD_A, O>;
impl<'a, const O: u8> SRAM_IF18_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if18_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF18_APD_A::PDSLEEPCFG2_SRAM_IF18_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if18_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF18_APD_A::PDSLEEPCFG2_SRAM_IF18_APD_1)
    }
}
#[doc = "Field `SRAM_IF19_APD` reader - Array power for RAM interface 19"]
pub type SRAM_IF19_APD_R = crate::BitReader<SRAM_IF19_APD_A>;
#[doc = "Array power for RAM interface 19\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF19_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF19_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF19_APD_1 = 1,
}
impl From<SRAM_IF19_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF19_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF19_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF19_APD_A {
        match self.bits {
            false => SRAM_IF19_APD_A::PDSLEEPCFG2_SRAM_IF19_APD_0,
            true => SRAM_IF19_APD_A::PDSLEEPCFG2_SRAM_IF19_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF19_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if19_apd_0(&self) -> bool {
        *self == SRAM_IF19_APD_A::PDSLEEPCFG2_SRAM_IF19_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF19_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if19_apd_1(&self) -> bool {
        *self == SRAM_IF19_APD_A::PDSLEEPCFG2_SRAM_IF19_APD_1
    }
}
#[doc = "Field `SRAM_IF19_APD` writer - Array power for RAM interface 19"]
pub type SRAM_IF19_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF19_APD_A, O>;
impl<'a, const O: u8> SRAM_IF19_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if19_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF19_APD_A::PDSLEEPCFG2_SRAM_IF19_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if19_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF19_APD_A::PDSLEEPCFG2_SRAM_IF19_APD_1)
    }
}
#[doc = "Field `SRAM_IF20_APD` reader - Array power for RAM interface 20"]
pub type SRAM_IF20_APD_R = crate::BitReader<SRAM_IF20_APD_A>;
#[doc = "Array power for RAM interface 20\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF20_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF20_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF20_APD_1 = 1,
}
impl From<SRAM_IF20_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF20_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF20_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF20_APD_A {
        match self.bits {
            false => SRAM_IF20_APD_A::PDSLEEPCFG2_SRAM_IF20_APD_0,
            true => SRAM_IF20_APD_A::PDSLEEPCFG2_SRAM_IF20_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF20_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if20_apd_0(&self) -> bool {
        *self == SRAM_IF20_APD_A::PDSLEEPCFG2_SRAM_IF20_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF20_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if20_apd_1(&self) -> bool {
        *self == SRAM_IF20_APD_A::PDSLEEPCFG2_SRAM_IF20_APD_1
    }
}
#[doc = "Field `SRAM_IF20_APD` writer - Array power for RAM interface 20"]
pub type SRAM_IF20_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF20_APD_A, O>;
impl<'a, const O: u8> SRAM_IF20_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if20_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF20_APD_A::PDSLEEPCFG2_SRAM_IF20_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if20_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF20_APD_A::PDSLEEPCFG2_SRAM_IF20_APD_1)
    }
}
#[doc = "Field `SRAM_IF21_APD` reader - Array power for RAM interface 21"]
pub type SRAM_IF21_APD_R = crate::BitReader<SRAM_IF21_APD_A>;
#[doc = "Array power for RAM interface 21\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF21_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF21_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF21_APD_1 = 1,
}
impl From<SRAM_IF21_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF21_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF21_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF21_APD_A {
        match self.bits {
            false => SRAM_IF21_APD_A::PDSLEEPCFG2_SRAM_IF21_APD_0,
            true => SRAM_IF21_APD_A::PDSLEEPCFG2_SRAM_IF21_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF21_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if21_apd_0(&self) -> bool {
        *self == SRAM_IF21_APD_A::PDSLEEPCFG2_SRAM_IF21_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF21_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if21_apd_1(&self) -> bool {
        *self == SRAM_IF21_APD_A::PDSLEEPCFG2_SRAM_IF21_APD_1
    }
}
#[doc = "Field `SRAM_IF21_APD` writer - Array power for RAM interface 21"]
pub type SRAM_IF21_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF21_APD_A, O>;
impl<'a, const O: u8> SRAM_IF21_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if21_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF21_APD_A::PDSLEEPCFG2_SRAM_IF21_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if21_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF21_APD_A::PDSLEEPCFG2_SRAM_IF21_APD_1)
    }
}
#[doc = "Field `SRAM_IF22_APD` reader - Array power for RAM interface 22"]
pub type SRAM_IF22_APD_R = crate::BitReader<SRAM_IF22_APD_A>;
#[doc = "Array power for RAM interface 22\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF22_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF22_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF22_APD_1 = 1,
}
impl From<SRAM_IF22_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF22_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF22_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF22_APD_A {
        match self.bits {
            false => SRAM_IF22_APD_A::PDSLEEPCFG2_SRAM_IF22_APD_0,
            true => SRAM_IF22_APD_A::PDSLEEPCFG2_SRAM_IF22_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF22_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if22_apd_0(&self) -> bool {
        *self == SRAM_IF22_APD_A::PDSLEEPCFG2_SRAM_IF22_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF22_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if22_apd_1(&self) -> bool {
        *self == SRAM_IF22_APD_A::PDSLEEPCFG2_SRAM_IF22_APD_1
    }
}
#[doc = "Field `SRAM_IF22_APD` writer - Array power for RAM interface 22"]
pub type SRAM_IF22_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF22_APD_A, O>;
impl<'a, const O: u8> SRAM_IF22_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if22_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF22_APD_A::PDSLEEPCFG2_SRAM_IF22_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if22_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF22_APD_A::PDSLEEPCFG2_SRAM_IF22_APD_1)
    }
}
#[doc = "Field `SRAM_IF23_APD` reader - Array power for RAM interface 23"]
pub type SRAM_IF23_APD_R = crate::BitReader<SRAM_IF23_APD_A>;
#[doc = "Array power for RAM interface 23\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF23_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF23_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF23_APD_1 = 1,
}
impl From<SRAM_IF23_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF23_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF23_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF23_APD_A {
        match self.bits {
            false => SRAM_IF23_APD_A::PDSLEEPCFG2_SRAM_IF23_APD_0,
            true => SRAM_IF23_APD_A::PDSLEEPCFG2_SRAM_IF23_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF23_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if23_apd_0(&self) -> bool {
        *self == SRAM_IF23_APD_A::PDSLEEPCFG2_SRAM_IF23_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF23_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if23_apd_1(&self) -> bool {
        *self == SRAM_IF23_APD_A::PDSLEEPCFG2_SRAM_IF23_APD_1
    }
}
#[doc = "Field `SRAM_IF23_APD` writer - Array power for RAM interface 23"]
pub type SRAM_IF23_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF23_APD_A, O>;
impl<'a, const O: u8> SRAM_IF23_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if23_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF23_APD_A::PDSLEEPCFG2_SRAM_IF23_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if23_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF23_APD_A::PDSLEEPCFG2_SRAM_IF23_APD_1)
    }
}
#[doc = "Field `SRAM_IF24_APD` reader - Array power for RAM interface 24"]
pub type SRAM_IF24_APD_R = crate::BitReader<SRAM_IF24_APD_A>;
#[doc = "Array power for RAM interface 24\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF24_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF24_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF24_APD_1 = 1,
}
impl From<SRAM_IF24_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF24_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF24_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF24_APD_A {
        match self.bits {
            false => SRAM_IF24_APD_A::PDSLEEPCFG2_SRAM_IF24_APD_0,
            true => SRAM_IF24_APD_A::PDSLEEPCFG2_SRAM_IF24_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF24_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if24_apd_0(&self) -> bool {
        *self == SRAM_IF24_APD_A::PDSLEEPCFG2_SRAM_IF24_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF24_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if24_apd_1(&self) -> bool {
        *self == SRAM_IF24_APD_A::PDSLEEPCFG2_SRAM_IF24_APD_1
    }
}
#[doc = "Field `SRAM_IF24_APD` writer - Array power for RAM interface 24"]
pub type SRAM_IF24_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF24_APD_A, O>;
impl<'a, const O: u8> SRAM_IF24_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if24_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF24_APD_A::PDSLEEPCFG2_SRAM_IF24_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if24_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF24_APD_A::PDSLEEPCFG2_SRAM_IF24_APD_1)
    }
}
#[doc = "Field `SRAM_IF25_APD` reader - Array power for RAM interface 25"]
pub type SRAM_IF25_APD_R = crate::BitReader<SRAM_IF25_APD_A>;
#[doc = "Array power for RAM interface 25\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF25_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF25_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF25_APD_1 = 1,
}
impl From<SRAM_IF25_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF25_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF25_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF25_APD_A {
        match self.bits {
            false => SRAM_IF25_APD_A::PDSLEEPCFG2_SRAM_IF25_APD_0,
            true => SRAM_IF25_APD_A::PDSLEEPCFG2_SRAM_IF25_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF25_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if25_apd_0(&self) -> bool {
        *self == SRAM_IF25_APD_A::PDSLEEPCFG2_SRAM_IF25_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF25_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if25_apd_1(&self) -> bool {
        *self == SRAM_IF25_APD_A::PDSLEEPCFG2_SRAM_IF25_APD_1
    }
}
#[doc = "Field `SRAM_IF25_APD` writer - Array power for RAM interface 25"]
pub type SRAM_IF25_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF25_APD_A, O>;
impl<'a, const O: u8> SRAM_IF25_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if25_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF25_APD_A::PDSLEEPCFG2_SRAM_IF25_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if25_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF25_APD_A::PDSLEEPCFG2_SRAM_IF25_APD_1)
    }
}
#[doc = "Field `SRAM_IF26_APD` reader - Array power for RAM interface 26"]
pub type SRAM_IF26_APD_R = crate::BitReader<SRAM_IF26_APD_A>;
#[doc = "Array power for RAM interface 26\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF26_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF26_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF26_APD_1 = 1,
}
impl From<SRAM_IF26_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF26_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF26_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF26_APD_A {
        match self.bits {
            false => SRAM_IF26_APD_A::PDSLEEPCFG2_SRAM_IF26_APD_0,
            true => SRAM_IF26_APD_A::PDSLEEPCFG2_SRAM_IF26_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF26_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if26_apd_0(&self) -> bool {
        *self == SRAM_IF26_APD_A::PDSLEEPCFG2_SRAM_IF26_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF26_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if26_apd_1(&self) -> bool {
        *self == SRAM_IF26_APD_A::PDSLEEPCFG2_SRAM_IF26_APD_1
    }
}
#[doc = "Field `SRAM_IF26_APD` writer - Array power for RAM interface 26"]
pub type SRAM_IF26_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF26_APD_A, O>;
impl<'a, const O: u8> SRAM_IF26_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if26_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF26_APD_A::PDSLEEPCFG2_SRAM_IF26_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if26_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF26_APD_A::PDSLEEPCFG2_SRAM_IF26_APD_1)
    }
}
#[doc = "Field `SRAM_IF27_APD` reader - Array power for RAM interface 27"]
pub type SRAM_IF27_APD_R = crate::BitReader<SRAM_IF27_APD_A>;
#[doc = "Array power for RAM interface 27\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF27_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF27_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF27_APD_1 = 1,
}
impl From<SRAM_IF27_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF27_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF27_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF27_APD_A {
        match self.bits {
            false => SRAM_IF27_APD_A::PDSLEEPCFG2_SRAM_IF27_APD_0,
            true => SRAM_IF27_APD_A::PDSLEEPCFG2_SRAM_IF27_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF27_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if27_apd_0(&self) -> bool {
        *self == SRAM_IF27_APD_A::PDSLEEPCFG2_SRAM_IF27_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF27_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if27_apd_1(&self) -> bool {
        *self == SRAM_IF27_APD_A::PDSLEEPCFG2_SRAM_IF27_APD_1
    }
}
#[doc = "Field `SRAM_IF27_APD` writer - Array power for RAM interface 27"]
pub type SRAM_IF27_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF27_APD_A, O>;
impl<'a, const O: u8> SRAM_IF27_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if27_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF27_APD_A::PDSLEEPCFG2_SRAM_IF27_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if27_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF27_APD_A::PDSLEEPCFG2_SRAM_IF27_APD_1)
    }
}
#[doc = "Field `SRAM_IF28_APD` reader - Array power for RAM interface 28"]
pub type SRAM_IF28_APD_R = crate::BitReader<SRAM_IF28_APD_A>;
#[doc = "Array power for RAM interface 28\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF28_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF28_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF28_APD_1 = 1,
}
impl From<SRAM_IF28_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF28_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF28_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF28_APD_A {
        match self.bits {
            false => SRAM_IF28_APD_A::PDSLEEPCFG2_SRAM_IF28_APD_0,
            true => SRAM_IF28_APD_A::PDSLEEPCFG2_SRAM_IF28_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF28_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if28_apd_0(&self) -> bool {
        *self == SRAM_IF28_APD_A::PDSLEEPCFG2_SRAM_IF28_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF28_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if28_apd_1(&self) -> bool {
        *self == SRAM_IF28_APD_A::PDSLEEPCFG2_SRAM_IF28_APD_1
    }
}
#[doc = "Field `SRAM_IF28_APD` writer - Array power for RAM interface 28"]
pub type SRAM_IF28_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF28_APD_A, O>;
impl<'a, const O: u8> SRAM_IF28_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if28_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF28_APD_A::PDSLEEPCFG2_SRAM_IF28_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if28_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF28_APD_A::PDSLEEPCFG2_SRAM_IF28_APD_1)
    }
}
#[doc = "Field `SRAM_IF29_APD` reader - Array power for RAM interface 29"]
pub type SRAM_IF29_APD_R = crate::BitReader<SRAM_IF29_APD_A>;
#[doc = "Array power for RAM interface 29\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF29_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF29_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF29_APD_1 = 1,
}
impl From<SRAM_IF29_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF29_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF29_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF29_APD_A {
        match self.bits {
            false => SRAM_IF29_APD_A::PDSLEEPCFG2_SRAM_IF29_APD_0,
            true => SRAM_IF29_APD_A::PDSLEEPCFG2_SRAM_IF29_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF29_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if29_apd_0(&self) -> bool {
        *self == SRAM_IF29_APD_A::PDSLEEPCFG2_SRAM_IF29_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF29_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if29_apd_1(&self) -> bool {
        *self == SRAM_IF29_APD_A::PDSLEEPCFG2_SRAM_IF29_APD_1
    }
}
#[doc = "Field `SRAM_IF29_APD` writer - Array power for RAM interface 29"]
pub type SRAM_IF29_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF29_APD_A, O>;
impl<'a, const O: u8> SRAM_IF29_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if29_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF29_APD_A::PDSLEEPCFG2_SRAM_IF29_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if29_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF29_APD_A::PDSLEEPCFG2_SRAM_IF29_APD_1)
    }
}
#[doc = "Field `SRAM_IF30_APD` reader - Array power for RAM interface 30"]
pub type SRAM_IF30_APD_R = crate::BitReader<SRAM_IF30_APD_A>;
#[doc = "Array power for RAM interface 30\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF30_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF30_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF30_APD_1 = 1,
}
impl From<SRAM_IF30_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF30_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF30_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF30_APD_A {
        match self.bits {
            false => SRAM_IF30_APD_A::PDSLEEPCFG2_SRAM_IF30_APD_0,
            true => SRAM_IF30_APD_A::PDSLEEPCFG2_SRAM_IF30_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF30_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if30_apd_0(&self) -> bool {
        *self == SRAM_IF30_APD_A::PDSLEEPCFG2_SRAM_IF30_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF30_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if30_apd_1(&self) -> bool {
        *self == SRAM_IF30_APD_A::PDSLEEPCFG2_SRAM_IF30_APD_1
    }
}
#[doc = "Field `SRAM_IF30_APD` writer - Array power for RAM interface 30"]
pub type SRAM_IF30_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF30_APD_A, O>;
impl<'a, const O: u8> SRAM_IF30_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if30_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF30_APD_A::PDSLEEPCFG2_SRAM_IF30_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if30_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF30_APD_A::PDSLEEPCFG2_SRAM_IF30_APD_1)
    }
}
#[doc = "Field `SRAM_IF31_APD` reader - Array power for RAM interface 31"]
pub type SRAM_IF31_APD_R = crate::BitReader<SRAM_IF31_APD_A>;
#[doc = "Array power for RAM interface 31\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF31_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PDSLEEPCFG2_SRAM_IF31_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PDSLEEPCFG2_SRAM_IF31_APD_1 = 1,
}
impl From<SRAM_IF31_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF31_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IF31_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IF31_APD_A {
        match self.bits {
            false => SRAM_IF31_APD_A::PDSLEEPCFG2_SRAM_IF31_APD_0,
            true => SRAM_IF31_APD_A::PDSLEEPCFG2_SRAM_IF31_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF31_APD_0`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if31_apd_0(&self) -> bool {
        *self == SRAM_IF31_APD_A::PDSLEEPCFG2_SRAM_IF31_APD_0
    }
    #[doc = "Checks if the value of the field is `PDSLEEPCFG2_SRAM_IF31_APD_1`"]
    #[inline(always)]
    pub fn is_pdsleepcfg2_sram_if31_apd_1(&self) -> bool {
        *self == SRAM_IF31_APD_A::PDSLEEPCFG2_SRAM_IF31_APD_1
    }
}
#[doc = "Field `SRAM_IF31_APD` writer - Array power for RAM interface 31"]
pub type SRAM_IF31_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG2_SPEC, SRAM_IF31_APD_A, O>;
impl<'a, const O: u8> SRAM_IF31_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if31_apd_0(self) -> &'a mut W {
        self.variant(SRAM_IF31_APD_A::PDSLEEPCFG2_SRAM_IF31_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pdsleepcfg2_sram_if31_apd_1(self) -> &'a mut W {
        self.variant(SRAM_IF31_APD_A::PDSLEEPCFG2_SRAM_IF31_APD_1)
    }
}
impl R {
    #[doc = "Bit 0 - Array power for RAM interface 0"]
    #[inline(always)]
    pub fn sram_if0_apd(&self) -> SRAM_IF0_APD_R {
        SRAM_IF0_APD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Array power for RAM interface 1"]
    #[inline(always)]
    pub fn sram_if1_apd(&self) -> SRAM_IF1_APD_R {
        SRAM_IF1_APD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Array power for RAM interface 2"]
    #[inline(always)]
    pub fn sram_if2_apd(&self) -> SRAM_IF2_APD_R {
        SRAM_IF2_APD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Array power for RAM interface 3"]
    #[inline(always)]
    pub fn sram_if3_apd(&self) -> SRAM_IF3_APD_R {
        SRAM_IF3_APD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Array power for RAM interface 4"]
    #[inline(always)]
    pub fn sram_if4_apd(&self) -> SRAM_IF4_APD_R {
        SRAM_IF4_APD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Array power for RAM interface 5"]
    #[inline(always)]
    pub fn sram_if5_apd(&self) -> SRAM_IF5_APD_R {
        SRAM_IF5_APD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Array power for RAM interface 6"]
    #[inline(always)]
    pub fn sram_if6_apd(&self) -> SRAM_IF6_APD_R {
        SRAM_IF6_APD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Array power for RAM interface 7"]
    #[inline(always)]
    pub fn sram_if7_apd(&self) -> SRAM_IF7_APD_R {
        SRAM_IF7_APD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Array power for RAM interface 8"]
    #[inline(always)]
    pub fn sram_if8_apd(&self) -> SRAM_IF8_APD_R {
        SRAM_IF8_APD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Array power for RAM interface 9"]
    #[inline(always)]
    pub fn sram_if9_apd(&self) -> SRAM_IF9_APD_R {
        SRAM_IF9_APD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Array power for RAM interface 10"]
    #[inline(always)]
    pub fn sram_if10_apd(&self) -> SRAM_IF10_APD_R {
        SRAM_IF10_APD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Array power for RAM interface 11"]
    #[inline(always)]
    pub fn sram_if11_apd(&self) -> SRAM_IF11_APD_R {
        SRAM_IF11_APD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Array power for RAM interface 12"]
    #[inline(always)]
    pub fn sram_if12_apd(&self) -> SRAM_IF12_APD_R {
        SRAM_IF12_APD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Array power for RAM interface 13"]
    #[inline(always)]
    pub fn sram_if13_apd(&self) -> SRAM_IF13_APD_R {
        SRAM_IF13_APD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Array power for RAM interface 14"]
    #[inline(always)]
    pub fn sram_if14_apd(&self) -> SRAM_IF14_APD_R {
        SRAM_IF14_APD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Array power for RAM interface 15"]
    #[inline(always)]
    pub fn sram_if15_apd(&self) -> SRAM_IF15_APD_R {
        SRAM_IF15_APD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Array power for RAM interface 16"]
    #[inline(always)]
    pub fn sram_if16_apd(&self) -> SRAM_IF16_APD_R {
        SRAM_IF16_APD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Array power for RAM interface 17"]
    #[inline(always)]
    pub fn sram_if17_apd(&self) -> SRAM_IF17_APD_R {
        SRAM_IF17_APD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Array power for RAM interface 18"]
    #[inline(always)]
    pub fn sram_if18_apd(&self) -> SRAM_IF18_APD_R {
        SRAM_IF18_APD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Array power for RAM interface 19"]
    #[inline(always)]
    pub fn sram_if19_apd(&self) -> SRAM_IF19_APD_R {
        SRAM_IF19_APD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Array power for RAM interface 20"]
    #[inline(always)]
    pub fn sram_if20_apd(&self) -> SRAM_IF20_APD_R {
        SRAM_IF20_APD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Array power for RAM interface 21"]
    #[inline(always)]
    pub fn sram_if21_apd(&self) -> SRAM_IF21_APD_R {
        SRAM_IF21_APD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Array power for RAM interface 22"]
    #[inline(always)]
    pub fn sram_if22_apd(&self) -> SRAM_IF22_APD_R {
        SRAM_IF22_APD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Array power for RAM interface 23"]
    #[inline(always)]
    pub fn sram_if23_apd(&self) -> SRAM_IF23_APD_R {
        SRAM_IF23_APD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Array power for RAM interface 24"]
    #[inline(always)]
    pub fn sram_if24_apd(&self) -> SRAM_IF24_APD_R {
        SRAM_IF24_APD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Array power for RAM interface 25"]
    #[inline(always)]
    pub fn sram_if25_apd(&self) -> SRAM_IF25_APD_R {
        SRAM_IF25_APD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Array power for RAM interface 26"]
    #[inline(always)]
    pub fn sram_if26_apd(&self) -> SRAM_IF26_APD_R {
        SRAM_IF26_APD_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Array power for RAM interface 27"]
    #[inline(always)]
    pub fn sram_if27_apd(&self) -> SRAM_IF27_APD_R {
        SRAM_IF27_APD_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Array power for RAM interface 28"]
    #[inline(always)]
    pub fn sram_if28_apd(&self) -> SRAM_IF28_APD_R {
        SRAM_IF28_APD_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Array power for RAM interface 29"]
    #[inline(always)]
    pub fn sram_if29_apd(&self) -> SRAM_IF29_APD_R {
        SRAM_IF29_APD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Array power for RAM interface 30"]
    #[inline(always)]
    pub fn sram_if30_apd(&self) -> SRAM_IF30_APD_R {
        SRAM_IF30_APD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Array power for RAM interface 31"]
    #[inline(always)]
    pub fn sram_if31_apd(&self) -> SRAM_IF31_APD_R {
        SRAM_IF31_APD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Array power for RAM interface 0"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if0_apd(&mut self) -> SRAM_IF0_APD_W<0> {
        SRAM_IF0_APD_W::new(self)
    }
    #[doc = "Bit 1 - Array power for RAM interface 1"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if1_apd(&mut self) -> SRAM_IF1_APD_W<1> {
        SRAM_IF1_APD_W::new(self)
    }
    #[doc = "Bit 2 - Array power for RAM interface 2"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if2_apd(&mut self) -> SRAM_IF2_APD_W<2> {
        SRAM_IF2_APD_W::new(self)
    }
    #[doc = "Bit 3 - Array power for RAM interface 3"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if3_apd(&mut self) -> SRAM_IF3_APD_W<3> {
        SRAM_IF3_APD_W::new(self)
    }
    #[doc = "Bit 4 - Array power for RAM interface 4"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if4_apd(&mut self) -> SRAM_IF4_APD_W<4> {
        SRAM_IF4_APD_W::new(self)
    }
    #[doc = "Bit 5 - Array power for RAM interface 5"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if5_apd(&mut self) -> SRAM_IF5_APD_W<5> {
        SRAM_IF5_APD_W::new(self)
    }
    #[doc = "Bit 6 - Array power for RAM interface 6"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if6_apd(&mut self) -> SRAM_IF6_APD_W<6> {
        SRAM_IF6_APD_W::new(self)
    }
    #[doc = "Bit 7 - Array power for RAM interface 7"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if7_apd(&mut self) -> SRAM_IF7_APD_W<7> {
        SRAM_IF7_APD_W::new(self)
    }
    #[doc = "Bit 8 - Array power for RAM interface 8"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if8_apd(&mut self) -> SRAM_IF8_APD_W<8> {
        SRAM_IF8_APD_W::new(self)
    }
    #[doc = "Bit 9 - Array power for RAM interface 9"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if9_apd(&mut self) -> SRAM_IF9_APD_W<9> {
        SRAM_IF9_APD_W::new(self)
    }
    #[doc = "Bit 10 - Array power for RAM interface 10"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if10_apd(&mut self) -> SRAM_IF10_APD_W<10> {
        SRAM_IF10_APD_W::new(self)
    }
    #[doc = "Bit 11 - Array power for RAM interface 11"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if11_apd(&mut self) -> SRAM_IF11_APD_W<11> {
        SRAM_IF11_APD_W::new(self)
    }
    #[doc = "Bit 12 - Array power for RAM interface 12"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if12_apd(&mut self) -> SRAM_IF12_APD_W<12> {
        SRAM_IF12_APD_W::new(self)
    }
    #[doc = "Bit 13 - Array power for RAM interface 13"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if13_apd(&mut self) -> SRAM_IF13_APD_W<13> {
        SRAM_IF13_APD_W::new(self)
    }
    #[doc = "Bit 14 - Array power for RAM interface 14"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if14_apd(&mut self) -> SRAM_IF14_APD_W<14> {
        SRAM_IF14_APD_W::new(self)
    }
    #[doc = "Bit 15 - Array power for RAM interface 15"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if15_apd(&mut self) -> SRAM_IF15_APD_W<15> {
        SRAM_IF15_APD_W::new(self)
    }
    #[doc = "Bit 16 - Array power for RAM interface 16"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if16_apd(&mut self) -> SRAM_IF16_APD_W<16> {
        SRAM_IF16_APD_W::new(self)
    }
    #[doc = "Bit 17 - Array power for RAM interface 17"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if17_apd(&mut self) -> SRAM_IF17_APD_W<17> {
        SRAM_IF17_APD_W::new(self)
    }
    #[doc = "Bit 18 - Array power for RAM interface 18"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if18_apd(&mut self) -> SRAM_IF18_APD_W<18> {
        SRAM_IF18_APD_W::new(self)
    }
    #[doc = "Bit 19 - Array power for RAM interface 19"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if19_apd(&mut self) -> SRAM_IF19_APD_W<19> {
        SRAM_IF19_APD_W::new(self)
    }
    #[doc = "Bit 20 - Array power for RAM interface 20"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if20_apd(&mut self) -> SRAM_IF20_APD_W<20> {
        SRAM_IF20_APD_W::new(self)
    }
    #[doc = "Bit 21 - Array power for RAM interface 21"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if21_apd(&mut self) -> SRAM_IF21_APD_W<21> {
        SRAM_IF21_APD_W::new(self)
    }
    #[doc = "Bit 22 - Array power for RAM interface 22"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if22_apd(&mut self) -> SRAM_IF22_APD_W<22> {
        SRAM_IF22_APD_W::new(self)
    }
    #[doc = "Bit 23 - Array power for RAM interface 23"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if23_apd(&mut self) -> SRAM_IF23_APD_W<23> {
        SRAM_IF23_APD_W::new(self)
    }
    #[doc = "Bit 24 - Array power for RAM interface 24"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if24_apd(&mut self) -> SRAM_IF24_APD_W<24> {
        SRAM_IF24_APD_W::new(self)
    }
    #[doc = "Bit 25 - Array power for RAM interface 25"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if25_apd(&mut self) -> SRAM_IF25_APD_W<25> {
        SRAM_IF25_APD_W::new(self)
    }
    #[doc = "Bit 26 - Array power for RAM interface 26"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if26_apd(&mut self) -> SRAM_IF26_APD_W<26> {
        SRAM_IF26_APD_W::new(self)
    }
    #[doc = "Bit 27 - Array power for RAM interface 27"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if27_apd(&mut self) -> SRAM_IF27_APD_W<27> {
        SRAM_IF27_APD_W::new(self)
    }
    #[doc = "Bit 28 - Array power for RAM interface 28"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if28_apd(&mut self) -> SRAM_IF28_APD_W<28> {
        SRAM_IF28_APD_W::new(self)
    }
    #[doc = "Bit 29 - Array power for RAM interface 29"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if29_apd(&mut self) -> SRAM_IF29_APD_W<29> {
        SRAM_IF29_APD_W::new(self)
    }
    #[doc = "Bit 30 - Array power for RAM interface 30"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if30_apd(&mut self) -> SRAM_IF30_APD_W<30> {
        SRAM_IF30_APD_W::new(self)
    }
    #[doc = "Bit 31 - Array power for RAM interface 31"]
    #[inline(always)]
    #[must_use]
    pub fn sram_if31_apd(&mut self) -> SRAM_IF31_APD_W<31> {
        SRAM_IF31_APD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sleep configuration 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdsleepcfg2](index.html) module"]
pub struct PDSLEEPCFG2_SPEC;
impl crate::RegisterSpec for PDSLEEPCFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdsleepcfg2::R](R) reader structure"]
impl crate::Readable for PDSLEEPCFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdsleepcfg2::W](W) writer structure"]
impl crate::Writable for PDSLEEPCFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDSLEEPCFG2 to value 0xffff_fffe"]
impl crate::Resettable for PDSLEEPCFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_fffe;
}
