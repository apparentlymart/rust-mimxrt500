#[doc = "Register `PDRUNCFG0_CLR` writer"]
pub struct W(crate::W<PDRUNCFG0_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDRUNCFG0_CLR_SPEC>;
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
impl From<crate::W<PDRUNCFG0_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDRUNCFG0_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Main clock shut off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAINCLK_SHUTOFF_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ENABLE = 1,
}
impl From<MAINCLK_SHUTOFF_AW> for bool {
    #[inline(always)]
    fn from(variant: MAINCLK_SHUTOFF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAINCLK_SHUTOFF` writer - Main clock shut off"]
pub type MAINCLK_SHUTOFF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, MAINCLK_SHUTOFF_AW, O>;
impl<'a, const O: u8> MAINCLK_SHUTOFF_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MAINCLK_SHUTOFF_AW::DISABLE)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MAINCLK_SHUTOFF_AW::ENABLE)
    }
}
#[doc = "PMIC_MODE0 device pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMIC_MODE0_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ENABLE = 1,
}
impl From<PMIC_MODE0_AW> for bool {
    #[inline(always)]
    fn from(variant: PMIC_MODE0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMIC_MODE0` writer - PMIC_MODE0 device pin"]
pub type PMIC_MODE0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, PMIC_MODE0_AW, O>;
impl<'a, const O: u8> PMIC_MODE0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PMIC_MODE0_AW::DISABLE)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PMIC_MODE0_AW::ENABLE)
    }
}
#[doc = "PMIC_MODE1 device pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMIC_MODE1_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ENABLE = 1,
}
impl From<PMIC_MODE1_AW> for bool {
    #[inline(always)]
    fn from(variant: PMIC_MODE1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMIC_MODE1` writer - PMIC_MODE1 device pin"]
pub type PMIC_MODE1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, PMIC_MODE1_AW, O>;
impl<'a, const O: u8> PMIC_MODE1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PMIC_MODE1_AW::DISABLE)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PMIC_MODE1_AW::ENABLE)
    }
}
#[doc = "Deep power-down mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEEP_PD_AW {
    #[doc = "0: VDDCORE supply remains on during WFI (deep_sleep mode)"]
    DEEP_PD_0 = 0,
    #[doc = "1: VDDCORE powered-off during WFI (deep_powerdown mode)"]
    DEEP_PD_1 = 1,
}
impl From<DEEP_PD_AW> for bool {
    #[inline(always)]
    fn from(variant: DEEP_PD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEEP_PD` writer - Deep power-down mode"]
pub type DEEP_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, DEEP_PD_AW, O>;
impl<'a, const O: u8> DEEP_PD_W<'a, O> {
    #[doc = "VDDCORE supply remains on during WFI (deep_sleep mode)"]
    #[inline(always)]
    pub fn deep_pd_0(self) -> &'a mut W {
        self.variant(DEEP_PD_AW::DEEP_PD_0)
    }
    #[doc = "VDDCORE powered-off during WFI (deep_powerdown mode)"]
    #[inline(always)]
    pub fn deep_pd_1(self) -> &'a mut W {
        self.variant(DEEP_PD_AW::DEEP_PD_1)
    }
}
#[doc = "Vddcore regulator mode when using on-chip regulator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDDCOREREG_LP_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ENABLE = 1,
}
impl From<VDDCOREREG_LP_AW> for bool {
    #[inline(always)]
    fn from(variant: VDDCOREREG_LP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDCOREREG_LP` writer - Vddcore regulator mode when using on-chip regulator"]
pub type VDDCOREREG_LP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, VDDCOREREG_LP_AW, O>;
impl<'a, const O: u8> VDDCOREREG_LP_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VDDCOREREG_LP_AW::DISABLE)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VDDCOREREG_LP_AW::ENABLE)
    }
}
#[doc = "192/96 FRO Clock Gate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRO_CG_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ENABLE = 1,
}
impl From<FRO_CG_AW> for bool {
    #[inline(always)]
    fn from(variant: FRO_CG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRO_CG` writer - 192/96 FRO Clock Gate"]
pub type FRO_CG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, FRO_CG_AW, O>;
impl<'a, const O: u8> FRO_CG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FRO_CG_AW::DISABLE)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FRO_CG_AW::ENABLE)
    }
}
#[doc = "Internal PMC references LP mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMCREF_LP_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ENABLE = 1,
}
impl From<PMCREF_LP_AW> for bool {
    #[inline(always)]
    fn from(variant: PMCREF_LP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMCREF_LP` writer - Internal PMC references LP mode"]
pub type PMCREF_LP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, PMCREF_LP_AW, O>;
impl<'a, const O: u8> PMCREF_LP_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PMCREF_LP_AW::DISABLE)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PMCREF_LP_AW::ENABLE)
    }
}
#[doc = "HVD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HVD1V8_PD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ENABLE = 1,
}
impl From<HVD1V8_PD_AW> for bool {
    #[inline(always)]
    fn from(variant: HVD1V8_PD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HVD1V8_PD` writer - HVD"]
pub type HVD1V8_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, HVD1V8_PD_AW, O>;
impl<'a, const O: u8> HVD1V8_PD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HVD1V8_PD_AW::DISABLE)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HVD1V8_PD_AW::ENABLE)
    }
}
#[doc = "LVD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORCORE_LP_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ENABLE = 1,
}
impl From<PORCORE_LP_AW> for bool {
    #[inline(always)]
    fn from(variant: PORCORE_LP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORCORE_LP` writer - LVD"]
pub type PORCORE_LP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, PORCORE_LP_AW, O>;
impl<'a, const O: u8> PORCORE_LP_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PORCORE_LP_AW::DISABLE)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PORCORE_LP_AW::ENABLE)
    }
}
#[doc = "LVD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVDCORE_LP_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ENABLE = 1,
}
impl From<LVDCORE_LP_AW> for bool {
    #[inline(always)]
    fn from(variant: LVDCORE_LP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVDCORE_LP` writer - LVD"]
pub type LVDCORE_LP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, LVDCORE_LP_AW, O>;
impl<'a, const O: u8> LVDCORE_LP_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LVDCORE_LP_AW::DISABLE)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LVDCORE_LP_AW::ENABLE)
    }
}
#[doc = "HVD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HVDCORE_PD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ENABLE = 1,
}
impl From<HVDCORE_PD_AW> for bool {
    #[inline(always)]
    fn from(variant: HVDCORE_PD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HVDCORE_PD` writer - HVD"]
pub type HVDCORE_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, HVDCORE_PD_AW, O>;
impl<'a, const O: u8> HVDCORE_PD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HVDCORE_PD_AW::DISABLE)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HVDCORE_PD_AW::ENABLE)
    }
}
#[doc = "Reverse body-bias\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBB_PD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ENABLE = 1,
}
impl From<RBB_PD_AW> for bool {
    #[inline(always)]
    fn from(variant: RBB_PD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBB_PD` writer - Reverse body-bias"]
pub type RBB_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, RBB_PD_AW, O>;
impl<'a, const O: u8> RBB_PD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RBB_PD_AW::DISABLE)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RBB_PD_AW::ENABLE)
    }
}
#[doc = "Forward body-bias\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FBB_PD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ENABLE = 1,
}
impl From<FBB_PD_AW> for bool {
    #[inline(always)]
    fn from(variant: FBB_PD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FBB_PD` writer - Forward body-bias"]
pub type FBB_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, FBB_PD_AW, O>;
impl<'a, const O: u8> FBB_PD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FBB_PD_AW::DISABLE)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FBB_PD_AW::ENABLE)
    }
}
#[doc = "Main crystal oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSXTAL_PD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ENABLE = 1,
}
impl From<SYSXTAL_PD_AW> for bool {
    #[inline(always)]
    fn from(variant: SYSXTAL_PD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSXTAL_PD` writer - Main crystal oscillator"]
pub type SYSXTAL_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, SYSXTAL_PD_AW, O>;
impl<'a, const O: u8> SYSXTAL_PD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SYSXTAL_PD_AW::DISABLE)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SYSXTAL_PD_AW::ENABLE)
    }
}
#[doc = "1 MHz Low-Power oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPOSC_PD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ENABLE = 1,
}
impl From<LPOSC_PD_AW> for bool {
    #[inline(always)]
    fn from(variant: LPOSC_PD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPOSC_PD` writer - 1 MHz Low-Power oscillator"]
pub type LPOSC_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, LPOSC_PD_AW, O>;
impl<'a, const O: u8> LPOSC_PD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LPOSC_PD_AW::DISABLE)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LPOSC_PD_AW::ENABLE)
    }
}
#[doc = "Reverse body-bias SRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBBSRAM_PD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ENABLE = 1,
}
impl From<RBBSRAM_PD_AW> for bool {
    #[inline(always)]
    fn from(variant: RBBSRAM_PD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBBSRAM_PD` writer - Reverse body-bias SRAM"]
pub type RBBSRAM_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, RBBSRAM_PD_AW, O>;
impl<'a, const O: u8> RBBSRAM_PD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RBBSRAM_PD_AW::DISABLE)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RBBSRAM_PD_AW::ENABLE)
    }
}
#[doc = "FRO 16 MHz internal oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FFRO_PD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ENABLE = 1,
}
impl From<FFRO_PD_AW> for bool {
    #[inline(always)]
    fn from(variant: FFRO_PD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFRO_PD` writer - FRO 16 MHz internal oscillator"]
pub type FFRO_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, FFRO_PD_AW, O>;
impl<'a, const O: u8> FFRO_PD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FFRO_PD_AW::DISABLE)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FFRO_PD_AW::ENABLE)
    }
}
#[doc = "System PLL internal regulator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSPLLLDO_PD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ENABLE = 1,
}
impl From<SYSPLLLDO_PD_AW> for bool {
    #[inline(always)]
    fn from(variant: SYSPLLLDO_PD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSPLLLDO_PD` writer - System PLL internal regulator"]
pub type SYSPLLLDO_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, SYSPLLLDO_PD_AW, O>;
impl<'a, const O: u8> SYSPLLLDO_PD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SYSPLLLDO_PD_AW::DISABLE)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SYSPLLLDO_PD_AW::ENABLE)
    }
}
#[doc = "System PLL analog functions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSPLLANA_PD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ENABLE = 1,
}
impl From<SYSPLLANA_PD_AW> for bool {
    #[inline(always)]
    fn from(variant: SYSPLLANA_PD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSPLLANA_PD` writer - System PLL analog functions"]
pub type SYSPLLANA_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, SYSPLLANA_PD_AW, O>;
impl<'a, const O: u8> SYSPLLANA_PD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SYSPLLANA_PD_AW::DISABLE)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SYSPLLANA_PD_AW::ENABLE)
    }
}
#[doc = "Audio PLL internal regulator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUDPLLLDO_PD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ENABLE = 1,
}
impl From<AUDPLLLDO_PD_AW> for bool {
    #[inline(always)]
    fn from(variant: AUDPLLLDO_PD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUDPLLLDO_PD` writer - Audio PLL internal regulator"]
pub type AUDPLLLDO_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, AUDPLLLDO_PD_AW, O>;
impl<'a, const O: u8> AUDPLLLDO_PD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AUDPLLLDO_PD_AW::DISABLE)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AUDPLLLDO_PD_AW::ENABLE)
    }
}
#[doc = "Audio PLL analog functions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUDPLLANA_PD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ENABLE = 1,
}
impl From<AUDPLLANA_PD_AW> for bool {
    #[inline(always)]
    fn from(variant: AUDPLLANA_PD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUDPLLANA_PD` writer - Audio PLL analog functions"]
pub type AUDPLLANA_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, AUDPLLANA_PD_AW, O>;
impl<'a, const O: u8> AUDPLLANA_PD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AUDPLLANA_PD_AW::DISABLE)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AUDPLLANA_PD_AW::ENABLE)
    }
}
#[doc = "ADC analog functions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_PD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ENABLE = 1,
}
impl From<ADC_PD_AW> for bool {
    #[inline(always)]
    fn from(variant: ADC_PD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_PD` writer - ADC analog functions"]
pub type ADC_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, ADC_PD_AW, O>;
impl<'a, const O: u8> ADC_PD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC_PD_AW::DISABLE)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC_PD_AW::ENABLE)
    }
}
#[doc = "ADC low power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_LP_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ENABLE = 1,
}
impl From<ADC_LP_AW> for bool {
    #[inline(always)]
    fn from(variant: ADC_LP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_LP` writer - ADC low power mode"]
pub type ADC_LP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, ADC_LP_AW, O>;
impl<'a, const O: u8> ADC_LP_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC_LP_AW::DISABLE)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC_LP_AW::ENABLE)
    }
}
#[doc = "ADC temperature sensor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_TEMPSNS_PD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ENABLE = 1,
}
impl From<ADC_TEMPSNS_PD_AW> for bool {
    #[inline(always)]
    fn from(variant: ADC_TEMPSNS_PD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_TEMPSNS_PD` writer - ADC temperature sensor"]
pub type ADC_TEMPSNS_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, ADC_TEMPSNS_PD_AW, O>;
impl<'a, const O: u8> ADC_TEMPSNS_PD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC_TEMPSNS_PD_AW::DISABLE)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC_TEMPSNS_PD_AW::ENABLE)
    }
}
#[doc = "PMC temperature sensor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMC_TEMPSNS_PD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ENABLE = 1,
}
impl From<PMC_TEMPSNS_PD_AW> for bool {
    #[inline(always)]
    fn from(variant: PMC_TEMPSNS_PD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMC_TEMPSNS_PD` writer - PMC temperature sensor"]
pub type PMC_TEMPSNS_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, PMC_TEMPSNS_PD_AW, O>;
impl<'a, const O: u8> PMC_TEMPSNS_PD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PMC_TEMPSNS_PD_AW::DISABLE)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PMC_TEMPSNS_PD_AW::ENABLE)
    }
}
#[doc = "Analog comparator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMP_PD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the PDRUNCFG0 Bit"]
    ENABLE = 1,
}
impl From<ACMP_PD_AW> for bool {
    #[inline(always)]
    fn from(variant: ACMP_PD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP_PD` writer - Analog comparator"]
pub type ACMP_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_CLR_SPEC, ACMP_PD_AW, O>;
impl<'a, const O: u8> ACMP_PD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ACMP_PD_AW::DISABLE)
    }
    #[doc = "Clears the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ACMP_PD_AW::ENABLE)
    }
}
impl W {
    #[doc = "Bit 0 - Main clock shut off"]
    #[inline(always)]
    #[must_use]
    pub fn mainclk_shutoff(&mut self) -> MAINCLK_SHUTOFF_W<0> {
        MAINCLK_SHUTOFF_W::new(self)
    }
    #[doc = "Bit 1 - PMIC_MODE0 device pin"]
    #[inline(always)]
    #[must_use]
    pub fn pmic_mode0(&mut self) -> PMIC_MODE0_W<1> {
        PMIC_MODE0_W::new(self)
    }
    #[doc = "Bit 2 - PMIC_MODE1 device pin"]
    #[inline(always)]
    #[must_use]
    pub fn pmic_mode1(&mut self) -> PMIC_MODE1_W<2> {
        PMIC_MODE1_W::new(self)
    }
    #[doc = "Bit 3 - Deep power-down mode"]
    #[inline(always)]
    #[must_use]
    pub fn deep_pd(&mut self) -> DEEP_PD_W<3> {
        DEEP_PD_W::new(self)
    }
    #[doc = "Bit 4 - Vddcore regulator mode when using on-chip regulator"]
    #[inline(always)]
    #[must_use]
    pub fn vddcorereg_lp(&mut self) -> VDDCOREREG_LP_W<4> {
        VDDCOREREG_LP_W::new(self)
    }
    #[doc = "Bit 5 - 192/96 FRO Clock Gate"]
    #[inline(always)]
    #[must_use]
    pub fn fro_cg(&mut self) -> FRO_CG_W<5> {
        FRO_CG_W::new(self)
    }
    #[doc = "Bit 6 - Internal PMC references LP mode"]
    #[inline(always)]
    #[must_use]
    pub fn pmcref_lp(&mut self) -> PMCREF_LP_W<6> {
        PMCREF_LP_W::new(self)
    }
    #[doc = "Bit 7 - HVD"]
    #[inline(always)]
    #[must_use]
    pub fn hvd1v8_pd(&mut self) -> HVD1V8_PD_W<7> {
        HVD1V8_PD_W::new(self)
    }
    #[doc = "Bit 8 - LVD"]
    #[inline(always)]
    #[must_use]
    pub fn porcore_lp(&mut self) -> PORCORE_LP_W<8> {
        PORCORE_LP_W::new(self)
    }
    #[doc = "Bit 9 - LVD"]
    #[inline(always)]
    #[must_use]
    pub fn lvdcore_lp(&mut self) -> LVDCORE_LP_W<9> {
        LVDCORE_LP_W::new(self)
    }
    #[doc = "Bit 10 - HVD"]
    #[inline(always)]
    #[must_use]
    pub fn hvdcore_pd(&mut self) -> HVDCORE_PD_W<10> {
        HVDCORE_PD_W::new(self)
    }
    #[doc = "Bit 11 - Reverse body-bias"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_pd(&mut self) -> RBB_PD_W<11> {
        RBB_PD_W::new(self)
    }
    #[doc = "Bit 12 - Forward body-bias"]
    #[inline(always)]
    #[must_use]
    pub fn fbb_pd(&mut self) -> FBB_PD_W<12> {
        FBB_PD_W::new(self)
    }
    #[doc = "Bit 13 - Main crystal oscillator"]
    #[inline(always)]
    #[must_use]
    pub fn sysxtal_pd(&mut self) -> SYSXTAL_PD_W<13> {
        SYSXTAL_PD_W::new(self)
    }
    #[doc = "Bit 14 - 1 MHz Low-Power oscillator"]
    #[inline(always)]
    #[must_use]
    pub fn lposc_pd(&mut self) -> LPOSC_PD_W<14> {
        LPOSC_PD_W::new(self)
    }
    #[doc = "Bit 15 - Reverse body-bias SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn rbbsram_pd(&mut self) -> RBBSRAM_PD_W<15> {
        RBBSRAM_PD_W::new(self)
    }
    #[doc = "Bit 16 - FRO 16 MHz internal oscillator"]
    #[inline(always)]
    #[must_use]
    pub fn ffro_pd(&mut self) -> FFRO_PD_W<16> {
        FFRO_PD_W::new(self)
    }
    #[doc = "Bit 17 - System PLL internal regulator"]
    #[inline(always)]
    #[must_use]
    pub fn syspllldo_pd(&mut self) -> SYSPLLLDO_PD_W<17> {
        SYSPLLLDO_PD_W::new(self)
    }
    #[doc = "Bit 18 - System PLL analog functions"]
    #[inline(always)]
    #[must_use]
    pub fn syspllana_pd(&mut self) -> SYSPLLANA_PD_W<18> {
        SYSPLLANA_PD_W::new(self)
    }
    #[doc = "Bit 19 - Audio PLL internal regulator"]
    #[inline(always)]
    #[must_use]
    pub fn audpllldo_pd(&mut self) -> AUDPLLLDO_PD_W<19> {
        AUDPLLLDO_PD_W::new(self)
    }
    #[doc = "Bit 20 - Audio PLL analog functions"]
    #[inline(always)]
    #[must_use]
    pub fn audpllana_pd(&mut self) -> AUDPLLANA_PD_W<20> {
        AUDPLLANA_PD_W::new(self)
    }
    #[doc = "Bit 21 - ADC analog functions"]
    #[inline(always)]
    #[must_use]
    pub fn adc_pd(&mut self) -> ADC_PD_W<21> {
        ADC_PD_W::new(self)
    }
    #[doc = "Bit 22 - ADC low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn adc_lp(&mut self) -> ADC_LP_W<22> {
        ADC_LP_W::new(self)
    }
    #[doc = "Bit 23 - ADC temperature sensor"]
    #[inline(always)]
    #[must_use]
    pub fn adc_tempsns_pd(&mut self) -> ADC_TEMPSNS_PD_W<23> {
        ADC_TEMPSNS_PD_W::new(self)
    }
    #[doc = "Bit 24 - PMC temperature sensor"]
    #[inline(always)]
    #[must_use]
    pub fn pmc_tempsns_pd(&mut self) -> PMC_TEMPSNS_PD_W<24> {
        PMC_TEMPSNS_PD_W::new(self)
    }
    #[doc = "Bit 25 - Analog comparator"]
    #[inline(always)]
    #[must_use]
    pub fn acmp_pd(&mut self) -> ACMP_PD_W<25> {
        ACMP_PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Run configuration 0 clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdruncfg0_clr](index.html) module"]
pub struct PDRUNCFG0_CLR_SPEC;
impl crate::RegisterSpec for PDRUNCFG0_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pdruncfg0_clr::W](W) writer structure"]
impl crate::Writable for PDRUNCFG0_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDRUNCFG0_CLR to value 0"]
impl crate::Resettable for PDRUNCFG0_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
