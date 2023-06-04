#[doc = "Register `NOT[%s]` writer"]
pub struct W(crate::W<NOT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NOT_SPEC>;
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
impl From<crate::W<NOT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NOT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP0_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP0_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP0` writer - Toggle output bits"]
pub type NOTP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP0_AW, O>;
impl<'a, const O: u8> NOTP0_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP0_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP0_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP1_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP1_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP1` writer - Toggle output bits"]
pub type NOTP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP1_AW, O>;
impl<'a, const O: u8> NOTP1_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP1_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP1_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP2_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP2_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP2` writer - Toggle output bits"]
pub type NOTP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP2_AW, O>;
impl<'a, const O: u8> NOTP2_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP2_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP2_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP3_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP3_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP3` writer - Toggle output bits"]
pub type NOTP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP3_AW, O>;
impl<'a, const O: u8> NOTP3_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP3_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP3_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP4_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP4_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP4` writer - Toggle output bits"]
pub type NOTP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP4_AW, O>;
impl<'a, const O: u8> NOTP4_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP4_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP4_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP5_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP5_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP5` writer - Toggle output bits"]
pub type NOTP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP5_AW, O>;
impl<'a, const O: u8> NOTP5_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP5_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP5_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP6_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP6_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP6` writer - Toggle output bits"]
pub type NOTP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP6_AW, O>;
impl<'a, const O: u8> NOTP6_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP6_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP6_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP7_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP7_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP7` writer - Toggle output bits"]
pub type NOTP7_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP7_AW, O>;
impl<'a, const O: u8> NOTP7_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP7_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP7_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP8_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP8_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP8` writer - Toggle output bits"]
pub type NOTP8_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP8_AW, O>;
impl<'a, const O: u8> NOTP8_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP8_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP8_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP9_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP9_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP9` writer - Toggle output bits"]
pub type NOTP9_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP9_AW, O>;
impl<'a, const O: u8> NOTP9_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP9_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP9_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP10_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP10_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP10` writer - Toggle output bits"]
pub type NOTP10_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP10_AW, O>;
impl<'a, const O: u8> NOTP10_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP10_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP10_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP11_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP11_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP11` writer - Toggle output bits"]
pub type NOTP11_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP11_AW, O>;
impl<'a, const O: u8> NOTP11_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP11_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP11_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP12_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP12_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP12` writer - Toggle output bits"]
pub type NOTP12_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP12_AW, O>;
impl<'a, const O: u8> NOTP12_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP12_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP12_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP13_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP13_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP13` writer - Toggle output bits"]
pub type NOTP13_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP13_AW, O>;
impl<'a, const O: u8> NOTP13_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP13_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP13_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP14_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP14_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP14` writer - Toggle output bits"]
pub type NOTP14_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP14_AW, O>;
impl<'a, const O: u8> NOTP14_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP14_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP14_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP15_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP15_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP15_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP15` writer - Toggle output bits"]
pub type NOTP15_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP15_AW, O>;
impl<'a, const O: u8> NOTP15_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP15_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP15_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP16_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP16_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP16_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP16` writer - Toggle output bits"]
pub type NOTP16_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP16_AW, O>;
impl<'a, const O: u8> NOTP16_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP16_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP16_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP17_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP17_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP17_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP17` writer - Toggle output bits"]
pub type NOTP17_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP17_AW, O>;
impl<'a, const O: u8> NOTP17_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP17_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP17_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP18_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP18_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP18_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP18` writer - Toggle output bits"]
pub type NOTP18_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP18_AW, O>;
impl<'a, const O: u8> NOTP18_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP18_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP18_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP19_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP19_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP19_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP19` writer - Toggle output bits"]
pub type NOTP19_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP19_AW, O>;
impl<'a, const O: u8> NOTP19_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP19_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP19_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP20_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP20_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP20_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP20` writer - Toggle output bits"]
pub type NOTP20_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP20_AW, O>;
impl<'a, const O: u8> NOTP20_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP20_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP20_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP21_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP21_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP21_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP21` writer - Toggle output bits"]
pub type NOTP21_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP21_AW, O>;
impl<'a, const O: u8> NOTP21_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP21_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP21_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP22_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP22_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP22_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP22` writer - Toggle output bits"]
pub type NOTP22_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP22_AW, O>;
impl<'a, const O: u8> NOTP22_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP22_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP22_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP23_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP23_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP23_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP23` writer - Toggle output bits"]
pub type NOTP23_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP23_AW, O>;
impl<'a, const O: u8> NOTP23_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP23_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP23_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP24_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP24_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP24_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP24` writer - Toggle output bits"]
pub type NOTP24_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP24_AW, O>;
impl<'a, const O: u8> NOTP24_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP24_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP24_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP25_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP25_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP25_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP25` writer - Toggle output bits"]
pub type NOTP25_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP25_AW, O>;
impl<'a, const O: u8> NOTP25_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP25_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP25_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP26_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP26_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP26_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP26` writer - Toggle output bits"]
pub type NOTP26_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP26_AW, O>;
impl<'a, const O: u8> NOTP26_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP26_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP26_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP27_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP27_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP27_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP27` writer - Toggle output bits"]
pub type NOTP27_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP27_AW, O>;
impl<'a, const O: u8> NOTP27_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP27_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP27_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP28_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP28_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP28_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP28` writer - Toggle output bits"]
pub type NOTP28_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP28_AW, O>;
impl<'a, const O: u8> NOTP28_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP28_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP28_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP29_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP29_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP29_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP29` writer - Toggle output bits"]
pub type NOTP29_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP29_AW, O>;
impl<'a, const O: u8> NOTP29_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP29_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP29_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP30_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP30_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP30_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP30` writer - Toggle output bits"]
pub type NOTP30_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP30_AW, O>;
impl<'a, const O: u8> NOTP30_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP30_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP30_AW::NOTP_1)
    }
}
#[doc = "Toggle output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTP31_AW {
    #[doc = "0: No operation"]
    NOTP_0 = 0,
    #[doc = "1: Toggle output bit"]
    NOTP_1 = 1,
}
impl From<NOTP31_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTP31_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTP31` writer - Toggle output bits"]
pub type NOTP31_W<'a, const O: u8> = crate::BitWriter<'a, u32, NOT_SPEC, NOTP31_AW, O>;
impl<'a, const O: u8> NOTP31_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn notp_0(self) -> &'a mut W {
        self.variant(NOTP31_AW::NOTP_0)
    }
    #[doc = "Toggle output bit"]
    #[inline(always)]
    pub fn notp_1(self) -> &'a mut W {
        self.variant(NOTP31_AW::NOTP_1)
    }
}
impl W {
    #[doc = "Bit 0 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp0(&mut self) -> NOTP0_W<0> {
        NOTP0_W::new(self)
    }
    #[doc = "Bit 1 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp1(&mut self) -> NOTP1_W<1> {
        NOTP1_W::new(self)
    }
    #[doc = "Bit 2 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp2(&mut self) -> NOTP2_W<2> {
        NOTP2_W::new(self)
    }
    #[doc = "Bit 3 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp3(&mut self) -> NOTP3_W<3> {
        NOTP3_W::new(self)
    }
    #[doc = "Bit 4 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp4(&mut self) -> NOTP4_W<4> {
        NOTP4_W::new(self)
    }
    #[doc = "Bit 5 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp5(&mut self) -> NOTP5_W<5> {
        NOTP5_W::new(self)
    }
    #[doc = "Bit 6 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp6(&mut self) -> NOTP6_W<6> {
        NOTP6_W::new(self)
    }
    #[doc = "Bit 7 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp7(&mut self) -> NOTP7_W<7> {
        NOTP7_W::new(self)
    }
    #[doc = "Bit 8 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp8(&mut self) -> NOTP8_W<8> {
        NOTP8_W::new(self)
    }
    #[doc = "Bit 9 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp9(&mut self) -> NOTP9_W<9> {
        NOTP9_W::new(self)
    }
    #[doc = "Bit 10 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp10(&mut self) -> NOTP10_W<10> {
        NOTP10_W::new(self)
    }
    #[doc = "Bit 11 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp11(&mut self) -> NOTP11_W<11> {
        NOTP11_W::new(self)
    }
    #[doc = "Bit 12 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp12(&mut self) -> NOTP12_W<12> {
        NOTP12_W::new(self)
    }
    #[doc = "Bit 13 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp13(&mut self) -> NOTP13_W<13> {
        NOTP13_W::new(self)
    }
    #[doc = "Bit 14 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp14(&mut self) -> NOTP14_W<14> {
        NOTP14_W::new(self)
    }
    #[doc = "Bit 15 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp15(&mut self) -> NOTP15_W<15> {
        NOTP15_W::new(self)
    }
    #[doc = "Bit 16 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp16(&mut self) -> NOTP16_W<16> {
        NOTP16_W::new(self)
    }
    #[doc = "Bit 17 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp17(&mut self) -> NOTP17_W<17> {
        NOTP17_W::new(self)
    }
    #[doc = "Bit 18 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp18(&mut self) -> NOTP18_W<18> {
        NOTP18_W::new(self)
    }
    #[doc = "Bit 19 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp19(&mut self) -> NOTP19_W<19> {
        NOTP19_W::new(self)
    }
    #[doc = "Bit 20 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp20(&mut self) -> NOTP20_W<20> {
        NOTP20_W::new(self)
    }
    #[doc = "Bit 21 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp21(&mut self) -> NOTP21_W<21> {
        NOTP21_W::new(self)
    }
    #[doc = "Bit 22 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp22(&mut self) -> NOTP22_W<22> {
        NOTP22_W::new(self)
    }
    #[doc = "Bit 23 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp23(&mut self) -> NOTP23_W<23> {
        NOTP23_W::new(self)
    }
    #[doc = "Bit 24 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp24(&mut self) -> NOTP24_W<24> {
        NOTP24_W::new(self)
    }
    #[doc = "Bit 25 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp25(&mut self) -> NOTP25_W<25> {
        NOTP25_W::new(self)
    }
    #[doc = "Bit 26 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp26(&mut self) -> NOTP26_W<26> {
        NOTP26_W::new(self)
    }
    #[doc = "Bit 27 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp27(&mut self) -> NOTP27_W<27> {
        NOTP27_W::new(self)
    }
    #[doc = "Bit 28 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp28(&mut self) -> NOTP28_W<28> {
        NOTP28_W::new(self)
    }
    #[doc = "Bit 29 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp29(&mut self) -> NOTP29_W<29> {
        NOTP29_W::new(self)
    }
    #[doc = "Bit 30 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp30(&mut self) -> NOTP30_W<30> {
        NOTP30_W::new(self)
    }
    #[doc = "Bit 31 - Toggle output bits"]
    #[inline(always)]
    #[must_use]
    pub fn notp31(&mut self) -> NOTP31_W<31> {
        NOTP31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port toggle\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [not](index.html) module"]
pub struct NOT_SPEC;
impl crate::RegisterSpec for NOT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [not::W](W) writer structure"]
impl crate::Writable for NOT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NOT[%s]
to value 0"]
impl crate::Resettable for NOT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
