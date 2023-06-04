#[doc = "Register `PDRUNCFG0` reader"]
pub struct R(crate::R<PDRUNCFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDRUNCFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDRUNCFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDRUNCFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDRUNCFG0` writer"]
pub struct W(crate::W<PDRUNCFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDRUNCFG0_SPEC>;
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
impl From<crate::W<PDRUNCFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDRUNCFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAINCLK_SHUTOFF` reader - Main clock shut off"]
pub type MAINCLK_SHUTOFF_R = crate::BitReader<MAINCLK_SHUTOFF_A>;
#[doc = "Main clock shut off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAINCLK_SHUTOFF_A {
    #[doc = "0: Clocks enabled"]
    MAINCLK_SHUTOFF_0 = 0,
    #[doc = "1: Clocks gated"]
    MAINCLK_SHUTOFF_1 = 1,
}
impl From<MAINCLK_SHUTOFF_A> for bool {
    #[inline(always)]
    fn from(variant: MAINCLK_SHUTOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl MAINCLK_SHUTOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAINCLK_SHUTOFF_A {
        match self.bits {
            false => MAINCLK_SHUTOFF_A::MAINCLK_SHUTOFF_0,
            true => MAINCLK_SHUTOFF_A::MAINCLK_SHUTOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `MAINCLK_SHUTOFF_0`"]
    #[inline(always)]
    pub fn is_mainclk_shutoff_0(&self) -> bool {
        *self == MAINCLK_SHUTOFF_A::MAINCLK_SHUTOFF_0
    }
    #[doc = "Checks if the value of the field is `MAINCLK_SHUTOFF_1`"]
    #[inline(always)]
    pub fn is_mainclk_shutoff_1(&self) -> bool {
        *self == MAINCLK_SHUTOFF_A::MAINCLK_SHUTOFF_1
    }
}
#[doc = "Field `MAINCLK_SHUTOFF` writer - Main clock shut off"]
pub type MAINCLK_SHUTOFF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, MAINCLK_SHUTOFF_A, O>;
impl<'a, const O: u8> MAINCLK_SHUTOFF_W<'a, O> {
    #[doc = "Clocks enabled"]
    #[inline(always)]
    pub fn mainclk_shutoff_0(self) -> &'a mut W {
        self.variant(MAINCLK_SHUTOFF_A::MAINCLK_SHUTOFF_0)
    }
    #[doc = "Clocks gated"]
    #[inline(always)]
    pub fn mainclk_shutoff_1(self) -> &'a mut W {
        self.variant(MAINCLK_SHUTOFF_A::MAINCLK_SHUTOFF_1)
    }
}
#[doc = "Field `PMIC_MODE0` reader - PMIC_MODE0 device pin"]
pub type PMIC_MODE0_R = crate::BitReader<PMIC_MODE0_A>;
#[doc = "PMIC_MODE0 device pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMIC_MODE0_A {
    #[doc = "0: Set mode to 0"]
    PMIC_MODE0_0 = 0,
    #[doc = "1: Set mode to 1"]
    PMIC_MODE0_1 = 1,
}
impl From<PMIC_MODE0_A> for bool {
    #[inline(always)]
    fn from(variant: PMIC_MODE0_A) -> Self {
        variant as u8 != 0
    }
}
impl PMIC_MODE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMIC_MODE0_A {
        match self.bits {
            false => PMIC_MODE0_A::PMIC_MODE0_0,
            true => PMIC_MODE0_A::PMIC_MODE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMIC_MODE0_0`"]
    #[inline(always)]
    pub fn is_pmic_mode0_0(&self) -> bool {
        *self == PMIC_MODE0_A::PMIC_MODE0_0
    }
    #[doc = "Checks if the value of the field is `PMIC_MODE0_1`"]
    #[inline(always)]
    pub fn is_pmic_mode0_1(&self) -> bool {
        *self == PMIC_MODE0_A::PMIC_MODE0_1
    }
}
#[doc = "Field `PMIC_MODE0` writer - PMIC_MODE0 device pin"]
pub type PMIC_MODE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, PMIC_MODE0_A, O>;
impl<'a, const O: u8> PMIC_MODE0_W<'a, O> {
    #[doc = "Set mode to 0"]
    #[inline(always)]
    pub fn pmic_mode0_0(self) -> &'a mut W {
        self.variant(PMIC_MODE0_A::PMIC_MODE0_0)
    }
    #[doc = "Set mode to 1"]
    #[inline(always)]
    pub fn pmic_mode0_1(self) -> &'a mut W {
        self.variant(PMIC_MODE0_A::PMIC_MODE0_1)
    }
}
#[doc = "Field `PMIC_MODE1` reader - PMIC_MODE1 device pin"]
pub type PMIC_MODE1_R = crate::BitReader<PMIC_MODE1_A>;
#[doc = "PMIC_MODE1 device pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMIC_MODE1_A {
    #[doc = "0: Set mode to 0"]
    PMIC_MODE0_0 = 0,
    #[doc = "1: Set mode to 1"]
    PMIC_MODE0_1 = 1,
}
impl From<PMIC_MODE1_A> for bool {
    #[inline(always)]
    fn from(variant: PMIC_MODE1_A) -> Self {
        variant as u8 != 0
    }
}
impl PMIC_MODE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMIC_MODE1_A {
        match self.bits {
            false => PMIC_MODE1_A::PMIC_MODE0_0,
            true => PMIC_MODE1_A::PMIC_MODE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMIC_MODE0_0`"]
    #[inline(always)]
    pub fn is_pmic_mode0_0(&self) -> bool {
        *self == PMIC_MODE1_A::PMIC_MODE0_0
    }
    #[doc = "Checks if the value of the field is `PMIC_MODE0_1`"]
    #[inline(always)]
    pub fn is_pmic_mode0_1(&self) -> bool {
        *self == PMIC_MODE1_A::PMIC_MODE0_1
    }
}
#[doc = "Field `PMIC_MODE1` writer - PMIC_MODE1 device pin"]
pub type PMIC_MODE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, PMIC_MODE1_A, O>;
impl<'a, const O: u8> PMIC_MODE1_W<'a, O> {
    #[doc = "Set mode to 0"]
    #[inline(always)]
    pub fn pmic_mode0_0(self) -> &'a mut W {
        self.variant(PMIC_MODE1_A::PMIC_MODE0_0)
    }
    #[doc = "Set mode to 1"]
    #[inline(always)]
    pub fn pmic_mode0_1(self) -> &'a mut W {
        self.variant(PMIC_MODE1_A::PMIC_MODE0_1)
    }
}
#[doc = "Field `DEEP_PD` reader - Deep power-down mode"]
pub type DEEP_PD_R = crate::BitReader<DEEP_PD_A>;
#[doc = "Deep power-down mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEEP_PD_A {
    #[doc = "0: VDDCORE supply remains on during WFI (deep_sleep mode)"]
    DEEP_PD_0 = 0,
    #[doc = "1: VDDCORE powered-off during WFI (deep_powerdown mode)"]
    DEEP_PD_1 = 1,
}
impl From<DEEP_PD_A> for bool {
    #[inline(always)]
    fn from(variant: DEEP_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl DEEP_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEEP_PD_A {
        match self.bits {
            false => DEEP_PD_A::DEEP_PD_0,
            true => DEEP_PD_A::DEEP_PD_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEEP_PD_0`"]
    #[inline(always)]
    pub fn is_deep_pd_0(&self) -> bool {
        *self == DEEP_PD_A::DEEP_PD_0
    }
    #[doc = "Checks if the value of the field is `DEEP_PD_1`"]
    #[inline(always)]
    pub fn is_deep_pd_1(&self) -> bool {
        *self == DEEP_PD_A::DEEP_PD_1
    }
}
#[doc = "Field `DEEP_PD` writer - Deep power-down mode"]
pub type DEEP_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, DEEP_PD_A, O>;
impl<'a, const O: u8> DEEP_PD_W<'a, O> {
    #[doc = "VDDCORE supply remains on during WFI (deep_sleep mode)"]
    #[inline(always)]
    pub fn deep_pd_0(self) -> &'a mut W {
        self.variant(DEEP_PD_A::DEEP_PD_0)
    }
    #[doc = "VDDCORE powered-off during WFI (deep_powerdown mode)"]
    #[inline(always)]
    pub fn deep_pd_1(self) -> &'a mut W {
        self.variant(DEEP_PD_A::DEEP_PD_1)
    }
}
#[doc = "Field `VDDCOREREG_LP` reader - Vddcore regulator mode"]
pub type VDDCOREREG_LP_R = crate::BitReader<VDDCOREREG_LP_A>;
#[doc = "Vddcore regulator mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDDCOREREG_LP_A {
    #[doc = "0: HP mode"]
    VDDCOREREG_LP_0 = 0,
    #[doc = "1: LP mode"]
    VDDCOREREG_LP_1 = 1,
}
impl From<VDDCOREREG_LP_A> for bool {
    #[inline(always)]
    fn from(variant: VDDCOREREG_LP_A) -> Self {
        variant as u8 != 0
    }
}
impl VDDCOREREG_LP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDCOREREG_LP_A {
        match self.bits {
            false => VDDCOREREG_LP_A::VDDCOREREG_LP_0,
            true => VDDCOREREG_LP_A::VDDCOREREG_LP_1,
        }
    }
    #[doc = "Checks if the value of the field is `VDDCOREREG_LP_0`"]
    #[inline(always)]
    pub fn is_vddcorereg_lp_0(&self) -> bool {
        *self == VDDCOREREG_LP_A::VDDCOREREG_LP_0
    }
    #[doc = "Checks if the value of the field is `VDDCOREREG_LP_1`"]
    #[inline(always)]
    pub fn is_vddcorereg_lp_1(&self) -> bool {
        *self == VDDCOREREG_LP_A::VDDCOREREG_LP_1
    }
}
#[doc = "Field `VDDCOREREG_LP` writer - Vddcore regulator mode"]
pub type VDDCOREREG_LP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, VDDCOREREG_LP_A, O>;
impl<'a, const O: u8> VDDCOREREG_LP_W<'a, O> {
    #[doc = "HP mode"]
    #[inline(always)]
    pub fn vddcorereg_lp_0(self) -> &'a mut W {
        self.variant(VDDCOREREG_LP_A::VDDCOREREG_LP_0)
    }
    #[doc = "LP mode"]
    #[inline(always)]
    pub fn vddcorereg_lp_1(self) -> &'a mut W {
        self.variant(VDDCOREREG_LP_A::VDDCOREREG_LP_1)
    }
}
#[doc = "Field `FRO_CG` reader - 192/96 FRO Clock Gate"]
pub type FRO_CG_R = crate::BitReader<FRO_CG_A>;
#[doc = "192/96 FRO Clock Gate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRO_CG_A {
    #[doc = "0: FRO clock to the dividers is NOT gated"]
    FRO_CG_0 = 0,
    #[doc = "1: FRO clock to the dividers is gated off"]
    FRO_CG_1 = 1,
}
impl From<FRO_CG_A> for bool {
    #[inline(always)]
    fn from(variant: FRO_CG_A) -> Self {
        variant as u8 != 0
    }
}
impl FRO_CG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRO_CG_A {
        match self.bits {
            false => FRO_CG_A::FRO_CG_0,
            true => FRO_CG_A::FRO_CG_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRO_CG_0`"]
    #[inline(always)]
    pub fn is_fro_cg_0(&self) -> bool {
        *self == FRO_CG_A::FRO_CG_0
    }
    #[doc = "Checks if the value of the field is `FRO_CG_1`"]
    #[inline(always)]
    pub fn is_fro_cg_1(&self) -> bool {
        *self == FRO_CG_A::FRO_CG_1
    }
}
#[doc = "Field `FRO_CG` writer - 192/96 FRO Clock Gate"]
pub type FRO_CG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, FRO_CG_A, O>;
impl<'a, const O: u8> FRO_CG_W<'a, O> {
    #[doc = "FRO clock to the dividers is NOT gated"]
    #[inline(always)]
    pub fn fro_cg_0(self) -> &'a mut W {
        self.variant(FRO_CG_A::FRO_CG_0)
    }
    #[doc = "FRO clock to the dividers is gated off"]
    #[inline(always)]
    pub fn fro_cg_1(self) -> &'a mut W {
        self.variant(FRO_CG_A::FRO_CG_1)
    }
}
#[doc = "Field `PMCREF_LP` reader - Internal PMC references LP mode"]
pub type PMCREF_LP_R = crate::BitReader<PMCREF_LP_A>;
#[doc = "Internal PMC references LP mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMCREF_LP_A {
    #[doc = "0: HP mode"]
    PMCREF_LP_0 = 0,
    #[doc = "1: LP mode"]
    PMCREF_LP_1 = 1,
}
impl From<PMCREF_LP_A> for bool {
    #[inline(always)]
    fn from(variant: PMCREF_LP_A) -> Self {
        variant as u8 != 0
    }
}
impl PMCREF_LP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMCREF_LP_A {
        match self.bits {
            false => PMCREF_LP_A::PMCREF_LP_0,
            true => PMCREF_LP_A::PMCREF_LP_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMCREF_LP_0`"]
    #[inline(always)]
    pub fn is_pmcref_lp_0(&self) -> bool {
        *self == PMCREF_LP_A::PMCREF_LP_0
    }
    #[doc = "Checks if the value of the field is `PMCREF_LP_1`"]
    #[inline(always)]
    pub fn is_pmcref_lp_1(&self) -> bool {
        *self == PMCREF_LP_A::PMCREF_LP_1
    }
}
#[doc = "Field `PMCREF_LP` writer - Internal PMC references LP mode"]
pub type PMCREF_LP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, PMCREF_LP_A, O>;
impl<'a, const O: u8> PMCREF_LP_W<'a, O> {
    #[doc = "HP mode"]
    #[inline(always)]
    pub fn pmcref_lp_0(self) -> &'a mut W {
        self.variant(PMCREF_LP_A::PMCREF_LP_0)
    }
    #[doc = "LP mode"]
    #[inline(always)]
    pub fn pmcref_lp_1(self) -> &'a mut W {
        self.variant(PMCREF_LP_A::PMCREF_LP_1)
    }
}
#[doc = "Field `HVD1V8_PD` reader - HVD"]
pub type HVD1V8_PD_R = crate::BitReader<HVD1V8_PD_A>;
#[doc = "HVD\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HVD1V8_PD_A {
    #[doc = "0: Enabled"]
    HVD1V8_PD_0 = 0,
    #[doc = "1: Powerdown"]
    HVD1V8_PD_1 = 1,
}
impl From<HVD1V8_PD_A> for bool {
    #[inline(always)]
    fn from(variant: HVD1V8_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl HVD1V8_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HVD1V8_PD_A {
        match self.bits {
            false => HVD1V8_PD_A::HVD1V8_PD_0,
            true => HVD1V8_PD_A::HVD1V8_PD_1,
        }
    }
    #[doc = "Checks if the value of the field is `HVD1V8_PD_0`"]
    #[inline(always)]
    pub fn is_hvd1v8_pd_0(&self) -> bool {
        *self == HVD1V8_PD_A::HVD1V8_PD_0
    }
    #[doc = "Checks if the value of the field is `HVD1V8_PD_1`"]
    #[inline(always)]
    pub fn is_hvd1v8_pd_1(&self) -> bool {
        *self == HVD1V8_PD_A::HVD1V8_PD_1
    }
}
#[doc = "Field `HVD1V8_PD` writer - HVD"]
pub type HVD1V8_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, HVD1V8_PD_A, O>;
impl<'a, const O: u8> HVD1V8_PD_W<'a, O> {
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn hvd1v8_pd_0(self) -> &'a mut W {
        self.variant(HVD1V8_PD_A::HVD1V8_PD_0)
    }
    #[doc = "Powerdown"]
    #[inline(always)]
    pub fn hvd1v8_pd_1(self) -> &'a mut W {
        self.variant(HVD1V8_PD_A::HVD1V8_PD_1)
    }
}
#[doc = "Field `PORCORE_LP` reader - LVD"]
pub type PORCORE_LP_R = crate::BitReader<PORCORE_LP_A>;
#[doc = "LVD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORCORE_LP_A {
    #[doc = "0: HP mode"]
    PORCORE_LP_0 = 0,
    #[doc = "1: LP mode"]
    PORCORE_LP_1 = 1,
}
impl From<PORCORE_LP_A> for bool {
    #[inline(always)]
    fn from(variant: PORCORE_LP_A) -> Self {
        variant as u8 != 0
    }
}
impl PORCORE_LP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORCORE_LP_A {
        match self.bits {
            false => PORCORE_LP_A::PORCORE_LP_0,
            true => PORCORE_LP_A::PORCORE_LP_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORCORE_LP_0`"]
    #[inline(always)]
    pub fn is_porcore_lp_0(&self) -> bool {
        *self == PORCORE_LP_A::PORCORE_LP_0
    }
    #[doc = "Checks if the value of the field is `PORCORE_LP_1`"]
    #[inline(always)]
    pub fn is_porcore_lp_1(&self) -> bool {
        *self == PORCORE_LP_A::PORCORE_LP_1
    }
}
#[doc = "Field `PORCORE_LP` writer - LVD"]
pub type PORCORE_LP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, PORCORE_LP_A, O>;
impl<'a, const O: u8> PORCORE_LP_W<'a, O> {
    #[doc = "HP mode"]
    #[inline(always)]
    pub fn porcore_lp_0(self) -> &'a mut W {
        self.variant(PORCORE_LP_A::PORCORE_LP_0)
    }
    #[doc = "LP mode"]
    #[inline(always)]
    pub fn porcore_lp_1(self) -> &'a mut W {
        self.variant(PORCORE_LP_A::PORCORE_LP_1)
    }
}
#[doc = "Field `LVDCORE_LP` reader - LVD"]
pub type LVDCORE_LP_R = crate::BitReader<LVDCORE_LP_A>;
#[doc = "LVD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVDCORE_LP_A {
    #[doc = "0: HP mode"]
    LVDCORE_LP_0 = 0,
    #[doc = "1: LP mode"]
    LVDCORE_LP_1 = 1,
}
impl From<LVDCORE_LP_A> for bool {
    #[inline(always)]
    fn from(variant: LVDCORE_LP_A) -> Self {
        variant as u8 != 0
    }
}
impl LVDCORE_LP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDCORE_LP_A {
        match self.bits {
            false => LVDCORE_LP_A::LVDCORE_LP_0,
            true => LVDCORE_LP_A::LVDCORE_LP_1,
        }
    }
    #[doc = "Checks if the value of the field is `LVDCORE_LP_0`"]
    #[inline(always)]
    pub fn is_lvdcore_lp_0(&self) -> bool {
        *self == LVDCORE_LP_A::LVDCORE_LP_0
    }
    #[doc = "Checks if the value of the field is `LVDCORE_LP_1`"]
    #[inline(always)]
    pub fn is_lvdcore_lp_1(&self) -> bool {
        *self == LVDCORE_LP_A::LVDCORE_LP_1
    }
}
#[doc = "Field `LVDCORE_LP` writer - LVD"]
pub type LVDCORE_LP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, LVDCORE_LP_A, O>;
impl<'a, const O: u8> LVDCORE_LP_W<'a, O> {
    #[doc = "HP mode"]
    #[inline(always)]
    pub fn lvdcore_lp_0(self) -> &'a mut W {
        self.variant(LVDCORE_LP_A::LVDCORE_LP_0)
    }
    #[doc = "LP mode"]
    #[inline(always)]
    pub fn lvdcore_lp_1(self) -> &'a mut W {
        self.variant(LVDCORE_LP_A::LVDCORE_LP_1)
    }
}
#[doc = "Field `HVDCORE_PD` reader - HVD"]
pub type HVDCORE_PD_R = crate::BitReader<HVDCORE_PD_A>;
#[doc = "HVD\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HVDCORE_PD_A {
    #[doc = "0: Enabled"]
    HVDCORE_PD_0 = 0,
    #[doc = "1: Powerdown"]
    HVDCORE_PD_1 = 1,
}
impl From<HVDCORE_PD_A> for bool {
    #[inline(always)]
    fn from(variant: HVDCORE_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl HVDCORE_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HVDCORE_PD_A {
        match self.bits {
            false => HVDCORE_PD_A::HVDCORE_PD_0,
            true => HVDCORE_PD_A::HVDCORE_PD_1,
        }
    }
    #[doc = "Checks if the value of the field is `HVDCORE_PD_0`"]
    #[inline(always)]
    pub fn is_hvdcore_pd_0(&self) -> bool {
        *self == HVDCORE_PD_A::HVDCORE_PD_0
    }
    #[doc = "Checks if the value of the field is `HVDCORE_PD_1`"]
    #[inline(always)]
    pub fn is_hvdcore_pd_1(&self) -> bool {
        *self == HVDCORE_PD_A::HVDCORE_PD_1
    }
}
#[doc = "Field `HVDCORE_PD` writer - HVD"]
pub type HVDCORE_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, HVDCORE_PD_A, O>;
impl<'a, const O: u8> HVDCORE_PD_W<'a, O> {
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn hvdcore_pd_0(self) -> &'a mut W {
        self.variant(HVDCORE_PD_A::HVDCORE_PD_0)
    }
    #[doc = "Powerdown"]
    #[inline(always)]
    pub fn hvdcore_pd_1(self) -> &'a mut W {
        self.variant(HVDCORE_PD_A::HVDCORE_PD_1)
    }
}
#[doc = "Field `RBB_PD` reader - Reverse body-bias"]
pub type RBB_PD_R = crate::BitReader<RBB_PD_A>;
#[doc = "Reverse body-bias\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBB_PD_A {
    #[doc = "0: Enabled"]
    RBB_PD_0 = 0,
    #[doc = "1: Powerdown"]
    RBB_PD_1 = 1,
}
impl From<RBB_PD_A> for bool {
    #[inline(always)]
    fn from(variant: RBB_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl RBB_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBB_PD_A {
        match self.bits {
            false => RBB_PD_A::RBB_PD_0,
            true => RBB_PD_A::RBB_PD_1,
        }
    }
    #[doc = "Checks if the value of the field is `RBB_PD_0`"]
    #[inline(always)]
    pub fn is_rbb_pd_0(&self) -> bool {
        *self == RBB_PD_A::RBB_PD_0
    }
    #[doc = "Checks if the value of the field is `RBB_PD_1`"]
    #[inline(always)]
    pub fn is_rbb_pd_1(&self) -> bool {
        *self == RBB_PD_A::RBB_PD_1
    }
}
#[doc = "Field `RBB_PD` writer - Reverse body-bias"]
pub type RBB_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, RBB_PD_A, O>;
impl<'a, const O: u8> RBB_PD_W<'a, O> {
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn rbb_pd_0(self) -> &'a mut W {
        self.variant(RBB_PD_A::RBB_PD_0)
    }
    #[doc = "Powerdown"]
    #[inline(always)]
    pub fn rbb_pd_1(self) -> &'a mut W {
        self.variant(RBB_PD_A::RBB_PD_1)
    }
}
#[doc = "Field `FBB_PD` reader - Forward body-bias"]
pub type FBB_PD_R = crate::BitReader<FBB_PD_A>;
#[doc = "Forward body-bias\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FBB_PD_A {
    #[doc = "0: Enabled"]
    FBB_PD_0 = 0,
    #[doc = "1: Powerdown"]
    FBB_PD_1 = 1,
}
impl From<FBB_PD_A> for bool {
    #[inline(always)]
    fn from(variant: FBB_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl FBB_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FBB_PD_A {
        match self.bits {
            false => FBB_PD_A::FBB_PD_0,
            true => FBB_PD_A::FBB_PD_1,
        }
    }
    #[doc = "Checks if the value of the field is `FBB_PD_0`"]
    #[inline(always)]
    pub fn is_fbb_pd_0(&self) -> bool {
        *self == FBB_PD_A::FBB_PD_0
    }
    #[doc = "Checks if the value of the field is `FBB_PD_1`"]
    #[inline(always)]
    pub fn is_fbb_pd_1(&self) -> bool {
        *self == FBB_PD_A::FBB_PD_1
    }
}
#[doc = "Field `FBB_PD` writer - Forward body-bias"]
pub type FBB_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, FBB_PD_A, O>;
impl<'a, const O: u8> FBB_PD_W<'a, O> {
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn fbb_pd_0(self) -> &'a mut W {
        self.variant(FBB_PD_A::FBB_PD_0)
    }
    #[doc = "Powerdown"]
    #[inline(always)]
    pub fn fbb_pd_1(self) -> &'a mut W {
        self.variant(FBB_PD_A::FBB_PD_1)
    }
}
#[doc = "Field `SYSXTAL_PD` reader - Main crystal oscillator"]
pub type SYSXTAL_PD_R = crate::BitReader<SYSXTAL_PD_A>;
#[doc = "Main crystal oscillator\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSXTAL_PD_A {
    #[doc = "0: Enabled"]
    SYSXTAL_PD_0 = 0,
    #[doc = "1: Powerdown"]
    SYSXTAL_PD_1 = 1,
}
impl From<SYSXTAL_PD_A> for bool {
    #[inline(always)]
    fn from(variant: SYSXTAL_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSXTAL_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSXTAL_PD_A {
        match self.bits {
            false => SYSXTAL_PD_A::SYSXTAL_PD_0,
            true => SYSXTAL_PD_A::SYSXTAL_PD_1,
        }
    }
    #[doc = "Checks if the value of the field is `SYSXTAL_PD_0`"]
    #[inline(always)]
    pub fn is_sysxtal_pd_0(&self) -> bool {
        *self == SYSXTAL_PD_A::SYSXTAL_PD_0
    }
    #[doc = "Checks if the value of the field is `SYSXTAL_PD_1`"]
    #[inline(always)]
    pub fn is_sysxtal_pd_1(&self) -> bool {
        *self == SYSXTAL_PD_A::SYSXTAL_PD_1
    }
}
#[doc = "Field `SYSXTAL_PD` writer - Main crystal oscillator"]
pub type SYSXTAL_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, SYSXTAL_PD_A, O>;
impl<'a, const O: u8> SYSXTAL_PD_W<'a, O> {
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn sysxtal_pd_0(self) -> &'a mut W {
        self.variant(SYSXTAL_PD_A::SYSXTAL_PD_0)
    }
    #[doc = "Powerdown"]
    #[inline(always)]
    pub fn sysxtal_pd_1(self) -> &'a mut W {
        self.variant(SYSXTAL_PD_A::SYSXTAL_PD_1)
    }
}
#[doc = "Field `LPOSC_PD` reader - 1 MHz Low-Power oscillator"]
pub type LPOSC_PD_R = crate::BitReader<LPOSC_PD_A>;
#[doc = "1 MHz Low-Power oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPOSC_PD_A {
    #[doc = "0: Enabled"]
    LPOSC_PD_0 = 0,
    #[doc = "1: Powerdown"]
    LPOSC_PD_1 = 1,
}
impl From<LPOSC_PD_A> for bool {
    #[inline(always)]
    fn from(variant: LPOSC_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl LPOSC_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPOSC_PD_A {
        match self.bits {
            false => LPOSC_PD_A::LPOSC_PD_0,
            true => LPOSC_PD_A::LPOSC_PD_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPOSC_PD_0`"]
    #[inline(always)]
    pub fn is_lposc_pd_0(&self) -> bool {
        *self == LPOSC_PD_A::LPOSC_PD_0
    }
    #[doc = "Checks if the value of the field is `LPOSC_PD_1`"]
    #[inline(always)]
    pub fn is_lposc_pd_1(&self) -> bool {
        *self == LPOSC_PD_A::LPOSC_PD_1
    }
}
#[doc = "Field `LPOSC_PD` writer - 1 MHz Low-Power oscillator"]
pub type LPOSC_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, LPOSC_PD_A, O>;
impl<'a, const O: u8> LPOSC_PD_W<'a, O> {
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn lposc_pd_0(self) -> &'a mut W {
        self.variant(LPOSC_PD_A::LPOSC_PD_0)
    }
    #[doc = "Powerdown"]
    #[inline(always)]
    pub fn lposc_pd_1(self) -> &'a mut W {
        self.variant(LPOSC_PD_A::LPOSC_PD_1)
    }
}
#[doc = "Field `RBBSRAM_PD` reader - Reverse body-bias SRAM"]
pub type RBBSRAM_PD_R = crate::BitReader<RBBSRAM_PD_A>;
#[doc = "Reverse body-bias SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBBSRAM_PD_A {
    #[doc = "0: Enables SRAM Reverse Body Bias"]
    RBBSRAM_PD_0 = 0,
    #[doc = "1: Disables SRAM Reverse Body Bias"]
    RBBSRAM_PD_1 = 1,
}
impl From<RBBSRAM_PD_A> for bool {
    #[inline(always)]
    fn from(variant: RBBSRAM_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl RBBSRAM_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBBSRAM_PD_A {
        match self.bits {
            false => RBBSRAM_PD_A::RBBSRAM_PD_0,
            true => RBBSRAM_PD_A::RBBSRAM_PD_1,
        }
    }
    #[doc = "Checks if the value of the field is `RBBSRAM_PD_0`"]
    #[inline(always)]
    pub fn is_rbbsram_pd_0(&self) -> bool {
        *self == RBBSRAM_PD_A::RBBSRAM_PD_0
    }
    #[doc = "Checks if the value of the field is `RBBSRAM_PD_1`"]
    #[inline(always)]
    pub fn is_rbbsram_pd_1(&self) -> bool {
        *self == RBBSRAM_PD_A::RBBSRAM_PD_1
    }
}
#[doc = "Field `RBBSRAM_PD` writer - Reverse body-bias SRAM"]
pub type RBBSRAM_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, RBBSRAM_PD_A, O>;
impl<'a, const O: u8> RBBSRAM_PD_W<'a, O> {
    #[doc = "Enables SRAM Reverse Body Bias"]
    #[inline(always)]
    pub fn rbbsram_pd_0(self) -> &'a mut W {
        self.variant(RBBSRAM_PD_A::RBBSRAM_PD_0)
    }
    #[doc = "Disables SRAM Reverse Body Bias"]
    #[inline(always)]
    pub fn rbbsram_pd_1(self) -> &'a mut W {
        self.variant(RBBSRAM_PD_A::RBBSRAM_PD_1)
    }
}
#[doc = "Field `FFRO_PD` reader - FFRO 192/96 MHz internal oscillator"]
pub type FFRO_PD_R = crate::BitReader<FFRO_PD_A>;
#[doc = "FFRO 192/96 MHz internal oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FFRO_PD_A {
    #[doc = "0: Enabled"]
    FFRO_PD_0 = 0,
    #[doc = "1: Powerdown"]
    FFRO_PD_1 = 1,
}
impl From<FFRO_PD_A> for bool {
    #[inline(always)]
    fn from(variant: FFRO_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl FFRO_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFRO_PD_A {
        match self.bits {
            false => FFRO_PD_A::FFRO_PD_0,
            true => FFRO_PD_A::FFRO_PD_1,
        }
    }
    #[doc = "Checks if the value of the field is `FFRO_PD_0`"]
    #[inline(always)]
    pub fn is_ffro_pd_0(&self) -> bool {
        *self == FFRO_PD_A::FFRO_PD_0
    }
    #[doc = "Checks if the value of the field is `FFRO_PD_1`"]
    #[inline(always)]
    pub fn is_ffro_pd_1(&self) -> bool {
        *self == FFRO_PD_A::FFRO_PD_1
    }
}
#[doc = "Field `FFRO_PD` writer - FFRO 192/96 MHz internal oscillator"]
pub type FFRO_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, FFRO_PD_A, O>;
impl<'a, const O: u8> FFRO_PD_W<'a, O> {
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ffro_pd_0(self) -> &'a mut W {
        self.variant(FFRO_PD_A::FFRO_PD_0)
    }
    #[doc = "Powerdown"]
    #[inline(always)]
    pub fn ffro_pd_1(self) -> &'a mut W {
        self.variant(FFRO_PD_A::FFRO_PD_1)
    }
}
#[doc = "Field `SYSPLLLDO_PD` reader - System PLL internal regulator"]
pub type SYSPLLLDO_PD_R = crate::BitReader<SYSPLLLDO_PD_A>;
#[doc = "System PLL internal regulator\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSPLLLDO_PD_A {
    #[doc = "0: Enabled"]
    SYSPLLLDO_PD_0 = 0,
    #[doc = "1: Powerdown"]
    SYSPLLLDO_PD_1 = 1,
}
impl From<SYSPLLLDO_PD_A> for bool {
    #[inline(always)]
    fn from(variant: SYSPLLLDO_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSPLLLDO_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSPLLLDO_PD_A {
        match self.bits {
            false => SYSPLLLDO_PD_A::SYSPLLLDO_PD_0,
            true => SYSPLLLDO_PD_A::SYSPLLLDO_PD_1,
        }
    }
    #[doc = "Checks if the value of the field is `SYSPLLLDO_PD_0`"]
    #[inline(always)]
    pub fn is_syspllldo_pd_0(&self) -> bool {
        *self == SYSPLLLDO_PD_A::SYSPLLLDO_PD_0
    }
    #[doc = "Checks if the value of the field is `SYSPLLLDO_PD_1`"]
    #[inline(always)]
    pub fn is_syspllldo_pd_1(&self) -> bool {
        *self == SYSPLLLDO_PD_A::SYSPLLLDO_PD_1
    }
}
#[doc = "Field `SYSPLLLDO_PD` writer - System PLL internal regulator"]
pub type SYSPLLLDO_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, SYSPLLLDO_PD_A, O>;
impl<'a, const O: u8> SYSPLLLDO_PD_W<'a, O> {
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn syspllldo_pd_0(self) -> &'a mut W {
        self.variant(SYSPLLLDO_PD_A::SYSPLLLDO_PD_0)
    }
    #[doc = "Powerdown"]
    #[inline(always)]
    pub fn syspllldo_pd_1(self) -> &'a mut W {
        self.variant(SYSPLLLDO_PD_A::SYSPLLLDO_PD_1)
    }
}
#[doc = "Field `SYSPLLANA_PD` reader - System PLL analog functions"]
pub type SYSPLLANA_PD_R = crate::BitReader<SYSPLLANA_PD_A>;
#[doc = "System PLL analog functions\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSPLLANA_PD_A {
    #[doc = "0: Enabled"]
    SYSPLLANA_PD_0 = 0,
    #[doc = "1: Powerdown"]
    SYSPLLANA_PD_1 = 1,
}
impl From<SYSPLLANA_PD_A> for bool {
    #[inline(always)]
    fn from(variant: SYSPLLANA_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSPLLANA_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSPLLANA_PD_A {
        match self.bits {
            false => SYSPLLANA_PD_A::SYSPLLANA_PD_0,
            true => SYSPLLANA_PD_A::SYSPLLANA_PD_1,
        }
    }
    #[doc = "Checks if the value of the field is `SYSPLLANA_PD_0`"]
    #[inline(always)]
    pub fn is_syspllana_pd_0(&self) -> bool {
        *self == SYSPLLANA_PD_A::SYSPLLANA_PD_0
    }
    #[doc = "Checks if the value of the field is `SYSPLLANA_PD_1`"]
    #[inline(always)]
    pub fn is_syspllana_pd_1(&self) -> bool {
        *self == SYSPLLANA_PD_A::SYSPLLANA_PD_1
    }
}
#[doc = "Field `SYSPLLANA_PD` writer - System PLL analog functions"]
pub type SYSPLLANA_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, SYSPLLANA_PD_A, O>;
impl<'a, const O: u8> SYSPLLANA_PD_W<'a, O> {
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn syspllana_pd_0(self) -> &'a mut W {
        self.variant(SYSPLLANA_PD_A::SYSPLLANA_PD_0)
    }
    #[doc = "Powerdown"]
    #[inline(always)]
    pub fn syspllana_pd_1(self) -> &'a mut W {
        self.variant(SYSPLLANA_PD_A::SYSPLLANA_PD_1)
    }
}
#[doc = "Field `AUDPLLLDO_PD` reader - Audio PLL internal regulator"]
pub type AUDPLLLDO_PD_R = crate::BitReader<AUDPLLLDO_PD_A>;
#[doc = "Audio PLL internal regulator\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUDPLLLDO_PD_A {
    #[doc = "0: Enabled"]
    AUDPLLLDO_PD_0 = 0,
    #[doc = "1: Powerdown"]
    AUDPLLLDO_PD_1 = 1,
}
impl From<AUDPLLLDO_PD_A> for bool {
    #[inline(always)]
    fn from(variant: AUDPLLLDO_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl AUDPLLLDO_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUDPLLLDO_PD_A {
        match self.bits {
            false => AUDPLLLDO_PD_A::AUDPLLLDO_PD_0,
            true => AUDPLLLDO_PD_A::AUDPLLLDO_PD_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUDPLLLDO_PD_0`"]
    #[inline(always)]
    pub fn is_audpllldo_pd_0(&self) -> bool {
        *self == AUDPLLLDO_PD_A::AUDPLLLDO_PD_0
    }
    #[doc = "Checks if the value of the field is `AUDPLLLDO_PD_1`"]
    #[inline(always)]
    pub fn is_audpllldo_pd_1(&self) -> bool {
        *self == AUDPLLLDO_PD_A::AUDPLLLDO_PD_1
    }
}
#[doc = "Field `AUDPLLLDO_PD` writer - Audio PLL internal regulator"]
pub type AUDPLLLDO_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, AUDPLLLDO_PD_A, O>;
impl<'a, const O: u8> AUDPLLLDO_PD_W<'a, O> {
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn audpllldo_pd_0(self) -> &'a mut W {
        self.variant(AUDPLLLDO_PD_A::AUDPLLLDO_PD_0)
    }
    #[doc = "Powerdown"]
    #[inline(always)]
    pub fn audpllldo_pd_1(self) -> &'a mut W {
        self.variant(AUDPLLLDO_PD_A::AUDPLLLDO_PD_1)
    }
}
#[doc = "Field `AUDPLLANA_PD` reader - Audio PLL analog functions"]
pub type AUDPLLANA_PD_R = crate::BitReader<AUDPLLANA_PD_A>;
#[doc = "Audio PLL analog functions\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUDPLLANA_PD_A {
    #[doc = "0: Enabled"]
    AUDPLLANA_PD_0 = 0,
    #[doc = "1: Powerdown"]
    AUDPLLANA_PD_1 = 1,
}
impl From<AUDPLLANA_PD_A> for bool {
    #[inline(always)]
    fn from(variant: AUDPLLANA_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl AUDPLLANA_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUDPLLANA_PD_A {
        match self.bits {
            false => AUDPLLANA_PD_A::AUDPLLANA_PD_0,
            true => AUDPLLANA_PD_A::AUDPLLANA_PD_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUDPLLANA_PD_0`"]
    #[inline(always)]
    pub fn is_audpllana_pd_0(&self) -> bool {
        *self == AUDPLLANA_PD_A::AUDPLLANA_PD_0
    }
    #[doc = "Checks if the value of the field is `AUDPLLANA_PD_1`"]
    #[inline(always)]
    pub fn is_audpllana_pd_1(&self) -> bool {
        *self == AUDPLLANA_PD_A::AUDPLLANA_PD_1
    }
}
#[doc = "Field `AUDPLLANA_PD` writer - Audio PLL analog functions"]
pub type AUDPLLANA_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, AUDPLLANA_PD_A, O>;
impl<'a, const O: u8> AUDPLLANA_PD_W<'a, O> {
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn audpllana_pd_0(self) -> &'a mut W {
        self.variant(AUDPLLANA_PD_A::AUDPLLANA_PD_0)
    }
    #[doc = "Powerdown"]
    #[inline(always)]
    pub fn audpllana_pd_1(self) -> &'a mut W {
        self.variant(AUDPLLANA_PD_A::AUDPLLANA_PD_1)
    }
}
#[doc = "Field `ADC_PD` reader - ADC analog functions"]
pub type ADC_PD_R = crate::BitReader<ADC_PD_A>;
#[doc = "ADC analog functions\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_PD_A {
    #[doc = "0: Enabled"]
    ADC_PD_0 = 0,
    #[doc = "1: Powerdown"]
    ADC_PD_1 = 1,
}
impl From<ADC_PD_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_PD_A {
        match self.bits {
            false => ADC_PD_A::ADC_PD_0,
            true => ADC_PD_A::ADC_PD_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_PD_0`"]
    #[inline(always)]
    pub fn is_adc_pd_0(&self) -> bool {
        *self == ADC_PD_A::ADC_PD_0
    }
    #[doc = "Checks if the value of the field is `ADC_PD_1`"]
    #[inline(always)]
    pub fn is_adc_pd_1(&self) -> bool {
        *self == ADC_PD_A::ADC_PD_1
    }
}
#[doc = "Field `ADC_PD` writer - ADC analog functions"]
pub type ADC_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, ADC_PD_A, O>;
impl<'a, const O: u8> ADC_PD_W<'a, O> {
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn adc_pd_0(self) -> &'a mut W {
        self.variant(ADC_PD_A::ADC_PD_0)
    }
    #[doc = "Powerdown"]
    #[inline(always)]
    pub fn adc_pd_1(self) -> &'a mut W {
        self.variant(ADC_PD_A::ADC_PD_1)
    }
}
#[doc = "Field `ADC_LP` reader - ADC low power mode"]
pub type ADC_LP_R = crate::BitReader<ADC_LP_A>;
#[doc = "ADC low power mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_LP_A {
    #[doc = "0: Enabled"]
    ADC_LP_0 = 0,
    #[doc = "1: Powerdown"]
    ADC_LP_1 = 1,
}
impl From<ADC_LP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_LP_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_LP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_LP_A {
        match self.bits {
            false => ADC_LP_A::ADC_LP_0,
            true => ADC_LP_A::ADC_LP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_LP_0`"]
    #[inline(always)]
    pub fn is_adc_lp_0(&self) -> bool {
        *self == ADC_LP_A::ADC_LP_0
    }
    #[doc = "Checks if the value of the field is `ADC_LP_1`"]
    #[inline(always)]
    pub fn is_adc_lp_1(&self) -> bool {
        *self == ADC_LP_A::ADC_LP_1
    }
}
#[doc = "Field `ADC_LP` writer - ADC low power mode"]
pub type ADC_LP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, ADC_LP_A, O>;
impl<'a, const O: u8> ADC_LP_W<'a, O> {
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn adc_lp_0(self) -> &'a mut W {
        self.variant(ADC_LP_A::ADC_LP_0)
    }
    #[doc = "Powerdown"]
    #[inline(always)]
    pub fn adc_lp_1(self) -> &'a mut W {
        self.variant(ADC_LP_A::ADC_LP_1)
    }
}
#[doc = "Field `ADC_TEMPSNS_PD` reader - ADC temperature sensor"]
pub type ADC_TEMPSNS_PD_R = crate::BitReader<ADC_TEMPSNS_PD_A>;
#[doc = "ADC temperature sensor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_TEMPSNS_PD_A {
    #[doc = "0: Enabled"]
    ADC_TEMPSNS_PD_0 = 0,
    #[doc = "1: Powerdown"]
    ADC_TEMPSNS_PD_1 = 1,
}
impl From<ADC_TEMPSNS_PD_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_TEMPSNS_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_TEMPSNS_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_TEMPSNS_PD_A {
        match self.bits {
            false => ADC_TEMPSNS_PD_A::ADC_TEMPSNS_PD_0,
            true => ADC_TEMPSNS_PD_A::ADC_TEMPSNS_PD_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_TEMPSNS_PD_0`"]
    #[inline(always)]
    pub fn is_adc_tempsns_pd_0(&self) -> bool {
        *self == ADC_TEMPSNS_PD_A::ADC_TEMPSNS_PD_0
    }
    #[doc = "Checks if the value of the field is `ADC_TEMPSNS_PD_1`"]
    #[inline(always)]
    pub fn is_adc_tempsns_pd_1(&self) -> bool {
        *self == ADC_TEMPSNS_PD_A::ADC_TEMPSNS_PD_1
    }
}
#[doc = "Field `ADC_TEMPSNS_PD` writer - ADC temperature sensor"]
pub type ADC_TEMPSNS_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, ADC_TEMPSNS_PD_A, O>;
impl<'a, const O: u8> ADC_TEMPSNS_PD_W<'a, O> {
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn adc_tempsns_pd_0(self) -> &'a mut W {
        self.variant(ADC_TEMPSNS_PD_A::ADC_TEMPSNS_PD_0)
    }
    #[doc = "Powerdown"]
    #[inline(always)]
    pub fn adc_tempsns_pd_1(self) -> &'a mut W {
        self.variant(ADC_TEMPSNS_PD_A::ADC_TEMPSNS_PD_1)
    }
}
#[doc = "Field `PMC_TEMPSNS_PD` reader - PMC temperature sensor"]
pub type PMC_TEMPSNS_PD_R = crate::BitReader<PMC_TEMPSNS_PD_A>;
#[doc = "PMC temperature sensor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMC_TEMPSNS_PD_A {
    #[doc = "0: Enabled"]
    PMC_TEMPSNS_PD_0 = 0,
    #[doc = "1: Powerdown"]
    PMC_TEMPSNS_PD_1 = 1,
}
impl From<PMC_TEMPSNS_PD_A> for bool {
    #[inline(always)]
    fn from(variant: PMC_TEMPSNS_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl PMC_TEMPSNS_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMC_TEMPSNS_PD_A {
        match self.bits {
            false => PMC_TEMPSNS_PD_A::PMC_TEMPSNS_PD_0,
            true => PMC_TEMPSNS_PD_A::PMC_TEMPSNS_PD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMC_TEMPSNS_PD_0`"]
    #[inline(always)]
    pub fn is_pmc_tempsns_pd_0(&self) -> bool {
        *self == PMC_TEMPSNS_PD_A::PMC_TEMPSNS_PD_0
    }
    #[doc = "Checks if the value of the field is `PMC_TEMPSNS_PD_1`"]
    #[inline(always)]
    pub fn is_pmc_tempsns_pd_1(&self) -> bool {
        *self == PMC_TEMPSNS_PD_A::PMC_TEMPSNS_PD_1
    }
}
#[doc = "Field `PMC_TEMPSNS_PD` writer - PMC temperature sensor"]
pub type PMC_TEMPSNS_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, PMC_TEMPSNS_PD_A, O>;
impl<'a, const O: u8> PMC_TEMPSNS_PD_W<'a, O> {
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn pmc_tempsns_pd_0(self) -> &'a mut W {
        self.variant(PMC_TEMPSNS_PD_A::PMC_TEMPSNS_PD_0)
    }
    #[doc = "Powerdown"]
    #[inline(always)]
    pub fn pmc_tempsns_pd_1(self) -> &'a mut W {
        self.variant(PMC_TEMPSNS_PD_A::PMC_TEMPSNS_PD_1)
    }
}
#[doc = "Field `ACMP_PD` reader - Analog comparator"]
pub type ACMP_PD_R = crate::BitReader<ACMP_PD_A>;
#[doc = "Analog comparator\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMP_PD_A {
    #[doc = "0: Enabled"]
    ACMP_PD_0 = 0,
    #[doc = "1: Powerdown"]
    ACMP_PD_1 = 1,
}
impl From<ACMP_PD_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMP_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP_PD_A {
        match self.bits {
            false => ACMP_PD_A::ACMP_PD_0,
            true => ACMP_PD_A::ACMP_PD_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP_PD_0`"]
    #[inline(always)]
    pub fn is_acmp_pd_0(&self) -> bool {
        *self == ACMP_PD_A::ACMP_PD_0
    }
    #[doc = "Checks if the value of the field is `ACMP_PD_1`"]
    #[inline(always)]
    pub fn is_acmp_pd_1(&self) -> bool {
        *self == ACMP_PD_A::ACMP_PD_1
    }
}
#[doc = "Field `ACMP_PD` writer - Analog comparator"]
pub type ACMP_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, ACMP_PD_A, O>;
impl<'a, const O: u8> ACMP_PD_W<'a, O> {
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn acmp_pd_0(self) -> &'a mut W {
        self.variant(ACMP_PD_A::ACMP_PD_0)
    }
    #[doc = "Powerdown"]
    #[inline(always)]
    pub fn acmp_pd_1(self) -> &'a mut W {
        self.variant(ACMP_PD_A::ACMP_PD_1)
    }
}
impl R {
    #[doc = "Bit 0 - Main clock shut off"]
    #[inline(always)]
    pub fn mainclk_shutoff(&self) -> MAINCLK_SHUTOFF_R {
        MAINCLK_SHUTOFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PMIC_MODE0 device pin"]
    #[inline(always)]
    pub fn pmic_mode0(&self) -> PMIC_MODE0_R {
        PMIC_MODE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PMIC_MODE1 device pin"]
    #[inline(always)]
    pub fn pmic_mode1(&self) -> PMIC_MODE1_R {
        PMIC_MODE1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Deep power-down mode"]
    #[inline(always)]
    pub fn deep_pd(&self) -> DEEP_PD_R {
        DEEP_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Vddcore regulator mode"]
    #[inline(always)]
    pub fn vddcorereg_lp(&self) -> VDDCOREREG_LP_R {
        VDDCOREREG_LP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 192/96 FRO Clock Gate"]
    #[inline(always)]
    pub fn fro_cg(&self) -> FRO_CG_R {
        FRO_CG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Internal PMC references LP mode"]
    #[inline(always)]
    pub fn pmcref_lp(&self) -> PMCREF_LP_R {
        PMCREF_LP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HVD"]
    #[inline(always)]
    pub fn hvd1v8_pd(&self) -> HVD1V8_PD_R {
        HVD1V8_PD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LVD"]
    #[inline(always)]
    pub fn porcore_lp(&self) -> PORCORE_LP_R {
        PORCORE_LP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LVD"]
    #[inline(always)]
    pub fn lvdcore_lp(&self) -> LVDCORE_LP_R {
        LVDCORE_LP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HVD"]
    #[inline(always)]
    pub fn hvdcore_pd(&self) -> HVDCORE_PD_R {
        HVDCORE_PD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reverse body-bias"]
    #[inline(always)]
    pub fn rbb_pd(&self) -> RBB_PD_R {
        RBB_PD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Forward body-bias"]
    #[inline(always)]
    pub fn fbb_pd(&self) -> FBB_PD_R {
        FBB_PD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Main crystal oscillator"]
    #[inline(always)]
    pub fn sysxtal_pd(&self) -> SYSXTAL_PD_R {
        SYSXTAL_PD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1 MHz Low-Power oscillator"]
    #[inline(always)]
    pub fn lposc_pd(&self) -> LPOSC_PD_R {
        LPOSC_PD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reverse body-bias SRAM"]
    #[inline(always)]
    pub fn rbbsram_pd(&self) -> RBBSRAM_PD_R {
        RBBSRAM_PD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - FFRO 192/96 MHz internal oscillator"]
    #[inline(always)]
    pub fn ffro_pd(&self) -> FFRO_PD_R {
        FFRO_PD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - System PLL internal regulator"]
    #[inline(always)]
    pub fn syspllldo_pd(&self) -> SYSPLLLDO_PD_R {
        SYSPLLLDO_PD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - System PLL analog functions"]
    #[inline(always)]
    pub fn syspllana_pd(&self) -> SYSPLLANA_PD_R {
        SYSPLLANA_PD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Audio PLL internal regulator"]
    #[inline(always)]
    pub fn audpllldo_pd(&self) -> AUDPLLLDO_PD_R {
        AUDPLLLDO_PD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Audio PLL analog functions"]
    #[inline(always)]
    pub fn audpllana_pd(&self) -> AUDPLLANA_PD_R {
        AUDPLLANA_PD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ADC analog functions"]
    #[inline(always)]
    pub fn adc_pd(&self) -> ADC_PD_R {
        ADC_PD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ADC low power mode"]
    #[inline(always)]
    pub fn adc_lp(&self) -> ADC_LP_R {
        ADC_LP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ADC temperature sensor"]
    #[inline(always)]
    pub fn adc_tempsns_pd(&self) -> ADC_TEMPSNS_PD_R {
        ADC_TEMPSNS_PD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PMC temperature sensor"]
    #[inline(always)]
    pub fn pmc_tempsns_pd(&self) -> PMC_TEMPSNS_PD_R {
        PMC_TEMPSNS_PD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Analog comparator"]
    #[inline(always)]
    pub fn acmp_pd(&self) -> ACMP_PD_R {
        ACMP_PD_R::new(((self.bits >> 25) & 1) != 0)
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
    #[doc = "Bit 4 - Vddcore regulator mode"]
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
    #[doc = "Bit 16 - FFRO 192/96 MHz internal oscillator"]
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
#[doc = "Run configuration 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdruncfg0](index.html) module"]
pub struct PDRUNCFG0_SPEC;
impl crate::RegisterSpec for PDRUNCFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdruncfg0::R](R) reader structure"]
impl crate::Readable for PDRUNCFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdruncfg0::W](W) writer structure"]
impl crate::Writable for PDRUNCFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDRUNCFG0 to value 0x03fe_ac80"]
impl crate::Resettable for PDRUNCFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x03fe_ac80;
}
