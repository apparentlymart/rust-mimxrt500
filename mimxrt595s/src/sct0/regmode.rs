#[doc = "Register `REGMODE` reader"]
pub struct R(crate::R<REGMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGMODE` writer"]
pub struct W(crate::W<REGMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGMODE_SPEC>;
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
impl From<crate::W<REGMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGMOD_L0` reader - Register Mode Low n"]
pub type REGMOD_L0_R = crate::BitReader<REGMOD_L0_A>;
#[doc = "Register Mode Low n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_L0_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_L0_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_L0_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_L0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_L0_A {
        match self.bits {
            false => REGMOD_L0_A::MATCH,
            true => REGMOD_L0_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_L0_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_L0_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_L0` writer - Register Mode Low n"]
pub type REGMOD_L0_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_L0_A, O>;
impl<'a, const O: u8> REGMOD_L0_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_L0_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_L0_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_L1` reader - Register Mode Low n"]
pub type REGMOD_L1_R = crate::BitReader<REGMOD_L1_A>;
#[doc = "Register Mode Low n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_L1_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_L1_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_L1_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_L1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_L1_A {
        match self.bits {
            false => REGMOD_L1_A::MATCH,
            true => REGMOD_L1_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_L1_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_L1_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_L1` writer - Register Mode Low n"]
pub type REGMOD_L1_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_L1_A, O>;
impl<'a, const O: u8> REGMOD_L1_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_L1_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_L1_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_L2` reader - Register Mode Low n"]
pub type REGMOD_L2_R = crate::BitReader<REGMOD_L2_A>;
#[doc = "Register Mode Low n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_L2_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_L2_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_L2_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_L2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_L2_A {
        match self.bits {
            false => REGMOD_L2_A::MATCH,
            true => REGMOD_L2_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_L2_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_L2_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_L2` writer - Register Mode Low n"]
pub type REGMOD_L2_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_L2_A, O>;
impl<'a, const O: u8> REGMOD_L2_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_L2_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_L2_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_L3` reader - Register Mode Low n"]
pub type REGMOD_L3_R = crate::BitReader<REGMOD_L3_A>;
#[doc = "Register Mode Low n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_L3_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_L3_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_L3_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_L3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_L3_A {
        match self.bits {
            false => REGMOD_L3_A::MATCH,
            true => REGMOD_L3_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_L3_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_L3_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_L3` writer - Register Mode Low n"]
pub type REGMOD_L3_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_L3_A, O>;
impl<'a, const O: u8> REGMOD_L3_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_L3_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_L3_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_L4` reader - Register Mode Low n"]
pub type REGMOD_L4_R = crate::BitReader<REGMOD_L4_A>;
#[doc = "Register Mode Low n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_L4_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_L4_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_L4_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_L4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_L4_A {
        match self.bits {
            false => REGMOD_L4_A::MATCH,
            true => REGMOD_L4_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_L4_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_L4_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_L4` writer - Register Mode Low n"]
pub type REGMOD_L4_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_L4_A, O>;
impl<'a, const O: u8> REGMOD_L4_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_L4_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_L4_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_L5` reader - Register Mode Low n"]
pub type REGMOD_L5_R = crate::BitReader<REGMOD_L5_A>;
#[doc = "Register Mode Low n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_L5_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_L5_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_L5_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_L5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_L5_A {
        match self.bits {
            false => REGMOD_L5_A::MATCH,
            true => REGMOD_L5_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_L5_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_L5_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_L5` writer - Register Mode Low n"]
pub type REGMOD_L5_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_L5_A, O>;
impl<'a, const O: u8> REGMOD_L5_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_L5_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_L5_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_L6` reader - Register Mode Low n"]
pub type REGMOD_L6_R = crate::BitReader<REGMOD_L6_A>;
#[doc = "Register Mode Low n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_L6_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_L6_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_L6_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_L6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_L6_A {
        match self.bits {
            false => REGMOD_L6_A::MATCH,
            true => REGMOD_L6_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_L6_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_L6_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_L6` writer - Register Mode Low n"]
pub type REGMOD_L6_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_L6_A, O>;
impl<'a, const O: u8> REGMOD_L6_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_L6_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_L6_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_L7` reader - Register Mode Low n"]
pub type REGMOD_L7_R = crate::BitReader<REGMOD_L7_A>;
#[doc = "Register Mode Low n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_L7_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_L7_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_L7_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_L7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_L7_A {
        match self.bits {
            false => REGMOD_L7_A::MATCH,
            true => REGMOD_L7_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_L7_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_L7_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_L7` writer - Register Mode Low n"]
pub type REGMOD_L7_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_L7_A, O>;
impl<'a, const O: u8> REGMOD_L7_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_L7_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_L7_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_L8` reader - Register Mode Low n"]
pub type REGMOD_L8_R = crate::BitReader<REGMOD_L8_A>;
#[doc = "Register Mode Low n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_L8_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_L8_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_L8_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_L8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_L8_A {
        match self.bits {
            false => REGMOD_L8_A::MATCH,
            true => REGMOD_L8_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_L8_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_L8_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_L8` writer - Register Mode Low n"]
pub type REGMOD_L8_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_L8_A, O>;
impl<'a, const O: u8> REGMOD_L8_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_L8_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_L8_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_L9` reader - Register Mode Low n"]
pub type REGMOD_L9_R = crate::BitReader<REGMOD_L9_A>;
#[doc = "Register Mode Low n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_L9_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_L9_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_L9_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_L9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_L9_A {
        match self.bits {
            false => REGMOD_L9_A::MATCH,
            true => REGMOD_L9_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_L9_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_L9_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_L9` writer - Register Mode Low n"]
pub type REGMOD_L9_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_L9_A, O>;
impl<'a, const O: u8> REGMOD_L9_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_L9_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_L9_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_L10` reader - Register Mode Low n"]
pub type REGMOD_L10_R = crate::BitReader<REGMOD_L10_A>;
#[doc = "Register Mode Low n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_L10_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_L10_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_L10_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_L10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_L10_A {
        match self.bits {
            false => REGMOD_L10_A::MATCH,
            true => REGMOD_L10_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_L10_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_L10_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_L10` writer - Register Mode Low n"]
pub type REGMOD_L10_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_L10_A, O>;
impl<'a, const O: u8> REGMOD_L10_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_L10_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_L10_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_L11` reader - Register Mode Low n"]
pub type REGMOD_L11_R = crate::BitReader<REGMOD_L11_A>;
#[doc = "Register Mode Low n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_L11_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_L11_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_L11_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_L11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_L11_A {
        match self.bits {
            false => REGMOD_L11_A::MATCH,
            true => REGMOD_L11_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_L11_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_L11_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_L11` writer - Register Mode Low n"]
pub type REGMOD_L11_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_L11_A, O>;
impl<'a, const O: u8> REGMOD_L11_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_L11_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_L11_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_L12` reader - Register Mode Low n"]
pub type REGMOD_L12_R = crate::BitReader<REGMOD_L12_A>;
#[doc = "Register Mode Low n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_L12_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_L12_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_L12_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_L12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_L12_A {
        match self.bits {
            false => REGMOD_L12_A::MATCH,
            true => REGMOD_L12_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_L12_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_L12_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_L12` writer - Register Mode Low n"]
pub type REGMOD_L12_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_L12_A, O>;
impl<'a, const O: u8> REGMOD_L12_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_L12_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_L12_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_L13` reader - Register Mode Low n"]
pub type REGMOD_L13_R = crate::BitReader<REGMOD_L13_A>;
#[doc = "Register Mode Low n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_L13_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_L13_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_L13_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_L13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_L13_A {
        match self.bits {
            false => REGMOD_L13_A::MATCH,
            true => REGMOD_L13_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_L13_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_L13_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_L13` writer - Register Mode Low n"]
pub type REGMOD_L13_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_L13_A, O>;
impl<'a, const O: u8> REGMOD_L13_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_L13_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_L13_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_L14` reader - Register Mode Low n"]
pub type REGMOD_L14_R = crate::BitReader<REGMOD_L14_A>;
#[doc = "Register Mode Low n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_L14_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_L14_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_L14_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_L14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_L14_A {
        match self.bits {
            false => REGMOD_L14_A::MATCH,
            true => REGMOD_L14_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_L14_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_L14_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_L14` writer - Register Mode Low n"]
pub type REGMOD_L14_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_L14_A, O>;
impl<'a, const O: u8> REGMOD_L14_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_L14_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_L14_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_L15` reader - Register Mode Low n"]
pub type REGMOD_L15_R = crate::BitReader<REGMOD_L15_A>;
#[doc = "Register Mode Low n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_L15_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_L15_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_L15_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_L15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_L15_A {
        match self.bits {
            false => REGMOD_L15_A::MATCH,
            true => REGMOD_L15_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_L15_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_L15_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_L15` writer - Register Mode Low n"]
pub type REGMOD_L15_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_L15_A, O>;
impl<'a, const O: u8> REGMOD_L15_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_L15_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_L15_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_H0` reader - Register Mode High n"]
pub type REGMOD_H0_R = crate::BitReader<REGMOD_H0_A>;
#[doc = "Register Mode High n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_H0_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_H0_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_H0_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_H0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_H0_A {
        match self.bits {
            false => REGMOD_H0_A::MATCH,
            true => REGMOD_H0_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_H0_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_H0_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_H0` writer - Register Mode High n"]
pub type REGMOD_H0_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_H0_A, O>;
impl<'a, const O: u8> REGMOD_H0_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_H0_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_H0_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_H1` reader - Register Mode High n"]
pub type REGMOD_H1_R = crate::BitReader<REGMOD_H1_A>;
#[doc = "Register Mode High n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_H1_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_H1_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_H1_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_H1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_H1_A {
        match self.bits {
            false => REGMOD_H1_A::MATCH,
            true => REGMOD_H1_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_H1_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_H1_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_H1` writer - Register Mode High n"]
pub type REGMOD_H1_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_H1_A, O>;
impl<'a, const O: u8> REGMOD_H1_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_H1_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_H1_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_H2` reader - Register Mode High n"]
pub type REGMOD_H2_R = crate::BitReader<REGMOD_H2_A>;
#[doc = "Register Mode High n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_H2_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_H2_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_H2_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_H2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_H2_A {
        match self.bits {
            false => REGMOD_H2_A::MATCH,
            true => REGMOD_H2_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_H2_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_H2_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_H2` writer - Register Mode High n"]
pub type REGMOD_H2_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_H2_A, O>;
impl<'a, const O: u8> REGMOD_H2_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_H2_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_H2_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_H3` reader - Register Mode High n"]
pub type REGMOD_H3_R = crate::BitReader<REGMOD_H3_A>;
#[doc = "Register Mode High n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_H3_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_H3_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_H3_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_H3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_H3_A {
        match self.bits {
            false => REGMOD_H3_A::MATCH,
            true => REGMOD_H3_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_H3_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_H3_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_H3` writer - Register Mode High n"]
pub type REGMOD_H3_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_H3_A, O>;
impl<'a, const O: u8> REGMOD_H3_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_H3_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_H3_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_H4` reader - Register Mode High n"]
pub type REGMOD_H4_R = crate::BitReader<REGMOD_H4_A>;
#[doc = "Register Mode High n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_H4_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_H4_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_H4_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_H4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_H4_A {
        match self.bits {
            false => REGMOD_H4_A::MATCH,
            true => REGMOD_H4_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_H4_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_H4_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_H4` writer - Register Mode High n"]
pub type REGMOD_H4_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_H4_A, O>;
impl<'a, const O: u8> REGMOD_H4_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_H4_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_H4_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_H5` reader - Register Mode High n"]
pub type REGMOD_H5_R = crate::BitReader<REGMOD_H5_A>;
#[doc = "Register Mode High n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_H5_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_H5_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_H5_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_H5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_H5_A {
        match self.bits {
            false => REGMOD_H5_A::MATCH,
            true => REGMOD_H5_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_H5_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_H5_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_H5` writer - Register Mode High n"]
pub type REGMOD_H5_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_H5_A, O>;
impl<'a, const O: u8> REGMOD_H5_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_H5_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_H5_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_H6` reader - Register Mode High n"]
pub type REGMOD_H6_R = crate::BitReader<REGMOD_H6_A>;
#[doc = "Register Mode High n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_H6_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_H6_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_H6_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_H6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_H6_A {
        match self.bits {
            false => REGMOD_H6_A::MATCH,
            true => REGMOD_H6_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_H6_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_H6_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_H6` writer - Register Mode High n"]
pub type REGMOD_H6_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_H6_A, O>;
impl<'a, const O: u8> REGMOD_H6_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_H6_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_H6_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_H7` reader - Register Mode High n"]
pub type REGMOD_H7_R = crate::BitReader<REGMOD_H7_A>;
#[doc = "Register Mode High n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_H7_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_H7_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_H7_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_H7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_H7_A {
        match self.bits {
            false => REGMOD_H7_A::MATCH,
            true => REGMOD_H7_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_H7_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_H7_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_H7` writer - Register Mode High n"]
pub type REGMOD_H7_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_H7_A, O>;
impl<'a, const O: u8> REGMOD_H7_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_H7_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_H7_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_H8` reader - Register Mode High n"]
pub type REGMOD_H8_R = crate::BitReader<REGMOD_H8_A>;
#[doc = "Register Mode High n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_H8_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_H8_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_H8_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_H8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_H8_A {
        match self.bits {
            false => REGMOD_H8_A::MATCH,
            true => REGMOD_H8_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_H8_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_H8_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_H8` writer - Register Mode High n"]
pub type REGMOD_H8_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_H8_A, O>;
impl<'a, const O: u8> REGMOD_H8_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_H8_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_H8_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_H9` reader - Register Mode High n"]
pub type REGMOD_H9_R = crate::BitReader<REGMOD_H9_A>;
#[doc = "Register Mode High n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_H9_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_H9_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_H9_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_H9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_H9_A {
        match self.bits {
            false => REGMOD_H9_A::MATCH,
            true => REGMOD_H9_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_H9_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_H9_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_H9` writer - Register Mode High n"]
pub type REGMOD_H9_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_H9_A, O>;
impl<'a, const O: u8> REGMOD_H9_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_H9_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_H9_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_H10` reader - Register Mode High n"]
pub type REGMOD_H10_R = crate::BitReader<REGMOD_H10_A>;
#[doc = "Register Mode High n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_H10_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_H10_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_H10_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_H10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_H10_A {
        match self.bits {
            false => REGMOD_H10_A::MATCH,
            true => REGMOD_H10_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_H10_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_H10_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_H10` writer - Register Mode High n"]
pub type REGMOD_H10_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_H10_A, O>;
impl<'a, const O: u8> REGMOD_H10_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_H10_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_H10_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_H11` reader - Register Mode High n"]
pub type REGMOD_H11_R = crate::BitReader<REGMOD_H11_A>;
#[doc = "Register Mode High n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_H11_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_H11_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_H11_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_H11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_H11_A {
        match self.bits {
            false => REGMOD_H11_A::MATCH,
            true => REGMOD_H11_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_H11_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_H11_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_H11` writer - Register Mode High n"]
pub type REGMOD_H11_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_H11_A, O>;
impl<'a, const O: u8> REGMOD_H11_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_H11_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_H11_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_H12` reader - Register Mode High n"]
pub type REGMOD_H12_R = crate::BitReader<REGMOD_H12_A>;
#[doc = "Register Mode High n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_H12_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_H12_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_H12_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_H12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_H12_A {
        match self.bits {
            false => REGMOD_H12_A::MATCH,
            true => REGMOD_H12_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_H12_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_H12_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_H12` writer - Register Mode High n"]
pub type REGMOD_H12_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_H12_A, O>;
impl<'a, const O: u8> REGMOD_H12_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_H12_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_H12_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_H13` reader - Register Mode High n"]
pub type REGMOD_H13_R = crate::BitReader<REGMOD_H13_A>;
#[doc = "Register Mode High n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_H13_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_H13_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_H13_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_H13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_H13_A {
        match self.bits {
            false => REGMOD_H13_A::MATCH,
            true => REGMOD_H13_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_H13_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_H13_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_H13` writer - Register Mode High n"]
pub type REGMOD_H13_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_H13_A, O>;
impl<'a, const O: u8> REGMOD_H13_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_H13_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_H13_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_H14` reader - Register Mode High n"]
pub type REGMOD_H14_R = crate::BitReader<REGMOD_H14_A>;
#[doc = "Register Mode High n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_H14_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_H14_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_H14_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_H14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_H14_A {
        match self.bits {
            false => REGMOD_H14_A::MATCH,
            true => REGMOD_H14_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_H14_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_H14_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_H14` writer - Register Mode High n"]
pub type REGMOD_H14_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_H14_A, O>;
impl<'a, const O: u8> REGMOD_H14_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_H14_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_H14_A::CAPTURE)
    }
}
#[doc = "Field `REGMOD_H15` reader - Register Mode High n"]
pub type REGMOD_H15_R = crate::BitReader<REGMOD_H15_A>;
#[doc = "Register Mode High n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMOD_H15_A {
    #[doc = "0: Match. Register n operates as a match register"]
    MATCH = 0,
    #[doc = "1: Capture. Register n operates as a capture register"]
    CAPTURE = 1,
}
impl From<REGMOD_H15_A> for bool {
    #[inline(always)]
    fn from(variant: REGMOD_H15_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMOD_H15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMOD_H15_A {
        match self.bits {
            false => REGMOD_H15_A::MATCH,
            true => REGMOD_H15_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == REGMOD_H15_A::MATCH
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == REGMOD_H15_A::CAPTURE
    }
}
#[doc = "Field `REGMOD_H15` writer - Register Mode High n"]
pub type REGMOD_H15_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGMODE_SPEC, REGMOD_H15_A, O>;
impl<'a, const O: u8> REGMOD_H15_W<'a, O> {
    #[doc = "Match. Register n operates as a match register"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(REGMOD_H15_A::MATCH)
    }
    #[doc = "Capture. Register n operates as a capture register"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(REGMOD_H15_A::CAPTURE)
    }
}
impl R {
    #[doc = "Bit 0 - Register Mode Low n"]
    #[inline(always)]
    pub fn regmod_l0(&self) -> REGMOD_L0_R {
        REGMOD_L0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Register Mode Low n"]
    #[inline(always)]
    pub fn regmod_l1(&self) -> REGMOD_L1_R {
        REGMOD_L1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Register Mode Low n"]
    #[inline(always)]
    pub fn regmod_l2(&self) -> REGMOD_L2_R {
        REGMOD_L2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Register Mode Low n"]
    #[inline(always)]
    pub fn regmod_l3(&self) -> REGMOD_L3_R {
        REGMOD_L3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Register Mode Low n"]
    #[inline(always)]
    pub fn regmod_l4(&self) -> REGMOD_L4_R {
        REGMOD_L4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Register Mode Low n"]
    #[inline(always)]
    pub fn regmod_l5(&self) -> REGMOD_L5_R {
        REGMOD_L5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Register Mode Low n"]
    #[inline(always)]
    pub fn regmod_l6(&self) -> REGMOD_L6_R {
        REGMOD_L6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Register Mode Low n"]
    #[inline(always)]
    pub fn regmod_l7(&self) -> REGMOD_L7_R {
        REGMOD_L7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Register Mode Low n"]
    #[inline(always)]
    pub fn regmod_l8(&self) -> REGMOD_L8_R {
        REGMOD_L8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Register Mode Low n"]
    #[inline(always)]
    pub fn regmod_l9(&self) -> REGMOD_L9_R {
        REGMOD_L9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Register Mode Low n"]
    #[inline(always)]
    pub fn regmod_l10(&self) -> REGMOD_L10_R {
        REGMOD_L10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Register Mode Low n"]
    #[inline(always)]
    pub fn regmod_l11(&self) -> REGMOD_L11_R {
        REGMOD_L11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Register Mode Low n"]
    #[inline(always)]
    pub fn regmod_l12(&self) -> REGMOD_L12_R {
        REGMOD_L12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Register Mode Low n"]
    #[inline(always)]
    pub fn regmod_l13(&self) -> REGMOD_L13_R {
        REGMOD_L13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Register Mode Low n"]
    #[inline(always)]
    pub fn regmod_l14(&self) -> REGMOD_L14_R {
        REGMOD_L14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Register Mode Low n"]
    #[inline(always)]
    pub fn regmod_l15(&self) -> REGMOD_L15_R {
        REGMOD_L15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Register Mode High n"]
    #[inline(always)]
    pub fn regmod_h0(&self) -> REGMOD_H0_R {
        REGMOD_H0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Register Mode High n"]
    #[inline(always)]
    pub fn regmod_h1(&self) -> REGMOD_H1_R {
        REGMOD_H1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Register Mode High n"]
    #[inline(always)]
    pub fn regmod_h2(&self) -> REGMOD_H2_R {
        REGMOD_H2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Register Mode High n"]
    #[inline(always)]
    pub fn regmod_h3(&self) -> REGMOD_H3_R {
        REGMOD_H3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Register Mode High n"]
    #[inline(always)]
    pub fn regmod_h4(&self) -> REGMOD_H4_R {
        REGMOD_H4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Register Mode High n"]
    #[inline(always)]
    pub fn regmod_h5(&self) -> REGMOD_H5_R {
        REGMOD_H5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Register Mode High n"]
    #[inline(always)]
    pub fn regmod_h6(&self) -> REGMOD_H6_R {
        REGMOD_H6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Register Mode High n"]
    #[inline(always)]
    pub fn regmod_h7(&self) -> REGMOD_H7_R {
        REGMOD_H7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Register Mode High n"]
    #[inline(always)]
    pub fn regmod_h8(&self) -> REGMOD_H8_R {
        REGMOD_H8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Register Mode High n"]
    #[inline(always)]
    pub fn regmod_h9(&self) -> REGMOD_H9_R {
        REGMOD_H9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Register Mode High n"]
    #[inline(always)]
    pub fn regmod_h10(&self) -> REGMOD_H10_R {
        REGMOD_H10_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Register Mode High n"]
    #[inline(always)]
    pub fn regmod_h11(&self) -> REGMOD_H11_R {
        REGMOD_H11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Register Mode High n"]
    #[inline(always)]
    pub fn regmod_h12(&self) -> REGMOD_H12_R {
        REGMOD_H12_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Register Mode High n"]
    #[inline(always)]
    pub fn regmod_h13(&self) -> REGMOD_H13_R {
        REGMOD_H13_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Register Mode High n"]
    #[inline(always)]
    pub fn regmod_h14(&self) -> REGMOD_H14_R {
        REGMOD_H14_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Register Mode High n"]
    #[inline(always)]
    pub fn regmod_h15(&self) -> REGMOD_H15_R {
        REGMOD_H15_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Register Mode Low n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_l0(&mut self) -> REGMOD_L0_W<0> {
        REGMOD_L0_W::new(self)
    }
    #[doc = "Bit 1 - Register Mode Low n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_l1(&mut self) -> REGMOD_L1_W<1> {
        REGMOD_L1_W::new(self)
    }
    #[doc = "Bit 2 - Register Mode Low n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_l2(&mut self) -> REGMOD_L2_W<2> {
        REGMOD_L2_W::new(self)
    }
    #[doc = "Bit 3 - Register Mode Low n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_l3(&mut self) -> REGMOD_L3_W<3> {
        REGMOD_L3_W::new(self)
    }
    #[doc = "Bit 4 - Register Mode Low n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_l4(&mut self) -> REGMOD_L4_W<4> {
        REGMOD_L4_W::new(self)
    }
    #[doc = "Bit 5 - Register Mode Low n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_l5(&mut self) -> REGMOD_L5_W<5> {
        REGMOD_L5_W::new(self)
    }
    #[doc = "Bit 6 - Register Mode Low n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_l6(&mut self) -> REGMOD_L6_W<6> {
        REGMOD_L6_W::new(self)
    }
    #[doc = "Bit 7 - Register Mode Low n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_l7(&mut self) -> REGMOD_L7_W<7> {
        REGMOD_L7_W::new(self)
    }
    #[doc = "Bit 8 - Register Mode Low n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_l8(&mut self) -> REGMOD_L8_W<8> {
        REGMOD_L8_W::new(self)
    }
    #[doc = "Bit 9 - Register Mode Low n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_l9(&mut self) -> REGMOD_L9_W<9> {
        REGMOD_L9_W::new(self)
    }
    #[doc = "Bit 10 - Register Mode Low n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_l10(&mut self) -> REGMOD_L10_W<10> {
        REGMOD_L10_W::new(self)
    }
    #[doc = "Bit 11 - Register Mode Low n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_l11(&mut self) -> REGMOD_L11_W<11> {
        REGMOD_L11_W::new(self)
    }
    #[doc = "Bit 12 - Register Mode Low n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_l12(&mut self) -> REGMOD_L12_W<12> {
        REGMOD_L12_W::new(self)
    }
    #[doc = "Bit 13 - Register Mode Low n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_l13(&mut self) -> REGMOD_L13_W<13> {
        REGMOD_L13_W::new(self)
    }
    #[doc = "Bit 14 - Register Mode Low n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_l14(&mut self) -> REGMOD_L14_W<14> {
        REGMOD_L14_W::new(self)
    }
    #[doc = "Bit 15 - Register Mode Low n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_l15(&mut self) -> REGMOD_L15_W<15> {
        REGMOD_L15_W::new(self)
    }
    #[doc = "Bit 16 - Register Mode High n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_h0(&mut self) -> REGMOD_H0_W<16> {
        REGMOD_H0_W::new(self)
    }
    #[doc = "Bit 17 - Register Mode High n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_h1(&mut self) -> REGMOD_H1_W<17> {
        REGMOD_H1_W::new(self)
    }
    #[doc = "Bit 18 - Register Mode High n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_h2(&mut self) -> REGMOD_H2_W<18> {
        REGMOD_H2_W::new(self)
    }
    #[doc = "Bit 19 - Register Mode High n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_h3(&mut self) -> REGMOD_H3_W<19> {
        REGMOD_H3_W::new(self)
    }
    #[doc = "Bit 20 - Register Mode High n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_h4(&mut self) -> REGMOD_H4_W<20> {
        REGMOD_H4_W::new(self)
    }
    #[doc = "Bit 21 - Register Mode High n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_h5(&mut self) -> REGMOD_H5_W<21> {
        REGMOD_H5_W::new(self)
    }
    #[doc = "Bit 22 - Register Mode High n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_h6(&mut self) -> REGMOD_H6_W<22> {
        REGMOD_H6_W::new(self)
    }
    #[doc = "Bit 23 - Register Mode High n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_h7(&mut self) -> REGMOD_H7_W<23> {
        REGMOD_H7_W::new(self)
    }
    #[doc = "Bit 24 - Register Mode High n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_h8(&mut self) -> REGMOD_H8_W<24> {
        REGMOD_H8_W::new(self)
    }
    #[doc = "Bit 25 - Register Mode High n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_h9(&mut self) -> REGMOD_H9_W<25> {
        REGMOD_H9_W::new(self)
    }
    #[doc = "Bit 26 - Register Mode High n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_h10(&mut self) -> REGMOD_H10_W<26> {
        REGMOD_H10_W::new(self)
    }
    #[doc = "Bit 27 - Register Mode High n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_h11(&mut self) -> REGMOD_H11_W<27> {
        REGMOD_H11_W::new(self)
    }
    #[doc = "Bit 28 - Register Mode High n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_h12(&mut self) -> REGMOD_H12_W<28> {
        REGMOD_H12_W::new(self)
    }
    #[doc = "Bit 29 - Register Mode High n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_h13(&mut self) -> REGMOD_H13_W<29> {
        REGMOD_H13_W::new(self)
    }
    #[doc = "Bit 30 - Register Mode High n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_h14(&mut self) -> REGMOD_H14_W<30> {
        REGMOD_H14_W::new(self)
    }
    #[doc = "Bit 31 - Register Mode High n"]
    #[inline(always)]
    #[must_use]
    pub fn regmod_h15(&mut self) -> REGMOD_H15_W<31> {
        REGMOD_H15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Match/Capture Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regmode](index.html) module"]
pub struct REGMODE_SPEC;
impl crate::RegisterSpec for REGMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [regmode::R](R) reader structure"]
impl crate::Readable for REGMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [regmode::W](W) writer structure"]
impl crate::Writable for REGMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGMODE to value 0"]
impl crate::Resettable for REGMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
