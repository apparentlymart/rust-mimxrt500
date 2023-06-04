#[doc = "Register `PSCCTL0` reader"]
pub struct R(crate::R<PSCCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSCCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSCCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSCCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSCCTL0` writer"]
pub struct W(crate::W<PSCCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSCCTL0_SPEC>;
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
impl From<crate::W<PSCCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSCCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSP_CLK` reader - DSP clock control"]
pub type DSP_CLK_R = crate::BitReader<DSP_CLK_A>;
#[doc = "DSP clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSP_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<DSP_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl DSP_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_CLK_A {
        match self.bits {
            false => DSP_CLK_A::CLK_DISABLE,
            true => DSP_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == DSP_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == DSP_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `DSP_CLK` writer - DSP clock control"]
pub type DSP_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SPEC, DSP_CLK_A, O>;
impl<'a, const O: u8> DSP_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(DSP_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(DSP_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `ROM_CTRLR_CLK` reader - 128KB ROM Controller clock control"]
pub type ROM_CTRLR_CLK_R = crate::BitReader<ROM_CTRLR_CLK_A>;
#[doc = "128KB ROM Controller clock control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROM_CTRLR_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<ROM_CTRLR_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: ROM_CTRLR_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl ROM_CTRLR_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROM_CTRLR_CLK_A {
        match self.bits {
            false => ROM_CTRLR_CLK_A::CLK_DISABLE,
            true => ROM_CTRLR_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == ROM_CTRLR_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == ROM_CTRLR_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `ROM_CTRLR_CLK` writer - 128KB ROM Controller clock control"]
pub type ROM_CTRLR_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SPEC, ROM_CTRLR_CLK_A, O>;
impl<'a, const O: u8> ROM_CTRLR_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(ROM_CTRLR_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(ROM_CTRLR_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `AXI_SWITCH_CLK` reader - AXI Switch clock control"]
pub type AXI_SWITCH_CLK_R = crate::BitReader<AXI_SWITCH_CLK_A>;
#[doc = "AXI Switch clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AXI_SWITCH_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<AXI_SWITCH_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: AXI_SWITCH_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl AXI_SWITCH_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXI_SWITCH_CLK_A {
        match self.bits {
            false => AXI_SWITCH_CLK_A::CLK_DISABLE,
            true => AXI_SWITCH_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == AXI_SWITCH_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == AXI_SWITCH_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `AXI_SWITCH_CLK` writer - AXI Switch clock control"]
pub type AXI_SWITCH_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SPEC, AXI_SWITCH_CLK_A, O>;
impl<'a, const O: u8> AXI_SWITCH_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(AXI_SWITCH_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(AXI_SWITCH_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `AXI_CTLR_CLK` reader - AXI Controller clock control"]
pub type AXI_CTLR_CLK_R = crate::BitReader<AXI_CTLR_CLK_A>;
#[doc = "AXI Controller clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AXI_CTLR_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<AXI_CTLR_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: AXI_CTLR_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl AXI_CTLR_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXI_CTLR_CLK_A {
        match self.bits {
            false => AXI_CTLR_CLK_A::CLK_DISABLE,
            true => AXI_CTLR_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == AXI_CTLR_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == AXI_CTLR_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `AXI_CTLR_CLK` writer - AXI Controller clock control"]
pub type AXI_CTLR_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SPEC, AXI_CTLR_CLK_A, O>;
impl<'a, const O: u8> AXI_CTLR_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(AXI_CTLR_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(AXI_CTLR_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `POWERQUAD_CLK` reader - POWERQUAD clock control"]
pub type POWERQUAD_CLK_R = crate::BitReader<POWERQUAD_CLK_A>;
#[doc = "POWERQUAD clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POWERQUAD_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<POWERQUAD_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: POWERQUAD_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl POWERQUAD_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POWERQUAD_CLK_A {
        match self.bits {
            false => POWERQUAD_CLK_A::CLK_DISABLE,
            true => POWERQUAD_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == POWERQUAD_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == POWERQUAD_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `POWERQUAD_CLK` writer - POWERQUAD clock control"]
pub type POWERQUAD_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SPEC, POWERQUAD_CLK_A, O>;
impl<'a, const O: u8> POWERQUAD_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(POWERQUAD_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(POWERQUAD_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `CASPER_CLK` reader - CASPER clock control"]
pub type CASPER_CLK_R = crate::BitReader<CASPER_CLK_A>;
#[doc = "CASPER clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CASPER_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<CASPER_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: CASPER_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl CASPER_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CASPER_CLK_A {
        match self.bits {
            false => CASPER_CLK_A::CLK_DISABLE,
            true => CASPER_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == CASPER_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == CASPER_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `CASPER_CLK` writer - CASPER clock control"]
pub type CASPER_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SPEC, CASPER_CLK_A, O>;
impl<'a, const O: u8> CASPER_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(CASPER_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(CASPER_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `HASHCRYPT_CLK` reader - HASHCRYPT clock control"]
pub type HASHCRYPT_CLK_R = crate::BitReader<HASHCRYPT_CLK_A>;
#[doc = "HASHCRYPT clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HASHCRYPT_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<HASHCRYPT_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: HASHCRYPT_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl HASHCRYPT_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASHCRYPT_CLK_A {
        match self.bits {
            false => HASHCRYPT_CLK_A::CLK_DISABLE,
            true => HASHCRYPT_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == HASHCRYPT_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == HASHCRYPT_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `HASHCRYPT_CLK` writer - HASHCRYPT clock control"]
pub type HASHCRYPT_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SPEC, HASHCRYPT_CLK_A, O>;
impl<'a, const O: u8> HASHCRYPT_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(HASHCRYPT_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(HASHCRYPT_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `PUF_CLK` reader - PUF clock control"]
pub type PUF_CLK_R = crate::BitReader<PUF_CLK_A>;
#[doc = "PUF clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PUF_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<PUF_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: PUF_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl PUF_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUF_CLK_A {
        match self.bits {
            false => PUF_CLK_A::CLK_DISABLE,
            true => PUF_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == PUF_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == PUF_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `PUF_CLK` writer - PUF clock control"]
pub type PUF_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SPEC, PUF_CLK_A, O>;
impl<'a, const O: u8> PUF_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(PUF_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(PUF_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `RNG_CLK` reader - Random Number Generator (RNG) clock control"]
pub type RNG_CLK_R = crate::BitReader<RNG_CLK_A>;
#[doc = "Random Number Generator (RNG) clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNG_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<RNG_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: RNG_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl RNG_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNG_CLK_A {
        match self.bits {
            false => RNG_CLK_A::CLK_DISABLE,
            true => RNG_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == RNG_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == RNG_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `RNG_CLK` writer - Random Number Generator (RNG) clock control"]
pub type RNG_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SPEC, RNG_CLK_A, O>;
impl<'a, const O: u8> RNG_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(RNG_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(RNG_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `FLEXSPI0_OTFAD_CLK` reader - FLEXSPI0 / OTFAD clock control"]
pub type FLEXSPI0_OTFAD_CLK_R = crate::BitReader<FLEXSPI0_OTFAD_CLK_A>;
#[doc = "FLEXSPI0 / OTFAD clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI0_OTFAD_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<FLEXSPI0_OTFAD_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI0_OTFAD_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXSPI0_OTFAD_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXSPI0_OTFAD_CLK_A {
        match self.bits {
            false => FLEXSPI0_OTFAD_CLK_A::CLK_DISABLE,
            true => FLEXSPI0_OTFAD_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == FLEXSPI0_OTFAD_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == FLEXSPI0_OTFAD_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `FLEXSPI0_OTFAD_CLK` writer - FLEXSPI0 / OTFAD clock control"]
pub type FLEXSPI0_OTFAD_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SPEC, FLEXSPI0_OTFAD_CLK_A, O>;
impl<'a, const O: u8> FLEXSPI0_OTFAD_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(FLEXSPI0_OTFAD_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(FLEXSPI0_OTFAD_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `OTP_CTLR_CLK` reader - OTP Controller clock control"]
pub type OTP_CTLR_CLK_R = crate::BitReader<OTP_CTLR_CLK_A>;
#[doc = "OTP Controller clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTP_CTLR_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<OTP_CTLR_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: OTP_CTLR_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl OTP_CTLR_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTP_CTLR_CLK_A {
        match self.bits {
            false => OTP_CTLR_CLK_A::CLK_DISABLE,
            true => OTP_CTLR_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == OTP_CTLR_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == OTP_CTLR_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `OTP_CTLR_CLK` writer - OTP Controller clock control"]
pub type OTP_CTLR_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SPEC, OTP_CTLR_CLK_A, O>;
impl<'a, const O: u8> OTP_CTLR_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(OTP_CTLR_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(OTP_CTLR_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `FLEXSPI1_CLK` reader - FLEXSPI1 clock control"]
pub type FLEXSPI1_CLK_R = crate::BitReader<FLEXSPI1_CLK_A>;
#[doc = "FLEXSPI1 clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI1_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<FLEXSPI1_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI1_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXSPI1_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXSPI1_CLK_A {
        match self.bits {
            false => FLEXSPI1_CLK_A::CLK_DISABLE,
            true => FLEXSPI1_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == FLEXSPI1_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == FLEXSPI1_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `FLEXSPI1_CLK` writer - FLEXSPI1 clock control"]
pub type FLEXSPI1_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SPEC, FLEXSPI1_CLK_A, O>;
impl<'a, const O: u8> FLEXSPI1_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(FLEXSPI1_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(FLEXSPI1_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `USBHS_PHY_CLK` reader - USB HS PHY clock control"]
pub type USBHS_PHY_CLK_R = crate::BitReader<USBHS_PHY_CLK_A>;
#[doc = "USB HS PHY clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBHS_PHY_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<USBHS_PHY_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: USBHS_PHY_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl USBHS_PHY_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBHS_PHY_CLK_A {
        match self.bits {
            false => USBHS_PHY_CLK_A::CLK_DISABLE,
            true => USBHS_PHY_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == USBHS_PHY_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == USBHS_PHY_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `USBHS_PHY_CLK` writer - USB HS PHY clock control"]
pub type USBHS_PHY_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SPEC, USBHS_PHY_CLK_A, O>;
impl<'a, const O: u8> USBHS_PHY_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(USBHS_PHY_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(USBHS_PHY_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `USBHS_DEVICE_CLK` reader - USB HS Device clock control"]
pub type USBHS_DEVICE_CLK_R = crate::BitReader<USBHS_DEVICE_CLK_A>;
#[doc = "USB HS Device clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBHS_DEVICE_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<USBHS_DEVICE_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: USBHS_DEVICE_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl USBHS_DEVICE_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBHS_DEVICE_CLK_A {
        match self.bits {
            false => USBHS_DEVICE_CLK_A::CLK_DISABLE,
            true => USBHS_DEVICE_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == USBHS_DEVICE_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == USBHS_DEVICE_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `USBHS_DEVICE_CLK` writer - USB HS Device clock control"]
pub type USBHS_DEVICE_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SPEC, USBHS_DEVICE_CLK_A, O>;
impl<'a, const O: u8> USBHS_DEVICE_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(USBHS_DEVICE_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(USBHS_DEVICE_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `USBHS_HOST_CLK` reader - USB HS Host clock control"]
pub type USBHS_HOST_CLK_R = crate::BitReader<USBHS_HOST_CLK_A>;
#[doc = "USB HS Host clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBHS_HOST_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<USBHS_HOST_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: USBHS_HOST_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl USBHS_HOST_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBHS_HOST_CLK_A {
        match self.bits {
            false => USBHS_HOST_CLK_A::CLK_DISABLE,
            true => USBHS_HOST_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == USBHS_HOST_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == USBHS_HOST_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `USBHS_HOST_CLK` writer - USB HS Host clock control"]
pub type USBHS_HOST_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SPEC, USBHS_HOST_CLK_A, O>;
impl<'a, const O: u8> USBHS_HOST_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(USBHS_HOST_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(USBHS_HOST_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `USBHS_SRAM_CLK` reader - USB HS SRAM clock control"]
pub type USBHS_SRAM_CLK_R = crate::BitReader<USBHS_SRAM_CLK_A>;
#[doc = "USB HS SRAM clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBHS_SRAM_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<USBHS_SRAM_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: USBHS_SRAM_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl USBHS_SRAM_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBHS_SRAM_CLK_A {
        match self.bits {
            false => USBHS_SRAM_CLK_A::CLK_DISABLE,
            true => USBHS_SRAM_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == USBHS_SRAM_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == USBHS_SRAM_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `USBHS_SRAM_CLK` writer - USB HS SRAM clock control"]
pub type USBHS_SRAM_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SPEC, USBHS_SRAM_CLK_A, O>;
impl<'a, const O: u8> USBHS_SRAM_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(USBHS_SRAM_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(USBHS_SRAM_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `SCT_CLK` reader - SCT clock control"]
pub type SCT_CLK_R = crate::BitReader<SCT_CLK_A>;
#[doc = "SCT clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCT_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<SCT_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: SCT_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl SCT_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCT_CLK_A {
        match self.bits {
            false => SCT_CLK_A::CLK_DISABLE,
            true => SCT_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == SCT_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == SCT_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `SCT_CLK` writer - SCT clock control"]
pub type SCT_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SPEC, SCT_CLK_A, O>;
impl<'a, const O: u8> SCT_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(SCT_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(SCT_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `GPU_CLK` reader - GPU clock control"]
pub type GPU_CLK_R = crate::BitReader<GPU_CLK_A>;
#[doc = "GPU clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPU_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<GPU_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: GPU_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl GPU_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPU_CLK_A {
        match self.bits {
            false => GPU_CLK_A::CLK_DISABLE,
            true => GPU_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == GPU_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == GPU_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `GPU_CLK` writer - GPU clock control"]
pub type GPU_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SPEC, GPU_CLK_A, O>;
impl<'a, const O: u8> GPU_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(GPU_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(GPU_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `DISPLAY_CTLR_CLK` reader - Display Controller clock control"]
pub type DISPLAY_CTLR_CLK_R = crate::BitReader<DISPLAY_CTLR_CLK_A>;
#[doc = "Display Controller clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISPLAY_CTLR_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<DISPLAY_CTLR_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: DISPLAY_CTLR_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl DISPLAY_CTLR_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISPLAY_CTLR_CLK_A {
        match self.bits {
            false => DISPLAY_CTLR_CLK_A::CLK_DISABLE,
            true => DISPLAY_CTLR_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == DISPLAY_CTLR_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == DISPLAY_CTLR_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `DISPLAY_CTLR_CLK` writer - Display Controller clock control"]
pub type DISPLAY_CTLR_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SPEC, DISPLAY_CTLR_CLK_A, O>;
impl<'a, const O: u8> DISPLAY_CTLR_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(DISPLAY_CTLR_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(DISPLAY_CTLR_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `MIPI_DSI_CTLR_CLK` reader - MIPI-DSI Controller clock control"]
pub type MIPI_DSI_CTLR_CLK_R = crate::BitReader<MIPI_DSI_CTLR_CLK_A>;
#[doc = "MIPI-DSI Controller clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIPI_DSI_CTLR_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<MIPI_DSI_CTLR_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: MIPI_DSI_CTLR_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl MIPI_DSI_CTLR_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIPI_DSI_CTLR_CLK_A {
        match self.bits {
            false => MIPI_DSI_CTLR_CLK_A::CLK_DISABLE,
            true => MIPI_DSI_CTLR_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == MIPI_DSI_CTLR_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == MIPI_DSI_CTLR_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `MIPI_DSI_CTLR_CLK` writer - MIPI-DSI Controller clock control"]
pub type MIPI_DSI_CTLR_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SPEC, MIPI_DSI_CTLR_CLK_A, O>;
impl<'a, const O: u8> MIPI_DSI_CTLR_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(MIPI_DSI_CTLR_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(MIPI_DSI_CTLR_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `SMARTDMA_CLK` reader - Smart DMA clock control"]
pub type SMARTDMA_CLK_R = crate::BitReader<SMARTDMA_CLK_A>;
#[doc = "Smart DMA clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMARTDMA_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<SMARTDMA_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: SMARTDMA_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl SMARTDMA_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMARTDMA_CLK_A {
        match self.bits {
            false => SMARTDMA_CLK_A::CLK_DISABLE,
            true => SMARTDMA_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == SMARTDMA_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == SMARTDMA_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `SMARTDMA_CLK` writer - Smart DMA clock control"]
pub type SMARTDMA_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SPEC, SMARTDMA_CLK_A, O>;
impl<'a, const O: u8> SMARTDMA_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(SMARTDMA_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(SMARTDMA_CLK_A::CLK_ENABLE)
    }
}
impl R {
    #[doc = "Bit 1 - DSP clock control"]
    #[inline(always)]
    pub fn dsp_clk(&self) -> DSP_CLK_R {
        DSP_CLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 128KB ROM Controller clock control"]
    #[inline(always)]
    pub fn rom_ctrlr_clk(&self) -> ROM_CTRLR_CLK_R {
        ROM_CTRLR_CLK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AXI Switch clock control"]
    #[inline(always)]
    pub fn axi_switch_clk(&self) -> AXI_SWITCH_CLK_R {
        AXI_SWITCH_CLK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AXI Controller clock control"]
    #[inline(always)]
    pub fn axi_ctlr_clk(&self) -> AXI_CTLR_CLK_R {
        AXI_CTLR_CLK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - POWERQUAD clock control"]
    #[inline(always)]
    pub fn powerquad_clk(&self) -> POWERQUAD_CLK_R {
        POWERQUAD_CLK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CASPER clock control"]
    #[inline(always)]
    pub fn casper_clk(&self) -> CASPER_CLK_R {
        CASPER_CLK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HASHCRYPT clock control"]
    #[inline(always)]
    pub fn hashcrypt_clk(&self) -> HASHCRYPT_CLK_R {
        HASHCRYPT_CLK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PUF clock control"]
    #[inline(always)]
    pub fn puf_clk(&self) -> PUF_CLK_R {
        PUF_CLK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Random Number Generator (RNG) clock control"]
    #[inline(always)]
    pub fn rng_clk(&self) -> RNG_CLK_R {
        RNG_CLK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - FLEXSPI0 / OTFAD clock control"]
    #[inline(always)]
    pub fn flexspi0_otfad_clk(&self) -> FLEXSPI0_OTFAD_CLK_R {
        FLEXSPI0_OTFAD_CLK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - OTP Controller clock control"]
    #[inline(always)]
    pub fn otp_ctlr_clk(&self) -> OTP_CTLR_CLK_R {
        OTP_CTLR_CLK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - FLEXSPI1 clock control"]
    #[inline(always)]
    pub fn flexspi1_clk(&self) -> FLEXSPI1_CLK_R {
        FLEXSPI1_CLK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - USB HS PHY clock control"]
    #[inline(always)]
    pub fn usbhs_phy_clk(&self) -> USBHS_PHY_CLK_R {
        USBHS_PHY_CLK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - USB HS Device clock control"]
    #[inline(always)]
    pub fn usbhs_device_clk(&self) -> USBHS_DEVICE_CLK_R {
        USBHS_DEVICE_CLK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - USB HS Host clock control"]
    #[inline(always)]
    pub fn usbhs_host_clk(&self) -> USBHS_HOST_CLK_R {
        USBHS_HOST_CLK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USB HS SRAM clock control"]
    #[inline(always)]
    pub fn usbhs_sram_clk(&self) -> USBHS_SRAM_CLK_R {
        USBHS_SRAM_CLK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SCT clock control"]
    #[inline(always)]
    pub fn sct_clk(&self) -> SCT_CLK_R {
        SCT_CLK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - GPU clock control"]
    #[inline(always)]
    pub fn gpu_clk(&self) -> GPU_CLK_R {
        GPU_CLK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Display Controller clock control"]
    #[inline(always)]
    pub fn display_ctlr_clk(&self) -> DISPLAY_CTLR_CLK_R {
        DISPLAY_CTLR_CLK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - MIPI-DSI Controller clock control"]
    #[inline(always)]
    pub fn mipi_dsi_ctlr_clk(&self) -> MIPI_DSI_CTLR_CLK_R {
        MIPI_DSI_CTLR_CLK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Smart DMA clock control"]
    #[inline(always)]
    pub fn smartdma_clk(&self) -> SMARTDMA_CLK_R {
        SMARTDMA_CLK_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DSP clock control"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_clk(&mut self) -> DSP_CLK_W<1> {
        DSP_CLK_W::new(self)
    }
    #[doc = "Bit 2 - 128KB ROM Controller clock control"]
    #[inline(always)]
    #[must_use]
    pub fn rom_ctrlr_clk(&mut self) -> ROM_CTRLR_CLK_W<2> {
        ROM_CTRLR_CLK_W::new(self)
    }
    #[doc = "Bit 3 - AXI Switch clock control"]
    #[inline(always)]
    #[must_use]
    pub fn axi_switch_clk(&mut self) -> AXI_SWITCH_CLK_W<3> {
        AXI_SWITCH_CLK_W::new(self)
    }
    #[doc = "Bit 4 - AXI Controller clock control"]
    #[inline(always)]
    #[must_use]
    pub fn axi_ctlr_clk(&mut self) -> AXI_CTLR_CLK_W<4> {
        AXI_CTLR_CLK_W::new(self)
    }
    #[doc = "Bit 8 - POWERQUAD clock control"]
    #[inline(always)]
    #[must_use]
    pub fn powerquad_clk(&mut self) -> POWERQUAD_CLK_W<8> {
        POWERQUAD_CLK_W::new(self)
    }
    #[doc = "Bit 9 - CASPER clock control"]
    #[inline(always)]
    #[must_use]
    pub fn casper_clk(&mut self) -> CASPER_CLK_W<9> {
        CASPER_CLK_W::new(self)
    }
    #[doc = "Bit 10 - HASHCRYPT clock control"]
    #[inline(always)]
    #[must_use]
    pub fn hashcrypt_clk(&mut self) -> HASHCRYPT_CLK_W<10> {
        HASHCRYPT_CLK_W::new(self)
    }
    #[doc = "Bit 11 - PUF clock control"]
    #[inline(always)]
    #[must_use]
    pub fn puf_clk(&mut self) -> PUF_CLK_W<11> {
        PUF_CLK_W::new(self)
    }
    #[doc = "Bit 12 - Random Number Generator (RNG) clock control"]
    #[inline(always)]
    #[must_use]
    pub fn rng_clk(&mut self) -> RNG_CLK_W<12> {
        RNG_CLK_W::new(self)
    }
    #[doc = "Bit 16 - FLEXSPI0 / OTFAD clock control"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi0_otfad_clk(&mut self) -> FLEXSPI0_OTFAD_CLK_W<16> {
        FLEXSPI0_OTFAD_CLK_W::new(self)
    }
    #[doc = "Bit 17 - OTP Controller clock control"]
    #[inline(always)]
    #[must_use]
    pub fn otp_ctlr_clk(&mut self) -> OTP_CTLR_CLK_W<17> {
        OTP_CTLR_CLK_W::new(self)
    }
    #[doc = "Bit 18 - FLEXSPI1 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi1_clk(&mut self) -> FLEXSPI1_CLK_W<18> {
        FLEXSPI1_CLK_W::new(self)
    }
    #[doc = "Bit 20 - USB HS PHY clock control"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_phy_clk(&mut self) -> USBHS_PHY_CLK_W<20> {
        USBHS_PHY_CLK_W::new(self)
    }
    #[doc = "Bit 21 - USB HS Device clock control"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_device_clk(&mut self) -> USBHS_DEVICE_CLK_W<21> {
        USBHS_DEVICE_CLK_W::new(self)
    }
    #[doc = "Bit 22 - USB HS Host clock control"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_host_clk(&mut self) -> USBHS_HOST_CLK_W<22> {
        USBHS_HOST_CLK_W::new(self)
    }
    #[doc = "Bit 23 - USB HS SRAM clock control"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_sram_clk(&mut self) -> USBHS_SRAM_CLK_W<23> {
        USBHS_SRAM_CLK_W::new(self)
    }
    #[doc = "Bit 24 - SCT clock control"]
    #[inline(always)]
    #[must_use]
    pub fn sct_clk(&mut self) -> SCT_CLK_W<24> {
        SCT_CLK_W::new(self)
    }
    #[doc = "Bit 26 - GPU clock control"]
    #[inline(always)]
    #[must_use]
    pub fn gpu_clk(&mut self) -> GPU_CLK_W<26> {
        GPU_CLK_W::new(self)
    }
    #[doc = "Bit 27 - Display Controller clock control"]
    #[inline(always)]
    #[must_use]
    pub fn display_ctlr_clk(&mut self) -> DISPLAY_CTLR_CLK_W<27> {
        DISPLAY_CTLR_CLK_W::new(self)
    }
    #[doc = "Bit 28 - MIPI-DSI Controller clock control"]
    #[inline(always)]
    #[must_use]
    pub fn mipi_dsi_ctlr_clk(&mut self) -> MIPI_DSI_CTLR_CLK_W<28> {
        MIPI_DSI_CTLR_CLK_W::new(self)
    }
    #[doc = "Bit 30 - Smart DMA clock control"]
    #[inline(always)]
    #[must_use]
    pub fn smartdma_clk(&mut self) -> SMARTDMA_CLK_W<30> {
        SMARTDMA_CLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pscctl0](index.html) module"]
pub struct PSCCTL0_SPEC;
impl crate::RegisterSpec for PSCCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pscctl0::R](R) reader structure"]
impl crate::Readable for PSCCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pscctl0::W](W) writer structure"]
impl crate::Writable for PSCCTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSCCTL0 to value 0x05"]
impl crate::Resettable for PSCCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
