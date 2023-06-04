#[doc = "Register `EVFLAG` reader"]
pub struct R(crate::R<EVFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVFLAG` writer"]
pub struct W(crate::W<EVFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVFLAG_SPEC>;
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
impl From<crate::W<EVFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLAG0` reader - Event Flag n"]
pub type FLAG0_R = crate::BitReader<FLAG0_A>;
#[doc = "Event Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG0_A {
    #[doc = "0: No Flag"]
    NO_FLAG = 0,
    #[doc = "1: Event n Flag"]
    FLAG = 1,
}
impl From<FLAG0_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG0_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG0_A {
        match self.bits {
            false => FLAG0_A::NO_FLAG,
            true => FLAG0_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == FLAG0_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == FLAG0_A::FLAG
    }
}
#[doc = "Field `FLAG0` writer - Event Flag n"]
pub type FLAG0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAG_SPEC, FLAG0_A, O>;
impl<'a, const O: u8> FLAG0_W<'a, O> {
    #[doc = "No Flag"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(FLAG0_A::NO_FLAG)
    }
    #[doc = "Event n Flag"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(FLAG0_A::FLAG)
    }
}
#[doc = "Field `FLAG1` reader - Event Flag n"]
pub type FLAG1_R = crate::BitReader<FLAG1_A>;
#[doc = "Event Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG1_A {
    #[doc = "0: No Flag"]
    NO_FLAG = 0,
    #[doc = "1: Event n Flag"]
    FLAG = 1,
}
impl From<FLAG1_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG1_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG1_A {
        match self.bits {
            false => FLAG1_A::NO_FLAG,
            true => FLAG1_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == FLAG1_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == FLAG1_A::FLAG
    }
}
#[doc = "Field `FLAG1` writer - Event Flag n"]
pub type FLAG1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAG_SPEC, FLAG1_A, O>;
impl<'a, const O: u8> FLAG1_W<'a, O> {
    #[doc = "No Flag"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(FLAG1_A::NO_FLAG)
    }
    #[doc = "Event n Flag"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(FLAG1_A::FLAG)
    }
}
#[doc = "Field `FLAG2` reader - Event Flag n"]
pub type FLAG2_R = crate::BitReader<FLAG2_A>;
#[doc = "Event Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG2_A {
    #[doc = "0: No Flag"]
    NO_FLAG = 0,
    #[doc = "1: Event n Flag"]
    FLAG = 1,
}
impl From<FLAG2_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG2_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG2_A {
        match self.bits {
            false => FLAG2_A::NO_FLAG,
            true => FLAG2_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == FLAG2_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == FLAG2_A::FLAG
    }
}
#[doc = "Field `FLAG2` writer - Event Flag n"]
pub type FLAG2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAG_SPEC, FLAG2_A, O>;
impl<'a, const O: u8> FLAG2_W<'a, O> {
    #[doc = "No Flag"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(FLAG2_A::NO_FLAG)
    }
    #[doc = "Event n Flag"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(FLAG2_A::FLAG)
    }
}
#[doc = "Field `FLAG3` reader - Event Flag n"]
pub type FLAG3_R = crate::BitReader<FLAG3_A>;
#[doc = "Event Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG3_A {
    #[doc = "0: No Flag"]
    NO_FLAG = 0,
    #[doc = "1: Event n Flag"]
    FLAG = 1,
}
impl From<FLAG3_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG3_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG3_A {
        match self.bits {
            false => FLAG3_A::NO_FLAG,
            true => FLAG3_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == FLAG3_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == FLAG3_A::FLAG
    }
}
#[doc = "Field `FLAG3` writer - Event Flag n"]
pub type FLAG3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAG_SPEC, FLAG3_A, O>;
impl<'a, const O: u8> FLAG3_W<'a, O> {
    #[doc = "No Flag"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(FLAG3_A::NO_FLAG)
    }
    #[doc = "Event n Flag"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(FLAG3_A::FLAG)
    }
}
#[doc = "Field `FLAG4` reader - Event Flag n"]
pub type FLAG4_R = crate::BitReader<FLAG4_A>;
#[doc = "Event Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG4_A {
    #[doc = "0: No Flag"]
    NO_FLAG = 0,
    #[doc = "1: Event n Flag"]
    FLAG = 1,
}
impl From<FLAG4_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG4_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG4_A {
        match self.bits {
            false => FLAG4_A::NO_FLAG,
            true => FLAG4_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == FLAG4_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == FLAG4_A::FLAG
    }
}
#[doc = "Field `FLAG4` writer - Event Flag n"]
pub type FLAG4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAG_SPEC, FLAG4_A, O>;
impl<'a, const O: u8> FLAG4_W<'a, O> {
    #[doc = "No Flag"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(FLAG4_A::NO_FLAG)
    }
    #[doc = "Event n Flag"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(FLAG4_A::FLAG)
    }
}
#[doc = "Field `FLAG5` reader - Event Flag n"]
pub type FLAG5_R = crate::BitReader<FLAG5_A>;
#[doc = "Event Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG5_A {
    #[doc = "0: No Flag"]
    NO_FLAG = 0,
    #[doc = "1: Event n Flag"]
    FLAG = 1,
}
impl From<FLAG5_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG5_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG5_A {
        match self.bits {
            false => FLAG5_A::NO_FLAG,
            true => FLAG5_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == FLAG5_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == FLAG5_A::FLAG
    }
}
#[doc = "Field `FLAG5` writer - Event Flag n"]
pub type FLAG5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAG_SPEC, FLAG5_A, O>;
impl<'a, const O: u8> FLAG5_W<'a, O> {
    #[doc = "No Flag"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(FLAG5_A::NO_FLAG)
    }
    #[doc = "Event n Flag"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(FLAG5_A::FLAG)
    }
}
#[doc = "Field `FLAG6` reader - Event Flag n"]
pub type FLAG6_R = crate::BitReader<FLAG6_A>;
#[doc = "Event Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG6_A {
    #[doc = "0: No Flag"]
    NO_FLAG = 0,
    #[doc = "1: Event n Flag"]
    FLAG = 1,
}
impl From<FLAG6_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG6_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG6_A {
        match self.bits {
            false => FLAG6_A::NO_FLAG,
            true => FLAG6_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == FLAG6_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == FLAG6_A::FLAG
    }
}
#[doc = "Field `FLAG6` writer - Event Flag n"]
pub type FLAG6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAG_SPEC, FLAG6_A, O>;
impl<'a, const O: u8> FLAG6_W<'a, O> {
    #[doc = "No Flag"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(FLAG6_A::NO_FLAG)
    }
    #[doc = "Event n Flag"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(FLAG6_A::FLAG)
    }
}
#[doc = "Field `FLAG7` reader - Event Flag n"]
pub type FLAG7_R = crate::BitReader<FLAG7_A>;
#[doc = "Event Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG7_A {
    #[doc = "0: No Flag"]
    NO_FLAG = 0,
    #[doc = "1: Event n Flag"]
    FLAG = 1,
}
impl From<FLAG7_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG7_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG7_A {
        match self.bits {
            false => FLAG7_A::NO_FLAG,
            true => FLAG7_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == FLAG7_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == FLAG7_A::FLAG
    }
}
#[doc = "Field `FLAG7` writer - Event Flag n"]
pub type FLAG7_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAG_SPEC, FLAG7_A, O>;
impl<'a, const O: u8> FLAG7_W<'a, O> {
    #[doc = "No Flag"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(FLAG7_A::NO_FLAG)
    }
    #[doc = "Event n Flag"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(FLAG7_A::FLAG)
    }
}
#[doc = "Field `FLAG8` reader - Event Flag n"]
pub type FLAG8_R = crate::BitReader<FLAG8_A>;
#[doc = "Event Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG8_A {
    #[doc = "0: No Flag"]
    NO_FLAG = 0,
    #[doc = "1: Event n Flag"]
    FLAG = 1,
}
impl From<FLAG8_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG8_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG8_A {
        match self.bits {
            false => FLAG8_A::NO_FLAG,
            true => FLAG8_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == FLAG8_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == FLAG8_A::FLAG
    }
}
#[doc = "Field `FLAG8` writer - Event Flag n"]
pub type FLAG8_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAG_SPEC, FLAG8_A, O>;
impl<'a, const O: u8> FLAG8_W<'a, O> {
    #[doc = "No Flag"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(FLAG8_A::NO_FLAG)
    }
    #[doc = "Event n Flag"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(FLAG8_A::FLAG)
    }
}
#[doc = "Field `FLAG9` reader - Event Flag n"]
pub type FLAG9_R = crate::BitReader<FLAG9_A>;
#[doc = "Event Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG9_A {
    #[doc = "0: No Flag"]
    NO_FLAG = 0,
    #[doc = "1: Event n Flag"]
    FLAG = 1,
}
impl From<FLAG9_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG9_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG9_A {
        match self.bits {
            false => FLAG9_A::NO_FLAG,
            true => FLAG9_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == FLAG9_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == FLAG9_A::FLAG
    }
}
#[doc = "Field `FLAG9` writer - Event Flag n"]
pub type FLAG9_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAG_SPEC, FLAG9_A, O>;
impl<'a, const O: u8> FLAG9_W<'a, O> {
    #[doc = "No Flag"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(FLAG9_A::NO_FLAG)
    }
    #[doc = "Event n Flag"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(FLAG9_A::FLAG)
    }
}
#[doc = "Field `FLAG10` reader - Event Flag n"]
pub type FLAG10_R = crate::BitReader<FLAG10_A>;
#[doc = "Event Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG10_A {
    #[doc = "0: No Flag"]
    NO_FLAG = 0,
    #[doc = "1: Event n Flag"]
    FLAG = 1,
}
impl From<FLAG10_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG10_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG10_A {
        match self.bits {
            false => FLAG10_A::NO_FLAG,
            true => FLAG10_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == FLAG10_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == FLAG10_A::FLAG
    }
}
#[doc = "Field `FLAG10` writer - Event Flag n"]
pub type FLAG10_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAG_SPEC, FLAG10_A, O>;
impl<'a, const O: u8> FLAG10_W<'a, O> {
    #[doc = "No Flag"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(FLAG10_A::NO_FLAG)
    }
    #[doc = "Event n Flag"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(FLAG10_A::FLAG)
    }
}
#[doc = "Field `FLAG11` reader - Event Flag n"]
pub type FLAG11_R = crate::BitReader<FLAG11_A>;
#[doc = "Event Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG11_A {
    #[doc = "0: No Flag"]
    NO_FLAG = 0,
    #[doc = "1: Event n Flag"]
    FLAG = 1,
}
impl From<FLAG11_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG11_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG11_A {
        match self.bits {
            false => FLAG11_A::NO_FLAG,
            true => FLAG11_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == FLAG11_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == FLAG11_A::FLAG
    }
}
#[doc = "Field `FLAG11` writer - Event Flag n"]
pub type FLAG11_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAG_SPEC, FLAG11_A, O>;
impl<'a, const O: u8> FLAG11_W<'a, O> {
    #[doc = "No Flag"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(FLAG11_A::NO_FLAG)
    }
    #[doc = "Event n Flag"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(FLAG11_A::FLAG)
    }
}
#[doc = "Field `FLAG12` reader - Event Flag n"]
pub type FLAG12_R = crate::BitReader<FLAG12_A>;
#[doc = "Event Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG12_A {
    #[doc = "0: No Flag"]
    NO_FLAG = 0,
    #[doc = "1: Event n Flag"]
    FLAG = 1,
}
impl From<FLAG12_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG12_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG12_A {
        match self.bits {
            false => FLAG12_A::NO_FLAG,
            true => FLAG12_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == FLAG12_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == FLAG12_A::FLAG
    }
}
#[doc = "Field `FLAG12` writer - Event Flag n"]
pub type FLAG12_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAG_SPEC, FLAG12_A, O>;
impl<'a, const O: u8> FLAG12_W<'a, O> {
    #[doc = "No Flag"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(FLAG12_A::NO_FLAG)
    }
    #[doc = "Event n Flag"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(FLAG12_A::FLAG)
    }
}
#[doc = "Field `FLAG13` reader - Event Flag n"]
pub type FLAG13_R = crate::BitReader<FLAG13_A>;
#[doc = "Event Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG13_A {
    #[doc = "0: No Flag"]
    NO_FLAG = 0,
    #[doc = "1: Event n Flag"]
    FLAG = 1,
}
impl From<FLAG13_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG13_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG13_A {
        match self.bits {
            false => FLAG13_A::NO_FLAG,
            true => FLAG13_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == FLAG13_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == FLAG13_A::FLAG
    }
}
#[doc = "Field `FLAG13` writer - Event Flag n"]
pub type FLAG13_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAG_SPEC, FLAG13_A, O>;
impl<'a, const O: u8> FLAG13_W<'a, O> {
    #[doc = "No Flag"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(FLAG13_A::NO_FLAG)
    }
    #[doc = "Event n Flag"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(FLAG13_A::FLAG)
    }
}
#[doc = "Field `FLAG14` reader - Event Flag n"]
pub type FLAG14_R = crate::BitReader<FLAG14_A>;
#[doc = "Event Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG14_A {
    #[doc = "0: No Flag"]
    NO_FLAG = 0,
    #[doc = "1: Event n Flag"]
    FLAG = 1,
}
impl From<FLAG14_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG14_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG14_A {
        match self.bits {
            false => FLAG14_A::NO_FLAG,
            true => FLAG14_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == FLAG14_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == FLAG14_A::FLAG
    }
}
#[doc = "Field `FLAG14` writer - Event Flag n"]
pub type FLAG14_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAG_SPEC, FLAG14_A, O>;
impl<'a, const O: u8> FLAG14_W<'a, O> {
    #[doc = "No Flag"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(FLAG14_A::NO_FLAG)
    }
    #[doc = "Event n Flag"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(FLAG14_A::FLAG)
    }
}
#[doc = "Field `FLAG15` reader - Event Flag n"]
pub type FLAG15_R = crate::BitReader<FLAG15_A>;
#[doc = "Event Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG15_A {
    #[doc = "0: No Flag"]
    NO_FLAG = 0,
    #[doc = "1: Event n Flag"]
    FLAG = 1,
}
impl From<FLAG15_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG15_A) -> Self {
        variant as u8 != 0
    }
}
impl FLAG15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLAG15_A {
        match self.bits {
            false => FLAG15_A::NO_FLAG,
            true => FLAG15_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == FLAG15_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == FLAG15_A::FLAG
    }
}
#[doc = "Field `FLAG15` writer - Event Flag n"]
pub type FLAG15_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVFLAG_SPEC, FLAG15_A, O>;
impl<'a, const O: u8> FLAG15_W<'a, O> {
    #[doc = "No Flag"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(FLAG15_A::NO_FLAG)
    }
    #[doc = "Event n Flag"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(FLAG15_A::FLAG)
    }
}
impl R {
    #[doc = "Bit 0 - Event Flag n"]
    #[inline(always)]
    pub fn flag0(&self) -> FLAG0_R {
        FLAG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event Flag n"]
    #[inline(always)]
    pub fn flag1(&self) -> FLAG1_R {
        FLAG1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event Flag n"]
    #[inline(always)]
    pub fn flag2(&self) -> FLAG2_R {
        FLAG2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event Flag n"]
    #[inline(always)]
    pub fn flag3(&self) -> FLAG3_R {
        FLAG3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Event Flag n"]
    #[inline(always)]
    pub fn flag4(&self) -> FLAG4_R {
        FLAG4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Event Flag n"]
    #[inline(always)]
    pub fn flag5(&self) -> FLAG5_R {
        FLAG5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Event Flag n"]
    #[inline(always)]
    pub fn flag6(&self) -> FLAG6_R {
        FLAG6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Event Flag n"]
    #[inline(always)]
    pub fn flag7(&self) -> FLAG7_R {
        FLAG7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Event Flag n"]
    #[inline(always)]
    pub fn flag8(&self) -> FLAG8_R {
        FLAG8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event Flag n"]
    #[inline(always)]
    pub fn flag9(&self) -> FLAG9_R {
        FLAG9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Event Flag n"]
    #[inline(always)]
    pub fn flag10(&self) -> FLAG10_R {
        FLAG10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Event Flag n"]
    #[inline(always)]
    pub fn flag11(&self) -> FLAG11_R {
        FLAG11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Event Flag n"]
    #[inline(always)]
    pub fn flag12(&self) -> FLAG12_R {
        FLAG12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Event Flag n"]
    #[inline(always)]
    pub fn flag13(&self) -> FLAG13_R {
        FLAG13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Event Flag n"]
    #[inline(always)]
    pub fn flag14(&self) -> FLAG14_R {
        FLAG14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Event Flag n"]
    #[inline(always)]
    pub fn flag15(&self) -> FLAG15_R {
        FLAG15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn flag0(&mut self) -> FLAG0_W<0> {
        FLAG0_W::new(self)
    }
    #[doc = "Bit 1 - Event Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn flag1(&mut self) -> FLAG1_W<1> {
        FLAG1_W::new(self)
    }
    #[doc = "Bit 2 - Event Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn flag2(&mut self) -> FLAG2_W<2> {
        FLAG2_W::new(self)
    }
    #[doc = "Bit 3 - Event Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn flag3(&mut self) -> FLAG3_W<3> {
        FLAG3_W::new(self)
    }
    #[doc = "Bit 4 - Event Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn flag4(&mut self) -> FLAG4_W<4> {
        FLAG4_W::new(self)
    }
    #[doc = "Bit 5 - Event Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn flag5(&mut self) -> FLAG5_W<5> {
        FLAG5_W::new(self)
    }
    #[doc = "Bit 6 - Event Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn flag6(&mut self) -> FLAG6_W<6> {
        FLAG6_W::new(self)
    }
    #[doc = "Bit 7 - Event Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn flag7(&mut self) -> FLAG7_W<7> {
        FLAG7_W::new(self)
    }
    #[doc = "Bit 8 - Event Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn flag8(&mut self) -> FLAG8_W<8> {
        FLAG8_W::new(self)
    }
    #[doc = "Bit 9 - Event Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn flag9(&mut self) -> FLAG9_W<9> {
        FLAG9_W::new(self)
    }
    #[doc = "Bit 10 - Event Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn flag10(&mut self) -> FLAG10_W<10> {
        FLAG10_W::new(self)
    }
    #[doc = "Bit 11 - Event Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn flag11(&mut self) -> FLAG11_W<11> {
        FLAG11_W::new(self)
    }
    #[doc = "Bit 12 - Event Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn flag12(&mut self) -> FLAG12_W<12> {
        FLAG12_W::new(self)
    }
    #[doc = "Bit 13 - Event Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn flag13(&mut self) -> FLAG13_W<13> {
        FLAG13_W::new(self)
    }
    #[doc = "Bit 14 - Event Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn flag14(&mut self) -> FLAG14_W<14> {
        FLAG14_W::new(self)
    }
    #[doc = "Bit 15 - Event Flag n"]
    #[inline(always)]
    #[must_use]
    pub fn flag15(&mut self) -> FLAG15_W<15> {
        FLAG15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evflag](index.html) module"]
pub struct EVFLAG_SPEC;
impl crate::RegisterSpec for EVFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evflag::R](R) reader structure"]
impl crate::Readable for EVFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evflag::W](W) writer structure"]
impl crate::Writable for EVFLAG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVFLAG to value 0"]
impl crate::Resettable for EVFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
