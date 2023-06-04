#[doc = "Register `ABORT0` writer"]
pub struct W(crate::W<ABORT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ABORT0_SPEC>;
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
impl From<crate::W<ABORT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ABORT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT0_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT0_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT0` writer - Abort control for DMA channel."]
pub type ABORT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT0_AW, O>;
impl<'a, const O: u8> ABORT0_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT0_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT0_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT1_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT1_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT1` writer - Abort control for DMA channel."]
pub type ABORT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT1_AW, O>;
impl<'a, const O: u8> ABORT1_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT1_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT1_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT2_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT2_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT2` writer - Abort control for DMA channel."]
pub type ABORT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT2_AW, O>;
impl<'a, const O: u8> ABORT2_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT2_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT2_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT3_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT3_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT3` writer - Abort control for DMA channel."]
pub type ABORT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT3_AW, O>;
impl<'a, const O: u8> ABORT3_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT3_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT3_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT4_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT4_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT4` writer - Abort control for DMA channel."]
pub type ABORT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT4_AW, O>;
impl<'a, const O: u8> ABORT4_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT4_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT4_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT5_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT5_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT5` writer - Abort control for DMA channel."]
pub type ABORT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT5_AW, O>;
impl<'a, const O: u8> ABORT5_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT5_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT5_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT6_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT6_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT6` writer - Abort control for DMA channel."]
pub type ABORT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT6_AW, O>;
impl<'a, const O: u8> ABORT6_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT6_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT6_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT7_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT7_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT7` writer - Abort control for DMA channel."]
pub type ABORT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT7_AW, O>;
impl<'a, const O: u8> ABORT7_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT7_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT7_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT8_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT8_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT8` writer - Abort control for DMA channel."]
pub type ABORT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT8_AW, O>;
impl<'a, const O: u8> ABORT8_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT8_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT8_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT9_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT9_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT9` writer - Abort control for DMA channel."]
pub type ABORT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT9_AW, O>;
impl<'a, const O: u8> ABORT9_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT9_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT9_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT10_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT10_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT10` writer - Abort control for DMA channel."]
pub type ABORT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT10_AW, O>;
impl<'a, const O: u8> ABORT10_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT10_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT10_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT11_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT11_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT11` writer - Abort control for DMA channel."]
pub type ABORT11_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT11_AW, O>;
impl<'a, const O: u8> ABORT11_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT11_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT11_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT12_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT12_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT12` writer - Abort control for DMA channel."]
pub type ABORT12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT12_AW, O>;
impl<'a, const O: u8> ABORT12_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT12_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT12_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT13_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT13_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT13` writer - Abort control for DMA channel."]
pub type ABORT13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT13_AW, O>;
impl<'a, const O: u8> ABORT13_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT13_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT13_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT14_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT14_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT14` writer - Abort control for DMA channel."]
pub type ABORT14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT14_AW, O>;
impl<'a, const O: u8> ABORT14_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT14_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT14_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT15_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT15_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT15_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT15` writer - Abort control for DMA channel."]
pub type ABORT15_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT15_AW, O>;
impl<'a, const O: u8> ABORT15_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT15_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT15_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT16_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT16_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT16_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT16` writer - Abort control for DMA channel."]
pub type ABORT16_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT16_AW, O>;
impl<'a, const O: u8> ABORT16_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT16_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT16_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT17_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT17_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT17_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT17` writer - Abort control for DMA channel."]
pub type ABORT17_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT17_AW, O>;
impl<'a, const O: u8> ABORT17_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT17_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT17_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT18_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT18_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT18_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT18` writer - Abort control for DMA channel."]
pub type ABORT18_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT18_AW, O>;
impl<'a, const O: u8> ABORT18_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT18_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT18_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT19_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT19_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT19_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT19` writer - Abort control for DMA channel."]
pub type ABORT19_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT19_AW, O>;
impl<'a, const O: u8> ABORT19_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT19_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT19_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT20_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT20_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT20_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT20` writer - Abort control for DMA channel."]
pub type ABORT20_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT20_AW, O>;
impl<'a, const O: u8> ABORT20_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT20_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT20_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT21_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT21_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT21_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT21` writer - Abort control for DMA channel."]
pub type ABORT21_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT21_AW, O>;
impl<'a, const O: u8> ABORT21_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT21_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT21_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT22_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT22_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT22_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT22` writer - Abort control for DMA channel."]
pub type ABORT22_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT22_AW, O>;
impl<'a, const O: u8> ABORT22_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT22_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT22_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT23_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT23_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT23_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT23` writer - Abort control for DMA channel."]
pub type ABORT23_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT23_AW, O>;
impl<'a, const O: u8> ABORT23_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT23_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT23_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT24_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT24_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT24_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT24` writer - Abort control for DMA channel."]
pub type ABORT24_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT24_AW, O>;
impl<'a, const O: u8> ABORT24_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT24_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT24_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT25_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT25_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT25_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT25` writer - Abort control for DMA channel."]
pub type ABORT25_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT25_AW, O>;
impl<'a, const O: u8> ABORT25_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT25_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT25_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT26_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT26_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT26_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT26` writer - Abort control for DMA channel."]
pub type ABORT26_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT26_AW, O>;
impl<'a, const O: u8> ABORT26_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT26_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT26_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT27_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT27_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT27_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT27` writer - Abort control for DMA channel."]
pub type ABORT27_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT27_AW, O>;
impl<'a, const O: u8> ABORT27_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT27_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT27_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT28_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT28_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT28_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT28` writer - Abort control for DMA channel."]
pub type ABORT28_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT28_AW, O>;
impl<'a, const O: u8> ABORT28_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT28_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT28_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT29_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT29_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT29_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT29` writer - Abort control for DMA channel."]
pub type ABORT29_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT29_AW, O>;
impl<'a, const O: u8> ABORT29_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT29_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT29_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT30_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT30_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT30_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT30` writer - Abort control for DMA channel."]
pub type ABORT30_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT30_AW, O>;
impl<'a, const O: u8> ABORT30_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT30_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT30_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT31_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT31_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT31_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT31` writer - Abort control for DMA channel."]
pub type ABORT31_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT0_SPEC, ABORT31_AW, O>;
impl<'a, const O: u8> ABORT31_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT31_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT31_AW::EFFECT)
    }
}
impl W {
    #[doc = "Bit 0 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort0(&mut self) -> ABORT0_W<0> {
        ABORT0_W::new(self)
    }
    #[doc = "Bit 1 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort1(&mut self) -> ABORT1_W<1> {
        ABORT1_W::new(self)
    }
    #[doc = "Bit 2 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort2(&mut self) -> ABORT2_W<2> {
        ABORT2_W::new(self)
    }
    #[doc = "Bit 3 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort3(&mut self) -> ABORT3_W<3> {
        ABORT3_W::new(self)
    }
    #[doc = "Bit 4 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort4(&mut self) -> ABORT4_W<4> {
        ABORT4_W::new(self)
    }
    #[doc = "Bit 5 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort5(&mut self) -> ABORT5_W<5> {
        ABORT5_W::new(self)
    }
    #[doc = "Bit 6 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort6(&mut self) -> ABORT6_W<6> {
        ABORT6_W::new(self)
    }
    #[doc = "Bit 7 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort7(&mut self) -> ABORT7_W<7> {
        ABORT7_W::new(self)
    }
    #[doc = "Bit 8 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort8(&mut self) -> ABORT8_W<8> {
        ABORT8_W::new(self)
    }
    #[doc = "Bit 9 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort9(&mut self) -> ABORT9_W<9> {
        ABORT9_W::new(self)
    }
    #[doc = "Bit 10 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort10(&mut self) -> ABORT10_W<10> {
        ABORT10_W::new(self)
    }
    #[doc = "Bit 11 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort11(&mut self) -> ABORT11_W<11> {
        ABORT11_W::new(self)
    }
    #[doc = "Bit 12 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort12(&mut self) -> ABORT12_W<12> {
        ABORT12_W::new(self)
    }
    #[doc = "Bit 13 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort13(&mut self) -> ABORT13_W<13> {
        ABORT13_W::new(self)
    }
    #[doc = "Bit 14 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort14(&mut self) -> ABORT14_W<14> {
        ABORT14_W::new(self)
    }
    #[doc = "Bit 15 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort15(&mut self) -> ABORT15_W<15> {
        ABORT15_W::new(self)
    }
    #[doc = "Bit 16 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort16(&mut self) -> ABORT16_W<16> {
        ABORT16_W::new(self)
    }
    #[doc = "Bit 17 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort17(&mut self) -> ABORT17_W<17> {
        ABORT17_W::new(self)
    }
    #[doc = "Bit 18 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort18(&mut self) -> ABORT18_W<18> {
        ABORT18_W::new(self)
    }
    #[doc = "Bit 19 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort19(&mut self) -> ABORT19_W<19> {
        ABORT19_W::new(self)
    }
    #[doc = "Bit 20 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort20(&mut self) -> ABORT20_W<20> {
        ABORT20_W::new(self)
    }
    #[doc = "Bit 21 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort21(&mut self) -> ABORT21_W<21> {
        ABORT21_W::new(self)
    }
    #[doc = "Bit 22 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort22(&mut self) -> ABORT22_W<22> {
        ABORT22_W::new(self)
    }
    #[doc = "Bit 23 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort23(&mut self) -> ABORT23_W<23> {
        ABORT23_W::new(self)
    }
    #[doc = "Bit 24 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort24(&mut self) -> ABORT24_W<24> {
        ABORT24_W::new(self)
    }
    #[doc = "Bit 25 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort25(&mut self) -> ABORT25_W<25> {
        ABORT25_W::new(self)
    }
    #[doc = "Bit 26 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort26(&mut self) -> ABORT26_W<26> {
        ABORT26_W::new(self)
    }
    #[doc = "Bit 27 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort27(&mut self) -> ABORT27_W<27> {
        ABORT27_W::new(self)
    }
    #[doc = "Bit 28 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort28(&mut self) -> ABORT28_W<28> {
        ABORT28_W::new(self)
    }
    #[doc = "Bit 29 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort29(&mut self) -> ABORT29_W<29> {
        ABORT29_W::new(self)
    }
    #[doc = "Bit 30 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort30(&mut self) -> ABORT30_W<30> {
        ABORT30_W::new(self)
    }
    #[doc = "Bit 31 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort31(&mut self) -> ABORT31_W<31> {
        ABORT31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Abort control for all DMA channels\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [abort0](index.html) module"]
pub struct ABORT0_SPEC;
impl crate::RegisterSpec for ABORT0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [abort0::W](W) writer structure"]
impl crate::Writable for ABORT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ABORT0 to value 0"]
impl crate::Resettable for ABORT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
