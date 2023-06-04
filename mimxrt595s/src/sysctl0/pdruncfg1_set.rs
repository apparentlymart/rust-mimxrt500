#[doc = "Register `PDRUNCFG1_SET` writer"]
pub struct W(crate::W<PDRUNCFG1_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDRUNCFG1_SET_SPEC>;
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
impl From<crate::W<PDRUNCFG1_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDRUNCFG1_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Array power for PowerQuad RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PQ_SRAM_APD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<PQ_SRAM_APD_AW> for bool {
    #[inline(always)]
    fn from(variant: PQ_SRAM_APD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PQ_SRAM_APD` writer - Array power for PowerQuad RAM"]
pub type PQ_SRAM_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, PQ_SRAM_APD_AW, O>;
impl<'a, const O: u8> PQ_SRAM_APD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PQ_SRAM_APD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PQ_SRAM_APD_AW::ENABLE)
    }
}
#[doc = "Periphery power for PowerQuad RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PQ_SRAM_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<PQ_SRAM_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: PQ_SRAM_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PQ_SRAM_PPD` writer - Periphery power for PowerQuad RAM"]
pub type PQ_SRAM_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, PQ_SRAM_PPD_AW, O>;
impl<'a, const O: u8> PQ_SRAM_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PQ_SRAM_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PQ_SRAM_PPD_AW::ENABLE)
    }
}
#[doc = "Array power for FLEXSPI0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI0_SRAM_APD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<FLEXSPI0_SRAM_APD_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI0_SRAM_APD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXSPI0_SRAM_APD` writer - Array power for FLEXSPI0"]
pub type FLEXSPI0_SRAM_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, FLEXSPI0_SRAM_APD_AW, O>;
impl<'a, const O: u8> FLEXSPI0_SRAM_APD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXSPI0_SRAM_APD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXSPI0_SRAM_APD_AW::ENABLE)
    }
}
#[doc = "Periphery power for FLEXSPI0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI0_SRAM_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<FLEXSPI0_SRAM_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI0_SRAM_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXSPI0_SRAM_PPD` writer - Periphery power for FLEXSPI0"]
pub type FLEXSPI0_SRAM_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, FLEXSPI0_SRAM_PPD_AW, O>;
impl<'a, const O: u8> FLEXSPI0_SRAM_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXSPI0_SRAM_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXSPI0_SRAM_PPD_AW::ENABLE)
    }
}
#[doc = "Array power for FLEXSPI1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI1_SRAM_APD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<FLEXSPI1_SRAM_APD_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI1_SRAM_APD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXSPI1_SRAM_APD` writer - Array power for FLEXSPI1"]
pub type FLEXSPI1_SRAM_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, FLEXSPI1_SRAM_APD_AW, O>;
impl<'a, const O: u8> FLEXSPI1_SRAM_APD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXSPI1_SRAM_APD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXSPI1_SRAM_APD_AW::ENABLE)
    }
}
#[doc = "Periphery power for FLEXSPI1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI1_SRAM_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<FLEXSPI1_SRAM_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI1_SRAM_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXSPI1_SRAM_PPD` writer - Periphery power for FLEXSPI1"]
pub type FLEXSPI1_SRAM_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, FLEXSPI1_SRAM_PPD_AW, O>;
impl<'a, const O: u8> FLEXSPI1_SRAM_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXSPI1_SRAM_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXSPI1_SRAM_PPD_AW::ENABLE)
    }
}
#[doc = "Array power for USB RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBHS_SRAM_APD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<USBHS_SRAM_APD_AW> for bool {
    #[inline(always)]
    fn from(variant: USBHS_SRAM_APD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_SRAM_APD` writer - Array power for USB RAM"]
pub type USBHS_SRAM_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, USBHS_SRAM_APD_AW, O>;
impl<'a, const O: u8> USBHS_SRAM_APD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USBHS_SRAM_APD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USBHS_SRAM_APD_AW::ENABLE)
    }
}
#[doc = "Periphery power for USB RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBHS_SRAM_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<USBHS_SRAM_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: USBHS_SRAM_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBHS_SRAM_PPD` writer - Periphery power for USB RAM"]
pub type USBHS_SRAM_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, USBHS_SRAM_PPD_AW, O>;
impl<'a, const O: u8> USBHS_SRAM_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USBHS_SRAM_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USBHS_SRAM_PPD_AW::ENABLE)
    }
}
#[doc = "Array power for uSDHC0 (SD/MMC/SDIO interface) RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USDHC0_SRAM_APD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<USDHC0_SRAM_APD_AW> for bool {
    #[inline(always)]
    fn from(variant: USDHC0_SRAM_APD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USDHC0_SRAM_APD` writer - Array power for uSDHC0 (SD/MMC/SDIO interface) RAM"]
pub type USDHC0_SRAM_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, USDHC0_SRAM_APD_AW, O>;
impl<'a, const O: u8> USDHC0_SRAM_APD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USDHC0_SRAM_APD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USDHC0_SRAM_APD_AW::ENABLE)
    }
}
#[doc = "Periphery power for uSDHC0 (SD/MMC/SDIO interface) RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USDHC0_SRAM_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<USDHC0_SRAM_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: USDHC0_SRAM_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USDHC0_SRAM_PPD` writer - Periphery power for uSDHC0 (SD/MMC/SDIO interface) RAM"]
pub type USDHC0_SRAM_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, USDHC0_SRAM_PPD_AW, O>;
impl<'a, const O: u8> USDHC0_SRAM_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USDHC0_SRAM_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USDHC0_SRAM_PPD_AW::ENABLE)
    }
}
#[doc = "Array power for uSDHC1 (SD/MMC/SDIO interface) RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USDHC1_SRAM_APD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<USDHC1_SRAM_APD_AW> for bool {
    #[inline(always)]
    fn from(variant: USDHC1_SRAM_APD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USDHC1_SRAM_APD` writer - Array power for uSDHC1 (SD/MMC/SDIO interface) RAM"]
pub type USDHC1_SRAM_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, USDHC1_SRAM_APD_AW, O>;
impl<'a, const O: u8> USDHC1_SRAM_APD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USDHC1_SRAM_APD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USDHC1_SRAM_APD_AW::ENABLE)
    }
}
#[doc = "Periphery power for uSDHC1 (SD/MMC/SDIO interface) RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USDHC1_SRAM_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<USDHC1_SRAM_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: USDHC1_SRAM_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USDHC1_SRAM_PPD` writer - Periphery power for uSDHC1 (SD/MMC/SDIO interface) RAM"]
pub type USDHC1_SRAM_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, USDHC1_SRAM_PPD_AW, O>;
impl<'a, const O: u8> USDHC1_SRAM_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USDHC1_SRAM_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USDHC1_SRAM_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for Casper RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CASPER_SRAM_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<CASPER_SRAM_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: CASPER_SRAM_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CASPER_SRAM_PPD` writer - Periphery power for Casper RAM"]
pub type CASPER_SRAM_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, CASPER_SRAM_PPD_AW, O>;
impl<'a, const O: u8> CASPER_SRAM_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CASPER_SRAM_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CASPER_SRAM_PPD_AW::ENABLE)
    }
}
#[doc = "Array power for GPU SRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPU_SRAM_APD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<GPU_SRAM_APD_AW> for bool {
    #[inline(always)]
    fn from(variant: GPU_SRAM_APD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPU_SRAM_APD` writer - Array power for GPU SRAM"]
pub type GPU_SRAM_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, GPU_SRAM_APD_AW, O>;
impl<'a, const O: u8> GPU_SRAM_APD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPU_SRAM_APD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPU_SRAM_APD_AW::ENABLE)
    }
}
#[doc = "Periphery power for GPU SRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPU_SRAM_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<GPU_SRAM_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: GPU_SRAM_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPU_SRAM_PPD` writer - Periphery power for GPU SRAM"]
pub type GPU_SRAM_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, GPU_SRAM_PPD_AW, O>;
impl<'a, const O: u8> GPU_SRAM_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPU_SRAM_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPU_SRAM_PPD_AW::ENABLE)
    }
}
#[doc = "Array power for SMARTDMA SRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMARTDMA_SRAM_APD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<SMARTDMA_SRAM_APD_AW> for bool {
    #[inline(always)]
    fn from(variant: SMARTDMA_SRAM_APD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMARTDMA_SRAM_APD` writer - Array power for SMARTDMA SRAM"]
pub type SMARTDMA_SRAM_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, SMARTDMA_SRAM_APD_AW, O>;
impl<'a, const O: u8> SMARTDMA_SRAM_APD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SMARTDMA_SRAM_APD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SMARTDMA_SRAM_APD_AW::ENABLE)
    }
}
#[doc = "Periphery power for SMARTDMA SRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMARTDMA_SRAM_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<SMARTDMA_SRAM_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SMARTDMA_SRAM_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMARTDMA_SRAM_PPD` writer - Periphery power for SMARTDMA SRAM"]
pub type SMARTDMA_SRAM_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, SMARTDMA_SRAM_PPD_AW, O>;
impl<'a, const O: u8> SMARTDMA_SRAM_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SMARTDMA_SRAM_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SMARTDMA_SRAM_PPD_AW::ENABLE)
    }
}
#[doc = "Array power for MIPIDSI SRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIPIDSI_SRAM_APD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<MIPIDSI_SRAM_APD_AW> for bool {
    #[inline(always)]
    fn from(variant: MIPIDSI_SRAM_APD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIPIDSI_SRAM_APD` writer - Array power for MIPIDSI SRAM"]
pub type MIPIDSI_SRAM_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, MIPIDSI_SRAM_APD_AW, O>;
impl<'a, const O: u8> MIPIDSI_SRAM_APD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MIPIDSI_SRAM_APD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MIPIDSI_SRAM_APD_AW::ENABLE)
    }
}
#[doc = "Periphery power for MIPIDSI SRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIPIDSI_SRAM_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<MIPIDSI_SRAM_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: MIPIDSI_SRAM_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIPIDSI_SRAM_PPD` writer - Periphery power for MIPIDSI SRAM"]
pub type MIPIDSI_SRAM_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, MIPIDSI_SRAM_PPD_AW, O>;
impl<'a, const O: u8> MIPIDSI_SRAM_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MIPIDSI_SRAM_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MIPIDSI_SRAM_PPD_AW::ENABLE)
    }
}
#[doc = "Array power for LCDIF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDIF_SRAM_APD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<LCDIF_SRAM_APD_AW> for bool {
    #[inline(always)]
    fn from(variant: LCDIF_SRAM_APD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDIF_SRAM_APD` writer - Array power for LCDIF"]
pub type LCDIF_SRAM_APD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, LCDIF_SRAM_APD_AW, O>;
impl<'a, const O: u8> LCDIF_SRAM_APD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LCDIF_SRAM_APD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LCDIF_SRAM_APD_AW::ENABLE)
    }
}
#[doc = "Periphery power for LCDIF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDIF_SRAM_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<LCDIF_SRAM_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: LCDIF_SRAM_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDIF_SRAM_PPD` writer - Periphery power for LCDIF"]
pub type LCDIF_SRAM_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, LCDIF_SRAM_PPD_AW, O>;
impl<'a, const O: u8> LCDIF_SRAM_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LCDIF_SRAM_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LCDIF_SRAM_PPD_AW::ENABLE)
    }
}
#[doc = "Array and periphery power for DSP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSP_PD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<DSP_PD_AW> for bool {
    #[inline(always)]
    fn from(variant: DSP_PD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSP_PD` writer - Array and periphery power for DSP"]
pub type DSP_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, DSP_PD_AW, O>;
impl<'a, const O: u8> DSP_PD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DSP_PD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DSP_PD_AW::ENABLE)
    }
}
#[doc = "Array and periphery power for MIPIDSI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIPIDSI_PD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<MIPIDSI_PD_AW> for bool {
    #[inline(always)]
    fn from(variant: MIPIDSI_PD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIPIDSI_PD` writer - Array and periphery power for MIPIDSI"]
pub type MIPIDSI_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, MIPIDSI_PD_AW, O>;
impl<'a, const O: u8> MIPIDSI_PD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MIPIDSI_PD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MIPIDSI_PD_AW::ENABLE)
    }
}
#[doc = "Array and periphery power for OTP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTP_PD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<OTP_PD_AW> for bool {
    #[inline(always)]
    fn from(variant: OTP_PD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTP_PD` writer - Array and periphery power for OTP"]
pub type OTP_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, OTP_PD_AW, O>;
impl<'a, const O: u8> OTP_PD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OTP_PD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OTP_PD_AW::ENABLE)
    }
}
#[doc = "Array and periphery power for ROM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROM_PD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<ROM_PD_AW> for bool {
    #[inline(always)]
    fn from(variant: ROM_PD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROM_PD` writer - Array and periphery power for ROM"]
pub type ROM_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, ROM_PD_AW, O>;
impl<'a, const O: u8> ROM_PD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ROM_PD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ROM_PD_AW::ENABLE)
    }
}
#[doc = "SRAM sleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_SLEEP_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG1 Bit"]
    ENABLE = 1,
}
impl From<SRAM_SLEEP_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_SLEEP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_SLEEP` writer - SRAM sleep mode"]
pub type SRAM_SLEEP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG1_SET_SPEC, SRAM_SLEEP_AW, O>;
impl<'a, const O: u8> SRAM_SLEEP_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_SLEEP_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_SLEEP_AW::ENABLE)
    }
}
impl W {
    #[doc = "Bit 0 - Array power for PowerQuad RAM"]
    #[inline(always)]
    #[must_use]
    pub fn pq_sram_apd(&mut self) -> PQ_SRAM_APD_W<0> {
        PQ_SRAM_APD_W::new(self)
    }
    #[doc = "Bit 1 - Periphery power for PowerQuad RAM"]
    #[inline(always)]
    #[must_use]
    pub fn pq_sram_ppd(&mut self) -> PQ_SRAM_PPD_W<1> {
        PQ_SRAM_PPD_W::new(self)
    }
    #[doc = "Bit 2 - Array power for FLEXSPI0"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi0_sram_apd(&mut self) -> FLEXSPI0_SRAM_APD_W<2> {
        FLEXSPI0_SRAM_APD_W::new(self)
    }
    #[doc = "Bit 3 - Periphery power for FLEXSPI0"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi0_sram_ppd(&mut self) -> FLEXSPI0_SRAM_PPD_W<3> {
        FLEXSPI0_SRAM_PPD_W::new(self)
    }
    #[doc = "Bit 4 - Array power for FLEXSPI1"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi1_sram_apd(&mut self) -> FLEXSPI1_SRAM_APD_W<4> {
        FLEXSPI1_SRAM_APD_W::new(self)
    }
    #[doc = "Bit 5 - Periphery power for FLEXSPI1"]
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
    #[doc = "Bit 10 - Array power for uSDHC1 (SD/MMC/SDIO interface) RAM"]
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
    #[doc = "Bit 14 - Array power for GPU SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn gpu_sram_apd(&mut self) -> GPU_SRAM_APD_W<14> {
        GPU_SRAM_APD_W::new(self)
    }
    #[doc = "Bit 15 - Periphery power for GPU SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn gpu_sram_ppd(&mut self) -> GPU_SRAM_PPD_W<15> {
        GPU_SRAM_PPD_W::new(self)
    }
    #[doc = "Bit 16 - Array power for SMARTDMA SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn smartdma_sram_apd(&mut self) -> SMARTDMA_SRAM_APD_W<16> {
        SMARTDMA_SRAM_APD_W::new(self)
    }
    #[doc = "Bit 17 - Periphery power for SMARTDMA SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn smartdma_sram_ppd(&mut self) -> SMARTDMA_SRAM_PPD_W<17> {
        SMARTDMA_SRAM_PPD_W::new(self)
    }
    #[doc = "Bit 18 - Array power for MIPIDSI SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn mipidsi_sram_apd(&mut self) -> MIPIDSI_SRAM_APD_W<18> {
        MIPIDSI_SRAM_APD_W::new(self)
    }
    #[doc = "Bit 19 - Periphery power for MIPIDSI SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn mipidsi_sram_ppd(&mut self) -> MIPIDSI_SRAM_PPD_W<19> {
        MIPIDSI_SRAM_PPD_W::new(self)
    }
    #[doc = "Bit 20 - Array power for LCDIF"]
    #[inline(always)]
    #[must_use]
    pub fn lcdif_sram_apd(&mut self) -> LCDIF_SRAM_APD_W<20> {
        LCDIF_SRAM_APD_W::new(self)
    }
    #[doc = "Bit 21 - Periphery power for LCDIF"]
    #[inline(always)]
    #[must_use]
    pub fn lcdif_sram_ppd(&mut self) -> LCDIF_SRAM_PPD_W<21> {
        LCDIF_SRAM_PPD_W::new(self)
    }
    #[doc = "Bit 25 - Array and periphery power for DSP"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_pd(&mut self) -> DSP_PD_W<25> {
        DSP_PD_W::new(self)
    }
    #[doc = "Bit 26 - Array and periphery power for MIPIDSI"]
    #[inline(always)]
    #[must_use]
    pub fn mipidsi_pd(&mut self) -> MIPIDSI_PD_W<26> {
        MIPIDSI_PD_W::new(self)
    }
    #[doc = "Bit 27 - Array and periphery power for OTP"]
    #[inline(always)]
    #[must_use]
    pub fn otp_pd(&mut self) -> OTP_PD_W<27> {
        OTP_PD_W::new(self)
    }
    #[doc = "Bit 28 - Array and periphery power for ROM"]
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
#[doc = "Run configuration 1 set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdruncfg1_set](index.html) module"]
pub struct PDRUNCFG1_SET_SPEC;
impl crate::RegisterSpec for PDRUNCFG1_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pdruncfg1_set::W](W) writer structure"]
impl crate::Writable for PDRUNCFG1_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDRUNCFG1_SET to value 0"]
impl crate::Resettable for PDRUNCFG1_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
