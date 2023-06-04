#[doc = "Register `PDSLEEPCFG1` reader"]
pub struct R(crate::R<PDSLEEPCFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDSLEEPCFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDSLEEPCFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDSLEEPCFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDSLEEPCFG1` writer"]
pub struct W(crate::W<PDSLEEPCFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDSLEEPCFG1_SPEC>;
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
impl From<crate::W<PDSLEEPCFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDSLEEPCFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PQ_SRAM_PPD` reader - Periphery power for PowerQuad RAM"]
pub type PQ_SRAM_PPD_R = crate::BitReader<PQ_SRAM_PPD_A>;
#[doc = "Periphery power for PowerQuad RAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PQ_SRAM_PPD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    PQ_SRAM_PPD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    PQ_SRAM_PPD_1 = 1,
}
impl From<PQ_SRAM_PPD_A> for bool {
    #[inline(always)]
    fn from(variant: PQ_SRAM_PPD_A) -> Self {
        variant as u8 != 0
    }
}
impl PQ_SRAM_PPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PQ_SRAM_PPD_A {
        match self.bits {
            false => PQ_SRAM_PPD_A::PQ_SRAM_PPD_0,
            true => PQ_SRAM_PPD_A::PQ_SRAM_PPD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PQ_SRAM_PPD_0`"]
    #[inline(always)]
    pub fn is_pq_sram_ppd_0(&self) -> bool {
        *self == PQ_SRAM_PPD_A::PQ_SRAM_PPD_0
    }
    #[doc = "Checks if the value of the field is `PQ_SRAM_PPD_1`"]
    #[inline(always)]
    pub fn is_pq_sram_ppd_1(&self) -> bool {
        *self == PQ_SRAM_PPD_A::PQ_SRAM_PPD_1
    }
}
#[doc = "Field `PQ_SRAM_PPD` writer - Periphery power for PowerQuad RAM"]
pub type PQ_SRAM_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG1_SPEC, PQ_SRAM_PPD_A, O>;
impl<'a, const O: u8> PQ_SRAM_PPD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn pq_sram_ppd_0(self) -> &'a mut W {
        self.variant(PQ_SRAM_PPD_A::PQ_SRAM_PPD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn pq_sram_ppd_1(self) -> &'a mut W {
        self.variant(PQ_SRAM_PPD_A::PQ_SRAM_PPD_1)
    }
}
#[doc = "Field `FLEXSPI0_SRAM_APD` reader - Array power for FLEXSPI0 RAM"]
pub type FLEXSPI0_SRAM_APD_R = crate::BitReader<FLEXSPI0_SRAM_APD_A>;
#[doc = "Array power for FLEXSPI0 RAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI0_SRAM_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    FLEXSPI0_SRAM_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    FLEXSPI0_SRAM_APD_1 = 1,
}
impl From<FLEXSPI0_SRAM_APD_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI0_SRAM_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXSPI0_SRAM_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXSPI0_SRAM_APD_A {
        match self.bits {
            false => FLEXSPI0_SRAM_APD_A::FLEXSPI0_SRAM_APD_0,
            true => FLEXSPI0_SRAM_APD_A::FLEXSPI0_SRAM_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXSPI0_SRAM_APD_0`"]
    #[inline(always)]
    pub fn is_flexspi0_sram_apd_0(&self) -> bool {
        *self == FLEXSPI0_SRAM_APD_A::FLEXSPI0_SRAM_APD_0
    }
    #[doc = "Checks if the value of the field is `FLEXSPI0_SRAM_APD_1`"]
    #[inline(always)]
    pub fn is_flexspi0_sram_apd_1(&self) -> bool {
        *self == FLEXSPI0_SRAM_APD_A::FLEXSPI0_SRAM_APD_1
    }
}
#[doc = "Field `FLEXSPI0_SRAM_APD` writer - Array power for FLEXSPI0 RAM"]
pub type FLEXSPI0_SRAM_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG1_SPEC, FLEXSPI0_SRAM_APD_A, O>;
impl<'a, const O: u8> FLEXSPI0_SRAM_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn flexspi0_sram_apd_0(self) -> &'a mut W {
        self.variant(FLEXSPI0_SRAM_APD_A::FLEXSPI0_SRAM_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn flexspi0_sram_apd_1(self) -> &'a mut W {
        self.variant(FLEXSPI0_SRAM_APD_A::FLEXSPI0_SRAM_APD_1)
    }
}
#[doc = "Field `FLEXSPI0_SRAM_PPD` reader - Periphery power for FLEXSPI0 RAM"]
pub type FLEXSPI0_SRAM_PPD_R = crate::BitReader<FLEXSPI0_SRAM_PPD_A>;
#[doc = "Periphery power for FLEXSPI0 RAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI0_SRAM_PPD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    FLEXSPI0_SRAM_PPD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    FLEXSPI0_SRAM_PPD_1 = 1,
}
impl From<FLEXSPI0_SRAM_PPD_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI0_SRAM_PPD_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXSPI0_SRAM_PPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXSPI0_SRAM_PPD_A {
        match self.bits {
            false => FLEXSPI0_SRAM_PPD_A::FLEXSPI0_SRAM_PPD_0,
            true => FLEXSPI0_SRAM_PPD_A::FLEXSPI0_SRAM_PPD_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXSPI0_SRAM_PPD_0`"]
    #[inline(always)]
    pub fn is_flexspi0_sram_ppd_0(&self) -> bool {
        *self == FLEXSPI0_SRAM_PPD_A::FLEXSPI0_SRAM_PPD_0
    }
    #[doc = "Checks if the value of the field is `FLEXSPI0_SRAM_PPD_1`"]
    #[inline(always)]
    pub fn is_flexspi0_sram_ppd_1(&self) -> bool {
        *self == FLEXSPI0_SRAM_PPD_A::FLEXSPI0_SRAM_PPD_1
    }
}
#[doc = "Field `FLEXSPI0_SRAM_PPD` writer - Periphery power for FLEXSPI0 RAM"]
pub type FLEXSPI0_SRAM_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG1_SPEC, FLEXSPI0_SRAM_PPD_A, O>;
impl<'a, const O: u8> FLEXSPI0_SRAM_PPD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn flexspi0_sram_ppd_0(self) -> &'a mut W {
        self.variant(FLEXSPI0_SRAM_PPD_A::FLEXSPI0_SRAM_PPD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn flexspi0_sram_ppd_1(self) -> &'a mut W {
        self.variant(FLEXSPI0_SRAM_PPD_A::FLEXSPI0_SRAM_PPD_1)
    }
}
#[doc = "Field `FLEXSPI1_SRAM_APD` reader - Array power for FLEXSPI1 RAM"]
pub type FLEXSPI1_SRAM_APD_R = crate::BitReader<FLEXSPI1_SRAM_APD_A>;
#[doc = "Array power for FLEXSPI1 RAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI1_SRAM_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    FLEXSPI1_SRAM_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    FLEXSPI1_SRAM_APD_1 = 1,
}
impl From<FLEXSPI1_SRAM_APD_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI1_SRAM_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXSPI1_SRAM_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXSPI1_SRAM_APD_A {
        match self.bits {
            false => FLEXSPI1_SRAM_APD_A::FLEXSPI1_SRAM_APD_0,
            true => FLEXSPI1_SRAM_APD_A::FLEXSPI1_SRAM_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXSPI1_SRAM_APD_0`"]
    #[inline(always)]
    pub fn is_flexspi1_sram_apd_0(&self) -> bool {
        *self == FLEXSPI1_SRAM_APD_A::FLEXSPI1_SRAM_APD_0
    }
    #[doc = "Checks if the value of the field is `FLEXSPI1_SRAM_APD_1`"]
    #[inline(always)]
    pub fn is_flexspi1_sram_apd_1(&self) -> bool {
        *self == FLEXSPI1_SRAM_APD_A::FLEXSPI1_SRAM_APD_1
    }
}
#[doc = "Field `FLEXSPI1_SRAM_APD` writer - Array power for FLEXSPI1 RAM"]
pub type FLEXSPI1_SRAM_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG1_SPEC, FLEXSPI1_SRAM_APD_A, O>;
impl<'a, const O: u8> FLEXSPI1_SRAM_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn flexspi1_sram_apd_0(self) -> &'a mut W {
        self.variant(FLEXSPI1_SRAM_APD_A::FLEXSPI1_SRAM_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn flexspi1_sram_apd_1(self) -> &'a mut W {
        self.variant(FLEXSPI1_SRAM_APD_A::FLEXSPI1_SRAM_APD_1)
    }
}
#[doc = "Field `FLEXSPI1_SRAM_PPD` reader - Periphery power for FLEXSPI1 RAM"]
pub type FLEXSPI1_SRAM_PPD_R = crate::BitReader<FLEXSPI1_SRAM_PPD_A>;
#[doc = "Periphery power for FLEXSPI1 RAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI1_SRAM_PPD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    FLEXSPI1_SRAM_PPD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    FLEXSPI1_SRAM_PPD_1 = 1,
}
impl From<FLEXSPI1_SRAM_PPD_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI1_SRAM_PPD_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXSPI1_SRAM_PPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXSPI1_SRAM_PPD_A {
        match self.bits {
            false => FLEXSPI1_SRAM_PPD_A::FLEXSPI1_SRAM_PPD_0,
            true => FLEXSPI1_SRAM_PPD_A::FLEXSPI1_SRAM_PPD_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXSPI1_SRAM_PPD_0`"]
    #[inline(always)]
    pub fn is_flexspi1_sram_ppd_0(&self) -> bool {
        *self == FLEXSPI1_SRAM_PPD_A::FLEXSPI1_SRAM_PPD_0
    }
    #[doc = "Checks if the value of the field is `FLEXSPI1_SRAM_PPD_1`"]
    #[inline(always)]
    pub fn is_flexspi1_sram_ppd_1(&self) -> bool {
        *self == FLEXSPI1_SRAM_PPD_A::FLEXSPI1_SRAM_PPD_1
    }
}
#[doc = "Field `FLEXSPI1_SRAM_PPD` writer - Periphery power for FLEXSPI1 RAM"]
pub type FLEXSPI1_SRAM_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG1_SPEC, FLEXSPI1_SRAM_PPD_A, O>;
impl<'a, const O: u8> FLEXSPI1_SRAM_PPD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn flexspi1_sram_ppd_0(self) -> &'a mut W {
        self.variant(FLEXSPI1_SRAM_PPD_A::FLEXSPI1_SRAM_PPD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn flexspi1_sram_ppd_1(self) -> &'a mut W {
        self.variant(FLEXSPI1_SRAM_PPD_A::FLEXSPI1_SRAM_PPD_1)
    }
}
#[doc = "Field `USBHS_SRAM_APD` reader - Array power for USB RAM"]
pub type USBHS_SRAM_APD_R = crate::BitReader<USBHS_SRAM_APD_A>;
#[doc = "Array power for USB RAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBHS_SRAM_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    USBHS_SRAM_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    USBHS_SRAM_APD_1 = 1,
}
impl From<USBHS_SRAM_APD_A> for bool {
    #[inline(always)]
    fn from(variant: USBHS_SRAM_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl USBHS_SRAM_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBHS_SRAM_APD_A {
        match self.bits {
            false => USBHS_SRAM_APD_A::USBHS_SRAM_APD_0,
            true => USBHS_SRAM_APD_A::USBHS_SRAM_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `USBHS_SRAM_APD_0`"]
    #[inline(always)]
    pub fn is_usbhs_sram_apd_0(&self) -> bool {
        *self == USBHS_SRAM_APD_A::USBHS_SRAM_APD_0
    }
    #[doc = "Checks if the value of the field is `USBHS_SRAM_APD_1`"]
    #[inline(always)]
    pub fn is_usbhs_sram_apd_1(&self) -> bool {
        *self == USBHS_SRAM_APD_A::USBHS_SRAM_APD_1
    }
}
#[doc = "Field `USBHS_SRAM_APD` writer - Array power for USB RAM"]
pub type USBHS_SRAM_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG1_SPEC, USBHS_SRAM_APD_A, O>;
impl<'a, const O: u8> USBHS_SRAM_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn usbhs_sram_apd_0(self) -> &'a mut W {
        self.variant(USBHS_SRAM_APD_A::USBHS_SRAM_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn usbhs_sram_apd_1(self) -> &'a mut W {
        self.variant(USBHS_SRAM_APD_A::USBHS_SRAM_APD_1)
    }
}
#[doc = "Field `USBHS_SRAM_PPD` reader - Periphery power for USB RAM"]
pub type USBHS_SRAM_PPD_R = crate::BitReader<USBHS_SRAM_PPD_A>;
#[doc = "Periphery power for USB RAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBHS_SRAM_PPD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    USBHS_SRAM_PPD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    USBHS_SRAM_PPD_1 = 1,
}
impl From<USBHS_SRAM_PPD_A> for bool {
    #[inline(always)]
    fn from(variant: USBHS_SRAM_PPD_A) -> Self {
        variant as u8 != 0
    }
}
impl USBHS_SRAM_PPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBHS_SRAM_PPD_A {
        match self.bits {
            false => USBHS_SRAM_PPD_A::USBHS_SRAM_PPD_0,
            true => USBHS_SRAM_PPD_A::USBHS_SRAM_PPD_1,
        }
    }
    #[doc = "Checks if the value of the field is `USBHS_SRAM_PPD_0`"]
    #[inline(always)]
    pub fn is_usbhs_sram_ppd_0(&self) -> bool {
        *self == USBHS_SRAM_PPD_A::USBHS_SRAM_PPD_0
    }
    #[doc = "Checks if the value of the field is `USBHS_SRAM_PPD_1`"]
    #[inline(always)]
    pub fn is_usbhs_sram_ppd_1(&self) -> bool {
        *self == USBHS_SRAM_PPD_A::USBHS_SRAM_PPD_1
    }
}
#[doc = "Field `USBHS_SRAM_PPD` writer - Periphery power for USB RAM"]
pub type USBHS_SRAM_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG1_SPEC, USBHS_SRAM_PPD_A, O>;
impl<'a, const O: u8> USBHS_SRAM_PPD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn usbhs_sram_ppd_0(self) -> &'a mut W {
        self.variant(USBHS_SRAM_PPD_A::USBHS_SRAM_PPD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn usbhs_sram_ppd_1(self) -> &'a mut W {
        self.variant(USBHS_SRAM_PPD_A::USBHS_SRAM_PPD_1)
    }
}
#[doc = "Field `USDHC0_SRAM_APD` reader - Array power for uSDHC0 (SD/MMC/SDIO interface) RAM"]
pub type USDHC0_SRAM_APD_R = crate::BitReader<USDHC0_SRAM_APD_A>;
#[doc = "Array power for uSDHC0 (SD/MMC/SDIO interface) RAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USDHC0_SRAM_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    USDHC0_SRAM_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    USDHC0_SRAM_APD_1 = 1,
}
impl From<USDHC0_SRAM_APD_A> for bool {
    #[inline(always)]
    fn from(variant: USDHC0_SRAM_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl USDHC0_SRAM_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USDHC0_SRAM_APD_A {
        match self.bits {
            false => USDHC0_SRAM_APD_A::USDHC0_SRAM_APD_0,
            true => USDHC0_SRAM_APD_A::USDHC0_SRAM_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `USDHC0_SRAM_APD_0`"]
    #[inline(always)]
    pub fn is_usdhc0_sram_apd_0(&self) -> bool {
        *self == USDHC0_SRAM_APD_A::USDHC0_SRAM_APD_0
    }
    #[doc = "Checks if the value of the field is `USDHC0_SRAM_APD_1`"]
    #[inline(always)]
    pub fn is_usdhc0_sram_apd_1(&self) -> bool {
        *self == USDHC0_SRAM_APD_A::USDHC0_SRAM_APD_1
    }
}
#[doc = "Field `USDHC0_SRAM_APD` writer - Array power for uSDHC0 (SD/MMC/SDIO interface) RAM"]
pub type USDHC0_SRAM_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG1_SPEC, USDHC0_SRAM_APD_A, O>;
impl<'a, const O: u8> USDHC0_SRAM_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn usdhc0_sram_apd_0(self) -> &'a mut W {
        self.variant(USDHC0_SRAM_APD_A::USDHC0_SRAM_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn usdhc0_sram_apd_1(self) -> &'a mut W {
        self.variant(USDHC0_SRAM_APD_A::USDHC0_SRAM_APD_1)
    }
}
#[doc = "Field `USDHC0_SRAM_PPD` reader - Periphery power for uSDHC0 (SD/MMC/SDIO interface) RAM"]
pub type USDHC0_SRAM_PPD_R = crate::BitReader<USDHC0_SRAM_PPD_A>;
#[doc = "Periphery power for uSDHC0 (SD/MMC/SDIO interface) RAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USDHC0_SRAM_PPD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    USDHC0_SRAM_PPD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    USDHC0_SRAM_PPD_1 = 1,
}
impl From<USDHC0_SRAM_PPD_A> for bool {
    #[inline(always)]
    fn from(variant: USDHC0_SRAM_PPD_A) -> Self {
        variant as u8 != 0
    }
}
impl USDHC0_SRAM_PPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USDHC0_SRAM_PPD_A {
        match self.bits {
            false => USDHC0_SRAM_PPD_A::USDHC0_SRAM_PPD_0,
            true => USDHC0_SRAM_PPD_A::USDHC0_SRAM_PPD_1,
        }
    }
    #[doc = "Checks if the value of the field is `USDHC0_SRAM_PPD_0`"]
    #[inline(always)]
    pub fn is_usdhc0_sram_ppd_0(&self) -> bool {
        *self == USDHC0_SRAM_PPD_A::USDHC0_SRAM_PPD_0
    }
    #[doc = "Checks if the value of the field is `USDHC0_SRAM_PPD_1`"]
    #[inline(always)]
    pub fn is_usdhc0_sram_ppd_1(&self) -> bool {
        *self == USDHC0_SRAM_PPD_A::USDHC0_SRAM_PPD_1
    }
}
#[doc = "Field `USDHC0_SRAM_PPD` writer - Periphery power for uSDHC0 (SD/MMC/SDIO interface) RAM"]
pub type USDHC0_SRAM_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG1_SPEC, USDHC0_SRAM_PPD_A, O>;
impl<'a, const O: u8> USDHC0_SRAM_PPD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn usdhc0_sram_ppd_0(self) -> &'a mut W {
        self.variant(USDHC0_SRAM_PPD_A::USDHC0_SRAM_PPD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn usdhc0_sram_ppd_1(self) -> &'a mut W {
        self.variant(USDHC0_SRAM_PPD_A::USDHC0_SRAM_PPD_1)
    }
}
#[doc = "Field `USDHC1_SRAM_APD` reader - Array power for Casper RAM"]
pub type USDHC1_SRAM_APD_R = crate::BitReader<USDHC1_SRAM_APD_A>;
#[doc = "Array power for Casper RAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USDHC1_SRAM_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    USDHC1_SRAM_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    USDHC1_SRAM_APD_1 = 1,
}
impl From<USDHC1_SRAM_APD_A> for bool {
    #[inline(always)]
    fn from(variant: USDHC1_SRAM_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl USDHC1_SRAM_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USDHC1_SRAM_APD_A {
        match self.bits {
            false => USDHC1_SRAM_APD_A::USDHC1_SRAM_APD_0,
            true => USDHC1_SRAM_APD_A::USDHC1_SRAM_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `USDHC1_SRAM_APD_0`"]
    #[inline(always)]
    pub fn is_usdhc1_sram_apd_0(&self) -> bool {
        *self == USDHC1_SRAM_APD_A::USDHC1_SRAM_APD_0
    }
    #[doc = "Checks if the value of the field is `USDHC1_SRAM_APD_1`"]
    #[inline(always)]
    pub fn is_usdhc1_sram_apd_1(&self) -> bool {
        *self == USDHC1_SRAM_APD_A::USDHC1_SRAM_APD_1
    }
}
#[doc = "Field `USDHC1_SRAM_APD` writer - Array power for Casper RAM"]
pub type USDHC1_SRAM_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG1_SPEC, USDHC1_SRAM_APD_A, O>;
impl<'a, const O: u8> USDHC1_SRAM_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn usdhc1_sram_apd_0(self) -> &'a mut W {
        self.variant(USDHC1_SRAM_APD_A::USDHC1_SRAM_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn usdhc1_sram_apd_1(self) -> &'a mut W {
        self.variant(USDHC1_SRAM_APD_A::USDHC1_SRAM_APD_1)
    }
}
#[doc = "Field `USDHC1_SRAM_PPD` reader - Periphery power for uSDHC1 (SD/MMC/SDIO interface) RAM"]
pub type USDHC1_SRAM_PPD_R = crate::BitReader<USDHC1_SRAM_PPD_A>;
#[doc = "Periphery power for uSDHC1 (SD/MMC/SDIO interface) RAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USDHC1_SRAM_PPD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    USDHC1_SRAM_PPD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    USDHC1_SRAM_PPD_1 = 1,
}
impl From<USDHC1_SRAM_PPD_A> for bool {
    #[inline(always)]
    fn from(variant: USDHC1_SRAM_PPD_A) -> Self {
        variant as u8 != 0
    }
}
impl USDHC1_SRAM_PPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USDHC1_SRAM_PPD_A {
        match self.bits {
            false => USDHC1_SRAM_PPD_A::USDHC1_SRAM_PPD_0,
            true => USDHC1_SRAM_PPD_A::USDHC1_SRAM_PPD_1,
        }
    }
    #[doc = "Checks if the value of the field is `USDHC1_SRAM_PPD_0`"]
    #[inline(always)]
    pub fn is_usdhc1_sram_ppd_0(&self) -> bool {
        *self == USDHC1_SRAM_PPD_A::USDHC1_SRAM_PPD_0
    }
    #[doc = "Checks if the value of the field is `USDHC1_SRAM_PPD_1`"]
    #[inline(always)]
    pub fn is_usdhc1_sram_ppd_1(&self) -> bool {
        *self == USDHC1_SRAM_PPD_A::USDHC1_SRAM_PPD_1
    }
}
#[doc = "Field `USDHC1_SRAM_PPD` writer - Periphery power for uSDHC1 (SD/MMC/SDIO interface) RAM"]
pub type USDHC1_SRAM_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG1_SPEC, USDHC1_SRAM_PPD_A, O>;
impl<'a, const O: u8> USDHC1_SRAM_PPD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn usdhc1_sram_ppd_0(self) -> &'a mut W {
        self.variant(USDHC1_SRAM_PPD_A::USDHC1_SRAM_PPD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn usdhc1_sram_ppd_1(self) -> &'a mut W {
        self.variant(USDHC1_SRAM_PPD_A::USDHC1_SRAM_PPD_1)
    }
}
#[doc = "Field `CASPER_SRAM_PPD` reader - Periphery power for Casper RAM"]
pub type CASPER_SRAM_PPD_R = crate::BitReader<CASPER_SRAM_PPD_A>;
#[doc = "Periphery power for Casper RAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CASPER_SRAM_PPD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    CASPER_SRAM_PPD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    CASPER_SRAM_PPD_1 = 1,
}
impl From<CASPER_SRAM_PPD_A> for bool {
    #[inline(always)]
    fn from(variant: CASPER_SRAM_PPD_A) -> Self {
        variant as u8 != 0
    }
}
impl CASPER_SRAM_PPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CASPER_SRAM_PPD_A {
        match self.bits {
            false => CASPER_SRAM_PPD_A::CASPER_SRAM_PPD_0,
            true => CASPER_SRAM_PPD_A::CASPER_SRAM_PPD_1,
        }
    }
    #[doc = "Checks if the value of the field is `CASPER_SRAM_PPD_0`"]
    #[inline(always)]
    pub fn is_casper_sram_ppd_0(&self) -> bool {
        *self == CASPER_SRAM_PPD_A::CASPER_SRAM_PPD_0
    }
    #[doc = "Checks if the value of the field is `CASPER_SRAM_PPD_1`"]
    #[inline(always)]
    pub fn is_casper_sram_ppd_1(&self) -> bool {
        *self == CASPER_SRAM_PPD_A::CASPER_SRAM_PPD_1
    }
}
#[doc = "Field `CASPER_SRAM_PPD` writer - Periphery power for Casper RAM"]
pub type CASPER_SRAM_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG1_SPEC, CASPER_SRAM_PPD_A, O>;
impl<'a, const O: u8> CASPER_SRAM_PPD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn casper_sram_ppd_0(self) -> &'a mut W {
        self.variant(CASPER_SRAM_PPD_A::CASPER_SRAM_PPD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn casper_sram_ppd_1(self) -> &'a mut W {
        self.variant(CASPER_SRAM_PPD_A::CASPER_SRAM_PPD_1)
    }
}
#[doc = "Field `GPU_SRAM_APD` reader - Array Power for GPU SRAM"]
pub type GPU_SRAM_APD_R = crate::BitReader<GPU_SRAM_APD_A>;
#[doc = "Array Power for GPU SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPU_SRAM_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    GPU_SRAM_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    GPU_SRAM_APD_1 = 1,
}
impl From<GPU_SRAM_APD_A> for bool {
    #[inline(always)]
    fn from(variant: GPU_SRAM_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl GPU_SRAM_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPU_SRAM_APD_A {
        match self.bits {
            false => GPU_SRAM_APD_A::GPU_SRAM_APD_0,
            true => GPU_SRAM_APD_A::GPU_SRAM_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPU_SRAM_APD_0`"]
    #[inline(always)]
    pub fn is_gpu_sram_apd_0(&self) -> bool {
        *self == GPU_SRAM_APD_A::GPU_SRAM_APD_0
    }
    #[doc = "Checks if the value of the field is `GPU_SRAM_APD_1`"]
    #[inline(always)]
    pub fn is_gpu_sram_apd_1(&self) -> bool {
        *self == GPU_SRAM_APD_A::GPU_SRAM_APD_1
    }
}
#[doc = "Field `GPU_SRAM_APD` writer - Array Power for GPU SRAM"]
pub type GPU_SRAM_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG1_SPEC, GPU_SRAM_APD_A, O>;
impl<'a, const O: u8> GPU_SRAM_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn gpu_sram_apd_0(self) -> &'a mut W {
        self.variant(GPU_SRAM_APD_A::GPU_SRAM_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn gpu_sram_apd_1(self) -> &'a mut W {
        self.variant(GPU_SRAM_APD_A::GPU_SRAM_APD_1)
    }
}
#[doc = "Field `GPU_SRAM_PPD` reader - Periphery Power for GPU SRAM"]
pub type GPU_SRAM_PPD_R = crate::BitReader<GPU_SRAM_PPD_A>;
#[doc = "Periphery Power for GPU SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPU_SRAM_PPD_A {
    #[doc = "0: Enable"]
    GPU_SRAM_PPD0 = 0,
    #[doc = "1: Powerdown"]
    GPU_SRAM_PPD1 = 1,
}
impl From<GPU_SRAM_PPD_A> for bool {
    #[inline(always)]
    fn from(variant: GPU_SRAM_PPD_A) -> Self {
        variant as u8 != 0
    }
}
impl GPU_SRAM_PPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPU_SRAM_PPD_A {
        match self.bits {
            false => GPU_SRAM_PPD_A::GPU_SRAM_PPD0,
            true => GPU_SRAM_PPD_A::GPU_SRAM_PPD1,
        }
    }
    #[doc = "Checks if the value of the field is `GPU_SRAM_PPD0`"]
    #[inline(always)]
    pub fn is_gpu_sram_ppd0(&self) -> bool {
        *self == GPU_SRAM_PPD_A::GPU_SRAM_PPD0
    }
    #[doc = "Checks if the value of the field is `GPU_SRAM_PPD1`"]
    #[inline(always)]
    pub fn is_gpu_sram_ppd1(&self) -> bool {
        *self == GPU_SRAM_PPD_A::GPU_SRAM_PPD1
    }
}
#[doc = "Field `GPU_SRAM_PPD` writer - Periphery Power for GPU SRAM"]
pub type GPU_SRAM_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG1_SPEC, GPU_SRAM_PPD_A, O>;
impl<'a, const O: u8> GPU_SRAM_PPD_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn gpu_sram_ppd0(self) -> &'a mut W {
        self.variant(GPU_SRAM_PPD_A::GPU_SRAM_PPD0)
    }
    #[doc = "Powerdown"]
    #[inline(always)]
    pub fn gpu_sram_ppd1(self) -> &'a mut W {
        self.variant(GPU_SRAM_PPD_A::GPU_SRAM_PPD1)
    }
}
#[doc = "Field `SMARTDMA_SRAM_APD` reader - Array Power for SMARTDMA SRAM"]
pub type SMARTDMA_SRAM_APD_R = crate::BitReader<SMARTDMA_SRAM_APD_A>;
#[doc = "Array Power for SMARTDMA SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMARTDMA_SRAM_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    SMARTDMA_SRAM_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    SMARTDMA_SRAM_APD_1 = 1,
}
impl From<SMARTDMA_SRAM_APD_A> for bool {
    #[inline(always)]
    fn from(variant: SMARTDMA_SRAM_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl SMARTDMA_SRAM_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMARTDMA_SRAM_APD_A {
        match self.bits {
            false => SMARTDMA_SRAM_APD_A::SMARTDMA_SRAM_APD_0,
            true => SMARTDMA_SRAM_APD_A::SMARTDMA_SRAM_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `SMARTDMA_SRAM_APD_0`"]
    #[inline(always)]
    pub fn is_smartdma_sram_apd_0(&self) -> bool {
        *self == SMARTDMA_SRAM_APD_A::SMARTDMA_SRAM_APD_0
    }
    #[doc = "Checks if the value of the field is `SMARTDMA_SRAM_APD_1`"]
    #[inline(always)]
    pub fn is_smartdma_sram_apd_1(&self) -> bool {
        *self == SMARTDMA_SRAM_APD_A::SMARTDMA_SRAM_APD_1
    }
}
#[doc = "Field `SMARTDMA_SRAM_APD` writer - Array Power for SMARTDMA SRAM"]
pub type SMARTDMA_SRAM_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG1_SPEC, SMARTDMA_SRAM_APD_A, O>;
impl<'a, const O: u8> SMARTDMA_SRAM_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn smartdma_sram_apd_0(self) -> &'a mut W {
        self.variant(SMARTDMA_SRAM_APD_A::SMARTDMA_SRAM_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn smartdma_sram_apd_1(self) -> &'a mut W {
        self.variant(SMARTDMA_SRAM_APD_A::SMARTDMA_SRAM_APD_1)
    }
}
#[doc = "Field `SMARTDMA_SRAM_PPD` reader - Periphery Power for SMARTDMA SRAM"]
pub type SMARTDMA_SRAM_PPD_R = crate::BitReader<SMARTDMA_SRAM_PPD_A>;
#[doc = "Periphery Power for SMARTDMA SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMARTDMA_SRAM_PPD_A {
    #[doc = "0: Enable"]
    SMARTDMA_SRAM_PPD0 = 0,
    #[doc = "1: Powerdown"]
    SMARTDMA_SRAM_PPD1 = 1,
}
impl From<SMARTDMA_SRAM_PPD_A> for bool {
    #[inline(always)]
    fn from(variant: SMARTDMA_SRAM_PPD_A) -> Self {
        variant as u8 != 0
    }
}
impl SMARTDMA_SRAM_PPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMARTDMA_SRAM_PPD_A {
        match self.bits {
            false => SMARTDMA_SRAM_PPD_A::SMARTDMA_SRAM_PPD0,
            true => SMARTDMA_SRAM_PPD_A::SMARTDMA_SRAM_PPD1,
        }
    }
    #[doc = "Checks if the value of the field is `SMARTDMA_SRAM_PPD0`"]
    #[inline(always)]
    pub fn is_smartdma_sram_ppd0(&self) -> bool {
        *self == SMARTDMA_SRAM_PPD_A::SMARTDMA_SRAM_PPD0
    }
    #[doc = "Checks if the value of the field is `SMARTDMA_SRAM_PPD1`"]
    #[inline(always)]
    pub fn is_smartdma_sram_ppd1(&self) -> bool {
        *self == SMARTDMA_SRAM_PPD_A::SMARTDMA_SRAM_PPD1
    }
}
#[doc = "Field `SMARTDMA_SRAM_PPD` writer - Periphery Power for SMARTDMA SRAM"]
pub type SMARTDMA_SRAM_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG1_SPEC, SMARTDMA_SRAM_PPD_A, O>;
impl<'a, const O: u8> SMARTDMA_SRAM_PPD_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn smartdma_sram_ppd0(self) -> &'a mut W {
        self.variant(SMARTDMA_SRAM_PPD_A::SMARTDMA_SRAM_PPD0)
    }
    #[doc = "Powerdown"]
    #[inline(always)]
    pub fn smartdma_sram_ppd1(self) -> &'a mut W {
        self.variant(SMARTDMA_SRAM_PPD_A::SMARTDMA_SRAM_PPD1)
    }
}
#[doc = "Field `MIPIDSI_SRAM_APD` reader - Array Power for MIPIDSI SRAM"]
pub type MIPIDSI_SRAM_APD_R = crate::BitReader<MIPIDSI_SRAM_APD_A>;
#[doc = "Array Power for MIPIDSI SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIPIDSI_SRAM_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    MIPIDSI_SRAM_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    MIPIDSI_SRAM_APD_1 = 1,
}
impl From<MIPIDSI_SRAM_APD_A> for bool {
    #[inline(always)]
    fn from(variant: MIPIDSI_SRAM_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl MIPIDSI_SRAM_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIPIDSI_SRAM_APD_A {
        match self.bits {
            false => MIPIDSI_SRAM_APD_A::MIPIDSI_SRAM_APD_0,
            true => MIPIDSI_SRAM_APD_A::MIPIDSI_SRAM_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `MIPIDSI_SRAM_APD_0`"]
    #[inline(always)]
    pub fn is_mipidsi_sram_apd_0(&self) -> bool {
        *self == MIPIDSI_SRAM_APD_A::MIPIDSI_SRAM_APD_0
    }
    #[doc = "Checks if the value of the field is `MIPIDSI_SRAM_APD_1`"]
    #[inline(always)]
    pub fn is_mipidsi_sram_apd_1(&self) -> bool {
        *self == MIPIDSI_SRAM_APD_A::MIPIDSI_SRAM_APD_1
    }
}
#[doc = "Field `MIPIDSI_SRAM_APD` writer - Array Power for MIPIDSI SRAM"]
pub type MIPIDSI_SRAM_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG1_SPEC, MIPIDSI_SRAM_APD_A, O>;
impl<'a, const O: u8> MIPIDSI_SRAM_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn mipidsi_sram_apd_0(self) -> &'a mut W {
        self.variant(MIPIDSI_SRAM_APD_A::MIPIDSI_SRAM_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn mipidsi_sram_apd_1(self) -> &'a mut W {
        self.variant(MIPIDSI_SRAM_APD_A::MIPIDSI_SRAM_APD_1)
    }
}
#[doc = "Field `MIPIDSI_SRAM_PPD` reader - Periphery Power for MIPIDSI SRAM"]
pub type MIPIDSI_SRAM_PPD_R = crate::BitReader<MIPIDSI_SRAM_PPD_A>;
#[doc = "Periphery Power for MIPIDSI SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIPIDSI_SRAM_PPD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    MIPIDSI_SRAM_PPD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    MIPIDSI_SRAM_PPD_1 = 1,
}
impl From<MIPIDSI_SRAM_PPD_A> for bool {
    #[inline(always)]
    fn from(variant: MIPIDSI_SRAM_PPD_A) -> Self {
        variant as u8 != 0
    }
}
impl MIPIDSI_SRAM_PPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIPIDSI_SRAM_PPD_A {
        match self.bits {
            false => MIPIDSI_SRAM_PPD_A::MIPIDSI_SRAM_PPD_0,
            true => MIPIDSI_SRAM_PPD_A::MIPIDSI_SRAM_PPD_1,
        }
    }
    #[doc = "Checks if the value of the field is `MIPIDSI_SRAM_PPD_0`"]
    #[inline(always)]
    pub fn is_mipidsi_sram_ppd_0(&self) -> bool {
        *self == MIPIDSI_SRAM_PPD_A::MIPIDSI_SRAM_PPD_0
    }
    #[doc = "Checks if the value of the field is `MIPIDSI_SRAM_PPD_1`"]
    #[inline(always)]
    pub fn is_mipidsi_sram_ppd_1(&self) -> bool {
        *self == MIPIDSI_SRAM_PPD_A::MIPIDSI_SRAM_PPD_1
    }
}
#[doc = "Field `MIPIDSI_SRAM_PPD` writer - Periphery Power for MIPIDSI SRAM"]
pub type MIPIDSI_SRAM_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG1_SPEC, MIPIDSI_SRAM_PPD_A, O>;
impl<'a, const O: u8> MIPIDSI_SRAM_PPD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn mipidsi_sram_ppd_0(self) -> &'a mut W {
        self.variant(MIPIDSI_SRAM_PPD_A::MIPIDSI_SRAM_PPD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn mipidsi_sram_ppd_1(self) -> &'a mut W {
        self.variant(MIPIDSI_SRAM_PPD_A::MIPIDSI_SRAM_PPD_1)
    }
}
#[doc = "Field `LCDIF_SRAM_APD` reader - Array Power for LCDIF SRAM"]
pub type LCDIF_SRAM_APD_R = crate::BitReader<LCDIF_SRAM_APD_A>;
#[doc = "Array Power for LCDIF SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDIF_SRAM_APD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    LCDIF_SRAM_APD_0 = 0,
    #[doc = "1: Power down enabled or Powered OFF"]
    LCDIF_SRAM_APD_1 = 1,
}
impl From<LCDIF_SRAM_APD_A> for bool {
    #[inline(always)]
    fn from(variant: LCDIF_SRAM_APD_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDIF_SRAM_APD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDIF_SRAM_APD_A {
        match self.bits {
            false => LCDIF_SRAM_APD_A::LCDIF_SRAM_APD_0,
            true => LCDIF_SRAM_APD_A::LCDIF_SRAM_APD_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDIF_SRAM_APD_0`"]
    #[inline(always)]
    pub fn is_lcdif_sram_apd_0(&self) -> bool {
        *self == LCDIF_SRAM_APD_A::LCDIF_SRAM_APD_0
    }
    #[doc = "Checks if the value of the field is `LCDIF_SRAM_APD_1`"]
    #[inline(always)]
    pub fn is_lcdif_sram_apd_1(&self) -> bool {
        *self == LCDIF_SRAM_APD_A::LCDIF_SRAM_APD_1
    }
}
#[doc = "Field `LCDIF_SRAM_APD` writer - Array Power for LCDIF SRAM"]
pub type LCDIF_SRAM_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG1_SPEC, LCDIF_SRAM_APD_A, O>;
impl<'a, const O: u8> LCDIF_SRAM_APD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn lcdif_sram_apd_0(self) -> &'a mut W {
        self.variant(LCDIF_SRAM_APD_A::LCDIF_SRAM_APD_0)
    }
    #[doc = "Power down enabled or Powered OFF"]
    #[inline(always)]
    pub fn lcdif_sram_apd_1(self) -> &'a mut W {
        self.variant(LCDIF_SRAM_APD_A::LCDIF_SRAM_APD_1)
    }
}
#[doc = "Field `LCDIF_SRAM_PPD` reader - Periphery Power for LCDIF SRAM"]
pub type LCDIF_SRAM_PPD_R = crate::BitReader<LCDIF_SRAM_PPD_A>;
#[doc = "Periphery Power for LCDIF SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDIF_SRAM_PPD_A {
    #[doc = "0: Power down disabled or Powered ON"]
    LCDIF_SRAM_PPD_0 = 0,
    #[doc = "1: Powerdown"]
    _LCDIF_SRAM_PPD1_1 = 1,
}
impl From<LCDIF_SRAM_PPD_A> for bool {
    #[inline(always)]
    fn from(variant: LCDIF_SRAM_PPD_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDIF_SRAM_PPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDIF_SRAM_PPD_A {
        match self.bits {
            false => LCDIF_SRAM_PPD_A::LCDIF_SRAM_PPD_0,
            true => LCDIF_SRAM_PPD_A::_LCDIF_SRAM_PPD1_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDIF_SRAM_PPD_0`"]
    #[inline(always)]
    pub fn is_lcdif_sram_ppd_0(&self) -> bool {
        *self == LCDIF_SRAM_PPD_A::LCDIF_SRAM_PPD_0
    }
    #[doc = "Checks if the value of the field is `_LCDIF_SRAM_PPD1_1`"]
    #[inline(always)]
    pub fn is_lcdif_sram_ppd1_1(&self) -> bool {
        *self == LCDIF_SRAM_PPD_A::_LCDIF_SRAM_PPD1_1
    }
}
#[doc = "Field `LCDIF_SRAM_PPD` writer - Periphery Power for LCDIF SRAM"]
pub type LCDIF_SRAM_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG1_SPEC, LCDIF_SRAM_PPD_A, O>;
impl<'a, const O: u8> LCDIF_SRAM_PPD_W<'a, O> {
    #[doc = "Power down disabled or Powered ON"]
    #[inline(always)]
    pub fn lcdif_sram_ppd_0(self) -> &'a mut W {
        self.variant(LCDIF_SRAM_PPD_A::LCDIF_SRAM_PPD_0)
    }
    #[doc = "Powerdown"]
    #[inline(always)]
    pub fn _lcdif_sram_ppd1_1(self) -> &'a mut W {
        self.variant(LCDIF_SRAM_PPD_A::_LCDIF_SRAM_PPD1_1)
    }
}
#[doc = "Field `DSP_PD` reader - DSP"]
pub type DSP_PD_R = crate::BitReader<DSP_PD_A>;
#[doc = "DSP\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSP_PD_A {
    #[doc = "0: DSP not power gated"]
    DSP_PD_0 = 0,
    #[doc = "1: DSP power gated"]
    DSP_PD_1 = 1,
}
impl From<DSP_PD_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl DSP_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_PD_A {
        match self.bits {
            false => DSP_PD_A::DSP_PD_0,
            true => DSP_PD_A::DSP_PD_1,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_PD_0`"]
    #[inline(always)]
    pub fn is_dsp_pd_0(&self) -> bool {
        *self == DSP_PD_A::DSP_PD_0
    }
    #[doc = "Checks if the value of the field is `DSP_PD_1`"]
    #[inline(always)]
    pub fn is_dsp_pd_1(&self) -> bool {
        *self == DSP_PD_A::DSP_PD_1
    }
}
#[doc = "Field `DSP_PD` writer - DSP"]
pub type DSP_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDSLEEPCFG1_SPEC, DSP_PD_A, O>;
impl<'a, const O: u8> DSP_PD_W<'a, O> {
    #[doc = "DSP not power gated"]
    #[inline(always)]
    pub fn dsp_pd_0(self) -> &'a mut W {
        self.variant(DSP_PD_A::DSP_PD_0)
    }
    #[doc = "DSP power gated"]
    #[inline(always)]
    pub fn dsp_pd_1(self) -> &'a mut W {
        self.variant(DSP_PD_A::DSP_PD_1)
    }
}
#[doc = "Field `MIPIDSI_PD` reader - MIPIDSI"]
pub type MIPIDSI_PD_R = crate::BitReader<MIPIDSI_PD_A>;
#[doc = "MIPIDSI\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIPIDSI_PD_A {
    #[doc = "0: MIPI DSI not power gated"]
    MIPIDSI_PD_0 = 0,
    #[doc = "1: MIPI DSI power gated"]
    MIPIDSI_PD_1 = 1,
}
impl From<MIPIDSI_PD_A> for bool {
    #[inline(always)]
    fn from(variant: MIPIDSI_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl MIPIDSI_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIPIDSI_PD_A {
        match self.bits {
            false => MIPIDSI_PD_A::MIPIDSI_PD_0,
            true => MIPIDSI_PD_A::MIPIDSI_PD_1,
        }
    }
    #[doc = "Checks if the value of the field is `MIPIDSI_PD_0`"]
    #[inline(always)]
    pub fn is_mipidsi_pd_0(&self) -> bool {
        *self == MIPIDSI_PD_A::MIPIDSI_PD_0
    }
    #[doc = "Checks if the value of the field is `MIPIDSI_PD_1`"]
    #[inline(always)]
    pub fn is_mipidsi_pd_1(&self) -> bool {
        *self == MIPIDSI_PD_A::MIPIDSI_PD_1
    }
}
#[doc = "Field `MIPIDSI_PD` writer - MIPIDSI"]
pub type MIPIDSI_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG1_SPEC, MIPIDSI_PD_A, O>;
impl<'a, const O: u8> MIPIDSI_PD_W<'a, O> {
    #[doc = "MIPI DSI not power gated"]
    #[inline(always)]
    pub fn mipidsi_pd_0(self) -> &'a mut W {
        self.variant(MIPIDSI_PD_A::MIPIDSI_PD_0)
    }
    #[doc = "MIPI DSI power gated"]
    #[inline(always)]
    pub fn mipidsi_pd_1(self) -> &'a mut W {
        self.variant(MIPIDSI_PD_A::MIPIDSI_PD_1)
    }
}
#[doc = "Field `OTP_PD` reader - OTP"]
pub type OTP_PD_R = crate::BitReader<OTP_PD_A>;
#[doc = "OTP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTP_PD_A {
    #[doc = "0: Powered"]
    OTP_PD_0 = 0,
    #[doc = "1: Not Powered"]
    OTP_PD_1 = 1,
}
impl From<OTP_PD_A> for bool {
    #[inline(always)]
    fn from(variant: OTP_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl OTP_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTP_PD_A {
        match self.bits {
            false => OTP_PD_A::OTP_PD_0,
            true => OTP_PD_A::OTP_PD_1,
        }
    }
    #[doc = "Checks if the value of the field is `OTP_PD_0`"]
    #[inline(always)]
    pub fn is_otp_pd_0(&self) -> bool {
        *self == OTP_PD_A::OTP_PD_0
    }
    #[doc = "Checks if the value of the field is `OTP_PD_1`"]
    #[inline(always)]
    pub fn is_otp_pd_1(&self) -> bool {
        *self == OTP_PD_A::OTP_PD_1
    }
}
#[doc = "Field `OTP_PD` writer - OTP"]
pub type OTP_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDSLEEPCFG1_SPEC, OTP_PD_A, O>;
impl<'a, const O: u8> OTP_PD_W<'a, O> {
    #[doc = "Powered"]
    #[inline(always)]
    pub fn otp_pd_0(self) -> &'a mut W {
        self.variant(OTP_PD_A::OTP_PD_0)
    }
    #[doc = "Not Powered"]
    #[inline(always)]
    pub fn otp_pd_1(self) -> &'a mut W {
        self.variant(OTP_PD_A::OTP_PD_1)
    }
}
#[doc = "Field `ROM_PD` reader - ROM"]
pub type ROM_PD_R = crate::BitReader<ROM_PD_A>;
#[doc = "ROM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROM_PD_A {
    #[doc = "0: ROM Powered"]
    ROM_PD_0 = 0,
    #[doc = "1: ROM not Powered"]
    ROM_PD_1 = 1,
}
impl From<ROM_PD_A> for bool {
    #[inline(always)]
    fn from(variant: ROM_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl ROM_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROM_PD_A {
        match self.bits {
            false => ROM_PD_A::ROM_PD_0,
            true => ROM_PD_A::ROM_PD_1,
        }
    }
    #[doc = "Checks if the value of the field is `ROM_PD_0`"]
    #[inline(always)]
    pub fn is_rom_pd_0(&self) -> bool {
        *self == ROM_PD_A::ROM_PD_0
    }
    #[doc = "Checks if the value of the field is `ROM_PD_1`"]
    #[inline(always)]
    pub fn is_rom_pd_1(&self) -> bool {
        *self == ROM_PD_A::ROM_PD_1
    }
}
#[doc = "Field `ROM_PD` writer - ROM"]
pub type ROM_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDSLEEPCFG1_SPEC, ROM_PD_A, O>;
impl<'a, const O: u8> ROM_PD_W<'a, O> {
    #[doc = "ROM Powered"]
    #[inline(always)]
    pub fn rom_pd_0(self) -> &'a mut W {
        self.variant(ROM_PD_A::ROM_PD_0)
    }
    #[doc = "ROM not Powered"]
    #[inline(always)]
    pub fn rom_pd_1(self) -> &'a mut W {
        self.variant(ROM_PD_A::ROM_PD_1)
    }
}
#[doc = "Field `SRAM_SLEEP` reader - SRAM sleep mode"]
pub type SRAM_SLEEP_R = crate::BitReader<SRAM_SLEEP_A>;
#[doc = "SRAM sleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_SLEEP_A {
    #[doc = "0: RAM Normal mode"]
    SRAM_SLEEP_0 = 0,
    #[doc = "1: RAM Sleep mode. Needed when vddcore can be < 0.6V to ensure contents retained. Memories not accessible in this mode."]
    SRAM_SLEEP_1 = 1,
}
impl From<SRAM_SLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_SLEEP_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_SLEEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_SLEEP_A {
        match self.bits {
            false => SRAM_SLEEP_A::SRAM_SLEEP_0,
            true => SRAM_SLEEP_A::SRAM_SLEEP_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM_SLEEP_0`"]
    #[inline(always)]
    pub fn is_sram_sleep_0(&self) -> bool {
        *self == SRAM_SLEEP_A::SRAM_SLEEP_0
    }
    #[doc = "Checks if the value of the field is `SRAM_SLEEP_1`"]
    #[inline(always)]
    pub fn is_sram_sleep_1(&self) -> bool {
        *self == SRAM_SLEEP_A::SRAM_SLEEP_1
    }
}
#[doc = "Field `SRAM_SLEEP` writer - SRAM sleep mode"]
pub type SRAM_SLEEP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDSLEEPCFG1_SPEC, SRAM_SLEEP_A, O>;
impl<'a, const O: u8> SRAM_SLEEP_W<'a, O> {
    #[doc = "RAM Normal mode"]
    #[inline(always)]
    pub fn sram_sleep_0(self) -> &'a mut W {
        self.variant(SRAM_SLEEP_A::SRAM_SLEEP_0)
    }
    #[doc = "RAM Sleep mode. Needed when vddcore can be < 0.6V to ensure contents retained. Memories not accessible in this mode."]
    #[inline(always)]
    pub fn sram_sleep_1(self) -> &'a mut W {
        self.variant(SRAM_SLEEP_A::SRAM_SLEEP_1)
    }
}
impl R {
    #[doc = "Bit 1 - Periphery power for PowerQuad RAM"]
    #[inline(always)]
    pub fn pq_sram_ppd(&self) -> PQ_SRAM_PPD_R {
        PQ_SRAM_PPD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Array power for FLEXSPI0 RAM"]
    #[inline(always)]
    pub fn flexspi0_sram_apd(&self) -> FLEXSPI0_SRAM_APD_R {
        FLEXSPI0_SRAM_APD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Periphery power for FLEXSPI0 RAM"]
    #[inline(always)]
    pub fn flexspi0_sram_ppd(&self) -> FLEXSPI0_SRAM_PPD_R {
        FLEXSPI0_SRAM_PPD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Array power for FLEXSPI1 RAM"]
    #[inline(always)]
    pub fn flexspi1_sram_apd(&self) -> FLEXSPI1_SRAM_APD_R {
        FLEXSPI1_SRAM_APD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Periphery power for FLEXSPI1 RAM"]
    #[inline(always)]
    pub fn flexspi1_sram_ppd(&self) -> FLEXSPI1_SRAM_PPD_R {
        FLEXSPI1_SRAM_PPD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Array power for USB RAM"]
    #[inline(always)]
    pub fn usbhs_sram_apd(&self) -> USBHS_SRAM_APD_R {
        USBHS_SRAM_APD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Periphery power for USB RAM"]
    #[inline(always)]
    pub fn usbhs_sram_ppd(&self) -> USBHS_SRAM_PPD_R {
        USBHS_SRAM_PPD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Array power for uSDHC0 (SD/MMC/SDIO interface) RAM"]
    #[inline(always)]
    pub fn usdhc0_sram_apd(&self) -> USDHC0_SRAM_APD_R {
        USDHC0_SRAM_APD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Periphery power for uSDHC0 (SD/MMC/SDIO interface) RAM"]
    #[inline(always)]
    pub fn usdhc0_sram_ppd(&self) -> USDHC0_SRAM_PPD_R {
        USDHC0_SRAM_PPD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Array power for Casper RAM"]
    #[inline(always)]
    pub fn usdhc1_sram_apd(&self) -> USDHC1_SRAM_APD_R {
        USDHC1_SRAM_APD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Periphery power for uSDHC1 (SD/MMC/SDIO interface) RAM"]
    #[inline(always)]
    pub fn usdhc1_sram_ppd(&self) -> USDHC1_SRAM_PPD_R {
        USDHC1_SRAM_PPD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Periphery power for Casper RAM"]
    #[inline(always)]
    pub fn casper_sram_ppd(&self) -> CASPER_SRAM_PPD_R {
        CASPER_SRAM_PPD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Array Power for GPU SRAM"]
    #[inline(always)]
    pub fn gpu_sram_apd(&self) -> GPU_SRAM_APD_R {
        GPU_SRAM_APD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Periphery Power for GPU SRAM"]
    #[inline(always)]
    pub fn gpu_sram_ppd(&self) -> GPU_SRAM_PPD_R {
        GPU_SRAM_PPD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Array Power for SMARTDMA SRAM"]
    #[inline(always)]
    pub fn smartdma_sram_apd(&self) -> SMARTDMA_SRAM_APD_R {
        SMARTDMA_SRAM_APD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Periphery Power for SMARTDMA SRAM"]
    #[inline(always)]
    pub fn smartdma_sram_ppd(&self) -> SMARTDMA_SRAM_PPD_R {
        SMARTDMA_SRAM_PPD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Array Power for MIPIDSI SRAM"]
    #[inline(always)]
    pub fn mipidsi_sram_apd(&self) -> MIPIDSI_SRAM_APD_R {
        MIPIDSI_SRAM_APD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Periphery Power for MIPIDSI SRAM"]
    #[inline(always)]
    pub fn mipidsi_sram_ppd(&self) -> MIPIDSI_SRAM_PPD_R {
        MIPIDSI_SRAM_PPD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Array Power for LCDIF SRAM"]
    #[inline(always)]
    pub fn lcdif_sram_apd(&self) -> LCDIF_SRAM_APD_R {
        LCDIF_SRAM_APD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Periphery Power for LCDIF SRAM"]
    #[inline(always)]
    pub fn lcdif_sram_ppd(&self) -> LCDIF_SRAM_PPD_R {
        LCDIF_SRAM_PPD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 25 - DSP"]
    #[inline(always)]
    pub fn dsp_pd(&self) -> DSP_PD_R {
        DSP_PD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - MIPIDSI"]
    #[inline(always)]
    pub fn mipidsi_pd(&self) -> MIPIDSI_PD_R {
        MIPIDSI_PD_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - OTP"]
    #[inline(always)]
    pub fn otp_pd(&self) -> OTP_PD_R {
        OTP_PD_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - ROM"]
    #[inline(always)]
    pub fn rom_pd(&self) -> ROM_PD_R {
        ROM_PD_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - SRAM sleep mode"]
    #[inline(always)]
    pub fn sram_sleep(&self) -> SRAM_SLEEP_R {
        SRAM_SLEEP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Periphery power for PowerQuad RAM"]
    #[inline(always)]
    #[must_use]
    pub fn pq_sram_ppd(&mut self) -> PQ_SRAM_PPD_W<1> {
        PQ_SRAM_PPD_W::new(self)
    }
    #[doc = "Bit 2 - Array power for FLEXSPI0 RAM"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi0_sram_apd(&mut self) -> FLEXSPI0_SRAM_APD_W<2> {
        FLEXSPI0_SRAM_APD_W::new(self)
    }
    #[doc = "Bit 3 - Periphery power for FLEXSPI0 RAM"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi0_sram_ppd(&mut self) -> FLEXSPI0_SRAM_PPD_W<3> {
        FLEXSPI0_SRAM_PPD_W::new(self)
    }
    #[doc = "Bit 4 - Array power for FLEXSPI1 RAM"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi1_sram_apd(&mut self) -> FLEXSPI1_SRAM_APD_W<4> {
        FLEXSPI1_SRAM_APD_W::new(self)
    }
    #[doc = "Bit 5 - Periphery power for FLEXSPI1 RAM"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi1_sram_ppd(&mut self) -> FLEXSPI1_SRAM_PPD_W<5> {
        FLEXSPI1_SRAM_PPD_W::new(self)
    }
    #[doc = "Bit 6 - Array power for USB RAM"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_sram_apd(&mut self) -> USBHS_SRAM_APD_W<6> {
        USBHS_SRAM_APD_W::new(self)
    }
    #[doc = "Bit 7 - Periphery power for USB RAM"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_sram_ppd(&mut self) -> USBHS_SRAM_PPD_W<7> {
        USBHS_SRAM_PPD_W::new(self)
    }
    #[doc = "Bit 8 - Array power for uSDHC0 (SD/MMC/SDIO interface) RAM"]
    #[inline(always)]
    #[must_use]
    pub fn usdhc0_sram_apd(&mut self) -> USDHC0_SRAM_APD_W<8> {
        USDHC0_SRAM_APD_W::new(self)
    }
    #[doc = "Bit 9 - Periphery power for uSDHC0 (SD/MMC/SDIO interface) RAM"]
    #[inline(always)]
    #[must_use]
    pub fn usdhc0_sram_ppd(&mut self) -> USDHC0_SRAM_PPD_W<9> {
        USDHC0_SRAM_PPD_W::new(self)
    }
    #[doc = "Bit 10 - Array power for Casper RAM"]
    #[inline(always)]
    #[must_use]
    pub fn usdhc1_sram_apd(&mut self) -> USDHC1_SRAM_APD_W<10> {
        USDHC1_SRAM_APD_W::new(self)
    }
    #[doc = "Bit 11 - Periphery power for uSDHC1 (SD/MMC/SDIO interface) RAM"]
    #[inline(always)]
    #[must_use]
    pub fn usdhc1_sram_ppd(&mut self) -> USDHC1_SRAM_PPD_W<11> {
        USDHC1_SRAM_PPD_W::new(self)
    }
    #[doc = "Bit 13 - Periphery power for Casper RAM"]
    #[inline(always)]
    #[must_use]
    pub fn casper_sram_ppd(&mut self) -> CASPER_SRAM_PPD_W<13> {
        CASPER_SRAM_PPD_W::new(self)
    }
    #[doc = "Bit 14 - Array Power for GPU SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn gpu_sram_apd(&mut self) -> GPU_SRAM_APD_W<14> {
        GPU_SRAM_APD_W::new(self)
    }
    #[doc = "Bit 15 - Periphery Power for GPU SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn gpu_sram_ppd(&mut self) -> GPU_SRAM_PPD_W<15> {
        GPU_SRAM_PPD_W::new(self)
    }
    #[doc = "Bit 16 - Array Power for SMARTDMA SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn smartdma_sram_apd(&mut self) -> SMARTDMA_SRAM_APD_W<16> {
        SMARTDMA_SRAM_APD_W::new(self)
    }
    #[doc = "Bit 17 - Periphery Power for SMARTDMA SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn smartdma_sram_ppd(&mut self) -> SMARTDMA_SRAM_PPD_W<17> {
        SMARTDMA_SRAM_PPD_W::new(self)
    }
    #[doc = "Bit 18 - Array Power for MIPIDSI SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn mipidsi_sram_apd(&mut self) -> MIPIDSI_SRAM_APD_W<18> {
        MIPIDSI_SRAM_APD_W::new(self)
    }
    #[doc = "Bit 19 - Periphery Power for MIPIDSI SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn mipidsi_sram_ppd(&mut self) -> MIPIDSI_SRAM_PPD_W<19> {
        MIPIDSI_SRAM_PPD_W::new(self)
    }
    #[doc = "Bit 20 - Array Power for LCDIF SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn lcdif_sram_apd(&mut self) -> LCDIF_SRAM_APD_W<20> {
        LCDIF_SRAM_APD_W::new(self)
    }
    #[doc = "Bit 21 - Periphery Power for LCDIF SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn lcdif_sram_ppd(&mut self) -> LCDIF_SRAM_PPD_W<21> {
        LCDIF_SRAM_PPD_W::new(self)
    }
    #[doc = "Bit 25 - DSP"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_pd(&mut self) -> DSP_PD_W<25> {
        DSP_PD_W::new(self)
    }
    #[doc = "Bit 26 - MIPIDSI"]
    #[inline(always)]
    #[must_use]
    pub fn mipidsi_pd(&mut self) -> MIPIDSI_PD_W<26> {
        MIPIDSI_PD_W::new(self)
    }
    #[doc = "Bit 27 - OTP"]
    #[inline(always)]
    #[must_use]
    pub fn otp_pd(&mut self) -> OTP_PD_W<27> {
        OTP_PD_W::new(self)
    }
    #[doc = "Bit 28 - ROM"]
    #[inline(always)]
    #[must_use]
    pub fn rom_pd(&mut self) -> ROM_PD_W<28> {
        ROM_PD_W::new(self)
    }
    #[doc = "Bit 31 - SRAM sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram_sleep(&mut self) -> SRAM_SLEEP_W<31> {
        SRAM_SLEEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sleep configuration 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdsleepcfg1](index.html) module"]
pub struct PDSLEEPCFG1_SPEC;
impl crate::RegisterSpec for PDSLEEPCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdsleepcfg1::R](R) reader structure"]
impl crate::Readable for PDSLEEPCFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdsleepcfg1::W](W) writer structure"]
impl crate::Writable for PDSLEEPCFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDSLEEPCFG1 to value 0x067f_ffff"]
impl crate::Resettable for PDSLEEPCFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x067f_ffff;
}
