#[doc = "Register `DIR[%s]` writer"]
pub struct W(crate::W<DIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIR_SPEC>;
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
impl From<crate::W<DIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP0_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP0_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP0` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP0_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP0_AW, O>;
impl<'a, const O: u8> DIRP0_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP0_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP0_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP1_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP1_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP1` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP1_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP1_AW, O>;
impl<'a, const O: u8> DIRP1_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP1_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP1_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP2_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP2_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP2` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP2_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP2_AW, O>;
impl<'a, const O: u8> DIRP2_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP2_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP2_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP3_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP3_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP3` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP3_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP3_AW, O>;
impl<'a, const O: u8> DIRP3_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP3_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP3_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP4_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP4_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP4` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP4_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP4_AW, O>;
impl<'a, const O: u8> DIRP4_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP4_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP4_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP5_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP5_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP5` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP5_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP5_AW, O>;
impl<'a, const O: u8> DIRP5_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP5_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP5_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP6_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP6_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP6` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP6_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP6_AW, O>;
impl<'a, const O: u8> DIRP6_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP6_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP6_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP7_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP7_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP7` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP7_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP7_AW, O>;
impl<'a, const O: u8> DIRP7_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP7_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP7_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP8_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP8_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP8` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP8_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP8_AW, O>;
impl<'a, const O: u8> DIRP8_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP8_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP8_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP9_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP9_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP9` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP9_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP9_AW, O>;
impl<'a, const O: u8> DIRP9_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP9_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP9_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP10_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP10_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP10` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP10_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP10_AW, O>;
impl<'a, const O: u8> DIRP10_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP10_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP10_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP11_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP11_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP11` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP11_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP11_AW, O>;
impl<'a, const O: u8> DIRP11_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP11_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP11_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP12_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP12_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP12` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP12_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP12_AW, O>;
impl<'a, const O: u8> DIRP12_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP12_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP12_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP13_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP13_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP13` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP13_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP13_AW, O>;
impl<'a, const O: u8> DIRP13_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP13_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP13_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP14_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP14_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP14` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP14_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP14_AW, O>;
impl<'a, const O: u8> DIRP14_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP14_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP14_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP15_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP15_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP15_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP15` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP15_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP15_AW, O>;
impl<'a, const O: u8> DIRP15_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP15_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP15_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP16_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP16_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP16_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP16` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP16_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP16_AW, O>;
impl<'a, const O: u8> DIRP16_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP16_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP16_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP17_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP17_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP17_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP17` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP17_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP17_AW, O>;
impl<'a, const O: u8> DIRP17_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP17_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP17_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP18_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP18_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP18_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP18` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP18_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP18_AW, O>;
impl<'a, const O: u8> DIRP18_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP18_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP18_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP19_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP19_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP19_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP19` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP19_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP19_AW, O>;
impl<'a, const O: u8> DIRP19_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP19_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP19_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP20_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP20_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP20_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP20` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP20_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP20_AW, O>;
impl<'a, const O: u8> DIRP20_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP20_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP20_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP21_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP21_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP21_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP21` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP21_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP21_AW, O>;
impl<'a, const O: u8> DIRP21_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP21_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP21_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP22_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP22_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP22_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP22` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP22_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP22_AW, O>;
impl<'a, const O: u8> DIRP22_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP22_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP22_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP23_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP23_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP23_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP23` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP23_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP23_AW, O>;
impl<'a, const O: u8> DIRP23_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP23_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP23_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP24_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP24_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP24_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP24` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP24_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP24_AW, O>;
impl<'a, const O: u8> DIRP24_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP24_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP24_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP25_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP25_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP25_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP25` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP25_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP25_AW, O>;
impl<'a, const O: u8> DIRP25_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP25_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP25_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP26_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP26_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP26_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP26` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP26_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP26_AW, O>;
impl<'a, const O: u8> DIRP26_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP26_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP26_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP27_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP27_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP27_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP27` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP27_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP27_AW, O>;
impl<'a, const O: u8> DIRP27_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP27_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP27_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP28_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP28_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP28_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP28` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP28_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP28_AW, O>;
impl<'a, const O: u8> DIRP28_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP28_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP28_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP29_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP29_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP29_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP29` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP29_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP29_AW, O>;
impl<'a, const O: u8> DIRP29_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP29_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP29_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP30_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP30_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP30_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP30` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP30_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP30_AW, O>;
impl<'a, const O: u8> DIRP30_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP30_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP30_AW::DIR_1)
    }
}
#[doc = "Selects pin direction for pin PIOa_b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRP31_AW {
    #[doc = "0: Input"]
    DIR_0 = 0,
    #[doc = "1: Output"]
    DIR_1 = 1,
}
impl From<DIRP31_AW> for bool {
    #[inline(always)]
    fn from(variant: DIRP31_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRP31` writer - Selects pin direction for pin PIOa_b."]
pub type DIRP31_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, DIR_SPEC, DIRP31_AW, O>;
impl<'a, const O: u8> DIRP31_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIRP31_AW::DIR_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIRP31_AW::DIR_1)
    }
}
impl W {
    #[doc = "Bit 0 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp0(&mut self) -> DIRP0_W<0> {
        DIRP0_W::new(self)
    }
    #[doc = "Bit 1 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp1(&mut self) -> DIRP1_W<1> {
        DIRP1_W::new(self)
    }
    #[doc = "Bit 2 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp2(&mut self) -> DIRP2_W<2> {
        DIRP2_W::new(self)
    }
    #[doc = "Bit 3 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp3(&mut self) -> DIRP3_W<3> {
        DIRP3_W::new(self)
    }
    #[doc = "Bit 4 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp4(&mut self) -> DIRP4_W<4> {
        DIRP4_W::new(self)
    }
    #[doc = "Bit 5 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp5(&mut self) -> DIRP5_W<5> {
        DIRP5_W::new(self)
    }
    #[doc = "Bit 6 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp6(&mut self) -> DIRP6_W<6> {
        DIRP6_W::new(self)
    }
    #[doc = "Bit 7 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp7(&mut self) -> DIRP7_W<7> {
        DIRP7_W::new(self)
    }
    #[doc = "Bit 8 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp8(&mut self) -> DIRP8_W<8> {
        DIRP8_W::new(self)
    }
    #[doc = "Bit 9 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp9(&mut self) -> DIRP9_W<9> {
        DIRP9_W::new(self)
    }
    #[doc = "Bit 10 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp10(&mut self) -> DIRP10_W<10> {
        DIRP10_W::new(self)
    }
    #[doc = "Bit 11 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp11(&mut self) -> DIRP11_W<11> {
        DIRP11_W::new(self)
    }
    #[doc = "Bit 12 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp12(&mut self) -> DIRP12_W<12> {
        DIRP12_W::new(self)
    }
    #[doc = "Bit 13 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp13(&mut self) -> DIRP13_W<13> {
        DIRP13_W::new(self)
    }
    #[doc = "Bit 14 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp14(&mut self) -> DIRP14_W<14> {
        DIRP14_W::new(self)
    }
    #[doc = "Bit 15 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp15(&mut self) -> DIRP15_W<15> {
        DIRP15_W::new(self)
    }
    #[doc = "Bit 16 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp16(&mut self) -> DIRP16_W<16> {
        DIRP16_W::new(self)
    }
    #[doc = "Bit 17 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp17(&mut self) -> DIRP17_W<17> {
        DIRP17_W::new(self)
    }
    #[doc = "Bit 18 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp18(&mut self) -> DIRP18_W<18> {
        DIRP18_W::new(self)
    }
    #[doc = "Bit 19 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp19(&mut self) -> DIRP19_W<19> {
        DIRP19_W::new(self)
    }
    #[doc = "Bit 20 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp20(&mut self) -> DIRP20_W<20> {
        DIRP20_W::new(self)
    }
    #[doc = "Bit 21 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp21(&mut self) -> DIRP21_W<21> {
        DIRP21_W::new(self)
    }
    #[doc = "Bit 22 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp22(&mut self) -> DIRP22_W<22> {
        DIRP22_W::new(self)
    }
    #[doc = "Bit 23 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp23(&mut self) -> DIRP23_W<23> {
        DIRP23_W::new(self)
    }
    #[doc = "Bit 24 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp24(&mut self) -> DIRP24_W<24> {
        DIRP24_W::new(self)
    }
    #[doc = "Bit 25 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp25(&mut self) -> DIRP25_W<25> {
        DIRP25_W::new(self)
    }
    #[doc = "Bit 26 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp26(&mut self) -> DIRP26_W<26> {
        DIRP26_W::new(self)
    }
    #[doc = "Bit 27 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp27(&mut self) -> DIRP27_W<27> {
        DIRP27_W::new(self)
    }
    #[doc = "Bit 28 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp28(&mut self) -> DIRP28_W<28> {
        DIRP28_W::new(self)
    }
    #[doc = "Bit 29 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp29(&mut self) -> DIRP29_W<29> {
        DIRP29_W::new(self)
    }
    #[doc = "Bit 30 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp30(&mut self) -> DIRP30_W<30> {
        DIRP30_W::new(self)
    }
    #[doc = "Bit 31 - Selects pin direction for pin PIOa_b."]
    #[inline(always)]
    #[must_use]
    pub fn dirp31(&mut self) -> DIRP31_W<31> {
        DIRP31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port direction\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir](index.html) module"]
pub struct DIR_SPEC;
impl crate::RegisterSpec for DIR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dir::W](W) writer structure"]
impl crate::Writable for DIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff_ffff;
}
#[doc = "`reset()` method sets DIR[%s]
to value 0"]
impl crate::Resettable for DIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
