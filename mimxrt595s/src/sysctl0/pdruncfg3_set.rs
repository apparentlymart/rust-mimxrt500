#[doc = "Register `PDRUNCFG3_SET` writer"]
pub struct W(crate::W<PDRUNCFG3_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDRUNCFG3_SET_SPEC>;
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
impl From<crate::W<PDRUNCFG3_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDRUNCFG3_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Periphery power for RAM interface 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF0_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF0_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF0_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF0_PPD` writer - Periphery power for RAM interface 0."]
pub type SRAM_IF0_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF0_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF0_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF0_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF0_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF1_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF1_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF1_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF1_PPD` writer - Periphery power for RAM interface 1."]
pub type SRAM_IF1_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF1_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF1_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF1_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF1_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF2_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF2_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF2_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF2_PPD` writer - Periphery power for RAM interface 2."]
pub type SRAM_IF2_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF2_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF2_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF2_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF2_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF3_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF3_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF3_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF3_PPD` writer - Periphery power for RAM interface 3."]
pub type SRAM_IF3_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF3_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF3_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF3_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF3_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF4_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF4_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF4_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF4_PPD` writer - Periphery power for RAM interface 4."]
pub type SRAM_IF4_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF4_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF4_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF4_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF4_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF5_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF5_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF5_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF5_PPD` writer - Periphery power for RAM interface 5."]
pub type SRAM_IF5_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF5_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF5_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF5_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF5_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF6_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF6_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF6_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF6_PPD` writer - Periphery power for RAM interface 6."]
pub type SRAM_IF6_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF6_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF6_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF6_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF6_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF7_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF7_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF7_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF7_PPD` writer - Periphery power for RAM interface 7."]
pub type SRAM_IF7_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF7_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF7_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF7_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF7_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF8_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF8_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF8_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF8_PPD` writer - Periphery power for RAM interface 8."]
pub type SRAM_IF8_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF8_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF8_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF8_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF8_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF9_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF9_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF9_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF9_PPD` writer - Periphery power for RAM interface 9."]
pub type SRAM_IF9_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF9_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF9_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF9_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF9_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF10_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF10_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF10_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF10_PPD` writer - Periphery power for RAM interface 10."]
pub type SRAM_IF10_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF10_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF10_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF10_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF10_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF11_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF11_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF11_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF11_PPD` writer - Periphery power for RAM interface 11."]
pub type SRAM_IF11_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF11_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF11_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF11_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF11_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF12_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF12_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF12_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF12_PPD` writer - Periphery power for RAM interface 12."]
pub type SRAM_IF12_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF12_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF12_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF12_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF12_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF13_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF13_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF13_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF13_PPD` writer - Periphery power for RAM interface 13."]
pub type SRAM_IF13_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF13_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF13_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF13_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF13_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF14_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF14_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF14_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF14_PPD` writer - Periphery power for RAM interface 14."]
pub type SRAM_IF14_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF14_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF14_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF14_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF14_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF15_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF15_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF15_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF15_PPD` writer - Periphery power for RAM interface 15."]
pub type SRAM_IF15_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF15_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF15_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF15_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF15_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF16_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF16_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF16_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF16_PPD` writer - Periphery power for RAM interface 16."]
pub type SRAM_IF16_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF16_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF16_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF16_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF16_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF17_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF17_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF17_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF17_PPD` writer - Periphery power for RAM interface 17."]
pub type SRAM_IF17_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF17_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF17_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF17_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF17_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF18_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF18_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF18_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF18_PPD` writer - Periphery power for RAM interface 18."]
pub type SRAM_IF18_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF18_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF18_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF18_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF18_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF19_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF19_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF19_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF19_PPD` writer - Periphery power for RAM interface 19."]
pub type SRAM_IF19_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF19_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF19_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF19_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF19_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF20_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF20_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF20_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF20_PPD` writer - Periphery power for RAM interface 20."]
pub type SRAM_IF20_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF20_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF20_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF20_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF20_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF21_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF21_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF21_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF21_PPD` writer - Periphery power for RAM interface 21."]
pub type SRAM_IF21_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF21_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF21_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF21_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF21_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF22_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF22_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF22_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF22_PPD` writer - Periphery power for RAM interface 22."]
pub type SRAM_IF22_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF22_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF22_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF22_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF22_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF23_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF23_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF23_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF23_PPD` writer - Periphery power for RAM interface 23."]
pub type SRAM_IF23_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF23_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF23_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF23_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF23_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF24_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF24_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF24_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF24_PPD` writer - Periphery power for RAM interface 24."]
pub type SRAM_IF24_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF24_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF24_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF24_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF24_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF25_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF25_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF25_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF25_PPD` writer - Periphery power for RAM interface 25."]
pub type SRAM_IF25_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF25_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF25_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF25_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF25_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF26_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF26_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF26_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF26_PPD` writer - Periphery power for RAM interface 26."]
pub type SRAM_IF26_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF26_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF26_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF26_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF26_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF27_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF27_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF27_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF27_PPD` writer - Periphery power for RAM interface 27."]
pub type SRAM_IF27_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF27_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF27_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF27_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF27_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF28_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF28_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF28_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF28_PPD` writer - Periphery power for RAM interface 28."]
pub type SRAM_IF28_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF28_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF28_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF28_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF28_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 29.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF29_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF29_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF29_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF29_PPD` writer - Periphery power for RAM interface 29."]
pub type SRAM_IF29_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF29_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF29_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF29_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF29_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 30.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF30_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF30_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF30_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF30_PPD` writer - Periphery power for RAM interface 30."]
pub type SRAM_IF30_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF30_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF30_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF30_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF30_PPD_AW::ENABLE)
    }
}
#[doc = "Periphery power for RAM interface 31.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IF31_PPD_AW {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the PDRUNCFG3 Bit"]
    ENABLE = 1,
}
impl From<SRAM_IF31_PPD_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IF31_PPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_IF31_PPD` writer - Periphery power for RAM interface 31."]
pub type SRAM_IF31_PPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG3_SET_SPEC, SRAM_IF31_PPD_AW, O>;
impl<'a, const O: u8> SRAM_IF31_PPD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_IF31_PPD_AW::DISABLE)
    }
    #[doc = "Sets the PDRUNCFG3 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_IF31_PPD_AW::ENABLE)
    }
}
impl W {
    #[doc = "Bit 0 - Periphery power for RAM interface 0."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if0_ppd(&mut self) -> SRAM_IF0_PPD_W<0> {
        SRAM_IF0_PPD_W::new(self)
    }
    #[doc = "Bit 1 - Periphery power for RAM interface 1."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if1_ppd(&mut self) -> SRAM_IF1_PPD_W<1> {
        SRAM_IF1_PPD_W::new(self)
    }
    #[doc = "Bit 2 - Periphery power for RAM interface 2."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if2_ppd(&mut self) -> SRAM_IF2_PPD_W<2> {
        SRAM_IF2_PPD_W::new(self)
    }
    #[doc = "Bit 3 - Periphery power for RAM interface 3."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if3_ppd(&mut self) -> SRAM_IF3_PPD_W<3> {
        SRAM_IF3_PPD_W::new(self)
    }
    #[doc = "Bit 4 - Periphery power for RAM interface 4."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if4_ppd(&mut self) -> SRAM_IF4_PPD_W<4> {
        SRAM_IF4_PPD_W::new(self)
    }
    #[doc = "Bit 5 - Periphery power for RAM interface 5."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if5_ppd(&mut self) -> SRAM_IF5_PPD_W<5> {
        SRAM_IF5_PPD_W::new(self)
    }
    #[doc = "Bit 6 - Periphery power for RAM interface 6."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if6_ppd(&mut self) -> SRAM_IF6_PPD_W<6> {
        SRAM_IF6_PPD_W::new(self)
    }
    #[doc = "Bit 7 - Periphery power for RAM interface 7."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if7_ppd(&mut self) -> SRAM_IF7_PPD_W<7> {
        SRAM_IF7_PPD_W::new(self)
    }
    #[doc = "Bit 8 - Periphery power for RAM interface 8."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if8_ppd(&mut self) -> SRAM_IF8_PPD_W<8> {
        SRAM_IF8_PPD_W::new(self)
    }
    #[doc = "Bit 9 - Periphery power for RAM interface 9."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if9_ppd(&mut self) -> SRAM_IF9_PPD_W<9> {
        SRAM_IF9_PPD_W::new(self)
    }
    #[doc = "Bit 10 - Periphery power for RAM interface 10."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if10_ppd(&mut self) -> SRAM_IF10_PPD_W<10> {
        SRAM_IF10_PPD_W::new(self)
    }
    #[doc = "Bit 11 - Periphery power for RAM interface 11."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if11_ppd(&mut self) -> SRAM_IF11_PPD_W<11> {
        SRAM_IF11_PPD_W::new(self)
    }
    #[doc = "Bit 12 - Periphery power for RAM interface 12."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if12_ppd(&mut self) -> SRAM_IF12_PPD_W<12> {
        SRAM_IF12_PPD_W::new(self)
    }
    #[doc = "Bit 13 - Periphery power for RAM interface 13."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if13_ppd(&mut self) -> SRAM_IF13_PPD_W<13> {
        SRAM_IF13_PPD_W::new(self)
    }
    #[doc = "Bit 14 - Periphery power for RAM interface 14."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if14_ppd(&mut self) -> SRAM_IF14_PPD_W<14> {
        SRAM_IF14_PPD_W::new(self)
    }
    #[doc = "Bit 15 - Periphery power for RAM interface 15."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if15_ppd(&mut self) -> SRAM_IF15_PPD_W<15> {
        SRAM_IF15_PPD_W::new(self)
    }
    #[doc = "Bit 16 - Periphery power for RAM interface 16."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if16_ppd(&mut self) -> SRAM_IF16_PPD_W<16> {
        SRAM_IF16_PPD_W::new(self)
    }
    #[doc = "Bit 17 - Periphery power for RAM interface 17."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if17_ppd(&mut self) -> SRAM_IF17_PPD_W<17> {
        SRAM_IF17_PPD_W::new(self)
    }
    #[doc = "Bit 18 - Periphery power for RAM interface 18."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if18_ppd(&mut self) -> SRAM_IF18_PPD_W<18> {
        SRAM_IF18_PPD_W::new(self)
    }
    #[doc = "Bit 19 - Periphery power for RAM interface 19."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if19_ppd(&mut self) -> SRAM_IF19_PPD_W<19> {
        SRAM_IF19_PPD_W::new(self)
    }
    #[doc = "Bit 20 - Periphery power for RAM interface 20."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if20_ppd(&mut self) -> SRAM_IF20_PPD_W<20> {
        SRAM_IF20_PPD_W::new(self)
    }
    #[doc = "Bit 21 - Periphery power for RAM interface 21."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if21_ppd(&mut self) -> SRAM_IF21_PPD_W<21> {
        SRAM_IF21_PPD_W::new(self)
    }
    #[doc = "Bit 22 - Periphery power for RAM interface 22."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if22_ppd(&mut self) -> SRAM_IF22_PPD_W<22> {
        SRAM_IF22_PPD_W::new(self)
    }
    #[doc = "Bit 23 - Periphery power for RAM interface 23."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if23_ppd(&mut self) -> SRAM_IF23_PPD_W<23> {
        SRAM_IF23_PPD_W::new(self)
    }
    #[doc = "Bit 24 - Periphery power for RAM interface 24."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if24_ppd(&mut self) -> SRAM_IF24_PPD_W<24> {
        SRAM_IF24_PPD_W::new(self)
    }
    #[doc = "Bit 25 - Periphery power for RAM interface 25."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if25_ppd(&mut self) -> SRAM_IF25_PPD_W<25> {
        SRAM_IF25_PPD_W::new(self)
    }
    #[doc = "Bit 26 - Periphery power for RAM interface 26."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if26_ppd(&mut self) -> SRAM_IF26_PPD_W<26> {
        SRAM_IF26_PPD_W::new(self)
    }
    #[doc = "Bit 27 - Periphery power for RAM interface 27."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if27_ppd(&mut self) -> SRAM_IF27_PPD_W<27> {
        SRAM_IF27_PPD_W::new(self)
    }
    #[doc = "Bit 28 - Periphery power for RAM interface 28."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if28_ppd(&mut self) -> SRAM_IF28_PPD_W<28> {
        SRAM_IF28_PPD_W::new(self)
    }
    #[doc = "Bit 29 - Periphery power for RAM interface 29."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if29_ppd(&mut self) -> SRAM_IF29_PPD_W<29> {
        SRAM_IF29_PPD_W::new(self)
    }
    #[doc = "Bit 30 - Periphery power for RAM interface 30."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if30_ppd(&mut self) -> SRAM_IF30_PPD_W<30> {
        SRAM_IF30_PPD_W::new(self)
    }
    #[doc = "Bit 31 - Periphery power for RAM interface 31."]
    #[inline(always)]
    #[must_use]
    pub fn sram_if31_ppd(&mut self) -> SRAM_IF31_PPD_W<31> {
        SRAM_IF31_PPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Run configuration 3 set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdruncfg3_set](index.html) module"]
pub struct PDRUNCFG3_SET_SPEC;
impl crate::RegisterSpec for PDRUNCFG3_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pdruncfg3_set::W](W) writer structure"]
impl crate::Writable for PDRUNCFG3_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDRUNCFG3_SET to value 0"]
impl crate::Resettable for PDRUNCFG3_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
