#[doc = "Register `SETVALID0` writer"]
pub struct W(crate::W<SETVALID0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETVALID0_SPEC>;
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
impl From<crate::W<SETVALID0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETVALID0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID0_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID0_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID0` writer - SetValid control for DMA channel."]
pub type SETVALID0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID0_AW, O>;
impl<'a, const O: u8> SETVALID0_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID0_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID0_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID1_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID1_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID1` writer - SetValid control for DMA channel."]
pub type SETVALID1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID1_AW, O>;
impl<'a, const O: u8> SETVALID1_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID1_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID1_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID2_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID2_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID2` writer - SetValid control for DMA channel."]
pub type SETVALID2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID2_AW, O>;
impl<'a, const O: u8> SETVALID2_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID2_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID2_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID3_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID3_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID3` writer - SetValid control for DMA channel."]
pub type SETVALID3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID3_AW, O>;
impl<'a, const O: u8> SETVALID3_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID3_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID3_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID4_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID4_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID4` writer - SetValid control for DMA channel."]
pub type SETVALID4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID4_AW, O>;
impl<'a, const O: u8> SETVALID4_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID4_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID4_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID5_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID5_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID5` writer - SetValid control for DMA channel."]
pub type SETVALID5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID5_AW, O>;
impl<'a, const O: u8> SETVALID5_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID5_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID5_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID6_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID6_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID6` writer - SetValid control for DMA channel."]
pub type SETVALID6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID6_AW, O>;
impl<'a, const O: u8> SETVALID6_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID6_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID6_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID7_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID7_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID7` writer - SetValid control for DMA channel."]
pub type SETVALID7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID7_AW, O>;
impl<'a, const O: u8> SETVALID7_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID7_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID7_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID8_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID8_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID8` writer - SetValid control for DMA channel."]
pub type SETVALID8_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID8_AW, O>;
impl<'a, const O: u8> SETVALID8_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID8_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID8_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID9_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID9_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID9` writer - SetValid control for DMA channel."]
pub type SETVALID9_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID9_AW, O>;
impl<'a, const O: u8> SETVALID9_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID9_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID9_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID10_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID10_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID10` writer - SetValid control for DMA channel."]
pub type SETVALID10_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID10_AW, O>;
impl<'a, const O: u8> SETVALID10_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID10_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID10_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID11_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID11_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID11` writer - SetValid control for DMA channel."]
pub type SETVALID11_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID11_AW, O>;
impl<'a, const O: u8> SETVALID11_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID11_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID11_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID12_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID12_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID12` writer - SetValid control for DMA channel."]
pub type SETVALID12_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID12_AW, O>;
impl<'a, const O: u8> SETVALID12_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID12_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID12_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID13_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID13_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID13` writer - SetValid control for DMA channel."]
pub type SETVALID13_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID13_AW, O>;
impl<'a, const O: u8> SETVALID13_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID13_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID13_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID14_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID14_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID14` writer - SetValid control for DMA channel."]
pub type SETVALID14_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID14_AW, O>;
impl<'a, const O: u8> SETVALID14_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID14_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID14_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID15_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID15_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID15_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID15` writer - SetValid control for DMA channel."]
pub type SETVALID15_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID15_AW, O>;
impl<'a, const O: u8> SETVALID15_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID15_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID15_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID16_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID16_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID16_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID16` writer - SetValid control for DMA channel."]
pub type SETVALID16_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID16_AW, O>;
impl<'a, const O: u8> SETVALID16_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID16_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID16_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID17_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID17_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID17_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID17` writer - SetValid control for DMA channel."]
pub type SETVALID17_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID17_AW, O>;
impl<'a, const O: u8> SETVALID17_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID17_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID17_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID18_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID18_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID18_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID18` writer - SetValid control for DMA channel."]
pub type SETVALID18_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID18_AW, O>;
impl<'a, const O: u8> SETVALID18_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID18_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID18_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID19_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID19_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID19_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID19` writer - SetValid control for DMA channel."]
pub type SETVALID19_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID19_AW, O>;
impl<'a, const O: u8> SETVALID19_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID19_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID19_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID20_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID20_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID20_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID20` writer - SetValid control for DMA channel."]
pub type SETVALID20_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID20_AW, O>;
impl<'a, const O: u8> SETVALID20_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID20_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID20_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID21_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID21_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID21_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID21` writer - SetValid control for DMA channel."]
pub type SETVALID21_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID21_AW, O>;
impl<'a, const O: u8> SETVALID21_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID21_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID21_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID22_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID22_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID22_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID22` writer - SetValid control for DMA channel."]
pub type SETVALID22_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID22_AW, O>;
impl<'a, const O: u8> SETVALID22_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID22_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID22_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID23_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID23_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID23_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID23` writer - SetValid control for DMA channel."]
pub type SETVALID23_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID23_AW, O>;
impl<'a, const O: u8> SETVALID23_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID23_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID23_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID24_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID24_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID24_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID24` writer - SetValid control for DMA channel."]
pub type SETVALID24_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID24_AW, O>;
impl<'a, const O: u8> SETVALID24_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID24_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID24_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID25_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID25_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID25_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID25` writer - SetValid control for DMA channel."]
pub type SETVALID25_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID25_AW, O>;
impl<'a, const O: u8> SETVALID25_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID25_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID25_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID26_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID26_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID26_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID26` writer - SetValid control for DMA channel."]
pub type SETVALID26_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID26_AW, O>;
impl<'a, const O: u8> SETVALID26_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID26_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID26_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID27_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID27_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID27_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID27` writer - SetValid control for DMA channel."]
pub type SETVALID27_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID27_AW, O>;
impl<'a, const O: u8> SETVALID27_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID27_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID27_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID28_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID28_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID28_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID28` writer - SetValid control for DMA channel."]
pub type SETVALID28_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID28_AW, O>;
impl<'a, const O: u8> SETVALID28_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID28_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID28_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID29_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID29_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID29_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID29` writer - SetValid control for DMA channel."]
pub type SETVALID29_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID29_AW, O>;
impl<'a, const O: u8> SETVALID29_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID29_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID29_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID30_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID30_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID30_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID30` writer - SetValid control for DMA channel."]
pub type SETVALID30_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID30_AW, O>;
impl<'a, const O: u8> SETVALID30_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID30_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID30_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID31_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID31_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID31_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID31` writer - SetValid control for DMA channel."]
pub type SETVALID31_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID0_SPEC, SETVALID31_AW, O>;
impl<'a, const O: u8> SETVALID31_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID31_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID31_AW::EFFECT)
    }
}
impl W {
    #[doc = "Bit 0 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid0(&mut self) -> SETVALID0_W<0> {
        SETVALID0_W::new(self)
    }
    #[doc = "Bit 1 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid1(&mut self) -> SETVALID1_W<1> {
        SETVALID1_W::new(self)
    }
    #[doc = "Bit 2 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid2(&mut self) -> SETVALID2_W<2> {
        SETVALID2_W::new(self)
    }
    #[doc = "Bit 3 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid3(&mut self) -> SETVALID3_W<3> {
        SETVALID3_W::new(self)
    }
    #[doc = "Bit 4 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid4(&mut self) -> SETVALID4_W<4> {
        SETVALID4_W::new(self)
    }
    #[doc = "Bit 5 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid5(&mut self) -> SETVALID5_W<5> {
        SETVALID5_W::new(self)
    }
    #[doc = "Bit 6 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid6(&mut self) -> SETVALID6_W<6> {
        SETVALID6_W::new(self)
    }
    #[doc = "Bit 7 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid7(&mut self) -> SETVALID7_W<7> {
        SETVALID7_W::new(self)
    }
    #[doc = "Bit 8 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid8(&mut self) -> SETVALID8_W<8> {
        SETVALID8_W::new(self)
    }
    #[doc = "Bit 9 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid9(&mut self) -> SETVALID9_W<9> {
        SETVALID9_W::new(self)
    }
    #[doc = "Bit 10 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid10(&mut self) -> SETVALID10_W<10> {
        SETVALID10_W::new(self)
    }
    #[doc = "Bit 11 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid11(&mut self) -> SETVALID11_W<11> {
        SETVALID11_W::new(self)
    }
    #[doc = "Bit 12 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid12(&mut self) -> SETVALID12_W<12> {
        SETVALID12_W::new(self)
    }
    #[doc = "Bit 13 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid13(&mut self) -> SETVALID13_W<13> {
        SETVALID13_W::new(self)
    }
    #[doc = "Bit 14 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid14(&mut self) -> SETVALID14_W<14> {
        SETVALID14_W::new(self)
    }
    #[doc = "Bit 15 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid15(&mut self) -> SETVALID15_W<15> {
        SETVALID15_W::new(self)
    }
    #[doc = "Bit 16 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid16(&mut self) -> SETVALID16_W<16> {
        SETVALID16_W::new(self)
    }
    #[doc = "Bit 17 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid17(&mut self) -> SETVALID17_W<17> {
        SETVALID17_W::new(self)
    }
    #[doc = "Bit 18 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid18(&mut self) -> SETVALID18_W<18> {
        SETVALID18_W::new(self)
    }
    #[doc = "Bit 19 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid19(&mut self) -> SETVALID19_W<19> {
        SETVALID19_W::new(self)
    }
    #[doc = "Bit 20 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid20(&mut self) -> SETVALID20_W<20> {
        SETVALID20_W::new(self)
    }
    #[doc = "Bit 21 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid21(&mut self) -> SETVALID21_W<21> {
        SETVALID21_W::new(self)
    }
    #[doc = "Bit 22 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid22(&mut self) -> SETVALID22_W<22> {
        SETVALID22_W::new(self)
    }
    #[doc = "Bit 23 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid23(&mut self) -> SETVALID23_W<23> {
        SETVALID23_W::new(self)
    }
    #[doc = "Bit 24 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid24(&mut self) -> SETVALID24_W<24> {
        SETVALID24_W::new(self)
    }
    #[doc = "Bit 25 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid25(&mut self) -> SETVALID25_W<25> {
        SETVALID25_W::new(self)
    }
    #[doc = "Bit 26 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid26(&mut self) -> SETVALID26_W<26> {
        SETVALID26_W::new(self)
    }
    #[doc = "Bit 27 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid27(&mut self) -> SETVALID27_W<27> {
        SETVALID27_W::new(self)
    }
    #[doc = "Bit 28 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid28(&mut self) -> SETVALID28_W<28> {
        SETVALID28_W::new(self)
    }
    #[doc = "Bit 29 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid29(&mut self) -> SETVALID29_W<29> {
        SETVALID29_W::new(self)
    }
    #[doc = "Bit 30 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid30(&mut self) -> SETVALID30_W<30> {
        SETVALID30_W::new(self)
    }
    #[doc = "Bit 31 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid31(&mut self) -> SETVALID31_W<31> {
        SETVALID31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set ValidPending control bits for all DMA channels\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setvalid0](index.html) module"]
pub struct SETVALID0_SPEC;
impl crate::RegisterSpec for SETVALID0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [setvalid0::W](W) writer structure"]
impl crate::Writable for SETVALID0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SETVALID0 to value 0"]
impl crate::Resettable for SETVALID0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
