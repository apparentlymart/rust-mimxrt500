#[doc = "Register `AHB_SRAM_ACCESS_DISABLE` reader"]
pub struct R(crate::R<AHB_SRAM_ACCESS_DISABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB_SRAM_ACCESS_DISABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB_SRAM_ACCESS_DISABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB_SRAM_ACCESS_DISABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB_SRAM_ACCESS_DISABLE` writer"]
pub struct W(crate::W<AHB_SRAM_ACCESS_DISABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB_SRAM_ACCESS_DISABLE_SPEC>;
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
impl From<crate::W<AHB_SRAM_ACCESS_DISABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB_SRAM_ACCESS_DISABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM00_IF` reader - Control AHB access to SRAM partition 0"]
pub type SRAM00_IF_R = crate::BitReader<SRAM00_IF_A>;
#[doc = "Control AHB access to SRAM partition 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM00_IF_A {
    #[doc = "0: Enable"]
    SRAM00_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM00_IF_1 = 1,
}
impl From<SRAM00_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM00_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM00_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM00_IF_A {
        match self.bits {
            false => SRAM00_IF_A::SRAM00_IF_0,
            true => SRAM00_IF_A::SRAM00_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM00_IF_0`"]
    #[inline(always)]
    pub fn is_sram00_if_0(&self) -> bool {
        *self == SRAM00_IF_A::SRAM00_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM00_IF_1`"]
    #[inline(always)]
    pub fn is_sram00_if_1(&self) -> bool {
        *self == SRAM00_IF_A::SRAM00_IF_1
    }
}
#[doc = "Field `SRAM00_IF` writer - Control AHB access to SRAM partition 0"]
pub type SRAM00_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM00_IF_A, O>;
impl<'a, const O: u8> SRAM00_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram00_if_0(self) -> &'a mut W {
        self.variant(SRAM00_IF_A::SRAM00_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram00_if_1(self) -> &'a mut W {
        self.variant(SRAM00_IF_A::SRAM00_IF_1)
    }
}
#[doc = "Field `SRAM01_IF` reader - Control AHB access to SRAM partition 1"]
pub type SRAM01_IF_R = crate::BitReader<SRAM01_IF_A>;
#[doc = "Control AHB access to SRAM partition 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM01_IF_A {
    #[doc = "0: Enable"]
    SRAM01_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM01_IF_1 = 1,
}
impl From<SRAM01_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM01_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM01_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM01_IF_A {
        match self.bits {
            false => SRAM01_IF_A::SRAM01_IF_0,
            true => SRAM01_IF_A::SRAM01_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM01_IF_0`"]
    #[inline(always)]
    pub fn is_sram01_if_0(&self) -> bool {
        *self == SRAM01_IF_A::SRAM01_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM01_IF_1`"]
    #[inline(always)]
    pub fn is_sram01_if_1(&self) -> bool {
        *self == SRAM01_IF_A::SRAM01_IF_1
    }
}
#[doc = "Field `SRAM01_IF` writer - Control AHB access to SRAM partition 1"]
pub type SRAM01_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM01_IF_A, O>;
impl<'a, const O: u8> SRAM01_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram01_if_0(self) -> &'a mut W {
        self.variant(SRAM01_IF_A::SRAM01_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram01_if_1(self) -> &'a mut W {
        self.variant(SRAM01_IF_A::SRAM01_IF_1)
    }
}
#[doc = "Field `SRAM02_IF` reader - Control AHB access to SRAM partition 2"]
pub type SRAM02_IF_R = crate::BitReader<SRAM02_IF_A>;
#[doc = "Control AHB access to SRAM partition 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM02_IF_A {
    #[doc = "0: Enable"]
    SRAM02_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM02_IF_1 = 1,
}
impl From<SRAM02_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM02_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM02_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM02_IF_A {
        match self.bits {
            false => SRAM02_IF_A::SRAM02_IF_0,
            true => SRAM02_IF_A::SRAM02_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM02_IF_0`"]
    #[inline(always)]
    pub fn is_sram02_if_0(&self) -> bool {
        *self == SRAM02_IF_A::SRAM02_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM02_IF_1`"]
    #[inline(always)]
    pub fn is_sram02_if_1(&self) -> bool {
        *self == SRAM02_IF_A::SRAM02_IF_1
    }
}
#[doc = "Field `SRAM02_IF` writer - Control AHB access to SRAM partition 2"]
pub type SRAM02_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM02_IF_A, O>;
impl<'a, const O: u8> SRAM02_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram02_if_0(self) -> &'a mut W {
        self.variant(SRAM02_IF_A::SRAM02_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram02_if_1(self) -> &'a mut W {
        self.variant(SRAM02_IF_A::SRAM02_IF_1)
    }
}
#[doc = "Field `SRAM03_IF` reader - Control AHB access to SRAM partition 3"]
pub type SRAM03_IF_R = crate::BitReader<SRAM03_IF_A>;
#[doc = "Control AHB access to SRAM partition 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM03_IF_A {
    #[doc = "0: Enable"]
    SRAM03_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM03_IF_1 = 1,
}
impl From<SRAM03_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM03_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM03_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM03_IF_A {
        match self.bits {
            false => SRAM03_IF_A::SRAM03_IF_0,
            true => SRAM03_IF_A::SRAM03_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM03_IF_0`"]
    #[inline(always)]
    pub fn is_sram03_if_0(&self) -> bool {
        *self == SRAM03_IF_A::SRAM03_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM03_IF_1`"]
    #[inline(always)]
    pub fn is_sram03_if_1(&self) -> bool {
        *self == SRAM03_IF_A::SRAM03_IF_1
    }
}
#[doc = "Field `SRAM03_IF` writer - Control AHB access to SRAM partition 3"]
pub type SRAM03_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM03_IF_A, O>;
impl<'a, const O: u8> SRAM03_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram03_if_0(self) -> &'a mut W {
        self.variant(SRAM03_IF_A::SRAM03_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram03_if_1(self) -> &'a mut W {
        self.variant(SRAM03_IF_A::SRAM03_IF_1)
    }
}
#[doc = "Field `SRAM04_IF` reader - Control AHB access to SRAM partition 4"]
pub type SRAM04_IF_R = crate::BitReader<SRAM04_IF_A>;
#[doc = "Control AHB access to SRAM partition 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM04_IF_A {
    #[doc = "0: Enable"]
    SRAM04_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM04_IF_1 = 1,
}
impl From<SRAM04_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM04_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM04_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM04_IF_A {
        match self.bits {
            false => SRAM04_IF_A::SRAM04_IF_0,
            true => SRAM04_IF_A::SRAM04_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM04_IF_0`"]
    #[inline(always)]
    pub fn is_sram04_if_0(&self) -> bool {
        *self == SRAM04_IF_A::SRAM04_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM04_IF_1`"]
    #[inline(always)]
    pub fn is_sram04_if_1(&self) -> bool {
        *self == SRAM04_IF_A::SRAM04_IF_1
    }
}
#[doc = "Field `SRAM04_IF` writer - Control AHB access to SRAM partition 4"]
pub type SRAM04_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM04_IF_A, O>;
impl<'a, const O: u8> SRAM04_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram04_if_0(self) -> &'a mut W {
        self.variant(SRAM04_IF_A::SRAM04_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram04_if_1(self) -> &'a mut W {
        self.variant(SRAM04_IF_A::SRAM04_IF_1)
    }
}
#[doc = "Field `SRAM05_IF` reader - Control AHB access to SRAM partition 5"]
pub type SRAM05_IF_R = crate::BitReader<SRAM05_IF_A>;
#[doc = "Control AHB access to SRAM partition 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM05_IF_A {
    #[doc = "0: Enable"]
    SRAM05_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM05_IF_1 = 1,
}
impl From<SRAM05_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM05_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM05_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM05_IF_A {
        match self.bits {
            false => SRAM05_IF_A::SRAM05_IF_0,
            true => SRAM05_IF_A::SRAM05_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM05_IF_0`"]
    #[inline(always)]
    pub fn is_sram05_if_0(&self) -> bool {
        *self == SRAM05_IF_A::SRAM05_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM05_IF_1`"]
    #[inline(always)]
    pub fn is_sram05_if_1(&self) -> bool {
        *self == SRAM05_IF_A::SRAM05_IF_1
    }
}
#[doc = "Field `SRAM05_IF` writer - Control AHB access to SRAM partition 5"]
pub type SRAM05_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM05_IF_A, O>;
impl<'a, const O: u8> SRAM05_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram05_if_0(self) -> &'a mut W {
        self.variant(SRAM05_IF_A::SRAM05_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram05_if_1(self) -> &'a mut W {
        self.variant(SRAM05_IF_A::SRAM05_IF_1)
    }
}
#[doc = "Field `SRAM06_IF` reader - Control AHB access to SRAM partition 6"]
pub type SRAM06_IF_R = crate::BitReader<SRAM06_IF_A>;
#[doc = "Control AHB access to SRAM partition 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM06_IF_A {
    #[doc = "0: Enable"]
    SRAM06_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM06_IF_1 = 1,
}
impl From<SRAM06_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM06_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM06_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM06_IF_A {
        match self.bits {
            false => SRAM06_IF_A::SRAM06_IF_0,
            true => SRAM06_IF_A::SRAM06_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM06_IF_0`"]
    #[inline(always)]
    pub fn is_sram06_if_0(&self) -> bool {
        *self == SRAM06_IF_A::SRAM06_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM06_IF_1`"]
    #[inline(always)]
    pub fn is_sram06_if_1(&self) -> bool {
        *self == SRAM06_IF_A::SRAM06_IF_1
    }
}
#[doc = "Field `SRAM06_IF` writer - Control AHB access to SRAM partition 6"]
pub type SRAM06_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM06_IF_A, O>;
impl<'a, const O: u8> SRAM06_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram06_if_0(self) -> &'a mut W {
        self.variant(SRAM06_IF_A::SRAM06_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram06_if_1(self) -> &'a mut W {
        self.variant(SRAM06_IF_A::SRAM06_IF_1)
    }
}
#[doc = "Field `SRAM07_IF` reader - Control AHB access to SRAM partition 7"]
pub type SRAM07_IF_R = crate::BitReader<SRAM07_IF_A>;
#[doc = "Control AHB access to SRAM partition 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM07_IF_A {
    #[doc = "0: Enable"]
    SRAM07_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM07_IF_1 = 1,
}
impl From<SRAM07_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM07_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM07_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM07_IF_A {
        match self.bits {
            false => SRAM07_IF_A::SRAM07_IF_0,
            true => SRAM07_IF_A::SRAM07_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM07_IF_0`"]
    #[inline(always)]
    pub fn is_sram07_if_0(&self) -> bool {
        *self == SRAM07_IF_A::SRAM07_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM07_IF_1`"]
    #[inline(always)]
    pub fn is_sram07_if_1(&self) -> bool {
        *self == SRAM07_IF_A::SRAM07_IF_1
    }
}
#[doc = "Field `SRAM07_IF` writer - Control AHB access to SRAM partition 7"]
pub type SRAM07_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM07_IF_A, O>;
impl<'a, const O: u8> SRAM07_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram07_if_0(self) -> &'a mut W {
        self.variant(SRAM07_IF_A::SRAM07_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram07_if_1(self) -> &'a mut W {
        self.variant(SRAM07_IF_A::SRAM07_IF_1)
    }
}
#[doc = "Field `SRAM08_IF` reader - Control AHB access to SRAM partition 8"]
pub type SRAM08_IF_R = crate::BitReader<SRAM08_IF_A>;
#[doc = "Control AHB access to SRAM partition 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM08_IF_A {
    #[doc = "0: Enable"]
    SRAM08_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM08_IF_1 = 1,
}
impl From<SRAM08_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM08_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM08_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM08_IF_A {
        match self.bits {
            false => SRAM08_IF_A::SRAM08_IF_0,
            true => SRAM08_IF_A::SRAM08_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM08_IF_0`"]
    #[inline(always)]
    pub fn is_sram08_if_0(&self) -> bool {
        *self == SRAM08_IF_A::SRAM08_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM08_IF_1`"]
    #[inline(always)]
    pub fn is_sram08_if_1(&self) -> bool {
        *self == SRAM08_IF_A::SRAM08_IF_1
    }
}
#[doc = "Field `SRAM08_IF` writer - Control AHB access to SRAM partition 8"]
pub type SRAM08_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM08_IF_A, O>;
impl<'a, const O: u8> SRAM08_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram08_if_0(self) -> &'a mut W {
        self.variant(SRAM08_IF_A::SRAM08_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram08_if_1(self) -> &'a mut W {
        self.variant(SRAM08_IF_A::SRAM08_IF_1)
    }
}
#[doc = "Field `SRAM09_IF` reader - Control AHB access to SRAM partition 9"]
pub type SRAM09_IF_R = crate::BitReader<SRAM09_IF_A>;
#[doc = "Control AHB access to SRAM partition 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM09_IF_A {
    #[doc = "0: Enable"]
    SRAM09_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM09_IF_1 = 1,
}
impl From<SRAM09_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM09_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM09_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM09_IF_A {
        match self.bits {
            false => SRAM09_IF_A::SRAM09_IF_0,
            true => SRAM09_IF_A::SRAM09_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM09_IF_0`"]
    #[inline(always)]
    pub fn is_sram09_if_0(&self) -> bool {
        *self == SRAM09_IF_A::SRAM09_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM09_IF_1`"]
    #[inline(always)]
    pub fn is_sram09_if_1(&self) -> bool {
        *self == SRAM09_IF_A::SRAM09_IF_1
    }
}
#[doc = "Field `SRAM09_IF` writer - Control AHB access to SRAM partition 9"]
pub type SRAM09_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM09_IF_A, O>;
impl<'a, const O: u8> SRAM09_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram09_if_0(self) -> &'a mut W {
        self.variant(SRAM09_IF_A::SRAM09_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram09_if_1(self) -> &'a mut W {
        self.variant(SRAM09_IF_A::SRAM09_IF_1)
    }
}
#[doc = "Field `SRAM010_IF` reader - Control AHB access to SRAM partition 10"]
pub type SRAM010_IF_R = crate::BitReader<SRAM010_IF_A>;
#[doc = "Control AHB access to SRAM partition 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM010_IF_A {
    #[doc = "0: Enable"]
    SRAM010_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM010_IF_1 = 1,
}
impl From<SRAM010_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM010_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM010_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM010_IF_A {
        match self.bits {
            false => SRAM010_IF_A::SRAM010_IF_0,
            true => SRAM010_IF_A::SRAM010_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM010_IF_0`"]
    #[inline(always)]
    pub fn is_sram010_if_0(&self) -> bool {
        *self == SRAM010_IF_A::SRAM010_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM010_IF_1`"]
    #[inline(always)]
    pub fn is_sram010_if_1(&self) -> bool {
        *self == SRAM010_IF_A::SRAM010_IF_1
    }
}
#[doc = "Field `SRAM010_IF` writer - Control AHB access to SRAM partition 10"]
pub type SRAM010_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM010_IF_A, O>;
impl<'a, const O: u8> SRAM010_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram010_if_0(self) -> &'a mut W {
        self.variant(SRAM010_IF_A::SRAM010_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram010_if_1(self) -> &'a mut W {
        self.variant(SRAM010_IF_A::SRAM010_IF_1)
    }
}
#[doc = "Field `SRAM011_IF` reader - Control AHB access to SRAM partition 11"]
pub type SRAM011_IF_R = crate::BitReader<SRAM011_IF_A>;
#[doc = "Control AHB access to SRAM partition 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM011_IF_A {
    #[doc = "0: Enable"]
    SRAM011_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM011_IF_1 = 1,
}
impl From<SRAM011_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM011_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM011_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM011_IF_A {
        match self.bits {
            false => SRAM011_IF_A::SRAM011_IF_0,
            true => SRAM011_IF_A::SRAM011_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM011_IF_0`"]
    #[inline(always)]
    pub fn is_sram011_if_0(&self) -> bool {
        *self == SRAM011_IF_A::SRAM011_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM011_IF_1`"]
    #[inline(always)]
    pub fn is_sram011_if_1(&self) -> bool {
        *self == SRAM011_IF_A::SRAM011_IF_1
    }
}
#[doc = "Field `SRAM011_IF` writer - Control AHB access to SRAM partition 11"]
pub type SRAM011_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM011_IF_A, O>;
impl<'a, const O: u8> SRAM011_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram011_if_0(self) -> &'a mut W {
        self.variant(SRAM011_IF_A::SRAM011_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram011_if_1(self) -> &'a mut W {
        self.variant(SRAM011_IF_A::SRAM011_IF_1)
    }
}
#[doc = "Field `SRAM012_IF` reader - Control AHB access to SRAM partition 12"]
pub type SRAM012_IF_R = crate::BitReader<SRAM012_IF_A>;
#[doc = "Control AHB access to SRAM partition 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM012_IF_A {
    #[doc = "0: Enable"]
    SRAM012_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM012_IF_1 = 1,
}
impl From<SRAM012_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM012_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM012_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM012_IF_A {
        match self.bits {
            false => SRAM012_IF_A::SRAM012_IF_0,
            true => SRAM012_IF_A::SRAM012_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM012_IF_0`"]
    #[inline(always)]
    pub fn is_sram012_if_0(&self) -> bool {
        *self == SRAM012_IF_A::SRAM012_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM012_IF_1`"]
    #[inline(always)]
    pub fn is_sram012_if_1(&self) -> bool {
        *self == SRAM012_IF_A::SRAM012_IF_1
    }
}
#[doc = "Field `SRAM012_IF` writer - Control AHB access to SRAM partition 12"]
pub type SRAM012_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM012_IF_A, O>;
impl<'a, const O: u8> SRAM012_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram012_if_0(self) -> &'a mut W {
        self.variant(SRAM012_IF_A::SRAM012_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram012_if_1(self) -> &'a mut W {
        self.variant(SRAM012_IF_A::SRAM012_IF_1)
    }
}
#[doc = "Field `SRAM013_IF` reader - Control AHB access to SRAM partition 13"]
pub type SRAM013_IF_R = crate::BitReader<SRAM013_IF_A>;
#[doc = "Control AHB access to SRAM partition 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM013_IF_A {
    #[doc = "0: Enable"]
    SRAM013_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM013_IF_1 = 1,
}
impl From<SRAM013_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM013_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM013_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM013_IF_A {
        match self.bits {
            false => SRAM013_IF_A::SRAM013_IF_0,
            true => SRAM013_IF_A::SRAM013_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM013_IF_0`"]
    #[inline(always)]
    pub fn is_sram013_if_0(&self) -> bool {
        *self == SRAM013_IF_A::SRAM013_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM013_IF_1`"]
    #[inline(always)]
    pub fn is_sram013_if_1(&self) -> bool {
        *self == SRAM013_IF_A::SRAM013_IF_1
    }
}
#[doc = "Field `SRAM013_IF` writer - Control AHB access to SRAM partition 13"]
pub type SRAM013_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM013_IF_A, O>;
impl<'a, const O: u8> SRAM013_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram013_if_0(self) -> &'a mut W {
        self.variant(SRAM013_IF_A::SRAM013_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram013_if_1(self) -> &'a mut W {
        self.variant(SRAM013_IF_A::SRAM013_IF_1)
    }
}
#[doc = "Field `SRAM014_IF` reader - Control AHB access to SRAM partition 14"]
pub type SRAM014_IF_R = crate::BitReader<SRAM014_IF_A>;
#[doc = "Control AHB access to SRAM partition 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM014_IF_A {
    #[doc = "0: Enable"]
    SRAM014_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM014_IF_1 = 1,
}
impl From<SRAM014_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM014_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM014_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM014_IF_A {
        match self.bits {
            false => SRAM014_IF_A::SRAM014_IF_0,
            true => SRAM014_IF_A::SRAM014_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM014_IF_0`"]
    #[inline(always)]
    pub fn is_sram014_if_0(&self) -> bool {
        *self == SRAM014_IF_A::SRAM014_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM014_IF_1`"]
    #[inline(always)]
    pub fn is_sram014_if_1(&self) -> bool {
        *self == SRAM014_IF_A::SRAM014_IF_1
    }
}
#[doc = "Field `SRAM014_IF` writer - Control AHB access to SRAM partition 14"]
pub type SRAM014_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM014_IF_A, O>;
impl<'a, const O: u8> SRAM014_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram014_if_0(self) -> &'a mut W {
        self.variant(SRAM014_IF_A::SRAM014_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram014_if_1(self) -> &'a mut W {
        self.variant(SRAM014_IF_A::SRAM014_IF_1)
    }
}
#[doc = "Field `SRAM015_IF` reader - Control AHB access to SRAM partition 15"]
pub type SRAM015_IF_R = crate::BitReader<SRAM015_IF_A>;
#[doc = "Control AHB access to SRAM partition 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM015_IF_A {
    #[doc = "0: Enable"]
    SRAM015_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM015_IF_1 = 1,
}
impl From<SRAM015_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM015_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM015_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM015_IF_A {
        match self.bits {
            false => SRAM015_IF_A::SRAM015_IF_0,
            true => SRAM015_IF_A::SRAM015_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM015_IF_0`"]
    #[inline(always)]
    pub fn is_sram015_if_0(&self) -> bool {
        *self == SRAM015_IF_A::SRAM015_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM015_IF_1`"]
    #[inline(always)]
    pub fn is_sram015_if_1(&self) -> bool {
        *self == SRAM015_IF_A::SRAM015_IF_1
    }
}
#[doc = "Field `SRAM015_IF` writer - Control AHB access to SRAM partition 15"]
pub type SRAM015_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM015_IF_A, O>;
impl<'a, const O: u8> SRAM015_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram015_if_0(self) -> &'a mut W {
        self.variant(SRAM015_IF_A::SRAM015_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram015_if_1(self) -> &'a mut W {
        self.variant(SRAM015_IF_A::SRAM015_IF_1)
    }
}
#[doc = "Field `SRAM016_IF` reader - Control AHB access to SRAM partition 16"]
pub type SRAM016_IF_R = crate::BitReader<SRAM016_IF_A>;
#[doc = "Control AHB access to SRAM partition 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM016_IF_A {
    #[doc = "0: Enable"]
    SRAM016_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM016_IF_1 = 1,
}
impl From<SRAM016_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM016_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM016_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM016_IF_A {
        match self.bits {
            false => SRAM016_IF_A::SRAM016_IF_0,
            true => SRAM016_IF_A::SRAM016_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM016_IF_0`"]
    #[inline(always)]
    pub fn is_sram016_if_0(&self) -> bool {
        *self == SRAM016_IF_A::SRAM016_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM016_IF_1`"]
    #[inline(always)]
    pub fn is_sram016_if_1(&self) -> bool {
        *self == SRAM016_IF_A::SRAM016_IF_1
    }
}
#[doc = "Field `SRAM016_IF` writer - Control AHB access to SRAM partition 16"]
pub type SRAM016_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM016_IF_A, O>;
impl<'a, const O: u8> SRAM016_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram016_if_0(self) -> &'a mut W {
        self.variant(SRAM016_IF_A::SRAM016_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram016_if_1(self) -> &'a mut W {
        self.variant(SRAM016_IF_A::SRAM016_IF_1)
    }
}
#[doc = "Field `SRAM017_IF` reader - Control AHB access to SRAM partition 17"]
pub type SRAM017_IF_R = crate::BitReader<SRAM017_IF_A>;
#[doc = "Control AHB access to SRAM partition 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM017_IF_A {
    #[doc = "0: Enable"]
    SRAM017_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM017_IF_1 = 1,
}
impl From<SRAM017_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM017_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM017_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM017_IF_A {
        match self.bits {
            false => SRAM017_IF_A::SRAM017_IF_0,
            true => SRAM017_IF_A::SRAM017_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM017_IF_0`"]
    #[inline(always)]
    pub fn is_sram017_if_0(&self) -> bool {
        *self == SRAM017_IF_A::SRAM017_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM017_IF_1`"]
    #[inline(always)]
    pub fn is_sram017_if_1(&self) -> bool {
        *self == SRAM017_IF_A::SRAM017_IF_1
    }
}
#[doc = "Field `SRAM017_IF` writer - Control AHB access to SRAM partition 17"]
pub type SRAM017_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM017_IF_A, O>;
impl<'a, const O: u8> SRAM017_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram017_if_0(self) -> &'a mut W {
        self.variant(SRAM017_IF_A::SRAM017_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram017_if_1(self) -> &'a mut W {
        self.variant(SRAM017_IF_A::SRAM017_IF_1)
    }
}
#[doc = "Field `SRAM018_IF` reader - Control AHB access to SRAM partition 18"]
pub type SRAM018_IF_R = crate::BitReader<SRAM018_IF_A>;
#[doc = "Control AHB access to SRAM partition 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM018_IF_A {
    #[doc = "0: Enable"]
    SRAM018_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM018_IF_1 = 1,
}
impl From<SRAM018_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM018_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM018_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM018_IF_A {
        match self.bits {
            false => SRAM018_IF_A::SRAM018_IF_0,
            true => SRAM018_IF_A::SRAM018_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM018_IF_0`"]
    #[inline(always)]
    pub fn is_sram018_if_0(&self) -> bool {
        *self == SRAM018_IF_A::SRAM018_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM018_IF_1`"]
    #[inline(always)]
    pub fn is_sram018_if_1(&self) -> bool {
        *self == SRAM018_IF_A::SRAM018_IF_1
    }
}
#[doc = "Field `SRAM018_IF` writer - Control AHB access to SRAM partition 18"]
pub type SRAM018_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM018_IF_A, O>;
impl<'a, const O: u8> SRAM018_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram018_if_0(self) -> &'a mut W {
        self.variant(SRAM018_IF_A::SRAM018_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram018_if_1(self) -> &'a mut W {
        self.variant(SRAM018_IF_A::SRAM018_IF_1)
    }
}
#[doc = "Field `SRAM019_IF` reader - Control AHB access to SRAM partition 19"]
pub type SRAM019_IF_R = crate::BitReader<SRAM019_IF_A>;
#[doc = "Control AHB access to SRAM partition 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM019_IF_A {
    #[doc = "0: Enable"]
    SRAM019_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM019_IF_1 = 1,
}
impl From<SRAM019_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM019_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM019_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM019_IF_A {
        match self.bits {
            false => SRAM019_IF_A::SRAM019_IF_0,
            true => SRAM019_IF_A::SRAM019_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM019_IF_0`"]
    #[inline(always)]
    pub fn is_sram019_if_0(&self) -> bool {
        *self == SRAM019_IF_A::SRAM019_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM019_IF_1`"]
    #[inline(always)]
    pub fn is_sram019_if_1(&self) -> bool {
        *self == SRAM019_IF_A::SRAM019_IF_1
    }
}
#[doc = "Field `SRAM019_IF` writer - Control AHB access to SRAM partition 19"]
pub type SRAM019_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM019_IF_A, O>;
impl<'a, const O: u8> SRAM019_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram019_if_0(self) -> &'a mut W {
        self.variant(SRAM019_IF_A::SRAM019_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram019_if_1(self) -> &'a mut W {
        self.variant(SRAM019_IF_A::SRAM019_IF_1)
    }
}
#[doc = "Field `SRAM020_IF` reader - Control AHB access to SRAM partition 20"]
pub type SRAM020_IF_R = crate::BitReader<SRAM020_IF_A>;
#[doc = "Control AHB access to SRAM partition 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM020_IF_A {
    #[doc = "0: Enable"]
    SRAM020_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM020_IF_1 = 1,
}
impl From<SRAM020_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM020_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM020_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM020_IF_A {
        match self.bits {
            false => SRAM020_IF_A::SRAM020_IF_0,
            true => SRAM020_IF_A::SRAM020_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM020_IF_0`"]
    #[inline(always)]
    pub fn is_sram020_if_0(&self) -> bool {
        *self == SRAM020_IF_A::SRAM020_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM020_IF_1`"]
    #[inline(always)]
    pub fn is_sram020_if_1(&self) -> bool {
        *self == SRAM020_IF_A::SRAM020_IF_1
    }
}
#[doc = "Field `SRAM020_IF` writer - Control AHB access to SRAM partition 20"]
pub type SRAM020_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM020_IF_A, O>;
impl<'a, const O: u8> SRAM020_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram020_if_0(self) -> &'a mut W {
        self.variant(SRAM020_IF_A::SRAM020_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram020_if_1(self) -> &'a mut W {
        self.variant(SRAM020_IF_A::SRAM020_IF_1)
    }
}
#[doc = "Field `SRAM021_IF` reader - Control AHB access to SRAM partition 21"]
pub type SRAM021_IF_R = crate::BitReader<SRAM021_IF_A>;
#[doc = "Control AHB access to SRAM partition 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM021_IF_A {
    #[doc = "0: Enable"]
    SRAM021_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM021_IF_1 = 1,
}
impl From<SRAM021_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM021_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM021_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM021_IF_A {
        match self.bits {
            false => SRAM021_IF_A::SRAM021_IF_0,
            true => SRAM021_IF_A::SRAM021_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM021_IF_0`"]
    #[inline(always)]
    pub fn is_sram021_if_0(&self) -> bool {
        *self == SRAM021_IF_A::SRAM021_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM021_IF_1`"]
    #[inline(always)]
    pub fn is_sram021_if_1(&self) -> bool {
        *self == SRAM021_IF_A::SRAM021_IF_1
    }
}
#[doc = "Field `SRAM021_IF` writer - Control AHB access to SRAM partition 21"]
pub type SRAM021_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM021_IF_A, O>;
impl<'a, const O: u8> SRAM021_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram021_if_0(self) -> &'a mut W {
        self.variant(SRAM021_IF_A::SRAM021_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram021_if_1(self) -> &'a mut W {
        self.variant(SRAM021_IF_A::SRAM021_IF_1)
    }
}
#[doc = "Field `SRAM022_IF` reader - Control AHB access to SRAM partition 22"]
pub type SRAM022_IF_R = crate::BitReader<SRAM022_IF_A>;
#[doc = "Control AHB access to SRAM partition 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM022_IF_A {
    #[doc = "0: Enable"]
    SRAM022_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM022_IF_1 = 1,
}
impl From<SRAM022_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM022_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM022_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM022_IF_A {
        match self.bits {
            false => SRAM022_IF_A::SRAM022_IF_0,
            true => SRAM022_IF_A::SRAM022_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM022_IF_0`"]
    #[inline(always)]
    pub fn is_sram022_if_0(&self) -> bool {
        *self == SRAM022_IF_A::SRAM022_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM022_IF_1`"]
    #[inline(always)]
    pub fn is_sram022_if_1(&self) -> bool {
        *self == SRAM022_IF_A::SRAM022_IF_1
    }
}
#[doc = "Field `SRAM022_IF` writer - Control AHB access to SRAM partition 22"]
pub type SRAM022_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM022_IF_A, O>;
impl<'a, const O: u8> SRAM022_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram022_if_0(self) -> &'a mut W {
        self.variant(SRAM022_IF_A::SRAM022_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram022_if_1(self) -> &'a mut W {
        self.variant(SRAM022_IF_A::SRAM022_IF_1)
    }
}
#[doc = "Field `SRAM023_IF` reader - Control AHB access to SRAM partition 23"]
pub type SRAM023_IF_R = crate::BitReader<SRAM023_IF_A>;
#[doc = "Control AHB access to SRAM partition 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM023_IF_A {
    #[doc = "0: Enable"]
    SRAM023_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM023_IF_1 = 1,
}
impl From<SRAM023_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM023_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM023_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM023_IF_A {
        match self.bits {
            false => SRAM023_IF_A::SRAM023_IF_0,
            true => SRAM023_IF_A::SRAM023_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM023_IF_0`"]
    #[inline(always)]
    pub fn is_sram023_if_0(&self) -> bool {
        *self == SRAM023_IF_A::SRAM023_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM023_IF_1`"]
    #[inline(always)]
    pub fn is_sram023_if_1(&self) -> bool {
        *self == SRAM023_IF_A::SRAM023_IF_1
    }
}
#[doc = "Field `SRAM023_IF` writer - Control AHB access to SRAM partition 23"]
pub type SRAM023_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM023_IF_A, O>;
impl<'a, const O: u8> SRAM023_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram023_if_0(self) -> &'a mut W {
        self.variant(SRAM023_IF_A::SRAM023_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram023_if_1(self) -> &'a mut W {
        self.variant(SRAM023_IF_A::SRAM023_IF_1)
    }
}
#[doc = "Field `SRAM024_IF` reader - Control AHB access to SRAM partition 24"]
pub type SRAM024_IF_R = crate::BitReader<SRAM024_IF_A>;
#[doc = "Control AHB access to SRAM partition 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM024_IF_A {
    #[doc = "0: Enable"]
    SRAM024_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM024_IF_1 = 1,
}
impl From<SRAM024_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM024_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM024_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM024_IF_A {
        match self.bits {
            false => SRAM024_IF_A::SRAM024_IF_0,
            true => SRAM024_IF_A::SRAM024_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM024_IF_0`"]
    #[inline(always)]
    pub fn is_sram024_if_0(&self) -> bool {
        *self == SRAM024_IF_A::SRAM024_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM024_IF_1`"]
    #[inline(always)]
    pub fn is_sram024_if_1(&self) -> bool {
        *self == SRAM024_IF_A::SRAM024_IF_1
    }
}
#[doc = "Field `SRAM024_IF` writer - Control AHB access to SRAM partition 24"]
pub type SRAM024_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM024_IF_A, O>;
impl<'a, const O: u8> SRAM024_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram024_if_0(self) -> &'a mut W {
        self.variant(SRAM024_IF_A::SRAM024_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram024_if_1(self) -> &'a mut W {
        self.variant(SRAM024_IF_A::SRAM024_IF_1)
    }
}
#[doc = "Field `SRAM025_IF` reader - Control AHB access to SRAM partition 25"]
pub type SRAM025_IF_R = crate::BitReader<SRAM025_IF_A>;
#[doc = "Control AHB access to SRAM partition 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM025_IF_A {
    #[doc = "0: Enable"]
    SRAM025_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM025_IF_1 = 1,
}
impl From<SRAM025_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM025_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM025_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM025_IF_A {
        match self.bits {
            false => SRAM025_IF_A::SRAM025_IF_0,
            true => SRAM025_IF_A::SRAM025_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM025_IF_0`"]
    #[inline(always)]
    pub fn is_sram025_if_0(&self) -> bool {
        *self == SRAM025_IF_A::SRAM025_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM025_IF_1`"]
    #[inline(always)]
    pub fn is_sram025_if_1(&self) -> bool {
        *self == SRAM025_IF_A::SRAM025_IF_1
    }
}
#[doc = "Field `SRAM025_IF` writer - Control AHB access to SRAM partition 25"]
pub type SRAM025_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM025_IF_A, O>;
impl<'a, const O: u8> SRAM025_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram025_if_0(self) -> &'a mut W {
        self.variant(SRAM025_IF_A::SRAM025_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram025_if_1(self) -> &'a mut W {
        self.variant(SRAM025_IF_A::SRAM025_IF_1)
    }
}
#[doc = "Field `SRAM026_IF` reader - Control AHB access to SRAM partition 26"]
pub type SRAM026_IF_R = crate::BitReader<SRAM026_IF_A>;
#[doc = "Control AHB access to SRAM partition 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM026_IF_A {
    #[doc = "0: Enable"]
    SRAM026_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM026_IF_1 = 1,
}
impl From<SRAM026_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM026_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM026_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM026_IF_A {
        match self.bits {
            false => SRAM026_IF_A::SRAM026_IF_0,
            true => SRAM026_IF_A::SRAM026_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM026_IF_0`"]
    #[inline(always)]
    pub fn is_sram026_if_0(&self) -> bool {
        *self == SRAM026_IF_A::SRAM026_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM026_IF_1`"]
    #[inline(always)]
    pub fn is_sram026_if_1(&self) -> bool {
        *self == SRAM026_IF_A::SRAM026_IF_1
    }
}
#[doc = "Field `SRAM026_IF` writer - Control AHB access to SRAM partition 26"]
pub type SRAM026_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM026_IF_A, O>;
impl<'a, const O: u8> SRAM026_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram026_if_0(self) -> &'a mut W {
        self.variant(SRAM026_IF_A::SRAM026_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram026_if_1(self) -> &'a mut W {
        self.variant(SRAM026_IF_A::SRAM026_IF_1)
    }
}
#[doc = "Field `SRAM027_IF` reader - Control AHB access to SRAM partition 27"]
pub type SRAM027_IF_R = crate::BitReader<SRAM027_IF_A>;
#[doc = "Control AHB access to SRAM partition 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM027_IF_A {
    #[doc = "0: Enable"]
    SRAM027_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM027_IF_1 = 1,
}
impl From<SRAM027_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM027_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM027_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM027_IF_A {
        match self.bits {
            false => SRAM027_IF_A::SRAM027_IF_0,
            true => SRAM027_IF_A::SRAM027_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM027_IF_0`"]
    #[inline(always)]
    pub fn is_sram027_if_0(&self) -> bool {
        *self == SRAM027_IF_A::SRAM027_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM027_IF_1`"]
    #[inline(always)]
    pub fn is_sram027_if_1(&self) -> bool {
        *self == SRAM027_IF_A::SRAM027_IF_1
    }
}
#[doc = "Field `SRAM027_IF` writer - Control AHB access to SRAM partition 27"]
pub type SRAM027_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM027_IF_A, O>;
impl<'a, const O: u8> SRAM027_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram027_if_0(self) -> &'a mut W {
        self.variant(SRAM027_IF_A::SRAM027_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram027_if_1(self) -> &'a mut W {
        self.variant(SRAM027_IF_A::SRAM027_IF_1)
    }
}
#[doc = "Field `SRAM028_IF` reader - Control AHB access to SRAM partition 28"]
pub type SRAM028_IF_R = crate::BitReader<SRAM028_IF_A>;
#[doc = "Control AHB access to SRAM partition 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM028_IF_A {
    #[doc = "0: Enable"]
    SRAM028_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM028_IF_1 = 1,
}
impl From<SRAM028_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM028_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM028_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM028_IF_A {
        match self.bits {
            false => SRAM028_IF_A::SRAM028_IF_0,
            true => SRAM028_IF_A::SRAM028_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM028_IF_0`"]
    #[inline(always)]
    pub fn is_sram028_if_0(&self) -> bool {
        *self == SRAM028_IF_A::SRAM028_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM028_IF_1`"]
    #[inline(always)]
    pub fn is_sram028_if_1(&self) -> bool {
        *self == SRAM028_IF_A::SRAM028_IF_1
    }
}
#[doc = "Field `SRAM028_IF` writer - Control AHB access to SRAM partition 28"]
pub type SRAM028_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM028_IF_A, O>;
impl<'a, const O: u8> SRAM028_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram028_if_0(self) -> &'a mut W {
        self.variant(SRAM028_IF_A::SRAM028_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram028_if_1(self) -> &'a mut W {
        self.variant(SRAM028_IF_A::SRAM028_IF_1)
    }
}
#[doc = "Field `SRAM029_IF` reader - Control AHB access to SRAM partition 29"]
pub type SRAM029_IF_R = crate::BitReader<SRAM029_IF_A>;
#[doc = "Control AHB access to SRAM partition 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM029_IF_A {
    #[doc = "0: Enable"]
    SRAM029_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM029_IF_1 = 1,
}
impl From<SRAM029_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM029_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM029_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM029_IF_A {
        match self.bits {
            false => SRAM029_IF_A::SRAM029_IF_0,
            true => SRAM029_IF_A::SRAM029_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM029_IF_0`"]
    #[inline(always)]
    pub fn is_sram029_if_0(&self) -> bool {
        *self == SRAM029_IF_A::SRAM029_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM029_IF_1`"]
    #[inline(always)]
    pub fn is_sram029_if_1(&self) -> bool {
        *self == SRAM029_IF_A::SRAM029_IF_1
    }
}
#[doc = "Field `SRAM029_IF` writer - Control AHB access to SRAM partition 29"]
pub type SRAM029_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM029_IF_A, O>;
impl<'a, const O: u8> SRAM029_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram029_if_0(self) -> &'a mut W {
        self.variant(SRAM029_IF_A::SRAM029_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram029_if_1(self) -> &'a mut W {
        self.variant(SRAM029_IF_A::SRAM029_IF_1)
    }
}
#[doc = "Field `SRAM030_IF` reader - Control AHB access to SRAM partition 30"]
pub type SRAM030_IF_R = crate::BitReader<SRAM030_IF_A>;
#[doc = "Control AHB access to SRAM partition 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM030_IF_A {
    #[doc = "0: Enable"]
    SRAM030_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM030_IF_1 = 1,
}
impl From<SRAM030_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM030_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM030_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM030_IF_A {
        match self.bits {
            false => SRAM030_IF_A::SRAM030_IF_0,
            true => SRAM030_IF_A::SRAM030_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM030_IF_0`"]
    #[inline(always)]
    pub fn is_sram030_if_0(&self) -> bool {
        *self == SRAM030_IF_A::SRAM030_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM030_IF_1`"]
    #[inline(always)]
    pub fn is_sram030_if_1(&self) -> bool {
        *self == SRAM030_IF_A::SRAM030_IF_1
    }
}
#[doc = "Field `SRAM030_IF` writer - Control AHB access to SRAM partition 30"]
pub type SRAM030_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM030_IF_A, O>;
impl<'a, const O: u8> SRAM030_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram030_if_0(self) -> &'a mut W {
        self.variant(SRAM030_IF_A::SRAM030_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram030_if_1(self) -> &'a mut W {
        self.variant(SRAM030_IF_A::SRAM030_IF_1)
    }
}
#[doc = "Field `SRAM031_IF` reader - Control AHB access to SRAM partition 31"]
pub type SRAM031_IF_R = crate::BitReader<SRAM031_IF_A>;
#[doc = "Control AHB access to SRAM partition 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM031_IF_A {
    #[doc = "0: Enable"]
    SRAM031_IF_0 = 0,
    #[doc = "1: Disable"]
    SRAM031_IF_1 = 1,
}
impl From<SRAM031_IF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM031_IF_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM031_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM031_IF_A {
        match self.bits {
            false => SRAM031_IF_A::SRAM031_IF_0,
            true => SRAM031_IF_A::SRAM031_IF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM031_IF_0`"]
    #[inline(always)]
    pub fn is_sram031_if_0(&self) -> bool {
        *self == SRAM031_IF_A::SRAM031_IF_0
    }
    #[doc = "Checks if the value of the field is `SRAM031_IF_1`"]
    #[inline(always)]
    pub fn is_sram031_if_1(&self) -> bool {
        *self == SRAM031_IF_A::SRAM031_IF_1
    }
}
#[doc = "Field `SRAM031_IF` writer - Control AHB access to SRAM partition 31"]
pub type SRAM031_IF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB_SRAM_ACCESS_DISABLE_SPEC, SRAM031_IF_A, O>;
impl<'a, const O: u8> SRAM031_IF_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram031_if_0(self) -> &'a mut W {
        self.variant(SRAM031_IF_A::SRAM031_IF_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram031_if_1(self) -> &'a mut W {
        self.variant(SRAM031_IF_A::SRAM031_IF_1)
    }
}
impl R {
    #[doc = "Bit 0 - Control AHB access to SRAM partition 0"]
    #[inline(always)]
    pub fn sram00_if(&self) -> SRAM00_IF_R {
        SRAM00_IF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control AHB access to SRAM partition 1"]
    #[inline(always)]
    pub fn sram01_if(&self) -> SRAM01_IF_R {
        SRAM01_IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Control AHB access to SRAM partition 2"]
    #[inline(always)]
    pub fn sram02_if(&self) -> SRAM02_IF_R {
        SRAM02_IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Control AHB access to SRAM partition 3"]
    #[inline(always)]
    pub fn sram03_if(&self) -> SRAM03_IF_R {
        SRAM03_IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Control AHB access to SRAM partition 4"]
    #[inline(always)]
    pub fn sram04_if(&self) -> SRAM04_IF_R {
        SRAM04_IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Control AHB access to SRAM partition 5"]
    #[inline(always)]
    pub fn sram05_if(&self) -> SRAM05_IF_R {
        SRAM05_IF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Control AHB access to SRAM partition 6"]
    #[inline(always)]
    pub fn sram06_if(&self) -> SRAM06_IF_R {
        SRAM06_IF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Control AHB access to SRAM partition 7"]
    #[inline(always)]
    pub fn sram07_if(&self) -> SRAM07_IF_R {
        SRAM07_IF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Control AHB access to SRAM partition 8"]
    #[inline(always)]
    pub fn sram08_if(&self) -> SRAM08_IF_R {
        SRAM08_IF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Control AHB access to SRAM partition 9"]
    #[inline(always)]
    pub fn sram09_if(&self) -> SRAM09_IF_R {
        SRAM09_IF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Control AHB access to SRAM partition 10"]
    #[inline(always)]
    pub fn sram010_if(&self) -> SRAM010_IF_R {
        SRAM010_IF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Control AHB access to SRAM partition 11"]
    #[inline(always)]
    pub fn sram011_if(&self) -> SRAM011_IF_R {
        SRAM011_IF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Control AHB access to SRAM partition 12"]
    #[inline(always)]
    pub fn sram012_if(&self) -> SRAM012_IF_R {
        SRAM012_IF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Control AHB access to SRAM partition 13"]
    #[inline(always)]
    pub fn sram013_if(&self) -> SRAM013_IF_R {
        SRAM013_IF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Control AHB access to SRAM partition 14"]
    #[inline(always)]
    pub fn sram014_if(&self) -> SRAM014_IF_R {
        SRAM014_IF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Control AHB access to SRAM partition 15"]
    #[inline(always)]
    pub fn sram015_if(&self) -> SRAM015_IF_R {
        SRAM015_IF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Control AHB access to SRAM partition 16"]
    #[inline(always)]
    pub fn sram016_if(&self) -> SRAM016_IF_R {
        SRAM016_IF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Control AHB access to SRAM partition 17"]
    #[inline(always)]
    pub fn sram017_if(&self) -> SRAM017_IF_R {
        SRAM017_IF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Control AHB access to SRAM partition 18"]
    #[inline(always)]
    pub fn sram018_if(&self) -> SRAM018_IF_R {
        SRAM018_IF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Control AHB access to SRAM partition 19"]
    #[inline(always)]
    pub fn sram019_if(&self) -> SRAM019_IF_R {
        SRAM019_IF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Control AHB access to SRAM partition 20"]
    #[inline(always)]
    pub fn sram020_if(&self) -> SRAM020_IF_R {
        SRAM020_IF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Control AHB access to SRAM partition 21"]
    #[inline(always)]
    pub fn sram021_if(&self) -> SRAM021_IF_R {
        SRAM021_IF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Control AHB access to SRAM partition 22"]
    #[inline(always)]
    pub fn sram022_if(&self) -> SRAM022_IF_R {
        SRAM022_IF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Control AHB access to SRAM partition 23"]
    #[inline(always)]
    pub fn sram023_if(&self) -> SRAM023_IF_R {
        SRAM023_IF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Control AHB access to SRAM partition 24"]
    #[inline(always)]
    pub fn sram024_if(&self) -> SRAM024_IF_R {
        SRAM024_IF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Control AHB access to SRAM partition 25"]
    #[inline(always)]
    pub fn sram025_if(&self) -> SRAM025_IF_R {
        SRAM025_IF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Control AHB access to SRAM partition 26"]
    #[inline(always)]
    pub fn sram026_if(&self) -> SRAM026_IF_R {
        SRAM026_IF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Control AHB access to SRAM partition 27"]
    #[inline(always)]
    pub fn sram027_if(&self) -> SRAM027_IF_R {
        SRAM027_IF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Control AHB access to SRAM partition 28"]
    #[inline(always)]
    pub fn sram028_if(&self) -> SRAM028_IF_R {
        SRAM028_IF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Control AHB access to SRAM partition 29"]
    #[inline(always)]
    pub fn sram029_if(&self) -> SRAM029_IF_R {
        SRAM029_IF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Control AHB access to SRAM partition 30"]
    #[inline(always)]
    pub fn sram030_if(&self) -> SRAM030_IF_R {
        SRAM030_IF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Control AHB access to SRAM partition 31"]
    #[inline(always)]
    pub fn sram031_if(&self) -> SRAM031_IF_R {
        SRAM031_IF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control AHB access to SRAM partition 0"]
    #[inline(always)]
    #[must_use]
    pub fn sram00_if(&mut self) -> SRAM00_IF_W<0> {
        SRAM00_IF_W::new(self)
    }
    #[doc = "Bit 1 - Control AHB access to SRAM partition 1"]
    #[inline(always)]
    #[must_use]
    pub fn sram01_if(&mut self) -> SRAM01_IF_W<1> {
        SRAM01_IF_W::new(self)
    }
    #[doc = "Bit 2 - Control AHB access to SRAM partition 2"]
    #[inline(always)]
    #[must_use]
    pub fn sram02_if(&mut self) -> SRAM02_IF_W<2> {
        SRAM02_IF_W::new(self)
    }
    #[doc = "Bit 3 - Control AHB access to SRAM partition 3"]
    #[inline(always)]
    #[must_use]
    pub fn sram03_if(&mut self) -> SRAM03_IF_W<3> {
        SRAM03_IF_W::new(self)
    }
    #[doc = "Bit 4 - Control AHB access to SRAM partition 4"]
    #[inline(always)]
    #[must_use]
    pub fn sram04_if(&mut self) -> SRAM04_IF_W<4> {
        SRAM04_IF_W::new(self)
    }
    #[doc = "Bit 5 - Control AHB access to SRAM partition 5"]
    #[inline(always)]
    #[must_use]
    pub fn sram05_if(&mut self) -> SRAM05_IF_W<5> {
        SRAM05_IF_W::new(self)
    }
    #[doc = "Bit 6 - Control AHB access to SRAM partition 6"]
    #[inline(always)]
    #[must_use]
    pub fn sram06_if(&mut self) -> SRAM06_IF_W<6> {
        SRAM06_IF_W::new(self)
    }
    #[doc = "Bit 7 - Control AHB access to SRAM partition 7"]
    #[inline(always)]
    #[must_use]
    pub fn sram07_if(&mut self) -> SRAM07_IF_W<7> {
        SRAM07_IF_W::new(self)
    }
    #[doc = "Bit 8 - Control AHB access to SRAM partition 8"]
    #[inline(always)]
    #[must_use]
    pub fn sram08_if(&mut self) -> SRAM08_IF_W<8> {
        SRAM08_IF_W::new(self)
    }
    #[doc = "Bit 9 - Control AHB access to SRAM partition 9"]
    #[inline(always)]
    #[must_use]
    pub fn sram09_if(&mut self) -> SRAM09_IF_W<9> {
        SRAM09_IF_W::new(self)
    }
    #[doc = "Bit 10 - Control AHB access to SRAM partition 10"]
    #[inline(always)]
    #[must_use]
    pub fn sram010_if(&mut self) -> SRAM010_IF_W<10> {
        SRAM010_IF_W::new(self)
    }
    #[doc = "Bit 11 - Control AHB access to SRAM partition 11"]
    #[inline(always)]
    #[must_use]
    pub fn sram011_if(&mut self) -> SRAM011_IF_W<11> {
        SRAM011_IF_W::new(self)
    }
    #[doc = "Bit 12 - Control AHB access to SRAM partition 12"]
    #[inline(always)]
    #[must_use]
    pub fn sram012_if(&mut self) -> SRAM012_IF_W<12> {
        SRAM012_IF_W::new(self)
    }
    #[doc = "Bit 13 - Control AHB access to SRAM partition 13"]
    #[inline(always)]
    #[must_use]
    pub fn sram013_if(&mut self) -> SRAM013_IF_W<13> {
        SRAM013_IF_W::new(self)
    }
    #[doc = "Bit 14 - Control AHB access to SRAM partition 14"]
    #[inline(always)]
    #[must_use]
    pub fn sram014_if(&mut self) -> SRAM014_IF_W<14> {
        SRAM014_IF_W::new(self)
    }
    #[doc = "Bit 15 - Control AHB access to SRAM partition 15"]
    #[inline(always)]
    #[must_use]
    pub fn sram015_if(&mut self) -> SRAM015_IF_W<15> {
        SRAM015_IF_W::new(self)
    }
    #[doc = "Bit 16 - Control AHB access to SRAM partition 16"]
    #[inline(always)]
    #[must_use]
    pub fn sram016_if(&mut self) -> SRAM016_IF_W<16> {
        SRAM016_IF_W::new(self)
    }
    #[doc = "Bit 17 - Control AHB access to SRAM partition 17"]
    #[inline(always)]
    #[must_use]
    pub fn sram017_if(&mut self) -> SRAM017_IF_W<17> {
        SRAM017_IF_W::new(self)
    }
    #[doc = "Bit 18 - Control AHB access to SRAM partition 18"]
    #[inline(always)]
    #[must_use]
    pub fn sram018_if(&mut self) -> SRAM018_IF_W<18> {
        SRAM018_IF_W::new(self)
    }
    #[doc = "Bit 19 - Control AHB access to SRAM partition 19"]
    #[inline(always)]
    #[must_use]
    pub fn sram019_if(&mut self) -> SRAM019_IF_W<19> {
        SRAM019_IF_W::new(self)
    }
    #[doc = "Bit 20 - Control AHB access to SRAM partition 20"]
    #[inline(always)]
    #[must_use]
    pub fn sram020_if(&mut self) -> SRAM020_IF_W<20> {
        SRAM020_IF_W::new(self)
    }
    #[doc = "Bit 21 - Control AHB access to SRAM partition 21"]
    #[inline(always)]
    #[must_use]
    pub fn sram021_if(&mut self) -> SRAM021_IF_W<21> {
        SRAM021_IF_W::new(self)
    }
    #[doc = "Bit 22 - Control AHB access to SRAM partition 22"]
    #[inline(always)]
    #[must_use]
    pub fn sram022_if(&mut self) -> SRAM022_IF_W<22> {
        SRAM022_IF_W::new(self)
    }
    #[doc = "Bit 23 - Control AHB access to SRAM partition 23"]
    #[inline(always)]
    #[must_use]
    pub fn sram023_if(&mut self) -> SRAM023_IF_W<23> {
        SRAM023_IF_W::new(self)
    }
    #[doc = "Bit 24 - Control AHB access to SRAM partition 24"]
    #[inline(always)]
    #[must_use]
    pub fn sram024_if(&mut self) -> SRAM024_IF_W<24> {
        SRAM024_IF_W::new(self)
    }
    #[doc = "Bit 25 - Control AHB access to SRAM partition 25"]
    #[inline(always)]
    #[must_use]
    pub fn sram025_if(&mut self) -> SRAM025_IF_W<25> {
        SRAM025_IF_W::new(self)
    }
    #[doc = "Bit 26 - Control AHB access to SRAM partition 26"]
    #[inline(always)]
    #[must_use]
    pub fn sram026_if(&mut self) -> SRAM026_IF_W<26> {
        SRAM026_IF_W::new(self)
    }
    #[doc = "Bit 27 - Control AHB access to SRAM partition 27"]
    #[inline(always)]
    #[must_use]
    pub fn sram027_if(&mut self) -> SRAM027_IF_W<27> {
        SRAM027_IF_W::new(self)
    }
    #[doc = "Bit 28 - Control AHB access to SRAM partition 28"]
    #[inline(always)]
    #[must_use]
    pub fn sram028_if(&mut self) -> SRAM028_IF_W<28> {
        SRAM028_IF_W::new(self)
    }
    #[doc = "Bit 29 - Control AHB access to SRAM partition 29"]
    #[inline(always)]
    #[must_use]
    pub fn sram029_if(&mut self) -> SRAM029_IF_W<29> {
        SRAM029_IF_W::new(self)
    }
    #[doc = "Bit 30 - Control AHB access to SRAM partition 30"]
    #[inline(always)]
    #[must_use]
    pub fn sram030_if(&mut self) -> SRAM030_IF_W<30> {
        SRAM030_IF_W::new(self)
    }
    #[doc = "Bit 31 - Control AHB access to SRAM partition 31"]
    #[inline(always)]
    #[must_use]
    pub fn sram031_if(&mut self) -> SRAM031_IF_W<31> {
        SRAM031_IF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB SRAM access disable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb_sram_access_disable](index.html) module"]
pub struct AHB_SRAM_ACCESS_DISABLE_SPEC;
impl crate::RegisterSpec for AHB_SRAM_ACCESS_DISABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb_sram_access_disable::R](R) reader structure"]
impl crate::Readable for AHB_SRAM_ACCESS_DISABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb_sram_access_disable::W](W) writer structure"]
impl crate::Writable for AHB_SRAM_ACCESS_DISABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB_SRAM_ACCESS_DISABLE to value 0"]
impl crate::Resettable for AHB_SRAM_ACCESS_DISABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
