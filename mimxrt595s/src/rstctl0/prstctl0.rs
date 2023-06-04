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
#[doc = "Field `DSP` reader - Fusion F1 DSP reset control"]
pub type DSP_R = crate::BitReader<DSP_A>;
#[doc = "Fusion F1 DSP reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSP_A {
    #[doc = "0: Clear Reset"]
    DSP_CLR = 0,
    #[doc = "1: Set Reset"]
    DSP_SET = 1,
}
impl From<DSP_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_A) -> Self {
        variant as u8 != 0
    }
}
impl DSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_A {
        match self.bits {
            false => DSP_A::DSP_CLR,
            true => DSP_A::DSP_SET,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_CLR`"]
    #[inline(always)]
    pub fn is_dsp_clr(&self) -> bool {
        *self == DSP_A::DSP_CLR
    }
    #[doc = "Checks if the value of the field is `DSP_SET`"]
    #[inline(always)]
    pub fn is_dsp_set(&self) -> bool {
        *self == DSP_A::DSP_SET
    }
}
#[doc = "Field `DSP` writer - Fusion F1 DSP reset control"]
pub type DSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, DSP_A, O>;
impl<'a, const O: u8> DSP_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn dsp_clr(self) -> &'a mut W {
        self.variant(DSP_A::DSP_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn dsp_set(self) -> &'a mut W {
        self.variant(DSP_A::DSP_SET)
    }
}
#[doc = "Field `AXI_SWITCH` reader - AXI Switch reset control"]
pub type AXI_SWITCH_R = crate::BitReader<AXI_SWITCH_A>;
#[doc = "AXI Switch reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AXI_SWITCH_A {
    #[doc = "0: Clear Reset"]
    AXI_SWITCH_CLR = 0,
    #[doc = "1: Set Reset"]
    AXI_SWITCH_SET = 1,
}
impl From<AXI_SWITCH_A> for bool {
    #[inline(always)]
    fn from(variant: AXI_SWITCH_A) -> Self {
        variant as u8 != 0
    }
}
impl AXI_SWITCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXI_SWITCH_A {
        match self.bits {
            false => AXI_SWITCH_A::AXI_SWITCH_CLR,
            true => AXI_SWITCH_A::AXI_SWITCH_SET,
        }
    }
    #[doc = "Checks if the value of the field is `AXI_SWITCH_CLR`"]
    #[inline(always)]
    pub fn is_axi_switch_clr(&self) -> bool {
        *self == AXI_SWITCH_A::AXI_SWITCH_CLR
    }
    #[doc = "Checks if the value of the field is `AXI_SWITCH_SET`"]
    #[inline(always)]
    pub fn is_axi_switch_set(&self) -> bool {
        *self == AXI_SWITCH_A::AXI_SWITCH_SET
    }
}
#[doc = "Field `AXI_SWITCH` writer - AXI Switch reset control"]
pub type AXI_SWITCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, AXI_SWITCH_A, O>;
impl<'a, const O: u8> AXI_SWITCH_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn axi_switch_clr(self) -> &'a mut W {
        self.variant(AXI_SWITCH_A::AXI_SWITCH_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn axi_switch_set(self) -> &'a mut W {
        self.variant(AXI_SWITCH_A::AXI_SWITCH_SET)
    }
}
#[doc = "Field `POWERQUAD` reader - POWERQUAD reset control"]
pub type POWERQUAD_R = crate::BitReader<POWERQUAD_A>;
#[doc = "POWERQUAD reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POWERQUAD_A {
    #[doc = "0: Clear Reset"]
    POWERQUAD_CLR = 0,
    #[doc = "1: Set Reset"]
    POWERQUAD_SET = 1,
}
impl From<POWERQUAD_A> for bool {
    #[inline(always)]
    fn from(variant: POWERQUAD_A) -> Self {
        variant as u8 != 0
    }
}
impl POWERQUAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POWERQUAD_A {
        match self.bits {
            false => POWERQUAD_A::POWERQUAD_CLR,
            true => POWERQUAD_A::POWERQUAD_SET,
        }
    }
    #[doc = "Checks if the value of the field is `POWERQUAD_CLR`"]
    #[inline(always)]
    pub fn is_powerquad_clr(&self) -> bool {
        *self == POWERQUAD_A::POWERQUAD_CLR
    }
    #[doc = "Checks if the value of the field is `POWERQUAD_SET`"]
    #[inline(always)]
    pub fn is_powerquad_set(&self) -> bool {
        *self == POWERQUAD_A::POWERQUAD_SET
    }
}
#[doc = "Field `POWERQUAD` writer - POWERQUAD reset control"]
pub type POWERQUAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, POWERQUAD_A, O>;
impl<'a, const O: u8> POWERQUAD_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn powerquad_clr(self) -> &'a mut W {
        self.variant(POWERQUAD_A::POWERQUAD_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn powerquad_set(self) -> &'a mut W {
        self.variant(POWERQUAD_A::POWERQUAD_SET)
    }
}
#[doc = "Field `CASPER` reader - CASPER reset control"]
pub type CASPER_R = crate::BitReader<CASPER_A>;
#[doc = "CASPER reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CASPER_A {
    #[doc = "0: Clear Reset"]
    CASPER_CLR = 0,
    #[doc = "1: Set Reset"]
    CASPER_SET = 1,
}
impl From<CASPER_A> for bool {
    #[inline(always)]
    fn from(variant: CASPER_A) -> Self {
        variant as u8 != 0
    }
}
impl CASPER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CASPER_A {
        match self.bits {
            false => CASPER_A::CASPER_CLR,
            true => CASPER_A::CASPER_SET,
        }
    }
    #[doc = "Checks if the value of the field is `CASPER_CLR`"]
    #[inline(always)]
    pub fn is_casper_clr(&self) -> bool {
        *self == CASPER_A::CASPER_CLR
    }
    #[doc = "Checks if the value of the field is `CASPER_SET`"]
    #[inline(always)]
    pub fn is_casper_set(&self) -> bool {
        *self == CASPER_A::CASPER_SET
    }
}
#[doc = "Field `CASPER` writer - CASPER reset control"]
pub type CASPER_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, CASPER_A, O>;
impl<'a, const O: u8> CASPER_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn casper_clr(self) -> &'a mut W {
        self.variant(CASPER_A::CASPER_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn casper_set(self) -> &'a mut W {
        self.variant(CASPER_A::CASPER_SET)
    }
}
#[doc = "Field `HASHCRYPT` reader - Hash-Crypt reset control"]
pub type HASHCRYPT_R = crate::BitReader<HASHCRYPT_A>;
#[doc = "Hash-Crypt reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HASHCRYPT_A {
    #[doc = "0: Clear Reset"]
    HASHCRYPT_CLR = 0,
    #[doc = "1: Set Reset"]
    HASHCRYPT_SET = 1,
}
impl From<HASHCRYPT_A> for bool {
    #[inline(always)]
    fn from(variant: HASHCRYPT_A) -> Self {
        variant as u8 != 0
    }
}
impl HASHCRYPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASHCRYPT_A {
        match self.bits {
            false => HASHCRYPT_A::HASHCRYPT_CLR,
            true => HASHCRYPT_A::HASHCRYPT_SET,
        }
    }
    #[doc = "Checks if the value of the field is `HASHCRYPT_CLR`"]
    #[inline(always)]
    pub fn is_hashcrypt_clr(&self) -> bool {
        *self == HASHCRYPT_A::HASHCRYPT_CLR
    }
    #[doc = "Checks if the value of the field is `HASHCRYPT_SET`"]
    #[inline(always)]
    pub fn is_hashcrypt_set(&self) -> bool {
        *self == HASHCRYPT_A::HASHCRYPT_SET
    }
}
#[doc = "Field `HASHCRYPT` writer - Hash-Crypt reset control"]
pub type HASHCRYPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, HASHCRYPT_A, O>;
impl<'a, const O: u8> HASHCRYPT_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn hashcrypt_clr(self) -> &'a mut W {
        self.variant(HASHCRYPT_A::HASHCRYPT_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn hashcrypt_set(self) -> &'a mut W {
        self.variant(HASHCRYPT_A::HASHCRYPT_SET)
    }
}
#[doc = "Field `PUF` reader - PUF reset control"]
pub type PUF_R = crate::BitReader<PUF_A>;
#[doc = "PUF reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PUF_A {
    #[doc = "0: Clear Reset"]
    PUF_CLR = 0,
    #[doc = "1: Set Reset"]
    PUF_SET = 1,
}
impl From<PUF_A> for bool {
    #[inline(always)]
    fn from(variant: PUF_A) -> Self {
        variant as u8 != 0
    }
}
impl PUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUF_A {
        match self.bits {
            false => PUF_A::PUF_CLR,
            true => PUF_A::PUF_SET,
        }
    }
    #[doc = "Checks if the value of the field is `PUF_CLR`"]
    #[inline(always)]
    pub fn is_puf_clr(&self) -> bool {
        *self == PUF_A::PUF_CLR
    }
    #[doc = "Checks if the value of the field is `PUF_SET`"]
    #[inline(always)]
    pub fn is_puf_set(&self) -> bool {
        *self == PUF_A::PUF_SET
    }
}
#[doc = "Field `PUF` writer - PUF reset control"]
pub type PUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, PUF_A, O>;
impl<'a, const O: u8> PUF_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn puf_clr(self) -> &'a mut W {
        self.variant(PUF_A::PUF_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn puf_set(self) -> &'a mut W {
        self.variant(PUF_A::PUF_SET)
    }
}
#[doc = "Field `RNG` reader - RNG reset control"]
pub type RNG_R = crate::BitReader<RNG_A>;
#[doc = "RNG reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNG_A {
    #[doc = "0: Clear Reset"]
    RNG_CLR = 0,
    #[doc = "1: Set Reset"]
    RNG_SET = 1,
}
impl From<RNG_A> for bool {
    #[inline(always)]
    fn from(variant: RNG_A) -> Self {
        variant as u8 != 0
    }
}
impl RNG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNG_A {
        match self.bits {
            false => RNG_A::RNG_CLR,
            true => RNG_A::RNG_SET,
        }
    }
    #[doc = "Checks if the value of the field is `RNG_CLR`"]
    #[inline(always)]
    pub fn is_rng_clr(&self) -> bool {
        *self == RNG_A::RNG_CLR
    }
    #[doc = "Checks if the value of the field is `RNG_SET`"]
    #[inline(always)]
    pub fn is_rng_set(&self) -> bool {
        *self == RNG_A::RNG_SET
    }
}
#[doc = "Field `RNG` writer - RNG reset control"]
pub type RNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, RNG_A, O>;
impl<'a, const O: u8> RNG_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn rng_clr(self) -> &'a mut W {
        self.variant(RNG_A::RNG_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn rng_set(self) -> &'a mut W {
        self.variant(RNG_A::RNG_SET)
    }
}
#[doc = "Field `FLEXSPI0_OTFAD` reader - FLEXSPI0 and OTFAD reset control"]
pub type FLEXSPI0_OTFAD_R = crate::BitReader<FLEXSPI0_OTFAD_A>;
#[doc = "FLEXSPI0 and OTFAD reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI0_OTFAD_A {
    #[doc = "0: Clear Reset"]
    FLEXSPI0_OTFAD_CLR = 0,
    #[doc = "1: Set Reset"]
    FLEXSPI0_OTFAD_SET = 1,
}
impl From<FLEXSPI0_OTFAD_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI0_OTFAD_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXSPI0_OTFAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXSPI0_OTFAD_A {
        match self.bits {
            false => FLEXSPI0_OTFAD_A::FLEXSPI0_OTFAD_CLR,
            true => FLEXSPI0_OTFAD_A::FLEXSPI0_OTFAD_SET,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXSPI0_OTFAD_CLR`"]
    #[inline(always)]
    pub fn is_flexspi0_otfad_clr(&self) -> bool {
        *self == FLEXSPI0_OTFAD_A::FLEXSPI0_OTFAD_CLR
    }
    #[doc = "Checks if the value of the field is `FLEXSPI0_OTFAD_SET`"]
    #[inline(always)]
    pub fn is_flexspi0_otfad_set(&self) -> bool {
        *self == FLEXSPI0_OTFAD_A::FLEXSPI0_OTFAD_SET
    }
}
#[doc = "Field `FLEXSPI0_OTFAD` writer - FLEXSPI0 and OTFAD reset control"]
pub type FLEXSPI0_OTFAD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_SPEC, FLEXSPI0_OTFAD_A, O>;
impl<'a, const O: u8> FLEXSPI0_OTFAD_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn flexspi0_otfad_clr(self) -> &'a mut W {
        self.variant(FLEXSPI0_OTFAD_A::FLEXSPI0_OTFAD_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn flexspi0_otfad_set(self) -> &'a mut W {
        self.variant(FLEXSPI0_OTFAD_A::FLEXSPI0_OTFAD_SET)
    }
}
#[doc = "Field `FLEXSPI1` reader - FLEXSPI1 reset control"]
pub type FLEXSPI1_R = crate::BitReader<FLEXSPI1_A>;
#[doc = "FLEXSPI1 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI1_A {
    #[doc = "0: Clear Reset"]
    FLEXSPI1_CLR = 0,
    #[doc = "1: Set Reset"]
    FLEXSPI1_SET = 1,
}
impl From<FLEXSPI1_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI1_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXSPI1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXSPI1_A {
        match self.bits {
            false => FLEXSPI1_A::FLEXSPI1_CLR,
            true => FLEXSPI1_A::FLEXSPI1_SET,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXSPI1_CLR`"]
    #[inline(always)]
    pub fn is_flexspi1_clr(&self) -> bool {
        *self == FLEXSPI1_A::FLEXSPI1_CLR
    }
    #[doc = "Checks if the value of the field is `FLEXSPI1_SET`"]
    #[inline(always)]
    pub fn is_flexspi1_set(&self) -> bool {
        *self == FLEXSPI1_A::FLEXSPI1_SET
    }
}
#[doc = "Field `FLEXSPI1` writer - FLEXSPI1 reset control"]
pub type FLEXSPI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, FLEXSPI1_A, O>;
impl<'a, const O: u8> FLEXSPI1_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn flexspi1_clr(self) -> &'a mut W {
        self.variant(FLEXSPI1_A::FLEXSPI1_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn flexspi1_set(self) -> &'a mut W {
        self.variant(FLEXSPI1_A::FLEXSPI1_SET)
    }
}
#[doc = "Field `USBHS_PHY` reader - USB PHY reset control"]
pub type USBHS_PHY_R = crate::BitReader<USBHS_PHY_A>;
#[doc = "USB PHY reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBHS_PHY_A {
    #[doc = "0: Clear Reset"]
    USBHS_PHY_CLR = 0,
    #[doc = "1: Set Reset"]
    USBHS_PHY_SET = 1,
}
impl From<USBHS_PHY_A> for bool {
    #[inline(always)]
    fn from(variant: USBHS_PHY_A) -> Self {
        variant as u8 != 0
    }
}
impl USBHS_PHY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBHS_PHY_A {
        match self.bits {
            false => USBHS_PHY_A::USBHS_PHY_CLR,
            true => USBHS_PHY_A::USBHS_PHY_SET,
        }
    }
    #[doc = "Checks if the value of the field is `USBHS_PHY_CLR`"]
    #[inline(always)]
    pub fn is_usbhs_phy_clr(&self) -> bool {
        *self == USBHS_PHY_A::USBHS_PHY_CLR
    }
    #[doc = "Checks if the value of the field is `USBHS_PHY_SET`"]
    #[inline(always)]
    pub fn is_usbhs_phy_set(&self) -> bool {
        *self == USBHS_PHY_A::USBHS_PHY_SET
    }
}
#[doc = "Field `USBHS_PHY` writer - USB PHY reset control"]
pub type USBHS_PHY_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, USBHS_PHY_A, O>;
impl<'a, const O: u8> USBHS_PHY_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn usbhs_phy_clr(self) -> &'a mut W {
        self.variant(USBHS_PHY_A::USBHS_PHY_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn usbhs_phy_set(self) -> &'a mut W {
        self.variant(USBHS_PHY_A::USBHS_PHY_SET)
    }
}
#[doc = "Field `USBHS_DEVICE` reader - USB HS Device reset control"]
pub type USBHS_DEVICE_R = crate::BitReader<USBHS_DEVICE_A>;
#[doc = "USB HS Device reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBHS_DEVICE_A {
    #[doc = "0: Clear Reset"]
    USBHS_DEVICE_CLR = 0,
    #[doc = "1: Set Reset"]
    USBHS_DEVICE_SET = 1,
}
impl From<USBHS_DEVICE_A> for bool {
    #[inline(always)]
    fn from(variant: USBHS_DEVICE_A) -> Self {
        variant as u8 != 0
    }
}
impl USBHS_DEVICE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBHS_DEVICE_A {
        match self.bits {
            false => USBHS_DEVICE_A::USBHS_DEVICE_CLR,
            true => USBHS_DEVICE_A::USBHS_DEVICE_SET,
        }
    }
    #[doc = "Checks if the value of the field is `USBHS_DEVICE_CLR`"]
    #[inline(always)]
    pub fn is_usbhs_device_clr(&self) -> bool {
        *self == USBHS_DEVICE_A::USBHS_DEVICE_CLR
    }
    #[doc = "Checks if the value of the field is `USBHS_DEVICE_SET`"]
    #[inline(always)]
    pub fn is_usbhs_device_set(&self) -> bool {
        *self == USBHS_DEVICE_A::USBHS_DEVICE_SET
    }
}
#[doc = "Field `USBHS_DEVICE` writer - USB HS Device reset control"]
pub type USBHS_DEVICE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_SPEC, USBHS_DEVICE_A, O>;
impl<'a, const O: u8> USBHS_DEVICE_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn usbhs_device_clr(self) -> &'a mut W {
        self.variant(USBHS_DEVICE_A::USBHS_DEVICE_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn usbhs_device_set(self) -> &'a mut W {
        self.variant(USBHS_DEVICE_A::USBHS_DEVICE_SET)
    }
}
#[doc = "Field `USBHS_HOST` reader - USB HOST reset control"]
pub type USBHS_HOST_R = crate::BitReader<USBHS_HOST_A>;
#[doc = "USB HOST reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBHS_HOST_A {
    #[doc = "0: Clear Reset"]
    USBHS_HOST_CLR = 0,
    #[doc = "1: Set Reset"]
    USBHS_HOST_SET = 1,
}
impl From<USBHS_HOST_A> for bool {
    #[inline(always)]
    fn from(variant: USBHS_HOST_A) -> Self {
        variant as u8 != 0
    }
}
impl USBHS_HOST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBHS_HOST_A {
        match self.bits {
            false => USBHS_HOST_A::USBHS_HOST_CLR,
            true => USBHS_HOST_A::USBHS_HOST_SET,
        }
    }
    #[doc = "Checks if the value of the field is `USBHS_HOST_CLR`"]
    #[inline(always)]
    pub fn is_usbhs_host_clr(&self) -> bool {
        *self == USBHS_HOST_A::USBHS_HOST_CLR
    }
    #[doc = "Checks if the value of the field is `USBHS_HOST_SET`"]
    #[inline(always)]
    pub fn is_usbhs_host_set(&self) -> bool {
        *self == USBHS_HOST_A::USBHS_HOST_SET
    }
}
#[doc = "Field `USBHS_HOST` writer - USB HOST reset control"]
pub type USBHS_HOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, USBHS_HOST_A, O>;
impl<'a, const O: u8> USBHS_HOST_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn usbhs_host_clr(self) -> &'a mut W {
        self.variant(USBHS_HOST_A::USBHS_HOST_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn usbhs_host_set(self) -> &'a mut W {
        self.variant(USBHS_HOST_A::USBHS_HOST_SET)
    }
}
#[doc = "Field `USBHS_SRAM` reader - USB RAM reset control"]
pub type USBHS_SRAM_R = crate::BitReader<USBHS_SRAM_A>;
#[doc = "USB RAM reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBHS_SRAM_A {
    #[doc = "0: Clear Reset"]
    USBHS_SRAM_CLR = 0,
    #[doc = "1: Set Reset"]
    USBHS_SRAM_SET = 1,
}
impl From<USBHS_SRAM_A> for bool {
    #[inline(always)]
    fn from(variant: USBHS_SRAM_A) -> Self {
        variant as u8 != 0
    }
}
impl USBHS_SRAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBHS_SRAM_A {
        match self.bits {
            false => USBHS_SRAM_A::USBHS_SRAM_CLR,
            true => USBHS_SRAM_A::USBHS_SRAM_SET,
        }
    }
    #[doc = "Checks if the value of the field is `USBHS_SRAM_CLR`"]
    #[inline(always)]
    pub fn is_usbhs_sram_clr(&self) -> bool {
        *self == USBHS_SRAM_A::USBHS_SRAM_CLR
    }
    #[doc = "Checks if the value of the field is `USBHS_SRAM_SET`"]
    #[inline(always)]
    pub fn is_usbhs_sram_set(&self) -> bool {
        *self == USBHS_SRAM_A::USBHS_SRAM_SET
    }
}
#[doc = "Field `USBHS_SRAM` writer - USB RAM reset control"]
pub type USBHS_SRAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, USBHS_SRAM_A, O>;
impl<'a, const O: u8> USBHS_SRAM_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn usbhs_sram_clr(self) -> &'a mut W {
        self.variant(USBHS_SRAM_A::USBHS_SRAM_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn usbhs_sram_set(self) -> &'a mut W {
        self.variant(USBHS_SRAM_A::USBHS_SRAM_SET)
    }
}
#[doc = "Field `SCT` reader - SCTimer reset control"]
pub type SCT_R = crate::BitReader<SCT_A>;
#[doc = "SCTimer reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCT_A {
    #[doc = "0: Clear Reset"]
    SCT_CLR = 0,
    #[doc = "1: Set Reset"]
    SCT_SET = 1,
}
impl From<SCT_A> for bool {
    #[inline(always)]
    fn from(variant: SCT_A) -> Self {
        variant as u8 != 0
    }
}
impl SCT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCT_A {
        match self.bits {
            false => SCT_A::SCT_CLR,
            true => SCT_A::SCT_SET,
        }
    }
    #[doc = "Checks if the value of the field is `SCT_CLR`"]
    #[inline(always)]
    pub fn is_sct_clr(&self) -> bool {
        *self == SCT_A::SCT_CLR
    }
    #[doc = "Checks if the value of the field is `SCT_SET`"]
    #[inline(always)]
    pub fn is_sct_set(&self) -> bool {
        *self == SCT_A::SCT_SET
    }
}
#[doc = "Field `SCT` writer - SCTimer reset control"]
pub type SCT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, SCT_A, O>;
impl<'a, const O: u8> SCT_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn sct_clr(self) -> &'a mut W {
        self.variant(SCT_A::SCT_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn sct_set(self) -> &'a mut W {
        self.variant(SCT_A::SCT_SET)
    }
}
#[doc = "Field `GPU` reader - GPU reset control"]
pub type GPU_R = crate::BitReader<GPU_A>;
#[doc = "GPU reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPU_A {
    #[doc = "0: Clear Reset"]
    GPU_CLR = 0,
    #[doc = "1: Set Reset"]
    GPU_SET = 1,
}
impl From<GPU_A> for bool {
    #[inline(always)]
    fn from(variant: GPU_A) -> Self {
        variant as u8 != 0
    }
}
impl GPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPU_A {
        match self.bits {
            false => GPU_A::GPU_CLR,
            true => GPU_A::GPU_SET,
        }
    }
    #[doc = "Checks if the value of the field is `GPU_CLR`"]
    #[inline(always)]
    pub fn is_gpu_clr(&self) -> bool {
        *self == GPU_A::GPU_CLR
    }
    #[doc = "Checks if the value of the field is `GPU_SET`"]
    #[inline(always)]
    pub fn is_gpu_set(&self) -> bool {
        *self == GPU_A::GPU_SET
    }
}
#[doc = "Field `GPU` writer - GPU reset control"]
pub type GPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, GPU_A, O>;
impl<'a, const O: u8> GPU_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn gpu_clr(self) -> &'a mut W {
        self.variant(GPU_A::GPU_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn gpu_set(self) -> &'a mut W {
        self.variant(GPU_A::GPU_SET)
    }
}
#[doc = "Field `DISPLAY_CONTROLLER` reader - LCDIF Display Controller reset control"]
pub type DISPLAY_CONTROLLER_R = crate::BitReader<DISPLAY_CONTROLLER_A>;
#[doc = "LCDIF Display Controller reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISPLAY_CONTROLLER_A {
    #[doc = "0: Clear Reset"]
    DISPLAY_CONTROLLER_CLR = 0,
    #[doc = "1: Set Reset"]
    DISPLAY_CONTROLLER_SET = 1,
}
impl From<DISPLAY_CONTROLLER_A> for bool {
    #[inline(always)]
    fn from(variant: DISPLAY_CONTROLLER_A) -> Self {
        variant as u8 != 0
    }
}
impl DISPLAY_CONTROLLER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISPLAY_CONTROLLER_A {
        match self.bits {
            false => DISPLAY_CONTROLLER_A::DISPLAY_CONTROLLER_CLR,
            true => DISPLAY_CONTROLLER_A::DISPLAY_CONTROLLER_SET,
        }
    }
    #[doc = "Checks if the value of the field is `DISPLAY_CONTROLLER_CLR`"]
    #[inline(always)]
    pub fn is_display_controller_clr(&self) -> bool {
        *self == DISPLAY_CONTROLLER_A::DISPLAY_CONTROLLER_CLR
    }
    #[doc = "Checks if the value of the field is `DISPLAY_CONTROLLER_SET`"]
    #[inline(always)]
    pub fn is_display_controller_set(&self) -> bool {
        *self == DISPLAY_CONTROLLER_A::DISPLAY_CONTROLLER_SET
    }
}
#[doc = "Field `DISPLAY_CONTROLLER` writer - LCDIF Display Controller reset control"]
pub type DISPLAY_CONTROLLER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_SPEC, DISPLAY_CONTROLLER_A, O>;
impl<'a, const O: u8> DISPLAY_CONTROLLER_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn display_controller_clr(self) -> &'a mut W {
        self.variant(DISPLAY_CONTROLLER_A::DISPLAY_CONTROLLER_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn display_controller_set(self) -> &'a mut W {
        self.variant(DISPLAY_CONTROLLER_A::DISPLAY_CONTROLLER_SET)
    }
}
#[doc = "Field `MIPI_DSI_CONTROLLER` reader - MIPI Digital serial Interface controller reset control"]
pub type MIPI_DSI_CONTROLLER_R = crate::BitReader<MIPI_DSI_CONTROLLER_A>;
#[doc = "MIPI Digital serial Interface controller reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIPI_DSI_CONTROLLER_A {
    #[doc = "0: Clear Reset"]
    MIPI_DSI_CONTROLLER_CLR = 0,
    #[doc = "1: Set Reset"]
    MIPI_DSI_CONTROLLER_SET = 1,
}
impl From<MIPI_DSI_CONTROLLER_A> for bool {
    #[inline(always)]
    fn from(variant: MIPI_DSI_CONTROLLER_A) -> Self {
        variant as u8 != 0
    }
}
impl MIPI_DSI_CONTROLLER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIPI_DSI_CONTROLLER_A {
        match self.bits {
            false => MIPI_DSI_CONTROLLER_A::MIPI_DSI_CONTROLLER_CLR,
            true => MIPI_DSI_CONTROLLER_A::MIPI_DSI_CONTROLLER_SET,
        }
    }
    #[doc = "Checks if the value of the field is `MIPI_DSI_CONTROLLER_CLR`"]
    #[inline(always)]
    pub fn is_mipi_dsi_controller_clr(&self) -> bool {
        *self == MIPI_DSI_CONTROLLER_A::MIPI_DSI_CONTROLLER_CLR
    }
    #[doc = "Checks if the value of the field is `MIPI_DSI_CONTROLLER_SET`"]
    #[inline(always)]
    pub fn is_mipi_dsi_controller_set(&self) -> bool {
        *self == MIPI_DSI_CONTROLLER_A::MIPI_DSI_CONTROLLER_SET
    }
}
#[doc = "Field `MIPI_DSI_CONTROLLER` writer - MIPI Digital serial Interface controller reset control"]
pub type MIPI_DSI_CONTROLLER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_SPEC, MIPI_DSI_CONTROLLER_A, O>;
impl<'a, const O: u8> MIPI_DSI_CONTROLLER_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn mipi_dsi_controller_clr(self) -> &'a mut W {
        self.variant(MIPI_DSI_CONTROLLER_A::MIPI_DSI_CONTROLLER_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn mipi_dsi_controller_set(self) -> &'a mut W {
        self.variant(MIPI_DSI_CONTROLLER_A::MIPI_DSI_CONTROLLER_SET)
    }
}
#[doc = "Field `MIPI_DSI_PHY` reader - MIPI DSI PHY reset control"]
pub type MIPI_DSI_PHY_R = crate::BitReader<MIPI_DSI_PHY_A>;
#[doc = "MIPI DSI PHY reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIPI_DSI_PHY_A {
    #[doc = "0: Clear Reset"]
    MIPI_DSI_PHY_CLR = 0,
    #[doc = "1: Set Reset"]
    MIPI_DSI_PHY_SET = 1,
}
impl From<MIPI_DSI_PHY_A> for bool {
    #[inline(always)]
    fn from(variant: MIPI_DSI_PHY_A) -> Self {
        variant as u8 != 0
    }
}
impl MIPI_DSI_PHY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIPI_DSI_PHY_A {
        match self.bits {
            false => MIPI_DSI_PHY_A::MIPI_DSI_PHY_CLR,
            true => MIPI_DSI_PHY_A::MIPI_DSI_PHY_SET,
        }
    }
    #[doc = "Checks if the value of the field is `MIPI_DSI_PHY_CLR`"]
    #[inline(always)]
    pub fn is_mipi_dsi_phy_clr(&self) -> bool {
        *self == MIPI_DSI_PHY_A::MIPI_DSI_PHY_CLR
    }
    #[doc = "Checks if the value of the field is `MIPI_DSI_PHY_SET`"]
    #[inline(always)]
    pub fn is_mipi_dsi_phy_set(&self) -> bool {
        *self == MIPI_DSI_PHY_A::MIPI_DSI_PHY_SET
    }
}
#[doc = "Field `MIPI_DSI_PHY` writer - MIPI DSI PHY reset control"]
pub type MIPI_DSI_PHY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_SPEC, MIPI_DSI_PHY_A, O>;
impl<'a, const O: u8> MIPI_DSI_PHY_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn mipi_dsi_phy_clr(self) -> &'a mut W {
        self.variant(MIPI_DSI_PHY_A::MIPI_DSI_PHY_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn mipi_dsi_phy_set(self) -> &'a mut W {
        self.variant(MIPI_DSI_PHY_A::MIPI_DSI_PHY_SET)
    }
}
#[doc = "Field `SMARTDMA` reader - SMARTDMA Event/Algorithm handler reset control"]
pub type SMARTDMA_R = crate::BitReader<SMARTDMA_A>;
#[doc = "SMARTDMA Event/Algorithm handler reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMARTDMA_A {
    #[doc = "0: Clear Reset"]
    SMARTDMA_CLR = 0,
    #[doc = "1: Set Reset"]
    SMARTDMA_SET = 1,
}
impl From<SMARTDMA_A> for bool {
    #[inline(always)]
    fn from(variant: SMARTDMA_A) -> Self {
        variant as u8 != 0
    }
}
impl SMARTDMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMARTDMA_A {
        match self.bits {
            false => SMARTDMA_A::SMARTDMA_CLR,
            true => SMARTDMA_A::SMARTDMA_SET,
        }
    }
    #[doc = "Checks if the value of the field is `SMARTDMA_CLR`"]
    #[inline(always)]
    pub fn is_smartdma_clr(&self) -> bool {
        *self == SMARTDMA_A::SMARTDMA_CLR
    }
    #[doc = "Checks if the value of the field is `SMARTDMA_SET`"]
    #[inline(always)]
    pub fn is_smartdma_set(&self) -> bool {
        *self == SMARTDMA_A::SMARTDMA_SET
    }
}
#[doc = "Field `SMARTDMA` writer - SMARTDMA Event/Algorithm handler reset control"]
pub type SMARTDMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SPEC, SMARTDMA_A, O>;
impl<'a, const O: u8> SMARTDMA_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn smartdma_clr(self) -> &'a mut W {
        self.variant(SMARTDMA_A::SMARTDMA_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn smartdma_set(self) -> &'a mut W {
        self.variant(SMARTDMA_A::SMARTDMA_SET)
    }
}
impl R {
    #[doc = "Bit 1 - Fusion F1 DSP reset control"]
    #[inline(always)]
    pub fn dsp(&self) -> DSP_R {
        DSP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - AXI Switch reset control"]
    #[inline(always)]
    pub fn axi_switch(&self) -> AXI_SWITCH_R {
        AXI_SWITCH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - POWERQUAD reset control"]
    #[inline(always)]
    pub fn powerquad(&self) -> POWERQUAD_R {
        POWERQUAD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CASPER reset control"]
    #[inline(always)]
    pub fn casper(&self) -> CASPER_R {
        CASPER_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hash-Crypt reset control"]
    #[inline(always)]
    pub fn hashcrypt(&self) -> HASHCRYPT_R {
        HASHCRYPT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PUF reset control"]
    #[inline(always)]
    pub fn puf(&self) -> PUF_R {
        PUF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RNG reset control"]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - FLEXSPI0 and OTFAD reset control"]
    #[inline(always)]
    pub fn flexspi0_otfad(&self) -> FLEXSPI0_OTFAD_R {
        FLEXSPI0_OTFAD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - FLEXSPI1 reset control"]
    #[inline(always)]
    pub fn flexspi1(&self) -> FLEXSPI1_R {
        FLEXSPI1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - USB PHY reset control"]
    #[inline(always)]
    pub fn usbhs_phy(&self) -> USBHS_PHY_R {
        USBHS_PHY_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - USB HS Device reset control"]
    #[inline(always)]
    pub fn usbhs_device(&self) -> USBHS_DEVICE_R {
        USBHS_DEVICE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - USB HOST reset control"]
    #[inline(always)]
    pub fn usbhs_host(&self) -> USBHS_HOST_R {
        USBHS_HOST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USB RAM reset control"]
    #[inline(always)]
    pub fn usbhs_sram(&self) -> USBHS_SRAM_R {
        USBHS_SRAM_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SCTimer reset control"]
    #[inline(always)]
    pub fn sct(&self) -> SCT_R {
        SCT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - GPU reset control"]
    #[inline(always)]
    pub fn gpu(&self) -> GPU_R {
        GPU_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LCDIF Display Controller reset control"]
    #[inline(always)]
    pub fn display_controller(&self) -> DISPLAY_CONTROLLER_R {
        DISPLAY_CONTROLLER_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - MIPI Digital serial Interface controller reset control"]
    #[inline(always)]
    pub fn mipi_dsi_controller(&self) -> MIPI_DSI_CONTROLLER_R {
        MIPI_DSI_CONTROLLER_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - MIPI DSI PHY reset control"]
    #[inline(always)]
    pub fn mipi_dsi_phy(&self) -> MIPI_DSI_PHY_R {
        MIPI_DSI_PHY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SMARTDMA Event/Algorithm handler reset control"]
    #[inline(always)]
    pub fn smartdma(&self) -> SMARTDMA_R {
        SMARTDMA_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Fusion F1 DSP reset control"]
    #[inline(always)]
    #[must_use]
    pub fn dsp(&mut self) -> DSP_W<1> {
        DSP_W::new(self)
    }
    #[doc = "Bit 3 - AXI Switch reset control"]
    #[inline(always)]
    #[must_use]
    pub fn axi_switch(&mut self) -> AXI_SWITCH_W<3> {
        AXI_SWITCH_W::new(self)
    }
    #[doc = "Bit 8 - POWERQUAD reset control"]
    #[inline(always)]
    #[must_use]
    pub fn powerquad(&mut self) -> POWERQUAD_W<8> {
        POWERQUAD_W::new(self)
    }
    #[doc = "Bit 9 - CASPER reset control"]
    #[inline(always)]
    #[must_use]
    pub fn casper(&mut self) -> CASPER_W<9> {
        CASPER_W::new(self)
    }
    #[doc = "Bit 10 - Hash-Crypt reset control"]
    #[inline(always)]
    #[must_use]
    pub fn hashcrypt(&mut self) -> HASHCRYPT_W<10> {
        HASHCRYPT_W::new(self)
    }
    #[doc = "Bit 11 - PUF reset control"]
    #[inline(always)]
    #[must_use]
    pub fn puf(&mut self) -> PUF_W<11> {
        PUF_W::new(self)
    }
    #[doc = "Bit 12 - RNG reset control"]
    #[inline(always)]
    #[must_use]
    pub fn rng(&mut self) -> RNG_W<12> {
        RNG_W::new(self)
    }
    #[doc = "Bit 16 - FLEXSPI0 and OTFAD reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi0_otfad(&mut self) -> FLEXSPI0_OTFAD_W<16> {
        FLEXSPI0_OTFAD_W::new(self)
    }
    #[doc = "Bit 18 - FLEXSPI1 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi1(&mut self) -> FLEXSPI1_W<18> {
        FLEXSPI1_W::new(self)
    }
    #[doc = "Bit 20 - USB PHY reset control"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_phy(&mut self) -> USBHS_PHY_W<20> {
        USBHS_PHY_W::new(self)
    }
    #[doc = "Bit 21 - USB HS Device reset control"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_device(&mut self) -> USBHS_DEVICE_W<21> {
        USBHS_DEVICE_W::new(self)
    }
    #[doc = "Bit 22 - USB HOST reset control"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_host(&mut self) -> USBHS_HOST_W<22> {
        USBHS_HOST_W::new(self)
    }
    #[doc = "Bit 23 - USB RAM reset control"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_sram(&mut self) -> USBHS_SRAM_W<23> {
        USBHS_SRAM_W::new(self)
    }
    #[doc = "Bit 24 - SCTimer reset control"]
    #[inline(always)]
    #[must_use]
    pub fn sct(&mut self) -> SCT_W<24> {
        SCT_W::new(self)
    }
    #[doc = "Bit 26 - GPU reset control"]
    #[inline(always)]
    #[must_use]
    pub fn gpu(&mut self) -> GPU_W<26> {
        GPU_W::new(self)
    }
    #[doc = "Bit 27 - LCDIF Display Controller reset control"]
    #[inline(always)]
    #[must_use]
    pub fn display_controller(&mut self) -> DISPLAY_CONTROLLER_W<27> {
        DISPLAY_CONTROLLER_W::new(self)
    }
    #[doc = "Bit 28 - MIPI Digital serial Interface controller reset control"]
    #[inline(always)]
    #[must_use]
    pub fn mipi_dsi_controller(&mut self) -> MIPI_DSI_CONTROLLER_W<28> {
        MIPI_DSI_CONTROLLER_W::new(self)
    }
    #[doc = "Bit 29 - MIPI DSI PHY reset control"]
    #[inline(always)]
    #[must_use]
    pub fn mipi_dsi_phy(&mut self) -> MIPI_DSI_PHY_W<29> {
        MIPI_DSI_PHY_W::new(self)
    }
    #[doc = "Bit 30 - SMARTDMA Event/Algorithm handler reset control"]
    #[inline(always)]
    #[must_use]
    pub fn smartdma(&mut self) -> SMARTDMA_W<30> {
        SMARTDMA_W::new(self)
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
#[doc = "`reset()` method sets PRSTCTL0 to value 0x7df5_1f0a"]
impl crate::Resettable for PRSTCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x7df5_1f0a;
}
