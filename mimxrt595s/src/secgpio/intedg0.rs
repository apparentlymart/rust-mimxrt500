#[doc = "Register `INTEDG0` reader"]
pub struct R(crate::R<INTEDG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEDG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEDG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEDG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEDG0` writer"]
pub struct W(crate::W<INTEDG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEDG0_SPEC>;
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
impl From<crate::W<INTEDG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEDG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDGE0` reader - Edge or level mode select bits."]
pub type EDGE0_R = crate::BitReader<EDGE0_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE0_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE0_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE0_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE0_A {
        match self.bits {
            false => EDGE0_A::LEVEL,
            true => EDGE0_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE0_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE0_A::EDGE
    }
}
#[doc = "Field `EDGE0` writer - Edge or level mode select bits."]
pub type EDGE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE0_A, O>;
impl<'a, const O: u8> EDGE0_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE0_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE0_A::EDGE)
    }
}
#[doc = "Field `EDGE1` reader - Edge or level mode select bits."]
pub type EDGE1_R = crate::BitReader<EDGE1_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE1_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE1_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE1_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE1_A {
        match self.bits {
            false => EDGE1_A::LEVEL,
            true => EDGE1_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE1_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE1_A::EDGE
    }
}
#[doc = "Field `EDGE1` writer - Edge or level mode select bits."]
pub type EDGE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE1_A, O>;
impl<'a, const O: u8> EDGE1_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE1_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE1_A::EDGE)
    }
}
#[doc = "Field `EDGE2` reader - Edge or level mode select bits."]
pub type EDGE2_R = crate::BitReader<EDGE2_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE2_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE2_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE2_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE2_A {
        match self.bits {
            false => EDGE2_A::LEVEL,
            true => EDGE2_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE2_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE2_A::EDGE
    }
}
#[doc = "Field `EDGE2` writer - Edge or level mode select bits."]
pub type EDGE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE2_A, O>;
impl<'a, const O: u8> EDGE2_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE2_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE2_A::EDGE)
    }
}
#[doc = "Field `EDGE3` reader - Edge or level mode select bits."]
pub type EDGE3_R = crate::BitReader<EDGE3_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE3_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE3_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE3_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE3_A {
        match self.bits {
            false => EDGE3_A::LEVEL,
            true => EDGE3_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE3_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE3_A::EDGE
    }
}
#[doc = "Field `EDGE3` writer - Edge or level mode select bits."]
pub type EDGE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE3_A, O>;
impl<'a, const O: u8> EDGE3_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE3_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE3_A::EDGE)
    }
}
#[doc = "Field `EDGE4` reader - Edge or level mode select bits."]
pub type EDGE4_R = crate::BitReader<EDGE4_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE4_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE4_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE4_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE4_A {
        match self.bits {
            false => EDGE4_A::LEVEL,
            true => EDGE4_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE4_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE4_A::EDGE
    }
}
#[doc = "Field `EDGE4` writer - Edge or level mode select bits."]
pub type EDGE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE4_A, O>;
impl<'a, const O: u8> EDGE4_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE4_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE4_A::EDGE)
    }
}
#[doc = "Field `EDGE5` reader - Edge or level mode select bits."]
pub type EDGE5_R = crate::BitReader<EDGE5_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE5_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE5_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE5_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE5_A {
        match self.bits {
            false => EDGE5_A::LEVEL,
            true => EDGE5_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE5_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE5_A::EDGE
    }
}
#[doc = "Field `EDGE5` writer - Edge or level mode select bits."]
pub type EDGE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE5_A, O>;
impl<'a, const O: u8> EDGE5_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE5_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE5_A::EDGE)
    }
}
#[doc = "Field `EDGE6` reader - Edge or level mode select bits."]
pub type EDGE6_R = crate::BitReader<EDGE6_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE6_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE6_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE6_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE6_A {
        match self.bits {
            false => EDGE6_A::LEVEL,
            true => EDGE6_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE6_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE6_A::EDGE
    }
}
#[doc = "Field `EDGE6` writer - Edge or level mode select bits."]
pub type EDGE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE6_A, O>;
impl<'a, const O: u8> EDGE6_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE6_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE6_A::EDGE)
    }
}
#[doc = "Field `EDGE7` reader - Edge or level mode select bits."]
pub type EDGE7_R = crate::BitReader<EDGE7_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE7_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE7_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE7_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE7_A {
        match self.bits {
            false => EDGE7_A::LEVEL,
            true => EDGE7_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE7_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE7_A::EDGE
    }
}
#[doc = "Field `EDGE7` writer - Edge or level mode select bits."]
pub type EDGE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE7_A, O>;
impl<'a, const O: u8> EDGE7_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE7_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE7_A::EDGE)
    }
}
#[doc = "Field `EDGE8` reader - Edge or level mode select bits."]
pub type EDGE8_R = crate::BitReader<EDGE8_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE8_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE8_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE8_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE8_A {
        match self.bits {
            false => EDGE8_A::LEVEL,
            true => EDGE8_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE8_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE8_A::EDGE
    }
}
#[doc = "Field `EDGE8` writer - Edge or level mode select bits."]
pub type EDGE8_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE8_A, O>;
impl<'a, const O: u8> EDGE8_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE8_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE8_A::EDGE)
    }
}
#[doc = "Field `EDGE9` reader - Edge or level mode select bits."]
pub type EDGE9_R = crate::BitReader<EDGE9_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE9_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE9_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE9_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE9_A {
        match self.bits {
            false => EDGE9_A::LEVEL,
            true => EDGE9_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE9_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE9_A::EDGE
    }
}
#[doc = "Field `EDGE9` writer - Edge or level mode select bits."]
pub type EDGE9_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE9_A, O>;
impl<'a, const O: u8> EDGE9_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE9_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE9_A::EDGE)
    }
}
#[doc = "Field `EDGE10` reader - Edge or level mode select bits."]
pub type EDGE10_R = crate::BitReader<EDGE10_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE10_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE10_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE10_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE10_A {
        match self.bits {
            false => EDGE10_A::LEVEL,
            true => EDGE10_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE10_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE10_A::EDGE
    }
}
#[doc = "Field `EDGE10` writer - Edge or level mode select bits."]
pub type EDGE10_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE10_A, O>;
impl<'a, const O: u8> EDGE10_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE10_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE10_A::EDGE)
    }
}
#[doc = "Field `EDGE11` reader - Edge or level mode select bits."]
pub type EDGE11_R = crate::BitReader<EDGE11_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE11_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE11_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE11_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE11_A {
        match self.bits {
            false => EDGE11_A::LEVEL,
            true => EDGE11_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE11_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE11_A::EDGE
    }
}
#[doc = "Field `EDGE11` writer - Edge or level mode select bits."]
pub type EDGE11_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE11_A, O>;
impl<'a, const O: u8> EDGE11_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE11_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE11_A::EDGE)
    }
}
#[doc = "Field `EDGE12` reader - Edge or level mode select bits."]
pub type EDGE12_R = crate::BitReader<EDGE12_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE12_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE12_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE12_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE12_A {
        match self.bits {
            false => EDGE12_A::LEVEL,
            true => EDGE12_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE12_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE12_A::EDGE
    }
}
#[doc = "Field `EDGE12` writer - Edge or level mode select bits."]
pub type EDGE12_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE12_A, O>;
impl<'a, const O: u8> EDGE12_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE12_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE12_A::EDGE)
    }
}
#[doc = "Field `EDGE13` reader - Edge or level mode select bits."]
pub type EDGE13_R = crate::BitReader<EDGE13_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE13_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE13_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE13_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE13_A {
        match self.bits {
            false => EDGE13_A::LEVEL,
            true => EDGE13_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE13_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE13_A::EDGE
    }
}
#[doc = "Field `EDGE13` writer - Edge or level mode select bits."]
pub type EDGE13_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE13_A, O>;
impl<'a, const O: u8> EDGE13_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE13_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE13_A::EDGE)
    }
}
#[doc = "Field `EDGE14` reader - Edge or level mode select bits."]
pub type EDGE14_R = crate::BitReader<EDGE14_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE14_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE14_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE14_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE14_A {
        match self.bits {
            false => EDGE14_A::LEVEL,
            true => EDGE14_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE14_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE14_A::EDGE
    }
}
#[doc = "Field `EDGE14` writer - Edge or level mode select bits."]
pub type EDGE14_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE14_A, O>;
impl<'a, const O: u8> EDGE14_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE14_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE14_A::EDGE)
    }
}
#[doc = "Field `EDGE15` reader - Edge or level mode select bits."]
pub type EDGE15_R = crate::BitReader<EDGE15_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE15_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE15_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE15_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE15_A {
        match self.bits {
            false => EDGE15_A::LEVEL,
            true => EDGE15_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE15_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE15_A::EDGE
    }
}
#[doc = "Field `EDGE15` writer - Edge or level mode select bits."]
pub type EDGE15_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE15_A, O>;
impl<'a, const O: u8> EDGE15_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE15_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE15_A::EDGE)
    }
}
#[doc = "Field `EDGE16` reader - Edge or level mode select bits."]
pub type EDGE16_R = crate::BitReader<EDGE16_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE16_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE16_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE16_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE16_A {
        match self.bits {
            false => EDGE16_A::LEVEL,
            true => EDGE16_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE16_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE16_A::EDGE
    }
}
#[doc = "Field `EDGE16` writer - Edge or level mode select bits."]
pub type EDGE16_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE16_A, O>;
impl<'a, const O: u8> EDGE16_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE16_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE16_A::EDGE)
    }
}
#[doc = "Field `EDGE17` reader - Edge or level mode select bits."]
pub type EDGE17_R = crate::BitReader<EDGE17_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE17_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE17_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE17_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE17_A {
        match self.bits {
            false => EDGE17_A::LEVEL,
            true => EDGE17_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE17_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE17_A::EDGE
    }
}
#[doc = "Field `EDGE17` writer - Edge or level mode select bits."]
pub type EDGE17_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE17_A, O>;
impl<'a, const O: u8> EDGE17_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE17_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE17_A::EDGE)
    }
}
#[doc = "Field `EDGE18` reader - Edge or level mode select bits."]
pub type EDGE18_R = crate::BitReader<EDGE18_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE18_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE18_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE18_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE18_A {
        match self.bits {
            false => EDGE18_A::LEVEL,
            true => EDGE18_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE18_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE18_A::EDGE
    }
}
#[doc = "Field `EDGE18` writer - Edge or level mode select bits."]
pub type EDGE18_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE18_A, O>;
impl<'a, const O: u8> EDGE18_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE18_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE18_A::EDGE)
    }
}
#[doc = "Field `EDGE19` reader - Edge or level mode select bits."]
pub type EDGE19_R = crate::BitReader<EDGE19_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE19_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE19_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE19_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE19_A {
        match self.bits {
            false => EDGE19_A::LEVEL,
            true => EDGE19_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE19_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE19_A::EDGE
    }
}
#[doc = "Field `EDGE19` writer - Edge or level mode select bits."]
pub type EDGE19_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE19_A, O>;
impl<'a, const O: u8> EDGE19_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE19_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE19_A::EDGE)
    }
}
#[doc = "Field `EDGE20` reader - Edge or level mode select bits."]
pub type EDGE20_R = crate::BitReader<EDGE20_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE20_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE20_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE20_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE20_A {
        match self.bits {
            false => EDGE20_A::LEVEL,
            true => EDGE20_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE20_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE20_A::EDGE
    }
}
#[doc = "Field `EDGE20` writer - Edge or level mode select bits."]
pub type EDGE20_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE20_A, O>;
impl<'a, const O: u8> EDGE20_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE20_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE20_A::EDGE)
    }
}
#[doc = "Field `EDGE21` reader - Edge or level mode select bits."]
pub type EDGE21_R = crate::BitReader<EDGE21_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE21_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE21_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE21_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE21_A {
        match self.bits {
            false => EDGE21_A::LEVEL,
            true => EDGE21_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE21_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE21_A::EDGE
    }
}
#[doc = "Field `EDGE21` writer - Edge or level mode select bits."]
pub type EDGE21_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE21_A, O>;
impl<'a, const O: u8> EDGE21_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE21_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE21_A::EDGE)
    }
}
#[doc = "Field `EDGE22` reader - Edge or level mode select bits."]
pub type EDGE22_R = crate::BitReader<EDGE22_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE22_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE22_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE22_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE22_A {
        match self.bits {
            false => EDGE22_A::LEVEL,
            true => EDGE22_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE22_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE22_A::EDGE
    }
}
#[doc = "Field `EDGE22` writer - Edge or level mode select bits."]
pub type EDGE22_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE22_A, O>;
impl<'a, const O: u8> EDGE22_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE22_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE22_A::EDGE)
    }
}
#[doc = "Field `EDGE23` reader - Edge or level mode select bits."]
pub type EDGE23_R = crate::BitReader<EDGE23_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE23_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE23_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE23_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE23_A {
        match self.bits {
            false => EDGE23_A::LEVEL,
            true => EDGE23_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE23_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE23_A::EDGE
    }
}
#[doc = "Field `EDGE23` writer - Edge or level mode select bits."]
pub type EDGE23_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE23_A, O>;
impl<'a, const O: u8> EDGE23_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE23_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE23_A::EDGE)
    }
}
#[doc = "Field `EDGE24` reader - Edge or level mode select bits."]
pub type EDGE24_R = crate::BitReader<EDGE24_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE24_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE24_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE24_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE24_A {
        match self.bits {
            false => EDGE24_A::LEVEL,
            true => EDGE24_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE24_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE24_A::EDGE
    }
}
#[doc = "Field `EDGE24` writer - Edge or level mode select bits."]
pub type EDGE24_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE24_A, O>;
impl<'a, const O: u8> EDGE24_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE24_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE24_A::EDGE)
    }
}
#[doc = "Field `EDGE25` reader - Edge or level mode select bits."]
pub type EDGE25_R = crate::BitReader<EDGE25_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE25_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE25_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE25_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE25_A {
        match self.bits {
            false => EDGE25_A::LEVEL,
            true => EDGE25_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE25_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE25_A::EDGE
    }
}
#[doc = "Field `EDGE25` writer - Edge or level mode select bits."]
pub type EDGE25_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE25_A, O>;
impl<'a, const O: u8> EDGE25_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE25_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE25_A::EDGE)
    }
}
#[doc = "Field `EDGE26` reader - Edge or level mode select bits."]
pub type EDGE26_R = crate::BitReader<EDGE26_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE26_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE26_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE26_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE26_A {
        match self.bits {
            false => EDGE26_A::LEVEL,
            true => EDGE26_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE26_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE26_A::EDGE
    }
}
#[doc = "Field `EDGE26` writer - Edge or level mode select bits."]
pub type EDGE26_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE26_A, O>;
impl<'a, const O: u8> EDGE26_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE26_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE26_A::EDGE)
    }
}
#[doc = "Field `EDGE27` reader - Edge or level mode select bits."]
pub type EDGE27_R = crate::BitReader<EDGE27_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE27_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE27_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE27_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE27_A {
        match self.bits {
            false => EDGE27_A::LEVEL,
            true => EDGE27_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE27_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE27_A::EDGE
    }
}
#[doc = "Field `EDGE27` writer - Edge or level mode select bits."]
pub type EDGE27_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE27_A, O>;
impl<'a, const O: u8> EDGE27_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE27_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE27_A::EDGE)
    }
}
#[doc = "Field `EDGE28` reader - Edge or level mode select bits."]
pub type EDGE28_R = crate::BitReader<EDGE28_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE28_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE28_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE28_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE28_A {
        match self.bits {
            false => EDGE28_A::LEVEL,
            true => EDGE28_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE28_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE28_A::EDGE
    }
}
#[doc = "Field `EDGE28` writer - Edge or level mode select bits."]
pub type EDGE28_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE28_A, O>;
impl<'a, const O: u8> EDGE28_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE28_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE28_A::EDGE)
    }
}
#[doc = "Field `EDGE29` reader - Edge or level mode select bits."]
pub type EDGE29_R = crate::BitReader<EDGE29_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE29_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE29_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE29_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE29_A {
        match self.bits {
            false => EDGE29_A::LEVEL,
            true => EDGE29_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE29_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE29_A::EDGE
    }
}
#[doc = "Field `EDGE29` writer - Edge or level mode select bits."]
pub type EDGE29_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE29_A, O>;
impl<'a, const O: u8> EDGE29_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE29_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE29_A::EDGE)
    }
}
#[doc = "Field `EDGE30` reader - Edge or level mode select bits."]
pub type EDGE30_R = crate::BitReader<EDGE30_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE30_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE30_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE30_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE30_A {
        match self.bits {
            false => EDGE30_A::LEVEL,
            true => EDGE30_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE30_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE30_A::EDGE
    }
}
#[doc = "Field `EDGE30` writer - Edge or level mode select bits."]
pub type EDGE30_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE30_A, O>;
impl<'a, const O: u8> EDGE30_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE30_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE30_A::EDGE)
    }
}
#[doc = "Field `EDGE31` reader - Edge or level mode select bits."]
pub type EDGE31_R = crate::BitReader<EDGE31_A>;
#[doc = "Edge or level mode select bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE31_A {
    #[doc = "0: Level mode"]
    LEVEL = 0,
    #[doc = "1: Edge mode"]
    EDGE = 1,
}
impl From<EDGE31_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE31_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE31_A {
        match self.bits {
            false => EDGE31_A::LEVEL,
            true => EDGE31_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EDGE31_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EDGE31_A::EDGE
    }
}
#[doc = "Field `EDGE31` writer - Edge or level mode select bits."]
pub type EDGE31_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEDG0_SPEC, EDGE31_A, O>;
impl<'a, const O: u8> EDGE31_W<'a, O> {
    #[doc = "Level mode"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EDGE31_A::LEVEL)
    }
    #[doc = "Edge mode"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EDGE31_A::EDGE)
    }
}
impl R {
    #[doc = "Bit 0 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge0(&self) -> EDGE0_R {
        EDGE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge1(&self) -> EDGE1_R {
        EDGE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge2(&self) -> EDGE2_R {
        EDGE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge3(&self) -> EDGE3_R {
        EDGE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge4(&self) -> EDGE4_R {
        EDGE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge5(&self) -> EDGE5_R {
        EDGE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge6(&self) -> EDGE6_R {
        EDGE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge7(&self) -> EDGE7_R {
        EDGE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge8(&self) -> EDGE8_R {
        EDGE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge9(&self) -> EDGE9_R {
        EDGE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge10(&self) -> EDGE10_R {
        EDGE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge11(&self) -> EDGE11_R {
        EDGE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge12(&self) -> EDGE12_R {
        EDGE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge13(&self) -> EDGE13_R {
        EDGE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge14(&self) -> EDGE14_R {
        EDGE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge15(&self) -> EDGE15_R {
        EDGE15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge16(&self) -> EDGE16_R {
        EDGE16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge17(&self) -> EDGE17_R {
        EDGE17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge18(&self) -> EDGE18_R {
        EDGE18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge19(&self) -> EDGE19_R {
        EDGE19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge20(&self) -> EDGE20_R {
        EDGE20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge21(&self) -> EDGE21_R {
        EDGE21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge22(&self) -> EDGE22_R {
        EDGE22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge23(&self) -> EDGE23_R {
        EDGE23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge24(&self) -> EDGE24_R {
        EDGE24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge25(&self) -> EDGE25_R {
        EDGE25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge26(&self) -> EDGE26_R {
        EDGE26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge27(&self) -> EDGE27_R {
        EDGE27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge28(&self) -> EDGE28_R {
        EDGE28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge29(&self) -> EDGE29_R {
        EDGE29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge30(&self) -> EDGE30_R {
        EDGE30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Edge or level mode select bits."]
    #[inline(always)]
    pub fn edge31(&self) -> EDGE31_R {
        EDGE31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge0(&mut self) -> EDGE0_W<0> {
        EDGE0_W::new(self)
    }
    #[doc = "Bit 1 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge1(&mut self) -> EDGE1_W<1> {
        EDGE1_W::new(self)
    }
    #[doc = "Bit 2 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge2(&mut self) -> EDGE2_W<2> {
        EDGE2_W::new(self)
    }
    #[doc = "Bit 3 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge3(&mut self) -> EDGE3_W<3> {
        EDGE3_W::new(self)
    }
    #[doc = "Bit 4 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge4(&mut self) -> EDGE4_W<4> {
        EDGE4_W::new(self)
    }
    #[doc = "Bit 5 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge5(&mut self) -> EDGE5_W<5> {
        EDGE5_W::new(self)
    }
    #[doc = "Bit 6 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge6(&mut self) -> EDGE6_W<6> {
        EDGE6_W::new(self)
    }
    #[doc = "Bit 7 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge7(&mut self) -> EDGE7_W<7> {
        EDGE7_W::new(self)
    }
    #[doc = "Bit 8 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge8(&mut self) -> EDGE8_W<8> {
        EDGE8_W::new(self)
    }
    #[doc = "Bit 9 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge9(&mut self) -> EDGE9_W<9> {
        EDGE9_W::new(self)
    }
    #[doc = "Bit 10 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge10(&mut self) -> EDGE10_W<10> {
        EDGE10_W::new(self)
    }
    #[doc = "Bit 11 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge11(&mut self) -> EDGE11_W<11> {
        EDGE11_W::new(self)
    }
    #[doc = "Bit 12 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge12(&mut self) -> EDGE12_W<12> {
        EDGE12_W::new(self)
    }
    #[doc = "Bit 13 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge13(&mut self) -> EDGE13_W<13> {
        EDGE13_W::new(self)
    }
    #[doc = "Bit 14 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge14(&mut self) -> EDGE14_W<14> {
        EDGE14_W::new(self)
    }
    #[doc = "Bit 15 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge15(&mut self) -> EDGE15_W<15> {
        EDGE15_W::new(self)
    }
    #[doc = "Bit 16 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge16(&mut self) -> EDGE16_W<16> {
        EDGE16_W::new(self)
    }
    #[doc = "Bit 17 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge17(&mut self) -> EDGE17_W<17> {
        EDGE17_W::new(self)
    }
    #[doc = "Bit 18 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge18(&mut self) -> EDGE18_W<18> {
        EDGE18_W::new(self)
    }
    #[doc = "Bit 19 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge19(&mut self) -> EDGE19_W<19> {
        EDGE19_W::new(self)
    }
    #[doc = "Bit 20 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge20(&mut self) -> EDGE20_W<20> {
        EDGE20_W::new(self)
    }
    #[doc = "Bit 21 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge21(&mut self) -> EDGE21_W<21> {
        EDGE21_W::new(self)
    }
    #[doc = "Bit 22 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge22(&mut self) -> EDGE22_W<22> {
        EDGE22_W::new(self)
    }
    #[doc = "Bit 23 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge23(&mut self) -> EDGE23_W<23> {
        EDGE23_W::new(self)
    }
    #[doc = "Bit 24 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge24(&mut self) -> EDGE24_W<24> {
        EDGE24_W::new(self)
    }
    #[doc = "Bit 25 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge25(&mut self) -> EDGE25_W<25> {
        EDGE25_W::new(self)
    }
    #[doc = "Bit 26 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge26(&mut self) -> EDGE26_W<26> {
        EDGE26_W::new(self)
    }
    #[doc = "Bit 27 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge27(&mut self) -> EDGE27_W<27> {
        EDGE27_W::new(self)
    }
    #[doc = "Bit 28 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge28(&mut self) -> EDGE28_W<28> {
        EDGE28_W::new(self)
    }
    #[doc = "Bit 29 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge29(&mut self) -> EDGE29_W<29> {
        EDGE29_W::new(self)
    }
    #[doc = "Bit 30 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge30(&mut self) -> EDGE30_W<30> {
        EDGE30_W::new(self)
    }
    #[doc = "Bit 31 - Edge or level mode select bits."]
    #[inline(always)]
    #[must_use]
    pub fn edge31(&mut self) -> EDGE31_W<31> {
        EDGE31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt edge select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intedg0](index.html) module"]
pub struct INTEDG0_SPEC;
impl crate::RegisterSpec for INTEDG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intedg0::R](R) reader structure"]
impl crate::Readable for INTEDG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intedg0::W](W) writer structure"]
impl crate::Writable for INTEDG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEDG0 to value 0"]
impl crate::Resettable for INTEDG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
