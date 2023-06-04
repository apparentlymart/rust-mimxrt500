#[doc = "Register `SETTRIG0` writer"]
pub struct W(crate::W<SETTRIG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETTRIG0_SPEC>;
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
impl From<crate::W<SETTRIG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETTRIG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG0_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG0_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG0` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG0_AW, O>;
impl<'a, const O: u8> SETTRIG0_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG0_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG0_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG1_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG1_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG1` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG1_AW, O>;
impl<'a, const O: u8> SETTRIG1_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG1_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG1_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG2_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG2_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG2` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG2_AW, O>;
impl<'a, const O: u8> SETTRIG2_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG2_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG2_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG3_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG3_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG3` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG3_AW, O>;
impl<'a, const O: u8> SETTRIG3_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG3_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG3_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG4_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG4_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG4` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG4_AW, O>;
impl<'a, const O: u8> SETTRIG4_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG4_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG4_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG5_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG5_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG5` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG5_AW, O>;
impl<'a, const O: u8> SETTRIG5_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG5_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG5_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG6_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG6_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG6` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG6_AW, O>;
impl<'a, const O: u8> SETTRIG6_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG6_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG6_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG7_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG7_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG7` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG7_AW, O>;
impl<'a, const O: u8> SETTRIG7_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG7_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG7_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG8_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG8_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG8` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG8_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG8_AW, O>;
impl<'a, const O: u8> SETTRIG8_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG8_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG8_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG9_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG9_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG9` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG9_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG9_AW, O>;
impl<'a, const O: u8> SETTRIG9_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG9_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG9_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG10_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG10_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG10` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG10_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG10_AW, O>;
impl<'a, const O: u8> SETTRIG10_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG10_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG10_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG11_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG11_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG11` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG11_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG11_AW, O>;
impl<'a, const O: u8> SETTRIG11_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG11_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG11_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG12_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG12_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG12` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG12_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG12_AW, O>;
impl<'a, const O: u8> SETTRIG12_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG12_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG12_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG13_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG13_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG13` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG13_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG13_AW, O>;
impl<'a, const O: u8> SETTRIG13_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG13_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG13_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG14_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG14_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG14` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG14_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG14_AW, O>;
impl<'a, const O: u8> SETTRIG14_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG14_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG14_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG15_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG15_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG15_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG15` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG15_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG15_AW, O>;
impl<'a, const O: u8> SETTRIG15_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG15_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG15_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG16_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG16_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG16_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG16` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG16_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG16_AW, O>;
impl<'a, const O: u8> SETTRIG16_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG16_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG16_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG17_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG17_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG17_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG17` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG17_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG17_AW, O>;
impl<'a, const O: u8> SETTRIG17_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG17_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG17_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG18_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG18_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG18_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG18` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG18_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG18_AW, O>;
impl<'a, const O: u8> SETTRIG18_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG18_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG18_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG19_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG19_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG19_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG19` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG19_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG19_AW, O>;
impl<'a, const O: u8> SETTRIG19_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG19_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG19_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG20_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG20_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG20_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG20` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG20_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG20_AW, O>;
impl<'a, const O: u8> SETTRIG20_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG20_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG20_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG21_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG21_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG21_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG21` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG21_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG21_AW, O>;
impl<'a, const O: u8> SETTRIG21_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG21_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG21_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG22_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG22_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG22_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG22` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG22_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG22_AW, O>;
impl<'a, const O: u8> SETTRIG22_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG22_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG22_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG23_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG23_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG23_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG23` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG23_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG23_AW, O>;
impl<'a, const O: u8> SETTRIG23_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG23_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG23_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG24_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG24_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG24_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG24` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG24_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG24_AW, O>;
impl<'a, const O: u8> SETTRIG24_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG24_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG24_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG25_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG25_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG25_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG25` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG25_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG25_AW, O>;
impl<'a, const O: u8> SETTRIG25_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG25_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG25_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG26_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG26_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG26_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG26` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG26_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG26_AW, O>;
impl<'a, const O: u8> SETTRIG26_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG26_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG26_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG27_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG27_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG27_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG27` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG27_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG27_AW, O>;
impl<'a, const O: u8> SETTRIG27_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG27_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG27_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG28_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG28_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG28_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG28` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG28_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG28_AW, O>;
impl<'a, const O: u8> SETTRIG28_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG28_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG28_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG29_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG29_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG29_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG29` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG29_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG29_AW, O>;
impl<'a, const O: u8> SETTRIG29_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG29_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG29_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG30_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG30_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG30_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG30` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG30_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG30_AW, O>;
impl<'a, const O: u8> SETTRIG30_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG30_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG30_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG31_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG31_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG31_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG31` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG31_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG0_SPEC, SETTRIG31_AW, O>;
impl<'a, const O: u8> SETTRIG31_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG31_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG31_AW::EFFECT)
    }
}
impl W {
    #[doc = "Bit 0 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig0(&mut self) -> SETTRIG0_W<0> {
        SETTRIG0_W::new(self)
    }
    #[doc = "Bit 1 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig1(&mut self) -> SETTRIG1_W<1> {
        SETTRIG1_W::new(self)
    }
    #[doc = "Bit 2 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig2(&mut self) -> SETTRIG2_W<2> {
        SETTRIG2_W::new(self)
    }
    #[doc = "Bit 3 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig3(&mut self) -> SETTRIG3_W<3> {
        SETTRIG3_W::new(self)
    }
    #[doc = "Bit 4 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig4(&mut self) -> SETTRIG4_W<4> {
        SETTRIG4_W::new(self)
    }
    #[doc = "Bit 5 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig5(&mut self) -> SETTRIG5_W<5> {
        SETTRIG5_W::new(self)
    }
    #[doc = "Bit 6 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig6(&mut self) -> SETTRIG6_W<6> {
        SETTRIG6_W::new(self)
    }
    #[doc = "Bit 7 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig7(&mut self) -> SETTRIG7_W<7> {
        SETTRIG7_W::new(self)
    }
    #[doc = "Bit 8 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig8(&mut self) -> SETTRIG8_W<8> {
        SETTRIG8_W::new(self)
    }
    #[doc = "Bit 9 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig9(&mut self) -> SETTRIG9_W<9> {
        SETTRIG9_W::new(self)
    }
    #[doc = "Bit 10 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig10(&mut self) -> SETTRIG10_W<10> {
        SETTRIG10_W::new(self)
    }
    #[doc = "Bit 11 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig11(&mut self) -> SETTRIG11_W<11> {
        SETTRIG11_W::new(self)
    }
    #[doc = "Bit 12 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig12(&mut self) -> SETTRIG12_W<12> {
        SETTRIG12_W::new(self)
    }
    #[doc = "Bit 13 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig13(&mut self) -> SETTRIG13_W<13> {
        SETTRIG13_W::new(self)
    }
    #[doc = "Bit 14 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig14(&mut self) -> SETTRIG14_W<14> {
        SETTRIG14_W::new(self)
    }
    #[doc = "Bit 15 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig15(&mut self) -> SETTRIG15_W<15> {
        SETTRIG15_W::new(self)
    }
    #[doc = "Bit 16 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig16(&mut self) -> SETTRIG16_W<16> {
        SETTRIG16_W::new(self)
    }
    #[doc = "Bit 17 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig17(&mut self) -> SETTRIG17_W<17> {
        SETTRIG17_W::new(self)
    }
    #[doc = "Bit 18 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig18(&mut self) -> SETTRIG18_W<18> {
        SETTRIG18_W::new(self)
    }
    #[doc = "Bit 19 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig19(&mut self) -> SETTRIG19_W<19> {
        SETTRIG19_W::new(self)
    }
    #[doc = "Bit 20 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig20(&mut self) -> SETTRIG20_W<20> {
        SETTRIG20_W::new(self)
    }
    #[doc = "Bit 21 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig21(&mut self) -> SETTRIG21_W<21> {
        SETTRIG21_W::new(self)
    }
    #[doc = "Bit 22 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig22(&mut self) -> SETTRIG22_W<22> {
        SETTRIG22_W::new(self)
    }
    #[doc = "Bit 23 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig23(&mut self) -> SETTRIG23_W<23> {
        SETTRIG23_W::new(self)
    }
    #[doc = "Bit 24 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig24(&mut self) -> SETTRIG24_W<24> {
        SETTRIG24_W::new(self)
    }
    #[doc = "Bit 25 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig25(&mut self) -> SETTRIG25_W<25> {
        SETTRIG25_W::new(self)
    }
    #[doc = "Bit 26 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig26(&mut self) -> SETTRIG26_W<26> {
        SETTRIG26_W::new(self)
    }
    #[doc = "Bit 27 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig27(&mut self) -> SETTRIG27_W<27> {
        SETTRIG27_W::new(self)
    }
    #[doc = "Bit 28 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig28(&mut self) -> SETTRIG28_W<28> {
        SETTRIG28_W::new(self)
    }
    #[doc = "Bit 29 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig29(&mut self) -> SETTRIG29_W<29> {
        SETTRIG29_W::new(self)
    }
    #[doc = "Bit 30 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig30(&mut self) -> SETTRIG30_W<30> {
        SETTRIG30_W::new(self)
    }
    #[doc = "Bit 31 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig31(&mut self) -> SETTRIG31_W<31> {
        SETTRIG31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set Trigger control bits for all DMA channels\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [settrig0](index.html) module"]
pub struct SETTRIG0_SPEC;
impl crate::RegisterSpec for SETTRIG0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [settrig0::W](W) writer structure"]
impl crate::Writable for SETTRIG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SETTRIG0 to value 0"]
impl crate::Resettable for SETTRIG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
