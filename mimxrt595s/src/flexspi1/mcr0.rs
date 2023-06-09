#[doc = "Register `MCR0` reader"]
pub struct R(crate::R<MCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR0` writer"]
pub struct W(crate::W<MCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR0_SPEC>;
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
impl From<crate::W<MCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRESET` reader - Software Reset"]
pub type SWRESET_R = crate::BitReader<SWRESET_A>;
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRESET_A {
    #[doc = "0: No impact"]
    VAL0 = 0,
    #[doc = "1: Software reset"]
    VAL1 = 1,
}
impl From<SWRESET_A> for bool {
    #[inline(always)]
    fn from(variant: SWRESET_A) -> Self {
        variant as u8 != 0
    }
}
impl SWRESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRESET_A {
        match self.bits {
            false => SWRESET_A::VAL0,
            true => SWRESET_A::VAL1,
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == SWRESET_A::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL1`"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == SWRESET_A::VAL1
    }
}
#[doc = "Field `SWRESET` writer - Software Reset"]
pub type SWRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR0_SPEC, SWRESET_A, O>;
impl<'a, const O: u8> SWRESET_W<'a, O> {
    #[doc = "No impact"]
    #[inline(always)]
    pub fn val0(self) -> &'a mut W {
        self.variant(SWRESET_A::VAL0)
    }
    #[doc = "Software reset"]
    #[inline(always)]
    pub fn val1(self) -> &'a mut W {
        self.variant(SWRESET_A::VAL1)
    }
}
#[doc = "Field `MDIS` reader - Module Disable"]
pub type MDIS_R = crate::BitReader<MDIS_A>;
#[doc = "Module Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDIS_A {
    #[doc = "0: No impact"]
    VAL0 = 0,
    #[doc = "1: Module disable"]
    VAL1 = 1,
}
impl From<MDIS_A> for bool {
    #[inline(always)]
    fn from(variant: MDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl MDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIS_A {
        match self.bits {
            false => MDIS_A::VAL0,
            true => MDIS_A::VAL1,
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == MDIS_A::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL1`"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == MDIS_A::VAL1
    }
}
#[doc = "Field `MDIS` writer - Module Disable"]
pub type MDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR0_SPEC, MDIS_A, O>;
impl<'a, const O: u8> MDIS_W<'a, O> {
    #[doc = "No impact"]
    #[inline(always)]
    pub fn val0(self) -> &'a mut W {
        self.variant(MDIS_A::VAL0)
    }
    #[doc = "Module disable"]
    #[inline(always)]
    pub fn val1(self) -> &'a mut W {
        self.variant(MDIS_A::VAL1)
    }
}
#[doc = "Field `RXCLKSRC` reader - Sample Clock source selection for Flash Reading"]
pub type RXCLKSRC_R = crate::FieldReader<u8, RXCLKSRC_A>;
#[doc = "Sample Clock source selection for Flash Reading\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXCLKSRC_A {
    #[doc = "0: Dummy Read strobe generated by FlexSPI Controller and loopback internally."]
    VAL0 = 0,
    #[doc = "1: Dummy Read strobe generated by FlexSPI Controller and loopback from DQS pad."]
    VAL1 = 1,
    #[doc = "3: Flash provided Read strobe and input from DQS pad"]
    VAL3 = 3,
}
impl From<RXCLKSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: RXCLKSRC_A) -> Self {
        variant as _
    }
}
impl RXCLKSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXCLKSRC_A> {
        match self.bits {
            0 => Some(RXCLKSRC_A::VAL0),
            1 => Some(RXCLKSRC_A::VAL1),
            3 => Some(RXCLKSRC_A::VAL3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == RXCLKSRC_A::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL1`"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == RXCLKSRC_A::VAL1
    }
    #[doc = "Checks if the value of the field is `VAL3`"]
    #[inline(always)]
    pub fn is_val3(&self) -> bool {
        *self == RXCLKSRC_A::VAL3
    }
}
#[doc = "Field `RXCLKSRC` writer - Sample Clock source selection for Flash Reading"]
pub type RXCLKSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCR0_SPEC, u8, RXCLKSRC_A, 2, O>;
impl<'a, const O: u8> RXCLKSRC_W<'a, O> {
    #[doc = "Dummy Read strobe generated by FlexSPI Controller and loopback internally."]
    #[inline(always)]
    pub fn val0(self) -> &'a mut W {
        self.variant(RXCLKSRC_A::VAL0)
    }
    #[doc = "Dummy Read strobe generated by FlexSPI Controller and loopback from DQS pad."]
    #[inline(always)]
    pub fn val1(self) -> &'a mut W {
        self.variant(RXCLKSRC_A::VAL1)
    }
    #[doc = "Flash provided Read strobe and input from DQS pad"]
    #[inline(always)]
    pub fn val3(self) -> &'a mut W {
        self.variant(RXCLKSRC_A::VAL3)
    }
}
#[doc = "Field `SERCLKDIV` reader - Serial root clock"]
pub type SERCLKDIV_R = crate::FieldReader<u8, SERCLKDIV_A>;
#[doc = "Serial root clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SERCLKDIV_A {
    #[doc = "0: Divided by 1"]
    VAL0 = 0,
    #[doc = "1: Divided by 2"]
    VAL1 = 1,
    #[doc = "2: Divided by 3"]
    VAL2 = 2,
    #[doc = "3: Divided by 4"]
    VAL3 = 3,
    #[doc = "4: Divided by 5"]
    VAL4 = 4,
    #[doc = "5: Divided by 6"]
    VAL5 = 5,
    #[doc = "6: Divided by 7"]
    VAL6 = 6,
    #[doc = "7: Divided by 8"]
    VAL7 = 7,
}
impl From<SERCLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: SERCLKDIV_A) -> Self {
        variant as _
    }
}
impl SERCLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SERCLKDIV_A {
        match self.bits {
            0 => SERCLKDIV_A::VAL0,
            1 => SERCLKDIV_A::VAL1,
            2 => SERCLKDIV_A::VAL2,
            3 => SERCLKDIV_A::VAL3,
            4 => SERCLKDIV_A::VAL4,
            5 => SERCLKDIV_A::VAL5,
            6 => SERCLKDIV_A::VAL6,
            7 => SERCLKDIV_A::VAL7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == SERCLKDIV_A::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL1`"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == SERCLKDIV_A::VAL1
    }
    #[doc = "Checks if the value of the field is `VAL2`"]
    #[inline(always)]
    pub fn is_val2(&self) -> bool {
        *self == SERCLKDIV_A::VAL2
    }
    #[doc = "Checks if the value of the field is `VAL3`"]
    #[inline(always)]
    pub fn is_val3(&self) -> bool {
        *self == SERCLKDIV_A::VAL3
    }
    #[doc = "Checks if the value of the field is `VAL4`"]
    #[inline(always)]
    pub fn is_val4(&self) -> bool {
        *self == SERCLKDIV_A::VAL4
    }
    #[doc = "Checks if the value of the field is `VAL5`"]
    #[inline(always)]
    pub fn is_val5(&self) -> bool {
        *self == SERCLKDIV_A::VAL5
    }
    #[doc = "Checks if the value of the field is `VAL6`"]
    #[inline(always)]
    pub fn is_val6(&self) -> bool {
        *self == SERCLKDIV_A::VAL6
    }
    #[doc = "Checks if the value of the field is `VAL7`"]
    #[inline(always)]
    pub fn is_val7(&self) -> bool {
        *self == SERCLKDIV_A::VAL7
    }
}
#[doc = "Field `SERCLKDIV` writer - Serial root clock"]
pub type SERCLKDIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MCR0_SPEC, u8, SERCLKDIV_A, 3, O>;
impl<'a, const O: u8> SERCLKDIV_W<'a, O> {
    #[doc = "Divided by 1"]
    #[inline(always)]
    pub fn val0(self) -> &'a mut W {
        self.variant(SERCLKDIV_A::VAL0)
    }
    #[doc = "Divided by 2"]
    #[inline(always)]
    pub fn val1(self) -> &'a mut W {
        self.variant(SERCLKDIV_A::VAL1)
    }
    #[doc = "Divided by 3"]
    #[inline(always)]
    pub fn val2(self) -> &'a mut W {
        self.variant(SERCLKDIV_A::VAL2)
    }
    #[doc = "Divided by 4"]
    #[inline(always)]
    pub fn val3(self) -> &'a mut W {
        self.variant(SERCLKDIV_A::VAL3)
    }
    #[doc = "Divided by 5"]
    #[inline(always)]
    pub fn val4(self) -> &'a mut W {
        self.variant(SERCLKDIV_A::VAL4)
    }
    #[doc = "Divided by 6"]
    #[inline(always)]
    pub fn val5(self) -> &'a mut W {
        self.variant(SERCLKDIV_A::VAL5)
    }
    #[doc = "Divided by 7"]
    #[inline(always)]
    pub fn val6(self) -> &'a mut W {
        self.variant(SERCLKDIV_A::VAL6)
    }
    #[doc = "Divided by 8"]
    #[inline(always)]
    pub fn val7(self) -> &'a mut W {
        self.variant(SERCLKDIV_A::VAL7)
    }
}
#[doc = "Field `HSEN` reader - Half Speed Serial Flash Access Enable."]
pub type HSEN_R = crate::BitReader<HSEN_A>;
#[doc = "Half Speed Serial Flash Access Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEN_A {
    #[doc = "0: Disable divide by 2 of serial flash clock for half clock frequency."]
    VAL0 = 0,
    #[doc = "1: Enable divide by 2 of serial flash clock for half clock frequency."]
    VAL1 = 1,
}
impl From<HSEN_A> for bool {
    #[inline(always)]
    fn from(variant: HSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl HSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSEN_A {
        match self.bits {
            false => HSEN_A::VAL0,
            true => HSEN_A::VAL1,
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == HSEN_A::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL1`"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == HSEN_A::VAL1
    }
}
#[doc = "Field `HSEN` writer - Half Speed Serial Flash Access Enable."]
pub type HSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR0_SPEC, HSEN_A, O>;
impl<'a, const O: u8> HSEN_W<'a, O> {
    #[doc = "Disable divide by 2 of serial flash clock for half clock frequency."]
    #[inline(always)]
    pub fn val0(self) -> &'a mut W {
        self.variant(HSEN_A::VAL0)
    }
    #[doc = "Enable divide by 2 of serial flash clock for half clock frequency."]
    #[inline(always)]
    pub fn val1(self) -> &'a mut W {
        self.variant(HSEN_A::VAL1)
    }
}
#[doc = "Field `DOZEEN` reader - Doze mode enable bit"]
pub type DOZEEN_R = crate::BitReader<DOZEEN_A>;
#[doc = "Doze mode enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOZEEN_A {
    #[doc = "0: Doze mode support disabled. AHB clock and serial clock will not be gated off when there is doze mode request from system."]
    VAL0 = 0,
    #[doc = "1: Doze mode support enabled. AHB clock and serial clock will be gated off when there is doze mode request from system."]
    VAL1 = 1,
}
impl From<DOZEEN_A> for bool {
    #[inline(always)]
    fn from(variant: DOZEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DOZEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOZEEN_A {
        match self.bits {
            false => DOZEEN_A::VAL0,
            true => DOZEEN_A::VAL1,
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == DOZEEN_A::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL1`"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == DOZEEN_A::VAL1
    }
}
#[doc = "Field `DOZEEN` writer - Doze mode enable bit"]
pub type DOZEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR0_SPEC, DOZEEN_A, O>;
impl<'a, const O: u8> DOZEEN_W<'a, O> {
    #[doc = "Doze mode support disabled. AHB clock and serial clock will not be gated off when there is doze mode request from system."]
    #[inline(always)]
    pub fn val0(self) -> &'a mut W {
        self.variant(DOZEEN_A::VAL0)
    }
    #[doc = "Doze mode support enabled. AHB clock and serial clock will be gated off when there is doze mode request from system."]
    #[inline(always)]
    pub fn val1(self) -> &'a mut W {
        self.variant(DOZEEN_A::VAL1)
    }
}
#[doc = "Field `SCKFREERUNEN` reader - This bit is used to enable SCLK output free-running. For FPGA applications, the external device may use SCLK as reference clock to its internal PLL."]
pub type SCKFREERUNEN_R = crate::BitReader<SCKFREERUNEN_A>;
#[doc = "This bit is used to enable SCLK output free-running. For FPGA applications, the external device may use SCLK as reference clock to its internal PLL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCKFREERUNEN_A {
    #[doc = "0: Disable SCLK output free-running."]
    DISABLE = 0,
    #[doc = "1: Enable SCLK output free-running."]
    ENABLE = 1,
}
impl From<SCKFREERUNEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCKFREERUNEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SCKFREERUNEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCKFREERUNEN_A {
        match self.bits {
            false => SCKFREERUNEN_A::DISABLE,
            true => SCKFREERUNEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SCKFREERUNEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SCKFREERUNEN_A::ENABLE
    }
}
#[doc = "Field `SCKFREERUNEN` writer - This bit is used to enable SCLK output free-running. For FPGA applications, the external device may use SCLK as reference clock to its internal PLL."]
pub type SCKFREERUNEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR0_SPEC, SCKFREERUNEN_A, O>;
impl<'a, const O: u8> SCKFREERUNEN_W<'a, O> {
    #[doc = "Disable SCLK output free-running."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SCKFREERUNEN_A::DISABLE)
    }
    #[doc = "Enable SCLK output free-running."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SCKFREERUNEN_A::ENABLE)
    }
}
#[doc = "Field `LEARNEN` reader - This bit is used to enable/disable the data learning feature."]
pub type LEARNEN_R = crate::BitReader<LEARNEN_A>;
#[doc = "This bit is used to enable/disable the data learning feature.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LEARNEN_A {
    #[doc = "0: Disable the data learning feature."]
    DISABLE = 0,
    #[doc = "1: Enable the data learning feature."]
    ENABLE = 1,
}
impl From<LEARNEN_A> for bool {
    #[inline(always)]
    fn from(variant: LEARNEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LEARNEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEARNEN_A {
        match self.bits {
            false => LEARNEN_A::DISABLE,
            true => LEARNEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LEARNEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LEARNEN_A::ENABLE
    }
}
#[doc = "Field `LEARNEN` writer - This bit is used to enable/disable the data learning feature."]
pub type LEARNEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR0_SPEC, LEARNEN_A, O>;
impl<'a, const O: u8> LEARNEN_W<'a, O> {
    #[doc = "Disable the data learning feature."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LEARNEN_A::DISABLE)
    }
    #[doc = "Enable the data learning feature."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LEARNEN_A::ENABLE)
    }
}
#[doc = "Field `IPGRANTWAIT` reader - Timeout wait cycle for IP command grant."]
pub type IPGRANTWAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPGRANTWAIT` writer - Timeout wait cycle for IP command grant."]
pub type IPGRANTWAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `AHBGRANTWAIT` reader - Timeout wait cycle for AHB command grant."]
pub type AHBGRANTWAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AHBGRANTWAIT` writer - Timeout wait cycle for AHB command grant."]
pub type AHBGRANTWAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCR0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swreset(&self) -> SWRESET_R {
        SWRESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&self) -> MDIS_R {
        MDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Sample Clock source selection for Flash Reading"]
    #[inline(always)]
    pub fn rxclksrc(&self) -> RXCLKSRC_R {
        RXCLKSRC_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Serial root clock"]
    #[inline(always)]
    pub fn serclkdiv(&self) -> SERCLKDIV_R {
        SERCLKDIV_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Half Speed Serial Flash Access Enable."]
    #[inline(always)]
    pub fn hsen(&self) -> HSEN_R {
        HSEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Doze mode enable bit"]
    #[inline(always)]
    pub fn dozeen(&self) -> DOZEEN_R {
        DOZEEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - This bit is used to enable SCLK output free-running. For FPGA applications, the external device may use SCLK as reference clock to its internal PLL."]
    #[inline(always)]
    pub fn sckfreerunen(&self) -> SCKFREERUNEN_R {
        SCKFREERUNEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This bit is used to enable/disable the data learning feature."]
    #[inline(always)]
    pub fn learnen(&self) -> LEARNEN_R {
        LEARNEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Timeout wait cycle for IP command grant."]
    #[inline(always)]
    pub fn ipgrantwait(&self) -> IPGRANTWAIT_R {
        IPGRANTWAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Timeout wait cycle for AHB command grant."]
    #[inline(always)]
    pub fn ahbgrantwait(&self) -> AHBGRANTWAIT_R {
        AHBGRANTWAIT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swreset(&mut self) -> SWRESET_W<0> {
        SWRESET_W::new(self)
    }
    #[doc = "Bit 1 - Module Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mdis(&mut self) -> MDIS_W<1> {
        MDIS_W::new(self)
    }
    #[doc = "Bits 4:5 - Sample Clock source selection for Flash Reading"]
    #[inline(always)]
    #[must_use]
    pub fn rxclksrc(&mut self) -> RXCLKSRC_W<4> {
        RXCLKSRC_W::new(self)
    }
    #[doc = "Bits 8:10 - Serial root clock"]
    #[inline(always)]
    #[must_use]
    pub fn serclkdiv(&mut self) -> SERCLKDIV_W<8> {
        SERCLKDIV_W::new(self)
    }
    #[doc = "Bit 11 - Half Speed Serial Flash Access Enable."]
    #[inline(always)]
    #[must_use]
    pub fn hsen(&mut self) -> HSEN_W<11> {
        HSEN_W::new(self)
    }
    #[doc = "Bit 12 - Doze mode enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn dozeen(&mut self) -> DOZEEN_W<12> {
        DOZEEN_W::new(self)
    }
    #[doc = "Bit 14 - This bit is used to enable SCLK output free-running. For FPGA applications, the external device may use SCLK as reference clock to its internal PLL."]
    #[inline(always)]
    #[must_use]
    pub fn sckfreerunen(&mut self) -> SCKFREERUNEN_W<14> {
        SCKFREERUNEN_W::new(self)
    }
    #[doc = "Bit 15 - This bit is used to enable/disable the data learning feature."]
    #[inline(always)]
    #[must_use]
    pub fn learnen(&mut self) -> LEARNEN_W<15> {
        LEARNEN_W::new(self)
    }
    #[doc = "Bits 16:23 - Timeout wait cycle for IP command grant."]
    #[inline(always)]
    #[must_use]
    pub fn ipgrantwait(&mut self) -> IPGRANTWAIT_W<16> {
        IPGRANTWAIT_W::new(self)
    }
    #[doc = "Bits 24:31 - Timeout wait cycle for AHB command grant."]
    #[inline(always)]
    #[must_use]
    pub fn ahbgrantwait(&mut self) -> AHBGRANTWAIT_W<24> {
        AHBGRANTWAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr0](index.html) module"]
pub struct MCR0_SPEC;
impl crate::RegisterSpec for MCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr0::R](R) reader structure"]
impl crate::Readable for MCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr0::W](W) writer structure"]
impl crate::Writable for MCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCR0 to value 0xffff_80c2"]
impl crate::Resettable for MCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_80c2;
}
