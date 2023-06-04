#[doc = "Register `DIRSET[%s]` writer"]
pub struct W(crate::W<DIRSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIRSET_SPEC>;
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
impl From<crate::W<DIRSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIRSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP0_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP0_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP0` writer - Direction set bits for Port pins"]
pub type DIRSETP0_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP0_AW, O>;
impl<'a, const O: u8> DIRSETP0_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP0_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP0_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP1_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP1_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP1` writer - Direction set bits for Port pins"]
pub type DIRSETP1_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP1_AW, O>;
impl<'a, const O: u8> DIRSETP1_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP1_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP1_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP2_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP2_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP2` writer - Direction set bits for Port pins"]
pub type DIRSETP2_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP2_AW, O>;
impl<'a, const O: u8> DIRSETP2_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP2_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP2_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP3_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP3_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP3` writer - Direction set bits for Port pins"]
pub type DIRSETP3_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP3_AW, O>;
impl<'a, const O: u8> DIRSETP3_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP3_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP3_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP4_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP4_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP4` writer - Direction set bits for Port pins"]
pub type DIRSETP4_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP4_AW, O>;
impl<'a, const O: u8> DIRSETP4_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP4_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP4_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP5_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP5_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP5` writer - Direction set bits for Port pins"]
pub type DIRSETP5_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP5_AW, O>;
impl<'a, const O: u8> DIRSETP5_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP5_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP5_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP6_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP6_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP6` writer - Direction set bits for Port pins"]
pub type DIRSETP6_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP6_AW, O>;
impl<'a, const O: u8> DIRSETP6_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP6_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP6_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP7_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP7_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP7` writer - Direction set bits for Port pins"]
pub type DIRSETP7_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP7_AW, O>;
impl<'a, const O: u8> DIRSETP7_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP7_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP7_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP8_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP8_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP8` writer - Direction set bits for Port pins"]
pub type DIRSETP8_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP8_AW, O>;
impl<'a, const O: u8> DIRSETP8_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP8_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP8_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP9_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP9_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP9` writer - Direction set bits for Port pins"]
pub type DIRSETP9_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP9_AW, O>;
impl<'a, const O: u8> DIRSETP9_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP9_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP9_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP10_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP10_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP10` writer - Direction set bits for Port pins"]
pub type DIRSETP10_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP10_AW, O>;
impl<'a, const O: u8> DIRSETP10_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP10_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP10_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP11_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP11_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP11` writer - Direction set bits for Port pins"]
pub type DIRSETP11_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP11_AW, O>;
impl<'a, const O: u8> DIRSETP11_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP11_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP11_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP12_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP12_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP12` writer - Direction set bits for Port pins"]
pub type DIRSETP12_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP12_AW, O>;
impl<'a, const O: u8> DIRSETP12_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP12_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP12_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP13_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP13_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP13` writer - Direction set bits for Port pins"]
pub type DIRSETP13_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP13_AW, O>;
impl<'a, const O: u8> DIRSETP13_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP13_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP13_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP14_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP14_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP14` writer - Direction set bits for Port pins"]
pub type DIRSETP14_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP14_AW, O>;
impl<'a, const O: u8> DIRSETP14_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP14_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP14_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP15_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP15_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP15_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP15` writer - Direction set bits for Port pins"]
pub type DIRSETP15_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP15_AW, O>;
impl<'a, const O: u8> DIRSETP15_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP15_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP15_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP16_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP16_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP16_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP16` writer - Direction set bits for Port pins"]
pub type DIRSETP16_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP16_AW, O>;
impl<'a, const O: u8> DIRSETP16_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP16_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP16_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP17_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP17_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP17_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP17` writer - Direction set bits for Port pins"]
pub type DIRSETP17_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP17_AW, O>;
impl<'a, const O: u8> DIRSETP17_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP17_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP17_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP18_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP18_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP18_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP18` writer - Direction set bits for Port pins"]
pub type DIRSETP18_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP18_AW, O>;
impl<'a, const O: u8> DIRSETP18_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP18_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP18_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP19_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP19_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP19_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP19` writer - Direction set bits for Port pins"]
pub type DIRSETP19_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP19_AW, O>;
impl<'a, const O: u8> DIRSETP19_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP19_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP19_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP20_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP20_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP20_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP20` writer - Direction set bits for Port pins"]
pub type DIRSETP20_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP20_AW, O>;
impl<'a, const O: u8> DIRSETP20_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP20_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP20_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP21_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP21_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP21_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP21` writer - Direction set bits for Port pins"]
pub type DIRSETP21_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP21_AW, O>;
impl<'a, const O: u8> DIRSETP21_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP21_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP21_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP22_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP22_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP22_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP22` writer - Direction set bits for Port pins"]
pub type DIRSETP22_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP22_AW, O>;
impl<'a, const O: u8> DIRSETP22_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP22_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP22_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP23_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP23_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP23_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP23` writer - Direction set bits for Port pins"]
pub type DIRSETP23_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP23_AW, O>;
impl<'a, const O: u8> DIRSETP23_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP23_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP23_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP24_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP24_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP24_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP24` writer - Direction set bits for Port pins"]
pub type DIRSETP24_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP24_AW, O>;
impl<'a, const O: u8> DIRSETP24_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP24_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP24_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP25_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP25_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP25_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP25` writer - Direction set bits for Port pins"]
pub type DIRSETP25_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP25_AW, O>;
impl<'a, const O: u8> DIRSETP25_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP25_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP25_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP26_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP26_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP26_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP26` writer - Direction set bits for Port pins"]
pub type DIRSETP26_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP26_AW, O>;
impl<'a, const O: u8> DIRSETP26_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP26_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP26_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP27_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP27_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP27_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP27` writer - Direction set bits for Port pins"]
pub type DIRSETP27_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP27_AW, O>;
impl<'a, const O: u8> DIRSETP27_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP27_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP27_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP28_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP28_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP28_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP28` writer - Direction set bits for Port pins"]
pub type DIRSETP28_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP28_AW, O>;
impl<'a, const O: u8> DIRSETP28_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP28_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP28_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP29_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP29_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP29_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP29` writer - Direction set bits for Port pins"]
pub type DIRSETP29_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP29_AW, O>;
impl<'a, const O: u8> DIRSETP29_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP29_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP29_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP30_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP30_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP30_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP30` writer - Direction set bits for Port pins"]
pub type DIRSETP30_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP30_AW, O>;
impl<'a, const O: u8> DIRSETP30_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP30_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP30_AW::DIRSETP_1)
    }
}
#[doc = "Direction set bits for Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSETP31_AW {
    #[doc = "0: No operation"]
    DIRSETP_0 = 0,
    #[doc = "1: Sets direction bit"]
    DIRSETP_1 = 1,
}
impl From<DIRSETP31_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRSETP31_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSETP31` writer - Direction set bits for Port pins"]
pub type DIRSETP31_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIRSET_SPEC, DIRSETP31_AW, O>;
impl<'a, const O: u8> DIRSETP31_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirsetp_0(self) -> &'a mut W {
        self.variant(DIRSETP31_AW::DIRSETP_0)
    }
    #[doc = "Sets direction bit"]
    #[inline(always)]
    pub fn dirsetp_1(self) -> &'a mut W {
        self.variant(DIRSETP31_AW::DIRSETP_1)
    }
}
impl W {
    #[doc = "Bit 0 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp0(&mut self) -> DIRSETP0_W<0> {
        DIRSETP0_W::new(self)
    }
    #[doc = "Bit 1 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp1(&mut self) -> DIRSETP1_W<1> {
        DIRSETP1_W::new(self)
    }
    #[doc = "Bit 2 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp2(&mut self) -> DIRSETP2_W<2> {
        DIRSETP2_W::new(self)
    }
    #[doc = "Bit 3 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp3(&mut self) -> DIRSETP3_W<3> {
        DIRSETP3_W::new(self)
    }
    #[doc = "Bit 4 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp4(&mut self) -> DIRSETP4_W<4> {
        DIRSETP4_W::new(self)
    }
    #[doc = "Bit 5 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp5(&mut self) -> DIRSETP5_W<5> {
        DIRSETP5_W::new(self)
    }
    #[doc = "Bit 6 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp6(&mut self) -> DIRSETP6_W<6> {
        DIRSETP6_W::new(self)
    }
    #[doc = "Bit 7 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp7(&mut self) -> DIRSETP7_W<7> {
        DIRSETP7_W::new(self)
    }
    #[doc = "Bit 8 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp8(&mut self) -> DIRSETP8_W<8> {
        DIRSETP8_W::new(self)
    }
    #[doc = "Bit 9 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp9(&mut self) -> DIRSETP9_W<9> {
        DIRSETP9_W::new(self)
    }
    #[doc = "Bit 10 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp10(&mut self) -> DIRSETP10_W<10> {
        DIRSETP10_W::new(self)
    }
    #[doc = "Bit 11 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp11(&mut self) -> DIRSETP11_W<11> {
        DIRSETP11_W::new(self)
    }
    #[doc = "Bit 12 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp12(&mut self) -> DIRSETP12_W<12> {
        DIRSETP12_W::new(self)
    }
    #[doc = "Bit 13 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp13(&mut self) -> DIRSETP13_W<13> {
        DIRSETP13_W::new(self)
    }
    #[doc = "Bit 14 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp14(&mut self) -> DIRSETP14_W<14> {
        DIRSETP14_W::new(self)
    }
    #[doc = "Bit 15 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp15(&mut self) -> DIRSETP15_W<15> {
        DIRSETP15_W::new(self)
    }
    #[doc = "Bit 16 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp16(&mut self) -> DIRSETP16_W<16> {
        DIRSETP16_W::new(self)
    }
    #[doc = "Bit 17 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp17(&mut self) -> DIRSETP17_W<17> {
        DIRSETP17_W::new(self)
    }
    #[doc = "Bit 18 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp18(&mut self) -> DIRSETP18_W<18> {
        DIRSETP18_W::new(self)
    }
    #[doc = "Bit 19 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp19(&mut self) -> DIRSETP19_W<19> {
        DIRSETP19_W::new(self)
    }
    #[doc = "Bit 20 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp20(&mut self) -> DIRSETP20_W<20> {
        DIRSETP20_W::new(self)
    }
    #[doc = "Bit 21 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp21(&mut self) -> DIRSETP21_W<21> {
        DIRSETP21_W::new(self)
    }
    #[doc = "Bit 22 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp22(&mut self) -> DIRSETP22_W<22> {
        DIRSETP22_W::new(self)
    }
    #[doc = "Bit 23 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp23(&mut self) -> DIRSETP23_W<23> {
        DIRSETP23_W::new(self)
    }
    #[doc = "Bit 24 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp24(&mut self) -> DIRSETP24_W<24> {
        DIRSETP24_W::new(self)
    }
    #[doc = "Bit 25 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp25(&mut self) -> DIRSETP25_W<25> {
        DIRSETP25_W::new(self)
    }
    #[doc = "Bit 26 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp26(&mut self) -> DIRSETP26_W<26> {
        DIRSETP26_W::new(self)
    }
    #[doc = "Bit 27 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp27(&mut self) -> DIRSETP27_W<27> {
        DIRSETP27_W::new(self)
    }
    #[doc = "Bit 28 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp28(&mut self) -> DIRSETP28_W<28> {
        DIRSETP28_W::new(self)
    }
    #[doc = "Bit 29 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp29(&mut self) -> DIRSETP29_W<29> {
        DIRSETP29_W::new(self)
    }
    #[doc = "Bit 30 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp30(&mut self) -> DIRSETP30_W<30> {
        DIRSETP30_W::new(self)
    }
    #[doc = "Bit 31 - Direction set bits for Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn dirsetp31(&mut self) -> DIRSETP31_W<31> {
        DIRSETP31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port direction set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dirset](index.html) module"]
pub struct DIRSET_SPEC;
impl crate::RegisterSpec for DIRSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dirset::W](W) writer structure"]
impl crate::Writable for DIRSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff_ffff;
}
#[doc = "`reset()` method sets DIRSET[%s]
to value 0"]
impl crate::Resettable for DIRSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
