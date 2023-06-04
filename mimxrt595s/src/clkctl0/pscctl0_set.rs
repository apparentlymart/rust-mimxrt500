#[doc = "Register `PSCCTL0_SET` writer"]
pub struct W(crate::W<PSCCTL0_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSCCTL0_SET_SPEC>;
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
impl From<crate::W<PSCCTL0_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSCCTL0_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DSP clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSP_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Sets the corresponding bit in PSCCTL0 register"]
    CLK_ENABLE_SET = 1,
}
impl From<DSP_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: DSP_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP_CLK` writer - DSP clock set"]
pub type DSP_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, DSP_CLK_AW, O>;
impl<'a, const O: u8> DSP_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(DSP_CLK_AW::NO_EFFECT)
    }
    #[doc = "Sets the corresponding bit in PSCCTL0 register"]
    #[inline(always)]
    pub fn clk_enable_set(self) -> &'a mut W {
        self.variant(DSP_CLK_AW::CLK_ENABLE_SET)
    }
}
#[doc = "128KB ROM Controller clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROM_CTRLR_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Sets the corresponding bit in PSCCTL0 register"]
    CLK_ENABLE_SET = 1,
}
impl From<ROM_CTRLR_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: ROM_CTRLR_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROM_CTRLR_CLK` writer - 128KB ROM Controller clock set"]
pub type ROM_CTRLR_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, ROM_CTRLR_CLK_AW, O>;
impl<'a, const O: u8> ROM_CTRLR_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ROM_CTRLR_CLK_AW::NO_EFFECT)
    }
    #[doc = "Sets the corresponding bit in PSCCTL0 register"]
    #[inline(always)]
    pub fn clk_enable_set(self) -> &'a mut W {
        self.variant(ROM_CTRLR_CLK_AW::CLK_ENABLE_SET)
    }
}
#[doc = "AXI Switch clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AXI_SWITCH_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Sets the corresponding bit in PSCCTL0 register"]
    CLK_ENABLE_SET = 1,
}
impl From<AXI_SWITCH_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: AXI_SWITCH_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AXI_SWITCH_CLK` writer - AXI Switch clock set"]
pub type AXI_SWITCH_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, AXI_SWITCH_CLK_AW, O>;
impl<'a, const O: u8> AXI_SWITCH_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(AXI_SWITCH_CLK_AW::NO_EFFECT)
    }
    #[doc = "Sets the corresponding bit in PSCCTL0 register"]
    #[inline(always)]
    pub fn clk_enable_set(self) -> &'a mut W {
        self.variant(AXI_SWITCH_CLK_AW::CLK_ENABLE_SET)
    }
}
#[doc = "AXI Controller clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AXI_CTLR_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Sets the corresponding bit in PSCCTL0 register"]
    CLK_ENABLE_SET = 1,
}
impl From<AXI_CTLR_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: AXI_CTLR_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AXI_CTLR_CLK` writer - AXI Controller clock set"]
pub type AXI_CTLR_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, AXI_CTLR_CLK_AW, O>;
impl<'a, const O: u8> AXI_CTLR_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(AXI_CTLR_CLK_AW::NO_EFFECT)
    }
    #[doc = "Sets the corresponding bit in PSCCTL0 register"]
    #[inline(always)]
    pub fn clk_enable_set(self) -> &'a mut W {
        self.variant(AXI_CTLR_CLK_AW::CLK_ENABLE_SET)
    }
}
#[doc = "POWERQUAD clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POWERQUAD_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Sets the corresponding bit in PSCCTL0 register"]
    CLK_ENABLE_SET = 1,
}
impl From<POWERQUAD_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: POWERQUAD_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POWERQUAD_CLK` writer - POWERQUAD clock set"]
pub type POWERQUAD_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, POWERQUAD_CLK_AW, O>;
impl<'a, const O: u8> POWERQUAD_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(POWERQUAD_CLK_AW::NO_EFFECT)
    }
    #[doc = "Sets the corresponding bit in PSCCTL0 register"]
    #[inline(always)]
    pub fn clk_enable_set(self) -> &'a mut W {
        self.variant(POWERQUAD_CLK_AW::CLK_ENABLE_SET)
    }
}
#[doc = "CASPER clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CASPER_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Sets the corresponding bit in PSCCTL0 register"]
    CLK_ENABLE_SET = 1,
}
impl From<CASPER_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: CASPER_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CASPER_CLK` writer - CASPER clock set"]
pub type CASPER_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, CASPER_CLK_AW, O>;
impl<'a, const O: u8> CASPER_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CASPER_CLK_AW::NO_EFFECT)
    }
    #[doc = "Sets the corresponding bit in PSCCTL0 register"]
    #[inline(always)]
    pub fn clk_enable_set(self) -> &'a mut W {
        self.variant(CASPER_CLK_AW::CLK_ENABLE_SET)
    }
}
#[doc = "HASHCRYPT clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HASHCRYPT_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Sets the corresponding bit in PSCCTL0 register"]
    CLK_ENABLE_SET = 1,
}
impl From<HASHCRYPT_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: HASHCRYPT_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASHCRYPT_CLK` writer - HASHCRYPT clock set"]
pub type HASHCRYPT_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, HASHCRYPT_CLK_AW, O>;
impl<'a, const O: u8> HASHCRYPT_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(HASHCRYPT_CLK_AW::NO_EFFECT)
    }
    #[doc = "Sets the corresponding bit in PSCCTL0 register"]
    #[inline(always)]
    pub fn clk_enable_set(self) -> &'a mut W {
        self.variant(HASHCRYPT_CLK_AW::CLK_ENABLE_SET)
    }
}
#[doc = "PUF clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PUF_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Sets the corresponding bit in PSCCTL0 register"]
    CLK_ENABLE_SET = 1,
}
impl From<PUF_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: PUF_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUF_CLK` writer - PUF clock set"]
pub type PUF_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, PUF_CLK_AW, O>;
impl<'a, const O: u8> PUF_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PUF_CLK_AW::NO_EFFECT)
    }
    #[doc = "Sets the corresponding bit in PSCCTL0 register"]
    #[inline(always)]
    pub fn clk_enable_set(self) -> &'a mut W {
        self.variant(PUF_CLK_AW::CLK_ENABLE_SET)
    }
}
#[doc = "Random Number Generator (RNG) clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNG_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Sets the corresponding bit in PSCCTL0 register"]
    CLK_ENABLE_SET = 1,
}
impl From<RNG_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: RNG_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNG_CLK` writer - Random Number Generator (RNG) clock set"]
pub type RNG_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, RNG_CLK_AW, O>;
impl<'a, const O: u8> RNG_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(RNG_CLK_AW::NO_EFFECT)
    }
    #[doc = "Sets the corresponding bit in PSCCTL0 register"]
    #[inline(always)]
    pub fn clk_enable_set(self) -> &'a mut W {
        self.variant(RNG_CLK_AW::CLK_ENABLE_SET)
    }
}
#[doc = "FLEXSPI0 / OTFAD clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI0_OTFAD_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Sets the corresponding bit in PSCCTL0 register"]
    CLK_ENABLE_SET = 1,
}
impl From<FLEXSPI0_OTFAD_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI0_OTFAD_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXSPI0_OTFAD_CLK` writer - FLEXSPI0 / OTFAD clock set"]
pub type FLEXSPI0_OTFAD_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, FLEXSPI0_OTFAD_CLK_AW, O>;
impl<'a, const O: u8> FLEXSPI0_OTFAD_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(FLEXSPI0_OTFAD_CLK_AW::NO_EFFECT)
    }
    #[doc = "Sets the corresponding bit in PSCCTL0 register"]
    #[inline(always)]
    pub fn clk_enable_set(self) -> &'a mut W {
        self.variant(FLEXSPI0_OTFAD_CLK_AW::CLK_ENABLE_SET)
    }
}
#[doc = "OTP Controller clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTP_CTLR_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Sets the corresponding bit in PSCCTL0 register"]
    CLK_ENABLE_SET = 1,
}
impl From<OTP_CTLR_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: OTP_CTLR_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTP_CTLR_CLK` writer - OTP Controller clock set"]
pub type OTP_CTLR_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, OTP_CTLR_CLK_AW, O>;
impl<'a, const O: u8> OTP_CTLR_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(OTP_CTLR_CLK_AW::NO_EFFECT)
    }
    #[doc = "Sets the corresponding bit in PSCCTL0 register"]
    #[inline(always)]
    pub fn clk_enable_set(self) -> &'a mut W {
        self.variant(OTP_CTLR_CLK_AW::CLK_ENABLE_SET)
    }
}
#[doc = "FLEXSPI1 clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI1_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Sets the corresponding bit in PSCCTL0 register"]
    CLK_ENABLE_SET = 1,
}
impl From<FLEXSPI1_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI1_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXSPI1_CLK` writer - FLEXSPI1 clock set"]
pub type FLEXSPI1_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, FLEXSPI1_CLK_AW, O>;
impl<'a, const O: u8> FLEXSPI1_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(FLEXSPI1_CLK_AW::NO_EFFECT)
    }
    #[doc = "Sets the corresponding bit in PSCCTL0 register"]
    #[inline(always)]
    pub fn clk_enable_set(self) -> &'a mut W {
        self.variant(FLEXSPI1_CLK_AW::CLK_ENABLE_SET)
    }
}
#[doc = "USB HS PHY clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBHS_PHY_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Sets the corresponding bit in PSCCTL0 register"]
    CLK_ENABLE_SET = 1,
}
impl From<USBHS_PHY_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: USBHS_PHY_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_PHY_CLK` writer - USB HS PHY clock set"]
pub type USBHS_PHY_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, USBHS_PHY_CLK_AW, O>;
impl<'a, const O: u8> USBHS_PHY_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(USBHS_PHY_CLK_AW::NO_EFFECT)
    }
    #[doc = "Sets the corresponding bit in PSCCTL0 register"]
    #[inline(always)]
    pub fn clk_enable_set(self) -> &'a mut W {
        self.variant(USBHS_PHY_CLK_AW::CLK_ENABLE_SET)
    }
}
#[doc = "USB HS Device clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBHS_DEVICE_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Sets the corresponding bit in PSCCTL0 register"]
    CLK_ENABLE_SET = 1,
}
impl From<USBHS_DEVICE_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: USBHS_DEVICE_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_DEVICE_CLK` writer - USB HS Device clock set"]
pub type USBHS_DEVICE_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, USBHS_DEVICE_CLK_AW, O>;
impl<'a, const O: u8> USBHS_DEVICE_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(USBHS_DEVICE_CLK_AW::NO_EFFECT)
    }
    #[doc = "Sets the corresponding bit in PSCCTL0 register"]
    #[inline(always)]
    pub fn clk_enable_set(self) -> &'a mut W {
        self.variant(USBHS_DEVICE_CLK_AW::CLK_ENABLE_SET)
    }
}
#[doc = "USB HS Host clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBHS_HOST_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Sets the corresponding bit in PSCCTL0 register"]
    CLK_ENABLE_SET = 1,
}
impl From<USBHS_HOST_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: USBHS_HOST_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_HOST_CLK` writer - USB HS Host clock set"]
pub type USBHS_HOST_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, USBHS_HOST_CLK_AW, O>;
impl<'a, const O: u8> USBHS_HOST_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(USBHS_HOST_CLK_AW::NO_EFFECT)
    }
    #[doc = "Sets the corresponding bit in PSCCTL0 register"]
    #[inline(always)]
    pub fn clk_enable_set(self) -> &'a mut W {
        self.variant(USBHS_HOST_CLK_AW::CLK_ENABLE_SET)
    }
}
#[doc = "USB HS SRAM clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBHS_SRAM_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Sets the corresponding bit in PSCCTL0 register"]
    CLK_ENABLE_SET = 1,
}
impl From<USBHS_SRAM_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: USBHS_SRAM_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_SRAM_CLK` writer - USB HS SRAM clock set"]
pub type USBHS_SRAM_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, USBHS_SRAM_CLK_AW, O>;
impl<'a, const O: u8> USBHS_SRAM_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(USBHS_SRAM_CLK_AW::NO_EFFECT)
    }
    #[doc = "Sets the corresponding bit in PSCCTL0 register"]
    #[inline(always)]
    pub fn clk_enable_set(self) -> &'a mut W {
        self.variant(USBHS_SRAM_CLK_AW::CLK_ENABLE_SET)
    }
}
#[doc = "SCT clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCT_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Sets the corresponding bit in PSCCTL0 register"]
    CLK_ENABLE_SET = 1,
}
impl From<SCT_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: SCT_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCT_CLK` writer - SCT clock set"]
pub type SCT_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, SCT_CLK_AW, O>;
impl<'a, const O: u8> SCT_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SCT_CLK_AW::NO_EFFECT)
    }
    #[doc = "Sets the corresponding bit in PSCCTL0 register"]
    #[inline(always)]
    pub fn clk_enable_set(self) -> &'a mut W {
        self.variant(SCT_CLK_AW::CLK_ENABLE_SET)
    }
}
#[doc = "GPU clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPU_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Sets the corresponding bit in PSCCTL0 register"]
    CLK_ENABLE_SET = 1,
}
impl From<GPU_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: GPU_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPU_CLK` writer - GPU clock set"]
pub type GPU_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, GPU_CLK_AW, O>;
impl<'a, const O: u8> GPU_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(GPU_CLK_AW::NO_EFFECT)
    }
    #[doc = "Sets the corresponding bit in PSCCTL0 register"]
    #[inline(always)]
    pub fn clk_enable_set(self) -> &'a mut W {
        self.variant(GPU_CLK_AW::CLK_ENABLE_SET)
    }
}
#[doc = "Display Controller clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISPLAY_CTLR_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Sets the corresponding bit in PSCCTL0 register"]
    CLK_ENABLE_SET = 1,
}
impl From<DISPLAY_CTLR_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: DISPLAY_CTLR_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISPLAY_CTLR_CLK` writer - Display Controller clock set"]
pub type DISPLAY_CTLR_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, DISPLAY_CTLR_CLK_AW, O>;
impl<'a, const O: u8> DISPLAY_CTLR_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(DISPLAY_CTLR_CLK_AW::NO_EFFECT)
    }
    #[doc = "Sets the corresponding bit in PSCCTL0 register"]
    #[inline(always)]
    pub fn clk_enable_set(self) -> &'a mut W {
        self.variant(DISPLAY_CTLR_CLK_AW::CLK_ENABLE_SET)
    }
}
#[doc = "MIPI-DSI Controller clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIPI_DSI_CTLR_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Sets the corresponding bit in PSCCTL0 register"]
    CLK_ENABLE_SET = 1,
}
impl From<MIPI_DSI_CTLR_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: MIPI_DSI_CTLR_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIPI_DSI_CTLR_CLK` writer - MIPI-DSI Controller clock set"]
pub type MIPI_DSI_CTLR_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, MIPI_DSI_CTLR_CLK_AW, O>;
impl<'a, const O: u8> MIPI_DSI_CTLR_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MIPI_DSI_CTLR_CLK_AW::NO_EFFECT)
    }
    #[doc = "Sets the corresponding bit in PSCCTL0 register"]
    #[inline(always)]
    pub fn clk_enable_set(self) -> &'a mut W {
        self.variant(MIPI_DSI_CTLR_CLK_AW::CLK_ENABLE_SET)
    }
}
#[doc = "Smart DMA clock set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMARTDMA_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Sets the corresponding bit in PSCCTL0 register"]
    CLK_ENABLE_SET = 1,
}
impl From<SMARTDMA_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: SMARTDMA_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMARTDMA_CLK` writer - Smart DMA clock set"]
pub type SMARTDMA_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL0_SET_SPEC, SMARTDMA_CLK_AW, O>;
impl<'a, const O: u8> SMARTDMA_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SMARTDMA_CLK_AW::NO_EFFECT)
    }
    #[doc = "Sets the corresponding bit in PSCCTL0 register"]
    #[inline(always)]
    pub fn clk_enable_set(self) -> &'a mut W {
        self.variant(SMARTDMA_CLK_AW::CLK_ENABLE_SET)
    }
}
impl W {
    #[doc = "Bit 1 - DSP clock set"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_clk(&mut self) -> DSP_CLK_W<1> {
        DSP_CLK_W::new(self)
    }
    #[doc = "Bit 2 - 128KB ROM Controller clock set"]
    #[inline(always)]
    #[must_use]
    pub fn rom_ctrlr_clk(&mut self) -> ROM_CTRLR_CLK_W<2> {
        ROM_CTRLR_CLK_W::new(self)
    }
    #[doc = "Bit 3 - AXI Switch clock set"]
    #[inline(always)]
    #[must_use]
    pub fn axi_switch_clk(&mut self) -> AXI_SWITCH_CLK_W<3> {
        AXI_SWITCH_CLK_W::new(self)
    }
    #[doc = "Bit 4 - AXI Controller clock set"]
    #[inline(always)]
    #[must_use]
    pub fn axi_ctlr_clk(&mut self) -> AXI_CTLR_CLK_W<4> {
        AXI_CTLR_CLK_W::new(self)
    }
    #[doc = "Bit 8 - POWERQUAD clock set"]
    #[inline(always)]
    #[must_use]
    pub fn powerquad_clk(&mut self) -> POWERQUAD_CLK_W<8> {
        POWERQUAD_CLK_W::new(self)
    }
    #[doc = "Bit 9 - CASPER clock set"]
    #[inline(always)]
    #[must_use]
    pub fn casper_clk(&mut self) -> CASPER_CLK_W<9> {
        CASPER_CLK_W::new(self)
    }
    #[doc = "Bit 10 - HASHCRYPT clock set"]
    #[inline(always)]
    #[must_use]
    pub fn hashcrypt_clk(&mut self) -> HASHCRYPT_CLK_W<10> {
        HASHCRYPT_CLK_W::new(self)
    }
    #[doc = "Bit 11 - PUF clock set"]
    #[inline(always)]
    #[must_use]
    pub fn puf_clk(&mut self) -> PUF_CLK_W<11> {
        PUF_CLK_W::new(self)
    }
    #[doc = "Bit 12 - Random Number Generator (RNG) clock set"]
    #[inline(always)]
    #[must_use]
    pub fn rng_clk(&mut self) -> RNG_CLK_W<12> {
        RNG_CLK_W::new(self)
    }
    #[doc = "Bit 16 - FLEXSPI0 / OTFAD clock set"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi0_otfad_clk(&mut self) -> FLEXSPI0_OTFAD_CLK_W<16> {
        FLEXSPI0_OTFAD_CLK_W::new(self)
    }
    #[doc = "Bit 17 - OTP Controller clock set"]
    #[inline(always)]
    #[must_use]
    pub fn otp_ctlr_clk(&mut self) -> OTP_CTLR_CLK_W<17> {
        OTP_CTLR_CLK_W::new(self)
    }
    #[doc = "Bit 18 - FLEXSPI1 clock set"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi1_clk(&mut self) -> FLEXSPI1_CLK_W<18> {
        FLEXSPI1_CLK_W::new(self)
    }
    #[doc = "Bit 20 - USB HS PHY clock set"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_phy_clk(&mut self) -> USBHS_PHY_CLK_W<20> {
        USBHS_PHY_CLK_W::new(self)
    }
    #[doc = "Bit 21 - USB HS Device clock set"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_device_clk(&mut self) -> USBHS_DEVICE_CLK_W<21> {
        USBHS_DEVICE_CLK_W::new(self)
    }
    #[doc = "Bit 22 - USB HS Host clock set"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_host_clk(&mut self) -> USBHS_HOST_CLK_W<22> {
        USBHS_HOST_CLK_W::new(self)
    }
    #[doc = "Bit 23 - USB HS SRAM clock set"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_sram_clk(&mut self) -> USBHS_SRAM_CLK_W<23> {
        USBHS_SRAM_CLK_W::new(self)
    }
    #[doc = "Bit 24 - SCT clock set"]
    #[inline(always)]
    #[must_use]
    pub fn sct_clk(&mut self) -> SCT_CLK_W<24> {
        SCT_CLK_W::new(self)
    }
    #[doc = "Bit 26 - GPU clock set"]
    #[inline(always)]
    #[must_use]
    pub fn gpu_clk(&mut self) -> GPU_CLK_W<26> {
        GPU_CLK_W::new(self)
    }
    #[doc = "Bit 27 - Display Controller clock set"]
    #[inline(always)]
    #[must_use]
    pub fn display_ctlr_clk(&mut self) -> DISPLAY_CTLR_CLK_W<27> {
        DISPLAY_CTLR_CLK_W::new(self)
    }
    #[doc = "Bit 28 - MIPI-DSI Controller clock set"]
    #[inline(always)]
    #[must_use]
    pub fn mipi_dsi_ctlr_clk(&mut self) -> MIPI_DSI_CTLR_CLK_W<28> {
        MIPI_DSI_CTLR_CLK_W::new(self)
    }
    #[doc = "Bit 30 - Smart DMA clock set"]
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
#[doc = "Clock Control 0 Set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pscctl0_set](index.html) module"]
pub struct PSCCTL0_SET_SPEC;
impl crate::RegisterSpec for PSCCTL0_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pscctl0_set::W](W) writer structure"]
impl crate::Writable for PSCCTL0_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSCCTL0_SET to value 0"]
impl crate::Resettable for PSCCTL0_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
