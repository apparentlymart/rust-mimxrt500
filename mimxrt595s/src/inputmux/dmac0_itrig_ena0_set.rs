#[doc = "Register `DMAC0_ITRIG_ENA0_SET` writer"]
pub struct W(crate::W<DMAC0_ITRIG_ENA0_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC0_ITRIG_ENA0_SET_SPEC>;
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
impl From<crate::W<DMAC0_ITRIG_ENA0_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC0_ITRIG_ENA0_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPIO_INT0 set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT0_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<GPIO_INT0_AW> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0` writer - GPIO_INT0 set"]
pub type GPIO_INT0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, GPIO_INT0_AW, O>;
impl<'a, const O: u8> GPIO_INT0_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INT0_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INT0_AW::ENABLE)
    }
}
#[doc = "GPIO_INT1 set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT1_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<GPIO_INT1_AW> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT1` writer - GPIO_INT1 set"]
pub type GPIO_INT1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, GPIO_INT1_AW, O>;
impl<'a, const O: u8> GPIO_INT1_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INT1_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INT1_AW::ENABLE)
    }
}
#[doc = "GPIO_INT2 set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT2_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<GPIO_INT2_AW> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT2` writer - GPIO_INT2 set"]
pub type GPIO_INT2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, GPIO_INT2_AW, O>;
impl<'a, const O: u8> GPIO_INT2_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INT2_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INT2_AW::ENABLE)
    }
}
#[doc = "GPIO_INT3 set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT3_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<GPIO_INT3_AW> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT3` writer - GPIO_INT3 set"]
pub type GPIO_INT3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, GPIO_INT3_AW, O>;
impl<'a, const O: u8> GPIO_INT3_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INT3_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INT3_AW::ENABLE)
    }
}
#[doc = "T0_DMAREQ_M0 set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T0_DMAREQ_M0_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<T0_DMAREQ_M0_AW> for bool {
    #[inline(always)]
    fn from(variant: T0_DMAREQ_M0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `T0_DMAREQ_M0` writer - T0_DMAREQ_M0 set"]
pub type T0_DMAREQ_M0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, T0_DMAREQ_M0_AW, O>;
impl<'a, const O: u8> T0_DMAREQ_M0_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(T0_DMAREQ_M0_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(T0_DMAREQ_M0_AW::ENABLE)
    }
}
#[doc = "T0_DMAREQ_M1 set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T0_DMAREQ_M1_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<T0_DMAREQ_M1_AW> for bool {
    #[inline(always)]
    fn from(variant: T0_DMAREQ_M1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `T0_DMAREQ_M1` writer - T0_DMAREQ_M1 set"]
pub type T0_DMAREQ_M1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, T0_DMAREQ_M1_AW, O>;
impl<'a, const O: u8> T0_DMAREQ_M1_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(T0_DMAREQ_M1_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(T0_DMAREQ_M1_AW::ENABLE)
    }
}
#[doc = "T1_DMAREQ_M0 set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T1_DMAREQ_M0_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<T1_DMAREQ_M0_AW> for bool {
    #[inline(always)]
    fn from(variant: T1_DMAREQ_M0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `T1_DMAREQ_M0` writer - T1_DMAREQ_M0 set"]
pub type T1_DMAREQ_M0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, T1_DMAREQ_M0_AW, O>;
impl<'a, const O: u8> T1_DMAREQ_M0_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(T1_DMAREQ_M0_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(T1_DMAREQ_M0_AW::ENABLE)
    }
}
#[doc = "T1_DMAREQ_M1 set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T1_DMAREQ_M1_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<T1_DMAREQ_M1_AW> for bool {
    #[inline(always)]
    fn from(variant: T1_DMAREQ_M1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `T1_DMAREQ_M1` writer - T1_DMAREQ_M1 set"]
pub type T1_DMAREQ_M1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, T1_DMAREQ_M1_AW, O>;
impl<'a, const O: u8> T1_DMAREQ_M1_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(T1_DMAREQ_M1_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(T1_DMAREQ_M1_AW::ENABLE)
    }
}
#[doc = "T2_DMAREQ_M0 set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T2_DMAREQ_M0_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<T2_DMAREQ_M0_AW> for bool {
    #[inline(always)]
    fn from(variant: T2_DMAREQ_M0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `T2_DMAREQ_M0` writer - T2_DMAREQ_M0 set"]
pub type T2_DMAREQ_M0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, T2_DMAREQ_M0_AW, O>;
impl<'a, const O: u8> T2_DMAREQ_M0_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(T2_DMAREQ_M0_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(T2_DMAREQ_M0_AW::ENABLE)
    }
}
#[doc = "T2_DMAREQ_M1 set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T2_DMAREQ_M1_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<T2_DMAREQ_M1_AW> for bool {
    #[inline(always)]
    fn from(variant: T2_DMAREQ_M1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `T2_DMAREQ_M1` writer - T2_DMAREQ_M1 set"]
pub type T2_DMAREQ_M1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, T2_DMAREQ_M1_AW, O>;
impl<'a, const O: u8> T2_DMAREQ_M1_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(T2_DMAREQ_M1_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(T2_DMAREQ_M1_AW::ENABLE)
    }
}
#[doc = "T3_DMAREQ_M0 set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T3_DMAREQ_M0_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<T3_DMAREQ_M0_AW> for bool {
    #[inline(always)]
    fn from(variant: T3_DMAREQ_M0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `T3_DMAREQ_M0` writer - T3_DMAREQ_M0 set"]
pub type T3_DMAREQ_M0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, T3_DMAREQ_M0_AW, O>;
impl<'a, const O: u8> T3_DMAREQ_M0_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(T3_DMAREQ_M0_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(T3_DMAREQ_M0_AW::ENABLE)
    }
}
#[doc = "T3_DMAREQ_M1 set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T3_DMAREQ_M1_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<T3_DMAREQ_M1_AW> for bool {
    #[inline(always)]
    fn from(variant: T3_DMAREQ_M1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `T3_DMAREQ_M1` writer - T3_DMAREQ_M1 set"]
pub type T3_DMAREQ_M1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, T3_DMAREQ_M1_AW, O>;
impl<'a, const O: u8> T3_DMAREQ_M1_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(T3_DMAREQ_M1_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(T3_DMAREQ_M1_AW::ENABLE)
    }
}
#[doc = "T4_DMAREQ_M0 set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T4_DMAREQ_M0_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<T4_DMAREQ_M0_AW> for bool {
    #[inline(always)]
    fn from(variant: T4_DMAREQ_M0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `T4_DMAREQ_M0` writer - T4_DMAREQ_M0 set"]
pub type T4_DMAREQ_M0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, T4_DMAREQ_M0_AW, O>;
impl<'a, const O: u8> T4_DMAREQ_M0_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(T4_DMAREQ_M0_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(T4_DMAREQ_M0_AW::ENABLE)
    }
}
#[doc = "T4_DMAREQ_M1 set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T4_DMAREQ_M1_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<T4_DMAREQ_M1_AW> for bool {
    #[inline(always)]
    fn from(variant: T4_DMAREQ_M1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `T4_DMAREQ_M1` writer - T4_DMAREQ_M1 set"]
pub type T4_DMAREQ_M1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, T4_DMAREQ_M1_AW, O>;
impl<'a, const O: u8> T4_DMAREQ_M1_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(T4_DMAREQ_M1_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(T4_DMAREQ_M1_AW::ENABLE)
    }
}
#[doc = "SDMA0_TRIGOUT_Aset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMA0_TRIGOUT_A_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<SDMA0_TRIGOUT_A_AW> for bool {
    #[inline(always)]
    fn from(variant: SDMA0_TRIGOUT_A_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMA0_TRIGOUT_A` writer - SDMA0_TRIGOUT_Aset"]
pub type SDMA0_TRIGOUT_A_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, SDMA0_TRIGOUT_A_AW, O>;
impl<'a, const O: u8> SDMA0_TRIGOUT_A_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDMA0_TRIGOUT_A_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDMA0_TRIGOUT_A_AW::ENABLE)
    }
}
#[doc = "SDMA0_TRIGOUT_B set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMA0_TRIGOUT_B_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<SDMA0_TRIGOUT_B_AW> for bool {
    #[inline(always)]
    fn from(variant: SDMA0_TRIGOUT_B_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMA0_TRIGOUT_B` writer - SDMA0_TRIGOUT_B set"]
pub type SDMA0_TRIGOUT_B_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, SDMA0_TRIGOUT_B_AW, O>;
impl<'a, const O: u8> SDMA0_TRIGOUT_B_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDMA0_TRIGOUT_B_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDMA0_TRIGOUT_B_AW::ENABLE)
    }
}
#[doc = "SDMA0_TRIGOUT_C set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMA0_TRIGOUT_C_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<SDMA0_TRIGOUT_C_AW> for bool {
    #[inline(always)]
    fn from(variant: SDMA0_TRIGOUT_C_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMA0_TRIGOUT_C` writer - SDMA0_TRIGOUT_C set"]
pub type SDMA0_TRIGOUT_C_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, SDMA0_TRIGOUT_C_AW, O>;
impl<'a, const O: u8> SDMA0_TRIGOUT_C_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDMA0_TRIGOUT_C_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDMA0_TRIGOUT_C_AW::ENABLE)
    }
}
#[doc = "SDMA0_TRIGOUT_D set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMA0_TRIGOUT_D_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<SDMA0_TRIGOUT_D_AW> for bool {
    #[inline(always)]
    fn from(variant: SDMA0_TRIGOUT_D_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMA0_TRIGOUT_D` writer - SDMA0_TRIGOUT_D set"]
pub type SDMA0_TRIGOUT_D_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, SDMA0_TRIGOUT_D_AW, O>;
impl<'a, const O: u8> SDMA0_TRIGOUT_D_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDMA0_TRIGOUT_D_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDMA0_TRIGOUT_D_AW::ENABLE)
    }
}
#[doc = "SCT_DMA_REQ0 set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCT_DMA_REQ0_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<SCT_DMA_REQ0_AW> for bool {
    #[inline(always)]
    fn from(variant: SCT_DMA_REQ0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCT_DMA_REQ0` writer - SCT_DMA_REQ0 set"]
pub type SCT_DMA_REQ0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, SCT_DMA_REQ0_AW, O>;
impl<'a, const O: u8> SCT_DMA_REQ0_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SCT_DMA_REQ0_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SCT_DMA_REQ0_AW::ENABLE)
    }
}
#[doc = "SCT_DMA_REQ1 set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCT_DMA_REQ1_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<SCT_DMA_REQ1_AW> for bool {
    #[inline(always)]
    fn from(variant: SCT_DMA_REQ1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCT_DMA_REQ1` writer - SCT_DMA_REQ1 set"]
pub type SCT_DMA_REQ1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, SCT_DMA_REQ1_AW, O>;
impl<'a, const O: u8> SCT_DMA_REQ1_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SCT_DMA_REQ1_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SCT_DMA_REQ1_AW::ENABLE)
    }
}
#[doc = "HASHCRYPT_OUT set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HASHCRYPT_OUT_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<HASHCRYPT_OUT_AW> for bool {
    #[inline(always)]
    fn from(variant: HASHCRYPT_OUT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASHCRYPT_OUT` writer - HASHCRYPT_OUT set"]
pub type HASHCRYPT_OUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, HASHCRYPT_OUT_AW, O>;
impl<'a, const O: u8> HASHCRYPT_OUT_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HASHCRYPT_OUT_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HASHCRYPT_OUT_AW::ENABLE)
    }
}
#[doc = "ACMP set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMP_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<ACMP_AW> for bool {
    #[inline(always)]
    fn from(variant: ACMP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP` writer - ACMP set"]
pub type ACMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, ACMP_AW, O>;
impl<'a, const O: u8> ACMP_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ACMP_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ACMP_AW::ENABLE)
    }
}
#[doc = "FlexSPI0_RX set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI0_RX_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<FLEXSPI0_RX_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI0_RX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXSPI0_RX` writer - FlexSPI0_RX set"]
pub type FLEXSPI0_RX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, FLEXSPI0_RX_AW, O>;
impl<'a, const O: u8> FLEXSPI0_RX_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXSPI0_RX_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXSPI0_RX_AW::ENABLE)
    }
}
#[doc = "FlexSPI0_TX set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI0_TX_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<FLEXSPI0_TX_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI0_TX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXSPI0_TX` writer - FlexSPI0_TX set"]
pub type FLEXSPI0_TX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, FLEXSPI0_TX_AW, O>;
impl<'a, const O: u8> FLEXSPI0_TX_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXSPI0_TX_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXSPI0_TX_AW::ENABLE)
    }
}
#[doc = "ADC set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<ADC_AW> for bool {
    #[inline(always)]
    fn from(variant: ADC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC` writer - ADC set"]
pub type ADC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, ADC_AW, O>;
impl<'a, const O: u8> ADC_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC_AW::ENABLE)
    }
}
#[doc = "FlexSPI1_RX set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI1_RX_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<FLEXSPI1_RX_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI1_RX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXSPI1_RX` writer - FlexSPI1_RX set"]
pub type FLEXSPI1_RX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, FLEXSPI1_RX_AW, O>;
impl<'a, const O: u8> FLEXSPI1_RX_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXSPI1_RX_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXSPI1_RX_AW::ENABLE)
    }
}
#[doc = "FlexSPI1_TX set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI1_TX_AW {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the ENA0 Bit"]
    ENABLE = 1,
}
impl From<FLEXSPI1_TX_AW> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI1_TX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXSPI1_TX` writer - FlexSPI1_TX set"]
pub type FLEXSPI1_TX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SET_SPEC, FLEXSPI1_TX_AW, O>;
impl<'a, const O: u8> FLEXSPI1_TX_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXSPI1_TX_AW::DISABLE)
    }
    #[doc = "Sets the ENA0 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXSPI1_TX_AW::ENABLE)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO_INT0 set"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0(&mut self) -> GPIO_INT0_W<0> {
        GPIO_INT0_W::new(self)
    }
    #[doc = "Bit 1 - GPIO_INT1 set"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int1(&mut self) -> GPIO_INT1_W<1> {
        GPIO_INT1_W::new(self)
    }
    #[doc = "Bit 2 - GPIO_INT2 set"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int2(&mut self) -> GPIO_INT2_W<2> {
        GPIO_INT2_W::new(self)
    }
    #[doc = "Bit 3 - GPIO_INT3 set"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int3(&mut self) -> GPIO_INT3_W<3> {
        GPIO_INT3_W::new(self)
    }
    #[doc = "Bit 4 - T0_DMAREQ_M0 set"]
    #[inline(always)]
    #[must_use]
    pub fn t0_dmareq_m0(&mut self) -> T0_DMAREQ_M0_W<4> {
        T0_DMAREQ_M0_W::new(self)
    }
    #[doc = "Bit 5 - T0_DMAREQ_M1 set"]
    #[inline(always)]
    #[must_use]
    pub fn t0_dmareq_m1(&mut self) -> T0_DMAREQ_M1_W<5> {
        T0_DMAREQ_M1_W::new(self)
    }
    #[doc = "Bit 6 - T1_DMAREQ_M0 set"]
    #[inline(always)]
    #[must_use]
    pub fn t1_dmareq_m0(&mut self) -> T1_DMAREQ_M0_W<6> {
        T1_DMAREQ_M0_W::new(self)
    }
    #[doc = "Bit 7 - T1_DMAREQ_M1 set"]
    #[inline(always)]
    #[must_use]
    pub fn t1_dmareq_m1(&mut self) -> T1_DMAREQ_M1_W<7> {
        T1_DMAREQ_M1_W::new(self)
    }
    #[doc = "Bit 8 - T2_DMAREQ_M0 set"]
    #[inline(always)]
    #[must_use]
    pub fn t2_dmareq_m0(&mut self) -> T2_DMAREQ_M0_W<8> {
        T2_DMAREQ_M0_W::new(self)
    }
    #[doc = "Bit 9 - T2_DMAREQ_M1 set"]
    #[inline(always)]
    #[must_use]
    pub fn t2_dmareq_m1(&mut self) -> T2_DMAREQ_M1_W<9> {
        T2_DMAREQ_M1_W::new(self)
    }
    #[doc = "Bit 10 - T3_DMAREQ_M0 set"]
    #[inline(always)]
    #[must_use]
    pub fn t3_dmareq_m0(&mut self) -> T3_DMAREQ_M0_W<10> {
        T3_DMAREQ_M0_W::new(self)
    }
    #[doc = "Bit 11 - T3_DMAREQ_M1 set"]
    #[inline(always)]
    #[must_use]
    pub fn t3_dmareq_m1(&mut self) -> T3_DMAREQ_M1_W<11> {
        T3_DMAREQ_M1_W::new(self)
    }
    #[doc = "Bit 12 - T4_DMAREQ_M0 set"]
    #[inline(always)]
    #[must_use]
    pub fn t4_dmareq_m0(&mut self) -> T4_DMAREQ_M0_W<12> {
        T4_DMAREQ_M0_W::new(self)
    }
    #[doc = "Bit 13 - T4_DMAREQ_M1 set"]
    #[inline(always)]
    #[must_use]
    pub fn t4_dmareq_m1(&mut self) -> T4_DMAREQ_M1_W<13> {
        T4_DMAREQ_M1_W::new(self)
    }
    #[doc = "Bit 14 - SDMA0_TRIGOUT_Aset"]
    #[inline(always)]
    #[must_use]
    pub fn sdma0_trigout_a(&mut self) -> SDMA0_TRIGOUT_A_W<14> {
        SDMA0_TRIGOUT_A_W::new(self)
    }
    #[doc = "Bit 15 - SDMA0_TRIGOUT_B set"]
    #[inline(always)]
    #[must_use]
    pub fn sdma0_trigout_b(&mut self) -> SDMA0_TRIGOUT_B_W<15> {
        SDMA0_TRIGOUT_B_W::new(self)
    }
    #[doc = "Bit 16 - SDMA0_TRIGOUT_C set"]
    #[inline(always)]
    #[must_use]
    pub fn sdma0_trigout_c(&mut self) -> SDMA0_TRIGOUT_C_W<16> {
        SDMA0_TRIGOUT_C_W::new(self)
    }
    #[doc = "Bit 17 - SDMA0_TRIGOUT_D set"]
    #[inline(always)]
    #[must_use]
    pub fn sdma0_trigout_d(&mut self) -> SDMA0_TRIGOUT_D_W<17> {
        SDMA0_TRIGOUT_D_W::new(self)
    }
    #[doc = "Bit 18 - SCT_DMA_REQ0 set"]
    #[inline(always)]
    #[must_use]
    pub fn sct_dma_req0(&mut self) -> SCT_DMA_REQ0_W<18> {
        SCT_DMA_REQ0_W::new(self)
    }
    #[doc = "Bit 19 - SCT_DMA_REQ1 set"]
    #[inline(always)]
    #[must_use]
    pub fn sct_dma_req1(&mut self) -> SCT_DMA_REQ1_W<19> {
        SCT_DMA_REQ1_W::new(self)
    }
    #[doc = "Bit 20 - HASHCRYPT_OUT set"]
    #[inline(always)]
    #[must_use]
    pub fn hashcrypt_out(&mut self) -> HASHCRYPT_OUT_W<20> {
        HASHCRYPT_OUT_W::new(self)
    }
    #[doc = "Bit 21 - ACMP set"]
    #[inline(always)]
    #[must_use]
    pub fn acmp(&mut self) -> ACMP_W<21> {
        ACMP_W::new(self)
    }
    #[doc = "Bit 22 - FlexSPI0_RX set"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi0_rx(&mut self) -> FLEXSPI0_RX_W<22> {
        FLEXSPI0_RX_W::new(self)
    }
    #[doc = "Bit 23 - FlexSPI0_TX set"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi0_tx(&mut self) -> FLEXSPI0_TX_W<23> {
        FLEXSPI0_TX_W::new(self)
    }
    #[doc = "Bit 24 - ADC set"]
    #[inline(always)]
    #[must_use]
    pub fn adc(&mut self) -> ADC_W<24> {
        ADC_W::new(self)
    }
    #[doc = "Bit 25 - FlexSPI1_RX set"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi1_rx(&mut self) -> FLEXSPI1_RX_W<25> {
        FLEXSPI1_RX_W::new(self)
    }
    #[doc = "Bit 26 - FlexSPI1_TX set"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi1_tx(&mut self) -> FLEXSPI1_TX_W<26> {
        FLEXSPI1_TX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC0 Input Trigger Enable 0 Set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac0_itrig_ena0_set](index.html) module"]
pub struct DMAC0_ITRIG_ENA0_SET_SPEC;
impl crate::RegisterSpec for DMAC0_ITRIG_ENA0_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dmac0_itrig_ena0_set::W](W) writer structure"]
impl crate::Writable for DMAC0_ITRIG_ENA0_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAC0_ITRIG_ENA0_SET to value 0"]
impl crate::Resettable for DMAC0_ITRIG_ENA0_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
