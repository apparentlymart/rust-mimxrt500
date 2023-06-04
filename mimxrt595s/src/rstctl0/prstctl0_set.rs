#[doc = "Register `PRSTCTL0_SET` writer"]
pub struct W(crate::W<PRSTCTL0_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRSTCTL0_SET_SPEC>;
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
impl From<crate::W<PRSTCTL0_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRSTCTL0_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Fusion_ DSP reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSP_AW {
    #[doc = "0: No Effect"]
    DSP_CLR = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    DSP_SET = 1,
}
impl From<DSP_AW> for bool {
    #[inline(always)]
    fn from(variant: DSP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP` writer - Fusion_ DSP reset set"]
pub type DSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SET_SPEC, DSP_AW, O>;
impl<'a, const O: u8> DSP_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn dsp_clr(self) -> &'a mut W {
        self.variant(DSP_AW::DSP_CLR)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn dsp_set(self) -> &'a mut W {
        self.variant(DSP_AW::DSP_SET)
    }
}
#[doc = "AXI SWITCH reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AXI_SWITCH_AW {
    #[doc = "0: No Effect"]
    AXI_SWITCH_CLR = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    AXI_SWITCH_SET = 1,
}
impl From<AXI_SWITCH_AW> for bool {
    #[inline(always)]
    fn from(variant: AXI_SWITCH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AXI_SWITCH` writer - AXI SWITCH reset set"]
pub type AXI_SWITCH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_SET_SPEC, AXI_SWITCH_AW, O>;
impl<'a, const O: u8> AXI_SWITCH_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn axi_switch_clr(self) -> &'a mut W {
        self.variant(AXI_SWITCH_AW::AXI_SWITCH_CLR)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn axi_switch_set(self) -> &'a mut W {
        self.variant(AXI_SWITCH_AW::AXI_SWITCH_SET)
    }
}
#[doc = "POWERQUAD reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POWERQUAD_AW {
    #[doc = "0: No Effect"]
    POWERQUAD_CLR = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    POWERQUAD_SET = 1,
}
impl From<POWERQUAD_AW> for bool {
    #[inline(always)]
    fn from(variant: POWERQUAD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POWERQUAD` writer - POWERQUAD reset set"]
pub type POWERQUAD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_SET_SPEC, POWERQUAD_AW, O>;
impl<'a, const O: u8> POWERQUAD_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn powerquad_clr(self) -> &'a mut W {
        self.variant(POWERQUAD_AW::POWERQUAD_CLR)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn powerquad_set(self) -> &'a mut W {
        self.variant(POWERQUAD_AW::POWERQUAD_SET)
    }
}
#[doc = "CASPER reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CASPER_AW {
    #[doc = "0: No Effect"]
    CASPER_CLR = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    CASPER_SET = 1,
}
impl From<CASPER_AW> for bool {
    #[inline(always)]
    fn from(variant: CASPER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CASPER` writer - CASPER reset set"]
pub type CASPER_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SET_SPEC, CASPER_AW, O>;
impl<'a, const O: u8> CASPER_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn casper_clr(self) -> &'a mut W {
        self.variant(CASPER_AW::CASPER_CLR)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn casper_set(self) -> &'a mut W {
        self.variant(CASPER_AW::CASPER_SET)
    }
}
#[doc = "HASHCRYPT reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HASHCRYPT_AW {
    #[doc = "0: No Effect"]
    HASHCRYPT_CLR = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    HASHCRYPT_SET = 1,
}
impl From<HASHCRYPT_AW> for bool {
    #[inline(always)]
    fn from(variant: HASHCRYPT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASHCRYPT` writer - HASHCRYPT reset set"]
pub type HASHCRYPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_SET_SPEC, HASHCRYPT_AW, O>;
impl<'a, const O: u8> HASHCRYPT_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn hashcrypt_clr(self) -> &'a mut W {
        self.variant(HASHCRYPT_AW::HASHCRYPT_CLR)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn hashcrypt_set(self) -> &'a mut W {
        self.variant(HASHCRYPT_AW::HASHCRYPT_SET)
    }
}
#[doc = "PUF reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PUF_AW {
    #[doc = "0: No Effect"]
    PUF_CLR = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    PUF_SET = 1,
}
impl From<PUF_AW> for bool {
    #[inline(always)]
    fn from(variant: PUF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUF` writer - PUF reset set"]
pub type PUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SET_SPEC, PUF_AW, O>;
impl<'a, const O: u8> PUF_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn puf_clr(self) -> &'a mut W {
        self.variant(PUF_AW::PUF_CLR)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn puf_set(self) -> &'a mut W {
        self.variant(PUF_AW::PUF_SET)
    }
}
#[doc = "RNG reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNG_AW {
    #[doc = "0: No Effect"]
    RNG_CLR = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    RNG_SET = 1,
}
impl From<RNG_AW> for bool {
    #[inline(always)]
    fn from(variant: RNG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNG` writer - RNG reset set"]
pub type RNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SET_SPEC, RNG_AW, O>;
impl<'a, const O: u8> RNG_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn rng_clr(self) -> &'a mut W {
        self.variant(RNG_AW::RNG_CLR)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn rng_set(self) -> &'a mut W {
        self.variant(RNG_AW::RNG_SET)
    }
}
#[doc = "FLEXSPI0 and OTFAD reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI0_OTFAD_AW {
    #[doc = "0: No Effect"]
    FLEXSPI0_OTFAD_CLR = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    FLEXSPI0_OTFAD_SET = 1,
}
impl From<FLEXSPI0_OTFAD_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI0_OTFAD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXSPI0_OTFAD` writer - FLEXSPI0 and OTFAD reset set"]
pub type FLEXSPI0_OTFAD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_SET_SPEC, FLEXSPI0_OTFAD_AW, O>;
impl<'a, const O: u8> FLEXSPI0_OTFAD_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn flexspi0_otfad_clr(self) -> &'a mut W {
        self.variant(FLEXSPI0_OTFAD_AW::FLEXSPI0_OTFAD_CLR)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn flexspi0_otfad_set(self) -> &'a mut W {
        self.variant(FLEXSPI0_OTFAD_AW::FLEXSPI0_OTFAD_SET)
    }
}
#[doc = "FLEXSPI1 reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI1_AW {
    #[doc = "0: No Effect"]
    FLEXSPI1_CLR = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    FLEXSPI1_SET = 1,
}
impl From<FLEXSPI1_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXSPI1` writer - FLEXSPI1 reset set"]
pub type FLEXSPI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SET_SPEC, FLEXSPI1_AW, O>;
impl<'a, const O: u8> FLEXSPI1_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn flexspi1_clr(self) -> &'a mut W {
        self.variant(FLEXSPI1_AW::FLEXSPI1_CLR)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn flexspi1_set(self) -> &'a mut W {
        self.variant(FLEXSPI1_AW::FLEXSPI1_SET)
    }
}
#[doc = "USB PHY reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBHS_PHY_AW {
    #[doc = "0: No Effect"]
    USBHS_PHY_CLR = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    USBHS_PHY_SET = 1,
}
impl From<USBHS_PHY_AW> for bool {
    #[inline(always)]
    fn from(variant: USBHS_PHY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_PHY` writer - USB PHY reset set"]
pub type USBHS_PHY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_SET_SPEC, USBHS_PHY_AW, O>;
impl<'a, const O: u8> USBHS_PHY_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn usbhs_phy_clr(self) -> &'a mut W {
        self.variant(USBHS_PHY_AW::USBHS_PHY_CLR)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn usbhs_phy_set(self) -> &'a mut W {
        self.variant(USBHS_PHY_AW::USBHS_PHY_SET)
    }
}
#[doc = "USB Device reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBHS_DEVICE_AW {
    #[doc = "0: No Effect"]
    USBHS_DEVICE_CLR = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    USBHS_DEVICE_SET = 1,
}
impl From<USBHS_DEVICE_AW> for bool {
    #[inline(always)]
    fn from(variant: USBHS_DEVICE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_DEVICE` writer - USB Device reset set"]
pub type USBHS_DEVICE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_SET_SPEC, USBHS_DEVICE_AW, O>;
impl<'a, const O: u8> USBHS_DEVICE_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn usbhs_device_clr(self) -> &'a mut W {
        self.variant(USBHS_DEVICE_AW::USBHS_DEVICE_CLR)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn usbhs_device_set(self) -> &'a mut W {
        self.variant(USBHS_DEVICE_AW::USBHS_DEVICE_SET)
    }
}
#[doc = "USB HOST reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBHS_HOST_AW {
    #[doc = "0: No Effect"]
    USBHS_HOST_CLR = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    USBHS_HOST_SET = 1,
}
impl From<USBHS_HOST_AW> for bool {
    #[inline(always)]
    fn from(variant: USBHS_HOST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_HOST` writer - USB HOST reset set"]
pub type USBHS_HOST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_SET_SPEC, USBHS_HOST_AW, O>;
impl<'a, const O: u8> USBHS_HOST_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn usbhs_host_clr(self) -> &'a mut W {
        self.variant(USBHS_HOST_AW::USBHS_HOST_CLR)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn usbhs_host_set(self) -> &'a mut W {
        self.variant(USBHS_HOST_AW::USBHS_HOST_SET)
    }
}
#[doc = "USBHS SRAM reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBHS_SRAM_AW {
    #[doc = "0: No Effect"]
    USBHS_SRAM_CLR = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    USBHS_SRAM_SET = 1,
}
impl From<USBHS_SRAM_AW> for bool {
    #[inline(always)]
    fn from(variant: USBHS_SRAM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_SRAM` writer - USBHS SRAM reset set"]
pub type USBHS_SRAM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_SET_SPEC, USBHS_SRAM_AW, O>;
impl<'a, const O: u8> USBHS_SRAM_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn usbhs_sram_clr(self) -> &'a mut W {
        self.variant(USBHS_SRAM_AW::USBHS_SRAM_CLR)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn usbhs_sram_set(self) -> &'a mut W {
        self.variant(USBHS_SRAM_AW::USBHS_SRAM_SET)
    }
}
#[doc = "SCTimer reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCT_AW {
    #[doc = "0: No Effect"]
    SCT_CLR = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SCT_SET = 1,
}
impl From<SCT_AW> for bool {
    #[inline(always)]
    fn from(variant: SCT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCT` writer - SCTimer reset set"]
pub type SCT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SET_SPEC, SCT_AW, O>;
impl<'a, const O: u8> SCT_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn sct_clr(self) -> &'a mut W {
        self.variant(SCT_AW::SCT_CLR)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn sct_set(self) -> &'a mut W {
        self.variant(SCT_AW::SCT_SET)
    }
}
#[doc = "GPU reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPU_AW {
    #[doc = "0: No Effect"]
    GPU_CLR = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    GPU_SET = 1,
}
impl From<GPU_AW> for bool {
    #[inline(always)]
    fn from(variant: GPU_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPU` writer - GPU reset set"]
pub type GPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SET_SPEC, GPU_AW, O>;
impl<'a, const O: u8> GPU_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn gpu_clr(self) -> &'a mut W {
        self.variant(GPU_AW::GPU_CLR)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn gpu_set(self) -> &'a mut W {
        self.variant(GPU_AW::GPU_SET)
    }
}
#[doc = "LCDIF DISPLAY CONTROLLER reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISPLAY_CONTROLLER_AW {
    #[doc = "0: No Effect"]
    DISPLAY_CONTROLLER_CLR = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    DISPLAY_CONTROLLER_SET = 1,
}
impl From<DISPLAY_CONTROLLER_AW> for bool {
    #[inline(always)]
    fn from(variant: DISPLAY_CONTROLLER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISPLAY_CONTROLLER` writer - LCDIF DISPLAY CONTROLLER reset set"]
pub type DISPLAY_CONTROLLER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_SET_SPEC, DISPLAY_CONTROLLER_AW, O>;
impl<'a, const O: u8> DISPLAY_CONTROLLER_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn display_controller_clr(self) -> &'a mut W {
        self.variant(DISPLAY_CONTROLLER_AW::DISPLAY_CONTROLLER_CLR)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn display_controller_set(self) -> &'a mut W {
        self.variant(DISPLAY_CONTROLLER_AW::DISPLAY_CONTROLLER_SET)
    }
}
#[doc = "MIPI DSI controller reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIPI_DSI_CONTROLLER_AW {
    #[doc = "0: No Effect"]
    MIPI_DSI_CONTROLLER_CLR = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    MIPI_DSI_CONTROLLER_SET = 1,
}
impl From<MIPI_DSI_CONTROLLER_AW> for bool {
    #[inline(always)]
    fn from(variant: MIPI_DSI_CONTROLLER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIPI_DSI_CONTROLLER` writer - MIPI DSI controller reset set"]
pub type MIPI_DSI_CONTROLLER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_SET_SPEC, MIPI_DSI_CONTROLLER_AW, O>;
impl<'a, const O: u8> MIPI_DSI_CONTROLLER_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn mipi_dsi_controller_clr(self) -> &'a mut W {
        self.variant(MIPI_DSI_CONTROLLER_AW::MIPI_DSI_CONTROLLER_CLR)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn mipi_dsi_controller_set(self) -> &'a mut W {
        self.variant(MIPI_DSI_CONTROLLER_AW::MIPI_DSI_CONTROLLER_SET)
    }
}
#[doc = "MIPI DSI PHY reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIPI_DSI_PHY_AW {
    #[doc = "0: No Effect"]
    MIPI_DSI_PHY_CLR = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    MIPI_DSI_PHY_SET = 1,
}
impl From<MIPI_DSI_PHY_AW> for bool {
    #[inline(always)]
    fn from(variant: MIPI_DSI_PHY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIPI_DSI_PHY` writer - MIPI DSI PHY reset set"]
pub type MIPI_DSI_PHY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRSTCTL0_SET_SPEC, MIPI_DSI_PHY_AW, O>;
impl<'a, const O: u8> MIPI_DSI_PHY_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn mipi_dsi_phy_clr(self) -> &'a mut W {
        self.variant(MIPI_DSI_PHY_AW::MIPI_DSI_PHY_CLR)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn mipi_dsi_phy_set(self) -> &'a mut W {
        self.variant(MIPI_DSI_PHY_AW::MIPI_DSI_PHY_SET)
    }
}
#[doc = "SMARTDMA Event/Algorithm handler reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMARTDMA_AW {
    #[doc = "0: No Effect"]
    SMARTDMA_CLR = 0,
    #[doc = "1: Sets the PRSTCTL0 Bit"]
    SMARTDMA_SET = 1,
}
impl From<SMARTDMA_AW> for bool {
    #[inline(always)]
    fn from(variant: SMARTDMA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMARTDMA` writer - SMARTDMA Event/Algorithm handler reset set"]
pub type SMARTDMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL0_SET_SPEC, SMARTDMA_AW, O>;
impl<'a, const O: u8> SMARTDMA_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn smartdma_clr(self) -> &'a mut W {
        self.variant(SMARTDMA_AW::SMARTDMA_CLR)
    }
    #[doc = "Sets the PRSTCTL0 Bit"]
    #[inline(always)]
    pub fn smartdma_set(self) -> &'a mut W {
        self.variant(SMARTDMA_AW::SMARTDMA_SET)
    }
}
impl W {
    #[doc = "Bit 1 - Fusion_ DSP reset set"]
    #[inline(always)]
    #[must_use]
    pub fn dsp(&mut self) -> DSP_W<1> {
        DSP_W::new(self)
    }
    #[doc = "Bit 3 - AXI SWITCH reset set"]
    #[inline(always)]
    #[must_use]
    pub fn axi_switch(&mut self) -> AXI_SWITCH_W<3> {
        AXI_SWITCH_W::new(self)
    }
    #[doc = "Bit 8 - POWERQUAD reset set"]
    #[inline(always)]
    #[must_use]
    pub fn powerquad(&mut self) -> POWERQUAD_W<8> {
        POWERQUAD_W::new(self)
    }
    #[doc = "Bit 9 - CASPER reset set"]
    #[inline(always)]
    #[must_use]
    pub fn casper(&mut self) -> CASPER_W<9> {
        CASPER_W::new(self)
    }
    #[doc = "Bit 10 - HASHCRYPT reset set"]
    #[inline(always)]
    #[must_use]
    pub fn hashcrypt(&mut self) -> HASHCRYPT_W<10> {
        HASHCRYPT_W::new(self)
    }
    #[doc = "Bit 11 - PUF reset set"]
    #[inline(always)]
    #[must_use]
    pub fn puf(&mut self) -> PUF_W<11> {
        PUF_W::new(self)
    }
    #[doc = "Bit 12 - RNG reset set"]
    #[inline(always)]
    #[must_use]
    pub fn rng(&mut self) -> RNG_W<12> {
        RNG_W::new(self)
    }
    #[doc = "Bit 16 - FLEXSPI0 and OTFAD reset set"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi0_otfad(&mut self) -> FLEXSPI0_OTFAD_W<16> {
        FLEXSPI0_OTFAD_W::new(self)
    }
    #[doc = "Bit 18 - FLEXSPI1 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi1(&mut self) -> FLEXSPI1_W<18> {
        FLEXSPI1_W::new(self)
    }
    #[doc = "Bit 20 - USB PHY reset set"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_phy(&mut self) -> USBHS_PHY_W<20> {
        USBHS_PHY_W::new(self)
    }
    #[doc = "Bit 21 - USB Device reset set"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_device(&mut self) -> USBHS_DEVICE_W<21> {
        USBHS_DEVICE_W::new(self)
    }
    #[doc = "Bit 22 - USB HOST reset set"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_host(&mut self) -> USBHS_HOST_W<22> {
        USBHS_HOST_W::new(self)
    }
    #[doc = "Bit 23 - USBHS SRAM reset set"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_sram(&mut self) -> USBHS_SRAM_W<23> {
        USBHS_SRAM_W::new(self)
    }
    #[doc = "Bit 24 - SCTimer reset set"]
    #[inline(always)]
    #[must_use]
    pub fn sct(&mut self) -> SCT_W<24> {
        SCT_W::new(self)
    }
    #[doc = "Bit 26 - GPU reset set"]
    #[inline(always)]
    #[must_use]
    pub fn gpu(&mut self) -> GPU_W<26> {
        GPU_W::new(self)
    }
    #[doc = "Bit 27 - LCDIF DISPLAY CONTROLLER reset set"]
    #[inline(always)]
    #[must_use]
    pub fn display_controller(&mut self) -> DISPLAY_CONTROLLER_W<27> {
        DISPLAY_CONTROLLER_W::new(self)
    }
    #[doc = "Bit 28 - MIPI DSI controller reset set"]
    #[inline(always)]
    #[must_use]
    pub fn mipi_dsi_controller(&mut self) -> MIPI_DSI_CONTROLLER_W<28> {
        MIPI_DSI_CONTROLLER_W::new(self)
    }
    #[doc = "Bit 29 - MIPI DSI PHY reset set"]
    #[inline(always)]
    #[must_use]
    pub fn mipi_dsi_phy(&mut self) -> MIPI_DSI_PHY_W<29> {
        MIPI_DSI_PHY_W::new(self)
    }
    #[doc = "Bit 30 - SMARTDMA Event/Algorithm handler reset set"]
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
#[doc = "Peripheral Reset Control Register 0 SET\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prstctl0_set](index.html) module"]
pub struct PRSTCTL0_SET_SPEC;
impl crate::RegisterSpec for PRSTCTL0_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [prstctl0_set::W](W) writer structure"]
impl crate::Writable for PRSTCTL0_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRSTCTL0_SET to value 0"]
impl crate::Resettable for PRSTCTL0_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
