#[doc = "Register `AUTOCLKGATEOVERRIDE1` reader"]
pub struct R(crate::R<AUTOCLKGATEOVERRIDE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTOCLKGATEOVERRIDE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUTOCLKGATEOVERRIDE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUTOCLKGATEOVERRIDE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUTOCLKGATEOVERRIDE1` writer"]
pub struct W(crate::W<AUTOCLKGATEOVERRIDE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUTOCLKGATEOVERRIDE1_SPEC>;
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
impl From<crate::W<AUTOCLKGATEOVERRIDE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUTOCLKGATEOVERRIDE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM0_IF` reader - SRAM0_IF"]
pub type SRAM0_IF_R = crate::BitReader<SRAM0_IF_A>;
#[doc = "SRAM0_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM0_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM00_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM00_IF_1 = 1,
}
impl From<SRAM0_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM0_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM0_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM0_IF_A {
        match self.bits {
            false => SRAM0_IF_A::SRAM00_IF_0,
            true => SRAM0_IF_A::SRAM00_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM00_IF_0`"]
    #[inline(always)]
    pub fn is_sram00_if_0(&self) -> bool {
        *self == SRAM0_IF_A::SRAM00_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM00_IF_1`"]
    #[inline(always)]
    pub fn is_sram00_if_1(&self) -> bool {
        *self == SRAM0_IF_A::SRAM00_IF_1
    }
}
#[doc = "Field `SRAM0_IF` writer - SRAM0_IF"]
pub type SRAM0_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM0_IF_A, O>;
impl<'a, const O: u8> SRAM0_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram00_if_0(self) -> &'a mut W {
        self.variant(SRAM0_IF_A::SRAM00_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram00_if_1(self) -> &'a mut W {
        self.variant(SRAM0_IF_A::SRAM00_IF_1)
    }
}
#[doc = "Field `SRAM1_IF` reader - SRAM1_IF"]
pub type SRAM1_IF_R = crate::BitReader<SRAM1_IF_A>;
#[doc = "SRAM1_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM1_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM01_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM01_IF_1 = 1,
}
impl From<SRAM1_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM1_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM1_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM1_IF_A {
        match self.bits {
            false => SRAM1_IF_A::SRAM01_IF_0,
            true => SRAM1_IF_A::SRAM01_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM01_IF_0`"]
    #[inline(always)]
    pub fn is_sram01_if_0(&self) -> bool {
        *self == SRAM1_IF_A::SRAM01_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM01_IF_1`"]
    #[inline(always)]
    pub fn is_sram01_if_1(&self) -> bool {
        *self == SRAM1_IF_A::SRAM01_IF_1
    }
}
#[doc = "Field `SRAM1_IF` writer - SRAM1_IF"]
pub type SRAM1_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM1_IF_A, O>;
impl<'a, const O: u8> SRAM1_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram01_if_0(self) -> &'a mut W {
        self.variant(SRAM1_IF_A::SRAM01_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram01_if_1(self) -> &'a mut W {
        self.variant(SRAM1_IF_A::SRAM01_IF_1)
    }
}
#[doc = "Field `SRAM2_IF` reader - SRAM2_IF"]
pub type SRAM2_IF_R = crate::BitReader<SRAM2_IF_A>;
#[doc = "SRAM2_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM02_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM02_IF_1 = 1,
}
impl From<SRAM2_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM2_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM2_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM2_IF_A {
        match self.bits {
            false => SRAM2_IF_A::SRAM02_IF_0,
            true => SRAM2_IF_A::SRAM02_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM02_IF_0`"]
    #[inline(always)]
    pub fn is_sram02_if_0(&self) -> bool {
        *self == SRAM2_IF_A::SRAM02_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM02_IF_1`"]
    #[inline(always)]
    pub fn is_sram02_if_1(&self) -> bool {
        *self == SRAM2_IF_A::SRAM02_IF_1
    }
}
#[doc = "Field `SRAM2_IF` writer - SRAM2_IF"]
pub type SRAM2_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM2_IF_A, O>;
impl<'a, const O: u8> SRAM2_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram02_if_0(self) -> &'a mut W {
        self.variant(SRAM2_IF_A::SRAM02_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram02_if_1(self) -> &'a mut W {
        self.variant(SRAM2_IF_A::SRAM02_IF_1)
    }
}
#[doc = "Field `SRAM3_IF` reader - SRAM3_IF"]
pub type SRAM3_IF_R = crate::BitReader<SRAM3_IF_A>;
#[doc = "SRAM3_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM3_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM03_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM03_IF_1 = 1,
}
impl From<SRAM3_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM3_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM3_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM3_IF_A {
        match self.bits {
            false => SRAM3_IF_A::SRAM03_IF_0,
            true => SRAM3_IF_A::SRAM03_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM03_IF_0`"]
    #[inline(always)]
    pub fn is_sram03_if_0(&self) -> bool {
        *self == SRAM3_IF_A::SRAM03_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM03_IF_1`"]
    #[inline(always)]
    pub fn is_sram03_if_1(&self) -> bool {
        *self == SRAM3_IF_A::SRAM03_IF_1
    }
}
#[doc = "Field `SRAM3_IF` writer - SRAM3_IF"]
pub type SRAM3_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM3_IF_A, O>;
impl<'a, const O: u8> SRAM3_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram03_if_0(self) -> &'a mut W {
        self.variant(SRAM3_IF_A::SRAM03_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram03_if_1(self) -> &'a mut W {
        self.variant(SRAM3_IF_A::SRAM03_IF_1)
    }
}
#[doc = "Field `SRAM4_IF` reader - SRAM4_IF"]
pub type SRAM4_IF_R = crate::BitReader<SRAM4_IF_A>;
#[doc = "SRAM4_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM4_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM04_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM04_IF_1 = 1,
}
impl From<SRAM4_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM4_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM4_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM4_IF_A {
        match self.bits {
            false => SRAM4_IF_A::SRAM04_IF_0,
            true => SRAM4_IF_A::SRAM04_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM04_IF_0`"]
    #[inline(always)]
    pub fn is_sram04_if_0(&self) -> bool {
        *self == SRAM4_IF_A::SRAM04_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM04_IF_1`"]
    #[inline(always)]
    pub fn is_sram04_if_1(&self) -> bool {
        *self == SRAM4_IF_A::SRAM04_IF_1
    }
}
#[doc = "Field `SRAM4_IF` writer - SRAM4_IF"]
pub type SRAM4_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM4_IF_A, O>;
impl<'a, const O: u8> SRAM4_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram04_if_0(self) -> &'a mut W {
        self.variant(SRAM4_IF_A::SRAM04_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram04_if_1(self) -> &'a mut W {
        self.variant(SRAM4_IF_A::SRAM04_IF_1)
    }
}
#[doc = "Field `SRAM5_IF` reader - SRAM5_IF"]
pub type SRAM5_IF_R = crate::BitReader<SRAM5_IF_A>;
#[doc = "SRAM5_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM5_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM05_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM05_IF_1 = 1,
}
impl From<SRAM5_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM5_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM5_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM5_IF_A {
        match self.bits {
            false => SRAM5_IF_A::SRAM05_IF_0,
            true => SRAM5_IF_A::SRAM05_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM05_IF_0`"]
    #[inline(always)]
    pub fn is_sram05_if_0(&self) -> bool {
        *self == SRAM5_IF_A::SRAM05_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM05_IF_1`"]
    #[inline(always)]
    pub fn is_sram05_if_1(&self) -> bool {
        *self == SRAM5_IF_A::SRAM05_IF_1
    }
}
#[doc = "Field `SRAM5_IF` writer - SRAM5_IF"]
pub type SRAM5_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM5_IF_A, O>;
impl<'a, const O: u8> SRAM5_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram05_if_0(self) -> &'a mut W {
        self.variant(SRAM5_IF_A::SRAM05_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram05_if_1(self) -> &'a mut W {
        self.variant(SRAM5_IF_A::SRAM05_IF_1)
    }
}
#[doc = "Field `SRAM6_IF` reader - SRAM6_IF"]
pub type SRAM6_IF_R = crate::BitReader<SRAM6_IF_A>;
#[doc = "SRAM6_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM6_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM06_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM06_IF_1 = 1,
}
impl From<SRAM6_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM6_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM6_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM6_IF_A {
        match self.bits {
            false => SRAM6_IF_A::SRAM06_IF_0,
            true => SRAM6_IF_A::SRAM06_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM06_IF_0`"]
    #[inline(always)]
    pub fn is_sram06_if_0(&self) -> bool {
        *self == SRAM6_IF_A::SRAM06_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM06_IF_1`"]
    #[inline(always)]
    pub fn is_sram06_if_1(&self) -> bool {
        *self == SRAM6_IF_A::SRAM06_IF_1
    }
}
#[doc = "Field `SRAM6_IF` writer - SRAM6_IF"]
pub type SRAM6_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM6_IF_A, O>;
impl<'a, const O: u8> SRAM6_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram06_if_0(self) -> &'a mut W {
        self.variant(SRAM6_IF_A::SRAM06_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram06_if_1(self) -> &'a mut W {
        self.variant(SRAM6_IF_A::SRAM06_IF_1)
    }
}
#[doc = "Field `SRAM7_IF` reader - SRAM7_IF"]
pub type SRAM7_IF_R = crate::BitReader<SRAM7_IF_A>;
#[doc = "SRAM7_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM7_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM07_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM07_IF_1 = 1,
}
impl From<SRAM7_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM7_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM7_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM7_IF_A {
        match self.bits {
            false => SRAM7_IF_A::SRAM07_IF_0,
            true => SRAM7_IF_A::SRAM07_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM07_IF_0`"]
    #[inline(always)]
    pub fn is_sram07_if_0(&self) -> bool {
        *self == SRAM7_IF_A::SRAM07_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM07_IF_1`"]
    #[inline(always)]
    pub fn is_sram07_if_1(&self) -> bool {
        *self == SRAM7_IF_A::SRAM07_IF_1
    }
}
#[doc = "Field `SRAM7_IF` writer - SRAM7_IF"]
pub type SRAM7_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM7_IF_A, O>;
impl<'a, const O: u8> SRAM7_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram07_if_0(self) -> &'a mut W {
        self.variant(SRAM7_IF_A::SRAM07_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram07_if_1(self) -> &'a mut W {
        self.variant(SRAM7_IF_A::SRAM07_IF_1)
    }
}
#[doc = "Field `SRAM8_IF` reader - SRAM8_IF"]
pub type SRAM8_IF_R = crate::BitReader<SRAM8_IF_A>;
#[doc = "SRAM8_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM8_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM08_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM08_IF_1 = 1,
}
impl From<SRAM8_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM8_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM8_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM8_IF_A {
        match self.bits {
            false => SRAM8_IF_A::SRAM08_IF_0,
            true => SRAM8_IF_A::SRAM08_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM08_IF_0`"]
    #[inline(always)]
    pub fn is_sram08_if_0(&self) -> bool {
        *self == SRAM8_IF_A::SRAM08_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM08_IF_1`"]
    #[inline(always)]
    pub fn is_sram08_if_1(&self) -> bool {
        *self == SRAM8_IF_A::SRAM08_IF_1
    }
}
#[doc = "Field `SRAM8_IF` writer - SRAM8_IF"]
pub type SRAM8_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM8_IF_A, O>;
impl<'a, const O: u8> SRAM8_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram08_if_0(self) -> &'a mut W {
        self.variant(SRAM8_IF_A::SRAM08_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram08_if_1(self) -> &'a mut W {
        self.variant(SRAM8_IF_A::SRAM08_IF_1)
    }
}
#[doc = "Field `SRAM9_IF` reader - SRAM9_IF"]
pub type SRAM9_IF_R = crate::BitReader<SRAM9_IF_A>;
#[doc = "SRAM9_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM9_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM09_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM09_IF_1 = 1,
}
impl From<SRAM9_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM9_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM9_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM9_IF_A {
        match self.bits {
            false => SRAM9_IF_A::SRAM09_IF_0,
            true => SRAM9_IF_A::SRAM09_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM09_IF_0`"]
    #[inline(always)]
    pub fn is_sram09_if_0(&self) -> bool {
        *self == SRAM9_IF_A::SRAM09_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM09_IF_1`"]
    #[inline(always)]
    pub fn is_sram09_if_1(&self) -> bool {
        *self == SRAM9_IF_A::SRAM09_IF_1
    }
}
#[doc = "Field `SRAM9_IF` writer - SRAM9_IF"]
pub type SRAM9_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM9_IF_A, O>;
impl<'a, const O: u8> SRAM9_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram09_if_0(self) -> &'a mut W {
        self.variant(SRAM9_IF_A::SRAM09_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram09_if_1(self) -> &'a mut W {
        self.variant(SRAM9_IF_A::SRAM09_IF_1)
    }
}
#[doc = "Field `SRAM10_IF` reader - SRAM10_IF"]
pub type SRAM10_IF_R = crate::BitReader<SRAM10_IF_A>;
#[doc = "SRAM10_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM10_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM010_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM010_IF_1 = 1,
}
impl From<SRAM10_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM10_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM10_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM10_IF_A {
        match self.bits {
            false => SRAM10_IF_A::SRAM010_IF_0,
            true => SRAM10_IF_A::SRAM010_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM010_IF_0`"]
    #[inline(always)]
    pub fn is_sram010_if_0(&self) -> bool {
        *self == SRAM10_IF_A::SRAM010_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM010_IF_1`"]
    #[inline(always)]
    pub fn is_sram010_if_1(&self) -> bool {
        *self == SRAM10_IF_A::SRAM010_IF_1
    }
}
#[doc = "Field `SRAM10_IF` writer - SRAM10_IF"]
pub type SRAM10_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM10_IF_A, O>;
impl<'a, const O: u8> SRAM10_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram010_if_0(self) -> &'a mut W {
        self.variant(SRAM10_IF_A::SRAM010_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram010_if_1(self) -> &'a mut W {
        self.variant(SRAM10_IF_A::SRAM010_IF_1)
    }
}
#[doc = "Field `SRAM11_IF` reader - SRAM11_IF"]
pub type SRAM11_IF_R = crate::BitReader<SRAM11_IF_A>;
#[doc = "SRAM11_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM11_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM011_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM011_IF_1 = 1,
}
impl From<SRAM11_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM11_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM11_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM11_IF_A {
        match self.bits {
            false => SRAM11_IF_A::SRAM011_IF_0,
            true => SRAM11_IF_A::SRAM011_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM011_IF_0`"]
    #[inline(always)]
    pub fn is_sram011_if_0(&self) -> bool {
        *self == SRAM11_IF_A::SRAM011_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM011_IF_1`"]
    #[inline(always)]
    pub fn is_sram011_if_1(&self) -> bool {
        *self == SRAM11_IF_A::SRAM011_IF_1
    }
}
#[doc = "Field `SRAM11_IF` writer - SRAM11_IF"]
pub type SRAM11_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM11_IF_A, O>;
impl<'a, const O: u8> SRAM11_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram011_if_0(self) -> &'a mut W {
        self.variant(SRAM11_IF_A::SRAM011_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram011_if_1(self) -> &'a mut W {
        self.variant(SRAM11_IF_A::SRAM011_IF_1)
    }
}
#[doc = "Field `SRAM12_IF` reader - SRAM12_IF"]
pub type SRAM12_IF_R = crate::BitReader<SRAM12_IF_A>;
#[doc = "SRAM12_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM12_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM012_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM012_IF_1 = 1,
}
impl From<SRAM12_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM12_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM12_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM12_IF_A {
        match self.bits {
            false => SRAM12_IF_A::SRAM012_IF_0,
            true => SRAM12_IF_A::SRAM012_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM012_IF_0`"]
    #[inline(always)]
    pub fn is_sram012_if_0(&self) -> bool {
        *self == SRAM12_IF_A::SRAM012_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM012_IF_1`"]
    #[inline(always)]
    pub fn is_sram012_if_1(&self) -> bool {
        *self == SRAM12_IF_A::SRAM012_IF_1
    }
}
#[doc = "Field `SRAM12_IF` writer - SRAM12_IF"]
pub type SRAM12_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM12_IF_A, O>;
impl<'a, const O: u8> SRAM12_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram012_if_0(self) -> &'a mut W {
        self.variant(SRAM12_IF_A::SRAM012_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram012_if_1(self) -> &'a mut W {
        self.variant(SRAM12_IF_A::SRAM012_IF_1)
    }
}
#[doc = "Field `SRAM13_IF` reader - SRAM13_IF"]
pub type SRAM13_IF_R = crate::BitReader<SRAM13_IF_A>;
#[doc = "SRAM13_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM13_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM013_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM013_IF_1 = 1,
}
impl From<SRAM13_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM13_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM13_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM13_IF_A {
        match self.bits {
            false => SRAM13_IF_A::SRAM013_IF_0,
            true => SRAM13_IF_A::SRAM013_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM013_IF_0`"]
    #[inline(always)]
    pub fn is_sram013_if_0(&self) -> bool {
        *self == SRAM13_IF_A::SRAM013_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM013_IF_1`"]
    #[inline(always)]
    pub fn is_sram013_if_1(&self) -> bool {
        *self == SRAM13_IF_A::SRAM013_IF_1
    }
}
#[doc = "Field `SRAM13_IF` writer - SRAM13_IF"]
pub type SRAM13_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM13_IF_A, O>;
impl<'a, const O: u8> SRAM13_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram013_if_0(self) -> &'a mut W {
        self.variant(SRAM13_IF_A::SRAM013_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram013_if_1(self) -> &'a mut W {
        self.variant(SRAM13_IF_A::SRAM013_IF_1)
    }
}
#[doc = "Field `SRAM14_IF` reader - SRAM14_IF"]
pub type SRAM14_IF_R = crate::BitReader<SRAM14_IF_A>;
#[doc = "SRAM14_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM14_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM014_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM014_IF_1 = 1,
}
impl From<SRAM14_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM14_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM14_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM14_IF_A {
        match self.bits {
            false => SRAM14_IF_A::SRAM014_IF_0,
            true => SRAM14_IF_A::SRAM014_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM014_IF_0`"]
    #[inline(always)]
    pub fn is_sram014_if_0(&self) -> bool {
        *self == SRAM14_IF_A::SRAM014_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM014_IF_1`"]
    #[inline(always)]
    pub fn is_sram014_if_1(&self) -> bool {
        *self == SRAM14_IF_A::SRAM014_IF_1
    }
}
#[doc = "Field `SRAM14_IF` writer - SRAM14_IF"]
pub type SRAM14_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM14_IF_A, O>;
impl<'a, const O: u8> SRAM14_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram014_if_0(self) -> &'a mut W {
        self.variant(SRAM14_IF_A::SRAM014_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram014_if_1(self) -> &'a mut W {
        self.variant(SRAM14_IF_A::SRAM014_IF_1)
    }
}
#[doc = "Field `SRAM15_IF` reader - SRAM15_IF"]
pub type SRAM15_IF_R = crate::BitReader<SRAM15_IF_A>;
#[doc = "SRAM15_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM15_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM015_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM015_IF_1 = 1,
}
impl From<SRAM15_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM15_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM15_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM15_IF_A {
        match self.bits {
            false => SRAM15_IF_A::SRAM015_IF_0,
            true => SRAM15_IF_A::SRAM015_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM015_IF_0`"]
    #[inline(always)]
    pub fn is_sram015_if_0(&self) -> bool {
        *self == SRAM15_IF_A::SRAM015_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM015_IF_1`"]
    #[inline(always)]
    pub fn is_sram015_if_1(&self) -> bool {
        *self == SRAM15_IF_A::SRAM015_IF_1
    }
}
#[doc = "Field `SRAM15_IF` writer - SRAM15_IF"]
pub type SRAM15_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM15_IF_A, O>;
impl<'a, const O: u8> SRAM15_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram015_if_0(self) -> &'a mut W {
        self.variant(SRAM15_IF_A::SRAM015_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram015_if_1(self) -> &'a mut W {
        self.variant(SRAM15_IF_A::SRAM015_IF_1)
    }
}
#[doc = "Field `SRAM16_IF` reader - SRAM16_IF"]
pub type SRAM16_IF_R = crate::BitReader<SRAM16_IF_A>;
#[doc = "SRAM16_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM16_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM016_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM016_IF_1 = 1,
}
impl From<SRAM16_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM16_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM16_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM16_IF_A {
        match self.bits {
            false => SRAM16_IF_A::SRAM016_IF_0,
            true => SRAM16_IF_A::SRAM016_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM016_IF_0`"]
    #[inline(always)]
    pub fn is_sram016_if_0(&self) -> bool {
        *self == SRAM16_IF_A::SRAM016_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM016_IF_1`"]
    #[inline(always)]
    pub fn is_sram016_if_1(&self) -> bool {
        *self == SRAM16_IF_A::SRAM016_IF_1
    }
}
#[doc = "Field `SRAM16_IF` writer - SRAM16_IF"]
pub type SRAM16_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM16_IF_A, O>;
impl<'a, const O: u8> SRAM16_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram016_if_0(self) -> &'a mut W {
        self.variant(SRAM16_IF_A::SRAM016_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram016_if_1(self) -> &'a mut W {
        self.variant(SRAM16_IF_A::SRAM016_IF_1)
    }
}
#[doc = "Field `SRAM17_IF` reader - SRAM17_IF"]
pub type SRAM17_IF_R = crate::BitReader<SRAM17_IF_A>;
#[doc = "SRAM17_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM17_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM017_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM017_IF_1 = 1,
}
impl From<SRAM17_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM17_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM17_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM17_IF_A {
        match self.bits {
            false => SRAM17_IF_A::SRAM017_IF_0,
            true => SRAM17_IF_A::SRAM017_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM017_IF_0`"]
    #[inline(always)]
    pub fn is_sram017_if_0(&self) -> bool {
        *self == SRAM17_IF_A::SRAM017_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM017_IF_1`"]
    #[inline(always)]
    pub fn is_sram017_if_1(&self) -> bool {
        *self == SRAM17_IF_A::SRAM017_IF_1
    }
}
#[doc = "Field `SRAM17_IF` writer - SRAM17_IF"]
pub type SRAM17_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM17_IF_A, O>;
impl<'a, const O: u8> SRAM17_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram017_if_0(self) -> &'a mut W {
        self.variant(SRAM17_IF_A::SRAM017_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram017_if_1(self) -> &'a mut W {
        self.variant(SRAM17_IF_A::SRAM017_IF_1)
    }
}
#[doc = "Field `SRAM18_IF` reader - SRAM18_IF"]
pub type SRAM18_IF_R = crate::BitReader<SRAM18_IF_A>;
#[doc = "SRAM18_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM18_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM018_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM018_IF_1 = 1,
}
impl From<SRAM18_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM18_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM18_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM18_IF_A {
        match self.bits {
            false => SRAM18_IF_A::SRAM018_IF_0,
            true => SRAM18_IF_A::SRAM018_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM018_IF_0`"]
    #[inline(always)]
    pub fn is_sram018_if_0(&self) -> bool {
        *self == SRAM18_IF_A::SRAM018_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM018_IF_1`"]
    #[inline(always)]
    pub fn is_sram018_if_1(&self) -> bool {
        *self == SRAM18_IF_A::SRAM018_IF_1
    }
}
#[doc = "Field `SRAM18_IF` writer - SRAM18_IF"]
pub type SRAM18_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM18_IF_A, O>;
impl<'a, const O: u8> SRAM18_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram018_if_0(self) -> &'a mut W {
        self.variant(SRAM18_IF_A::SRAM018_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram018_if_1(self) -> &'a mut W {
        self.variant(SRAM18_IF_A::SRAM018_IF_1)
    }
}
#[doc = "Field `SRAM19_IF` reader - SRAM19_IF"]
pub type SRAM19_IF_R = crate::BitReader<SRAM19_IF_A>;
#[doc = "SRAM19_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM19_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM019_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM019_IF_1 = 1,
}
impl From<SRAM19_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM19_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM19_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM19_IF_A {
        match self.bits {
            false => SRAM19_IF_A::SRAM019_IF_0,
            true => SRAM19_IF_A::SRAM019_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM019_IF_0`"]
    #[inline(always)]
    pub fn is_sram019_if_0(&self) -> bool {
        *self == SRAM19_IF_A::SRAM019_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM019_IF_1`"]
    #[inline(always)]
    pub fn is_sram019_if_1(&self) -> bool {
        *self == SRAM19_IF_A::SRAM019_IF_1
    }
}
#[doc = "Field `SRAM19_IF` writer - SRAM19_IF"]
pub type SRAM19_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM19_IF_A, O>;
impl<'a, const O: u8> SRAM19_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram019_if_0(self) -> &'a mut W {
        self.variant(SRAM19_IF_A::SRAM019_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram019_if_1(self) -> &'a mut W {
        self.variant(SRAM19_IF_A::SRAM019_IF_1)
    }
}
#[doc = "Field `SRAM20_IF` reader - SRAM20_IF"]
pub type SRAM20_IF_R = crate::BitReader<SRAM20_IF_A>;
#[doc = "SRAM20_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM20_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM020_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM020_IF_1 = 1,
}
impl From<SRAM20_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM20_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM20_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM20_IF_A {
        match self.bits {
            false => SRAM20_IF_A::SRAM020_IF_0,
            true => SRAM20_IF_A::SRAM020_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM020_IF_0`"]
    #[inline(always)]
    pub fn is_sram020_if_0(&self) -> bool {
        *self == SRAM20_IF_A::SRAM020_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM020_IF_1`"]
    #[inline(always)]
    pub fn is_sram020_if_1(&self) -> bool {
        *self == SRAM20_IF_A::SRAM020_IF_1
    }
}
#[doc = "Field `SRAM20_IF` writer - SRAM20_IF"]
pub type SRAM20_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM20_IF_A, O>;
impl<'a, const O: u8> SRAM20_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram020_if_0(self) -> &'a mut W {
        self.variant(SRAM20_IF_A::SRAM020_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram020_if_1(self) -> &'a mut W {
        self.variant(SRAM20_IF_A::SRAM020_IF_1)
    }
}
#[doc = "Field `SRAM21_IF` reader - SRAM21_IF"]
pub type SRAM21_IF_R = crate::BitReader<SRAM21_IF_A>;
#[doc = "SRAM21_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM21_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM021_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM021_IF_1 = 1,
}
impl From<SRAM21_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM21_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM21_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM21_IF_A {
        match self.bits {
            false => SRAM21_IF_A::SRAM021_IF_0,
            true => SRAM21_IF_A::SRAM021_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM021_IF_0`"]
    #[inline(always)]
    pub fn is_sram021_if_0(&self) -> bool {
        *self == SRAM21_IF_A::SRAM021_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM021_IF_1`"]
    #[inline(always)]
    pub fn is_sram021_if_1(&self) -> bool {
        *self == SRAM21_IF_A::SRAM021_IF_1
    }
}
#[doc = "Field `SRAM21_IF` writer - SRAM21_IF"]
pub type SRAM21_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM21_IF_A, O>;
impl<'a, const O: u8> SRAM21_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram021_if_0(self) -> &'a mut W {
        self.variant(SRAM21_IF_A::SRAM021_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram021_if_1(self) -> &'a mut W {
        self.variant(SRAM21_IF_A::SRAM021_IF_1)
    }
}
#[doc = "Field `SRAM22_IF` reader - SRAM22_IF"]
pub type SRAM22_IF_R = crate::BitReader<SRAM22_IF_A>;
#[doc = "SRAM22_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM22_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM022_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM022_IF_1 = 1,
}
impl From<SRAM22_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM22_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM22_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM22_IF_A {
        match self.bits {
            false => SRAM22_IF_A::SRAM022_IF_0,
            true => SRAM22_IF_A::SRAM022_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM022_IF_0`"]
    #[inline(always)]
    pub fn is_sram022_if_0(&self) -> bool {
        *self == SRAM22_IF_A::SRAM022_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM022_IF_1`"]
    #[inline(always)]
    pub fn is_sram022_if_1(&self) -> bool {
        *self == SRAM22_IF_A::SRAM022_IF_1
    }
}
#[doc = "Field `SRAM22_IF` writer - SRAM22_IF"]
pub type SRAM22_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM22_IF_A, O>;
impl<'a, const O: u8> SRAM22_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram022_if_0(self) -> &'a mut W {
        self.variant(SRAM22_IF_A::SRAM022_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram022_if_1(self) -> &'a mut W {
        self.variant(SRAM22_IF_A::SRAM022_IF_1)
    }
}
#[doc = "Field `SRAM23_IF` reader - SRAM23_IF"]
pub type SRAM23_IF_R = crate::BitReader<SRAM23_IF_A>;
#[doc = "SRAM23_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM23_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM023_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM023_IF_1 = 1,
}
impl From<SRAM23_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM23_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM23_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM23_IF_A {
        match self.bits {
            false => SRAM23_IF_A::SRAM023_IF_0,
            true => SRAM23_IF_A::SRAM023_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM023_IF_0`"]
    #[inline(always)]
    pub fn is_sram023_if_0(&self) -> bool {
        *self == SRAM23_IF_A::SRAM023_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM023_IF_1`"]
    #[inline(always)]
    pub fn is_sram023_if_1(&self) -> bool {
        *self == SRAM23_IF_A::SRAM023_IF_1
    }
}
#[doc = "Field `SRAM23_IF` writer - SRAM23_IF"]
pub type SRAM23_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM23_IF_A, O>;
impl<'a, const O: u8> SRAM23_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram023_if_0(self) -> &'a mut W {
        self.variant(SRAM23_IF_A::SRAM023_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram023_if_1(self) -> &'a mut W {
        self.variant(SRAM23_IF_A::SRAM023_IF_1)
    }
}
#[doc = "Field `SRAM24_IF` reader - SRAM24_IF"]
pub type SRAM24_IF_R = crate::BitReader<SRAM24_IF_A>;
#[doc = "SRAM24_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM24_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM024_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM024_IF_1 = 1,
}
impl From<SRAM24_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM24_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM24_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM24_IF_A {
        match self.bits {
            false => SRAM24_IF_A::SRAM024_IF_0,
            true => SRAM24_IF_A::SRAM024_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM024_IF_0`"]
    #[inline(always)]
    pub fn is_sram024_if_0(&self) -> bool {
        *self == SRAM24_IF_A::SRAM024_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM024_IF_1`"]
    #[inline(always)]
    pub fn is_sram024_if_1(&self) -> bool {
        *self == SRAM24_IF_A::SRAM024_IF_1
    }
}
#[doc = "Field `SRAM24_IF` writer - SRAM24_IF"]
pub type SRAM24_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM24_IF_A, O>;
impl<'a, const O: u8> SRAM24_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram024_if_0(self) -> &'a mut W {
        self.variant(SRAM24_IF_A::SRAM024_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram024_if_1(self) -> &'a mut W {
        self.variant(SRAM24_IF_A::SRAM024_IF_1)
    }
}
#[doc = "Field `SRAM25_IF` reader - SRAM25_IF"]
pub type SRAM25_IF_R = crate::BitReader<SRAM25_IF_A>;
#[doc = "SRAM25_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM25_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM025_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM025_IF_1 = 1,
}
impl From<SRAM25_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM25_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM25_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM25_IF_A {
        match self.bits {
            false => SRAM25_IF_A::SRAM025_IF_0,
            true => SRAM25_IF_A::SRAM025_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM025_IF_0`"]
    #[inline(always)]
    pub fn is_sram025_if_0(&self) -> bool {
        *self == SRAM25_IF_A::SRAM025_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM025_IF_1`"]
    #[inline(always)]
    pub fn is_sram025_if_1(&self) -> bool {
        *self == SRAM25_IF_A::SRAM025_IF_1
    }
}
#[doc = "Field `SRAM25_IF` writer - SRAM25_IF"]
pub type SRAM25_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM25_IF_A, O>;
impl<'a, const O: u8> SRAM25_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram025_if_0(self) -> &'a mut W {
        self.variant(SRAM25_IF_A::SRAM025_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram025_if_1(self) -> &'a mut W {
        self.variant(SRAM25_IF_A::SRAM025_IF_1)
    }
}
#[doc = "Field `SRAM26_IF` reader - SRAM26_IF"]
pub type SRAM26_IF_R = crate::BitReader<SRAM26_IF_A>;
#[doc = "SRAM26_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM26_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM026_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM026_IF_1 = 1,
}
impl From<SRAM26_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM26_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM26_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM26_IF_A {
        match self.bits {
            false => SRAM26_IF_A::SRAM026_IF_0,
            true => SRAM26_IF_A::SRAM026_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM026_IF_0`"]
    #[inline(always)]
    pub fn is_sram026_if_0(&self) -> bool {
        *self == SRAM26_IF_A::SRAM026_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM026_IF_1`"]
    #[inline(always)]
    pub fn is_sram026_if_1(&self) -> bool {
        *self == SRAM26_IF_A::SRAM026_IF_1
    }
}
#[doc = "Field `SRAM26_IF` writer - SRAM26_IF"]
pub type SRAM26_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM26_IF_A, O>;
impl<'a, const O: u8> SRAM26_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram026_if_0(self) -> &'a mut W {
        self.variant(SRAM26_IF_A::SRAM026_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram026_if_1(self) -> &'a mut W {
        self.variant(SRAM26_IF_A::SRAM026_IF_1)
    }
}
#[doc = "Field `SRAM27_IF` reader - SRAM27_IF"]
pub type SRAM27_IF_R = crate::BitReader<SRAM27_IF_A>;
#[doc = "SRAM27_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM27_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM027_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM027_IF_1 = 1,
}
impl From<SRAM27_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM27_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM27_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM27_IF_A {
        match self.bits {
            false => SRAM27_IF_A::SRAM027_IF_0,
            true => SRAM27_IF_A::SRAM027_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM027_IF_0`"]
    #[inline(always)]
    pub fn is_sram027_if_0(&self) -> bool {
        *self == SRAM27_IF_A::SRAM027_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM027_IF_1`"]
    #[inline(always)]
    pub fn is_sram027_if_1(&self) -> bool {
        *self == SRAM27_IF_A::SRAM027_IF_1
    }
}
#[doc = "Field `SRAM27_IF` writer - SRAM27_IF"]
pub type SRAM27_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM27_IF_A, O>;
impl<'a, const O: u8> SRAM27_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram027_if_0(self) -> &'a mut W {
        self.variant(SRAM27_IF_A::SRAM027_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram027_if_1(self) -> &'a mut W {
        self.variant(SRAM27_IF_A::SRAM027_IF_1)
    }
}
#[doc = "Field `SRAM28_IF` reader - SRAM28_IF"]
pub type SRAM28_IF_R = crate::BitReader<SRAM28_IF_A>;
#[doc = "SRAM28_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM28_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM028_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM028_IF_1 = 1,
}
impl From<SRAM28_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM28_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM28_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM28_IF_A {
        match self.bits {
            false => SRAM28_IF_A::SRAM028_IF_0,
            true => SRAM28_IF_A::SRAM028_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM028_IF_0`"]
    #[inline(always)]
    pub fn is_sram028_if_0(&self) -> bool {
        *self == SRAM28_IF_A::SRAM028_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM028_IF_1`"]
    #[inline(always)]
    pub fn is_sram028_if_1(&self) -> bool {
        *self == SRAM28_IF_A::SRAM028_IF_1
    }
}
#[doc = "Field `SRAM28_IF` writer - SRAM28_IF"]
pub type SRAM28_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM28_IF_A, O>;
impl<'a, const O: u8> SRAM28_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram028_if_0(self) -> &'a mut W {
        self.variant(SRAM28_IF_A::SRAM028_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram028_if_1(self) -> &'a mut W {
        self.variant(SRAM28_IF_A::SRAM028_IF_1)
    }
}
#[doc = "Field `SRAM29_IF` reader - SRAM29_IF"]
pub type SRAM29_IF_R = crate::BitReader<SRAM29_IF_A>;
#[doc = "SRAM29_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM29_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM029_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM029_IF_1 = 1,
}
impl From<SRAM29_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM29_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM29_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM29_IF_A {
        match self.bits {
            false => SRAM29_IF_A::SRAM029_IF_0,
            true => SRAM29_IF_A::SRAM029_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM029_IF_0`"]
    #[inline(always)]
    pub fn is_sram029_if_0(&self) -> bool {
        *self == SRAM29_IF_A::SRAM029_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM029_IF_1`"]
    #[inline(always)]
    pub fn is_sram029_if_1(&self) -> bool {
        *self == SRAM29_IF_A::SRAM029_IF_1
    }
}
#[doc = "Field `SRAM29_IF` writer - SRAM29_IF"]
pub type SRAM29_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM29_IF_A, O>;
impl<'a, const O: u8> SRAM29_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram029_if_0(self) -> &'a mut W {
        self.variant(SRAM29_IF_A::SRAM029_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram029_if_1(self) -> &'a mut W {
        self.variant(SRAM29_IF_A::SRAM029_IF_1)
    }
}
#[doc = "Field `SRAM30_IF` reader - SRAM30_IF"]
pub type SRAM30_IF_R = crate::BitReader<SRAM30_IF_A>;
#[doc = "SRAM30_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM30_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM030_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM030_IF_1 = 1,
}
impl From<SRAM30_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM30_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM30_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM30_IF_A {
        match self.bits {
            false => SRAM30_IF_A::SRAM030_IF_0,
            true => SRAM30_IF_A::SRAM030_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM030_IF_0`"]
    #[inline(always)]
    pub fn is_sram030_if_0(&self) -> bool {
        *self == SRAM30_IF_A::SRAM030_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM030_IF_1`"]
    #[inline(always)]
    pub fn is_sram030_if_1(&self) -> bool {
        *self == SRAM30_IF_A::SRAM030_IF_1
    }
}
#[doc = "Field `SRAM30_IF` writer - SRAM30_IF"]
pub type SRAM30_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM30_IF_A, O>;
impl<'a, const O: u8> SRAM30_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram030_if_0(self) -> &'a mut W {
        self.variant(SRAM30_IF_A::SRAM030_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram030_if_1(self) -> &'a mut W {
        self.variant(SRAM30_IF_A::SRAM030_IF_1)
    }
}
#[doc = "Field `SRAM31_IF` reader - SRAM31_IF"]
pub type SRAM31_IF_R = crate::BitReader<SRAM31_IF_A>;
#[doc = "SRAM31_IF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM31_IF_A {
    #[doc = "0: Enable clock gating"]
    SRAM031_IF_0 = 0,
    #[doc = "1: Continuous Clocking"]
    SRAM031_IF_1 = 1,
}
impl From<SRAM31_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM31_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM31_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM31_IF_A {
        match self.bits {
            false => SRAM31_IF_A::SRAM031_IF_0,
            true => SRAM31_IF_A::SRAM031_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM031_IF_0`"]
    #[inline(always)]
    pub fn is_sram031_if_0(&self) -> bool {
        *self == SRAM31_IF_A::SRAM031_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM031_IF_1`"]
    #[inline(always)]
    pub fn is_sram031_if_1(&self) -> bool {
        *self == SRAM31_IF_A::SRAM031_IF_1
    }
}
#[doc = "Field `SRAM31_IF` writer - SRAM31_IF"]
pub type SRAM31_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE1_SPEC, SRAM31_IF_A, O>;
impl<'a, const O: u8> SRAM31_IF_W<'a, O> {
    #[doc = "Enable clock gating"]
    #[inline(always)]
    pub fn sram031_if_0(self) -> &'a mut W {
        self.variant(SRAM31_IF_A::SRAM031_IF_0)
    }
    #[doc = "Continuous Clocking"]
    #[inline(always)]
    pub fn sram031_if_1(self) -> &'a mut W {
        self.variant(SRAM31_IF_A::SRAM031_IF_1)
    }
}
impl R {
    #[doc = "Bit 0 - SRAM0_IF"]
    #[inline(always)]
    pub fn sram0_if(&self) -> SRAM0_IF_R {
        SRAM0_IF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM1_IF"]
    #[inline(always)]
    pub fn sram1_if(&self) -> SRAM1_IF_R {
        SRAM1_IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM2_IF"]
    #[inline(always)]
    pub fn sram2_if(&self) -> SRAM2_IF_R {
        SRAM2_IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SRAM3_IF"]
    #[inline(always)]
    pub fn sram3_if(&self) -> SRAM3_IF_R {
        SRAM3_IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SRAM4_IF"]
    #[inline(always)]
    pub fn sram4_if(&self) -> SRAM4_IF_R {
        SRAM4_IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SRAM5_IF"]
    #[inline(always)]
    pub fn sram5_if(&self) -> SRAM5_IF_R {
        SRAM5_IF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SRAM6_IF"]
    #[inline(always)]
    pub fn sram6_if(&self) -> SRAM6_IF_R {
        SRAM6_IF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SRAM7_IF"]
    #[inline(always)]
    pub fn sram7_if(&self) -> SRAM7_IF_R {
        SRAM7_IF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM8_IF"]
    #[inline(always)]
    pub fn sram8_if(&self) -> SRAM8_IF_R {
        SRAM8_IF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM9_IF"]
    #[inline(always)]
    pub fn sram9_if(&self) -> SRAM9_IF_R {
        SRAM9_IF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SRAM10_IF"]
    #[inline(always)]
    pub fn sram10_if(&self) -> SRAM10_IF_R {
        SRAM10_IF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SRAM11_IF"]
    #[inline(always)]
    pub fn sram11_if(&self) -> SRAM11_IF_R {
        SRAM11_IF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SRAM12_IF"]
    #[inline(always)]
    pub fn sram12_if(&self) -> SRAM12_IF_R {
        SRAM12_IF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SRAM13_IF"]
    #[inline(always)]
    pub fn sram13_if(&self) -> SRAM13_IF_R {
        SRAM13_IF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SRAM14_IF"]
    #[inline(always)]
    pub fn sram14_if(&self) -> SRAM14_IF_R {
        SRAM14_IF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SRAM15_IF"]
    #[inline(always)]
    pub fn sram15_if(&self) -> SRAM15_IF_R {
        SRAM15_IF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SRAM16_IF"]
    #[inline(always)]
    pub fn sram16_if(&self) -> SRAM16_IF_R {
        SRAM16_IF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SRAM17_IF"]
    #[inline(always)]
    pub fn sram17_if(&self) -> SRAM17_IF_R {
        SRAM17_IF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SRAM18_IF"]
    #[inline(always)]
    pub fn sram18_if(&self) -> SRAM18_IF_R {
        SRAM18_IF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SRAM19_IF"]
    #[inline(always)]
    pub fn sram19_if(&self) -> SRAM19_IF_R {
        SRAM19_IF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SRAM20_IF"]
    #[inline(always)]
    pub fn sram20_if(&self) -> SRAM20_IF_R {
        SRAM20_IF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SRAM21_IF"]
    #[inline(always)]
    pub fn sram21_if(&self) -> SRAM21_IF_R {
        SRAM21_IF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SRAM22_IF"]
    #[inline(always)]
    pub fn sram22_if(&self) -> SRAM22_IF_R {
        SRAM22_IF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SRAM23_IF"]
    #[inline(always)]
    pub fn sram23_if(&self) -> SRAM23_IF_R {
        SRAM23_IF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SRAM24_IF"]
    #[inline(always)]
    pub fn sram24_if(&self) -> SRAM24_IF_R {
        SRAM24_IF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SRAM25_IF"]
    #[inline(always)]
    pub fn sram25_if(&self) -> SRAM25_IF_R {
        SRAM25_IF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SRAM26_IF"]
    #[inline(always)]
    pub fn sram26_if(&self) -> SRAM26_IF_R {
        SRAM26_IF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - SRAM27_IF"]
    #[inline(always)]
    pub fn sram27_if(&self) -> SRAM27_IF_R {
        SRAM27_IF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - SRAM28_IF"]
    #[inline(always)]
    pub fn sram28_if(&self) -> SRAM28_IF_R {
        SRAM28_IF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SRAM29_IF"]
    #[inline(always)]
    pub fn sram29_if(&self) -> SRAM29_IF_R {
        SRAM29_IF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SRAM30_IF"]
    #[inline(always)]
    pub fn sram30_if(&self) -> SRAM30_IF_R {
        SRAM30_IF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SRAM31_IF"]
    #[inline(always)]
    pub fn sram31_if(&self) -> SRAM31_IF_R {
        SRAM31_IF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM0_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram0_if(&mut self) -> SRAM0_IF_W<0> {
        SRAM0_IF_W::new(self)
    }
    #[doc = "Bit 1 - SRAM1_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram1_if(&mut self) -> SRAM1_IF_W<1> {
        SRAM1_IF_W::new(self)
    }
    #[doc = "Bit 2 - SRAM2_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram2_if(&mut self) -> SRAM2_IF_W<2> {
        SRAM2_IF_W::new(self)
    }
    #[doc = "Bit 3 - SRAM3_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram3_if(&mut self) -> SRAM3_IF_W<3> {
        SRAM3_IF_W::new(self)
    }
    #[doc = "Bit 4 - SRAM4_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram4_if(&mut self) -> SRAM4_IF_W<4> {
        SRAM4_IF_W::new(self)
    }
    #[doc = "Bit 5 - SRAM5_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram5_if(&mut self) -> SRAM5_IF_W<5> {
        SRAM5_IF_W::new(self)
    }
    #[doc = "Bit 6 - SRAM6_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram6_if(&mut self) -> SRAM6_IF_W<6> {
        SRAM6_IF_W::new(self)
    }
    #[doc = "Bit 7 - SRAM7_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram7_if(&mut self) -> SRAM7_IF_W<7> {
        SRAM7_IF_W::new(self)
    }
    #[doc = "Bit 8 - SRAM8_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram8_if(&mut self) -> SRAM8_IF_W<8> {
        SRAM8_IF_W::new(self)
    }
    #[doc = "Bit 9 - SRAM9_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram9_if(&mut self) -> SRAM9_IF_W<9> {
        SRAM9_IF_W::new(self)
    }
    #[doc = "Bit 10 - SRAM10_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram10_if(&mut self) -> SRAM10_IF_W<10> {
        SRAM10_IF_W::new(self)
    }
    #[doc = "Bit 11 - SRAM11_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram11_if(&mut self) -> SRAM11_IF_W<11> {
        SRAM11_IF_W::new(self)
    }
    #[doc = "Bit 12 - SRAM12_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram12_if(&mut self) -> SRAM12_IF_W<12> {
        SRAM12_IF_W::new(self)
    }
    #[doc = "Bit 13 - SRAM13_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram13_if(&mut self) -> SRAM13_IF_W<13> {
        SRAM13_IF_W::new(self)
    }
    #[doc = "Bit 14 - SRAM14_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram14_if(&mut self) -> SRAM14_IF_W<14> {
        SRAM14_IF_W::new(self)
    }
    #[doc = "Bit 15 - SRAM15_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram15_if(&mut self) -> SRAM15_IF_W<15> {
        SRAM15_IF_W::new(self)
    }
    #[doc = "Bit 16 - SRAM16_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram16_if(&mut self) -> SRAM16_IF_W<16> {
        SRAM16_IF_W::new(self)
    }
    #[doc = "Bit 17 - SRAM17_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram17_if(&mut self) -> SRAM17_IF_W<17> {
        SRAM17_IF_W::new(self)
    }
    #[doc = "Bit 18 - SRAM18_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram18_if(&mut self) -> SRAM18_IF_W<18> {
        SRAM18_IF_W::new(self)
    }
    #[doc = "Bit 19 - SRAM19_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram19_if(&mut self) -> SRAM19_IF_W<19> {
        SRAM19_IF_W::new(self)
    }
    #[doc = "Bit 20 - SRAM20_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram20_if(&mut self) -> SRAM20_IF_W<20> {
        SRAM20_IF_W::new(self)
    }
    #[doc = "Bit 21 - SRAM21_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram21_if(&mut self) -> SRAM21_IF_W<21> {
        SRAM21_IF_W::new(self)
    }
    #[doc = "Bit 22 - SRAM22_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram22_if(&mut self) -> SRAM22_IF_W<22> {
        SRAM22_IF_W::new(self)
    }
    #[doc = "Bit 23 - SRAM23_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram23_if(&mut self) -> SRAM23_IF_W<23> {
        SRAM23_IF_W::new(self)
    }
    #[doc = "Bit 24 - SRAM24_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram24_if(&mut self) -> SRAM24_IF_W<24> {
        SRAM24_IF_W::new(self)
    }
    #[doc = "Bit 25 - SRAM25_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram25_if(&mut self) -> SRAM25_IF_W<25> {
        SRAM25_IF_W::new(self)
    }
    #[doc = "Bit 26 - SRAM26_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram26_if(&mut self) -> SRAM26_IF_W<26> {
        SRAM26_IF_W::new(self)
    }
    #[doc = "Bit 27 - SRAM27_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram27_if(&mut self) -> SRAM27_IF_W<27> {
        SRAM27_IF_W::new(self)
    }
    #[doc = "Bit 28 - SRAM28_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram28_if(&mut self) -> SRAM28_IF_W<28> {
        SRAM28_IF_W::new(self)
    }
    #[doc = "Bit 29 - SRAM29_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram29_if(&mut self) -> SRAM29_IF_W<29> {
        SRAM29_IF_W::new(self)
    }
    #[doc = "Bit 30 - SRAM30_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram30_if(&mut self) -> SRAM30_IF_W<30> {
        SRAM30_IF_W::new(self)
    }
    #[doc = "Bit 31 - SRAM31_IF"]
    #[inline(always)]
    #[must_use]
    pub fn sram31_if(&mut self) -> SRAM31_IF_W<31> {
        SRAM31_IF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auto clock gate override 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [autoclkgateoverride1](index.html) module"]
pub struct AUTOCLKGATEOVERRIDE1_SPEC;
impl crate::RegisterSpec for AUTOCLKGATEOVERRIDE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [autoclkgateoverride1::R](R) reader structure"]
impl crate::Readable for AUTOCLKGATEOVERRIDE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [autoclkgateoverride1::W](W) writer structure"]
impl crate::Writable for AUTOCLKGATEOVERRIDE1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AUTOCLKGATEOVERRIDE1 to value 0xffff_ffff"]
impl crate::Resettable for AUTOCLKGATEOVERRIDE1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
