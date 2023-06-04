#[doc = "Register `CONEN` reader"]
pub struct R(crate::R<CONEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONEN` writer"]
pub struct W(crate::W<CONEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONEN_SPEC>;
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
impl From<crate::W<CONEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NCEN0` reader - No Change Conflict Event/Interrupt Enable"]
pub type NCEN0_R = crate::BitReader<NCEN0_A>;
#[doc = "No Change Conflict Event/Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCEN0_A {
    #[doc = "0: No interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: Interrupt"]
    INTERRUPT = 1,
}
impl From<NCEN0_A> for bool {
    #[inline(always)]
    fn from(variant: NCEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl NCEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCEN0_A {
        match self.bits {
            false => NCEN0_A::NO_INTERRUPT,
            true => NCEN0_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == NCEN0_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == NCEN0_A::INTERRUPT
    }
}
#[doc = "Field `NCEN0` writer - No Change Conflict Event/Interrupt Enable"]
pub type NCEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONEN_SPEC, NCEN0_A, O>;
impl<'a, const O: u8> NCEN0_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(NCEN0_A::NO_INTERRUPT)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(NCEN0_A::INTERRUPT)
    }
}
#[doc = "Field `NCEN1` reader - No Change Conflict Event/Interrupt Enable"]
pub type NCEN1_R = crate::BitReader<NCEN1_A>;
#[doc = "No Change Conflict Event/Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCEN1_A {
    #[doc = "0: No interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: Interrupt"]
    INTERRUPT = 1,
}
impl From<NCEN1_A> for bool {
    #[inline(always)]
    fn from(variant: NCEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl NCEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCEN1_A {
        match self.bits {
            false => NCEN1_A::NO_INTERRUPT,
            true => NCEN1_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == NCEN1_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == NCEN1_A::INTERRUPT
    }
}
#[doc = "Field `NCEN1` writer - No Change Conflict Event/Interrupt Enable"]
pub type NCEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONEN_SPEC, NCEN1_A, O>;
impl<'a, const O: u8> NCEN1_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(NCEN1_A::NO_INTERRUPT)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(NCEN1_A::INTERRUPT)
    }
}
#[doc = "Field `NCEN2` reader - No Change Conflict Event/Interrupt Enable"]
pub type NCEN2_R = crate::BitReader<NCEN2_A>;
#[doc = "No Change Conflict Event/Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCEN2_A {
    #[doc = "0: No interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: Interrupt"]
    INTERRUPT = 1,
}
impl From<NCEN2_A> for bool {
    #[inline(always)]
    fn from(variant: NCEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl NCEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCEN2_A {
        match self.bits {
            false => NCEN2_A::NO_INTERRUPT,
            true => NCEN2_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == NCEN2_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == NCEN2_A::INTERRUPT
    }
}
#[doc = "Field `NCEN2` writer - No Change Conflict Event/Interrupt Enable"]
pub type NCEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONEN_SPEC, NCEN2_A, O>;
impl<'a, const O: u8> NCEN2_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(NCEN2_A::NO_INTERRUPT)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(NCEN2_A::INTERRUPT)
    }
}
#[doc = "Field `NCEN3` reader - No Change Conflict Event/Interrupt Enable"]
pub type NCEN3_R = crate::BitReader<NCEN3_A>;
#[doc = "No Change Conflict Event/Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCEN3_A {
    #[doc = "0: No interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: Interrupt"]
    INTERRUPT = 1,
}
impl From<NCEN3_A> for bool {
    #[inline(always)]
    fn from(variant: NCEN3_A) -> Self {
        variant as u8 != 0
    }
}
impl NCEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCEN3_A {
        match self.bits {
            false => NCEN3_A::NO_INTERRUPT,
            true => NCEN3_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == NCEN3_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == NCEN3_A::INTERRUPT
    }
}
#[doc = "Field `NCEN3` writer - No Change Conflict Event/Interrupt Enable"]
pub type NCEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONEN_SPEC, NCEN3_A, O>;
impl<'a, const O: u8> NCEN3_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(NCEN3_A::NO_INTERRUPT)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(NCEN3_A::INTERRUPT)
    }
}
#[doc = "Field `NCEN4` reader - No Change Conflict Event/Interrupt Enable"]
pub type NCEN4_R = crate::BitReader<NCEN4_A>;
#[doc = "No Change Conflict Event/Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCEN4_A {
    #[doc = "0: No interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: Interrupt"]
    INTERRUPT = 1,
}
impl From<NCEN4_A> for bool {
    #[inline(always)]
    fn from(variant: NCEN4_A) -> Self {
        variant as u8 != 0
    }
}
impl NCEN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCEN4_A {
        match self.bits {
            false => NCEN4_A::NO_INTERRUPT,
            true => NCEN4_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == NCEN4_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == NCEN4_A::INTERRUPT
    }
}
#[doc = "Field `NCEN4` writer - No Change Conflict Event/Interrupt Enable"]
pub type NCEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONEN_SPEC, NCEN4_A, O>;
impl<'a, const O: u8> NCEN4_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(NCEN4_A::NO_INTERRUPT)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(NCEN4_A::INTERRUPT)
    }
}
#[doc = "Field `NCEN5` reader - No Change Conflict Event/Interrupt Enable"]
pub type NCEN5_R = crate::BitReader<NCEN5_A>;
#[doc = "No Change Conflict Event/Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCEN5_A {
    #[doc = "0: No interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: Interrupt"]
    INTERRUPT = 1,
}
impl From<NCEN5_A> for bool {
    #[inline(always)]
    fn from(variant: NCEN5_A) -> Self {
        variant as u8 != 0
    }
}
impl NCEN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCEN5_A {
        match self.bits {
            false => NCEN5_A::NO_INTERRUPT,
            true => NCEN5_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == NCEN5_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == NCEN5_A::INTERRUPT
    }
}
#[doc = "Field `NCEN5` writer - No Change Conflict Event/Interrupt Enable"]
pub type NCEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONEN_SPEC, NCEN5_A, O>;
impl<'a, const O: u8> NCEN5_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(NCEN5_A::NO_INTERRUPT)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(NCEN5_A::INTERRUPT)
    }
}
#[doc = "Field `NCEN6` reader - No Change Conflict Event/Interrupt Enable"]
pub type NCEN6_R = crate::BitReader<NCEN6_A>;
#[doc = "No Change Conflict Event/Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCEN6_A {
    #[doc = "0: No interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: Interrupt"]
    INTERRUPT = 1,
}
impl From<NCEN6_A> for bool {
    #[inline(always)]
    fn from(variant: NCEN6_A) -> Self {
        variant as u8 != 0
    }
}
impl NCEN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCEN6_A {
        match self.bits {
            false => NCEN6_A::NO_INTERRUPT,
            true => NCEN6_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == NCEN6_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == NCEN6_A::INTERRUPT
    }
}
#[doc = "Field `NCEN6` writer - No Change Conflict Event/Interrupt Enable"]
pub type NCEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONEN_SPEC, NCEN6_A, O>;
impl<'a, const O: u8> NCEN6_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(NCEN6_A::NO_INTERRUPT)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(NCEN6_A::INTERRUPT)
    }
}
#[doc = "Field `NCEN7` reader - No Change Conflict Event/Interrupt Enable"]
pub type NCEN7_R = crate::BitReader<NCEN7_A>;
#[doc = "No Change Conflict Event/Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCEN7_A {
    #[doc = "0: No interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: Interrupt"]
    INTERRUPT = 1,
}
impl From<NCEN7_A> for bool {
    #[inline(always)]
    fn from(variant: NCEN7_A) -> Self {
        variant as u8 != 0
    }
}
impl NCEN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCEN7_A {
        match self.bits {
            false => NCEN7_A::NO_INTERRUPT,
            true => NCEN7_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == NCEN7_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == NCEN7_A::INTERRUPT
    }
}
#[doc = "Field `NCEN7` writer - No Change Conflict Event/Interrupt Enable"]
pub type NCEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONEN_SPEC, NCEN7_A, O>;
impl<'a, const O: u8> NCEN7_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(NCEN7_A::NO_INTERRUPT)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(NCEN7_A::INTERRUPT)
    }
}
#[doc = "Field `NCEN8` reader - No Change Conflict Event/Interrupt Enable"]
pub type NCEN8_R = crate::BitReader<NCEN8_A>;
#[doc = "No Change Conflict Event/Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCEN8_A {
    #[doc = "0: No interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: Interrupt"]
    INTERRUPT = 1,
}
impl From<NCEN8_A> for bool {
    #[inline(always)]
    fn from(variant: NCEN8_A) -> Self {
        variant as u8 != 0
    }
}
impl NCEN8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCEN8_A {
        match self.bits {
            false => NCEN8_A::NO_INTERRUPT,
            true => NCEN8_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == NCEN8_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == NCEN8_A::INTERRUPT
    }
}
#[doc = "Field `NCEN8` writer - No Change Conflict Event/Interrupt Enable"]
pub type NCEN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONEN_SPEC, NCEN8_A, O>;
impl<'a, const O: u8> NCEN8_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(NCEN8_A::NO_INTERRUPT)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(NCEN8_A::INTERRUPT)
    }
}
#[doc = "Field `NCEN9` reader - No Change Conflict Event/Interrupt Enable"]
pub type NCEN9_R = crate::BitReader<NCEN9_A>;
#[doc = "No Change Conflict Event/Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCEN9_A {
    #[doc = "0: No interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: Interrupt"]
    INTERRUPT = 1,
}
impl From<NCEN9_A> for bool {
    #[inline(always)]
    fn from(variant: NCEN9_A) -> Self {
        variant as u8 != 0
    }
}
impl NCEN9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCEN9_A {
        match self.bits {
            false => NCEN9_A::NO_INTERRUPT,
            true => NCEN9_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == NCEN9_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == NCEN9_A::INTERRUPT
    }
}
#[doc = "Field `NCEN9` writer - No Change Conflict Event/Interrupt Enable"]
pub type NCEN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONEN_SPEC, NCEN9_A, O>;
impl<'a, const O: u8> NCEN9_W<'a, O> {
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(NCEN9_A::NO_INTERRUPT)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(NCEN9_A::INTERRUPT)
    }
}
impl R {
    #[doc = "Bit 0 - No Change Conflict Event/Interrupt Enable"]
    #[inline(always)]
    pub fn ncen0(&self) -> NCEN0_R {
        NCEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No Change Conflict Event/Interrupt Enable"]
    #[inline(always)]
    pub fn ncen1(&self) -> NCEN1_R {
        NCEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - No Change Conflict Event/Interrupt Enable"]
    #[inline(always)]
    pub fn ncen2(&self) -> NCEN2_R {
        NCEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - No Change Conflict Event/Interrupt Enable"]
    #[inline(always)]
    pub fn ncen3(&self) -> NCEN3_R {
        NCEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - No Change Conflict Event/Interrupt Enable"]
    #[inline(always)]
    pub fn ncen4(&self) -> NCEN4_R {
        NCEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No Change Conflict Event/Interrupt Enable"]
    #[inline(always)]
    pub fn ncen5(&self) -> NCEN5_R {
        NCEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - No Change Conflict Event/Interrupt Enable"]
    #[inline(always)]
    pub fn ncen6(&self) -> NCEN6_R {
        NCEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - No Change Conflict Event/Interrupt Enable"]
    #[inline(always)]
    pub fn ncen7(&self) -> NCEN7_R {
        NCEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - No Change Conflict Event/Interrupt Enable"]
    #[inline(always)]
    pub fn ncen8(&self) -> NCEN8_R {
        NCEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - No Change Conflict Event/Interrupt Enable"]
    #[inline(always)]
    pub fn ncen9(&self) -> NCEN9_R {
        NCEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Change Conflict Event/Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ncen0(&mut self) -> NCEN0_W<0> {
        NCEN0_W::new(self)
    }
    #[doc = "Bit 1 - No Change Conflict Event/Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ncen1(&mut self) -> NCEN1_W<1> {
        NCEN1_W::new(self)
    }
    #[doc = "Bit 2 - No Change Conflict Event/Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ncen2(&mut self) -> NCEN2_W<2> {
        NCEN2_W::new(self)
    }
    #[doc = "Bit 3 - No Change Conflict Event/Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ncen3(&mut self) -> NCEN3_W<3> {
        NCEN3_W::new(self)
    }
    #[doc = "Bit 4 - No Change Conflict Event/Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ncen4(&mut self) -> NCEN4_W<4> {
        NCEN4_W::new(self)
    }
    #[doc = "Bit 5 - No Change Conflict Event/Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ncen5(&mut self) -> NCEN5_W<5> {
        NCEN5_W::new(self)
    }
    #[doc = "Bit 6 - No Change Conflict Event/Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ncen6(&mut self) -> NCEN6_W<6> {
        NCEN6_W::new(self)
    }
    #[doc = "Bit 7 - No Change Conflict Event/Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ncen7(&mut self) -> NCEN7_W<7> {
        NCEN7_W::new(self)
    }
    #[doc = "Bit 8 - No Change Conflict Event/Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ncen8(&mut self) -> NCEN8_W<8> {
        NCEN8_W::new(self)
    }
    #[doc = "Bit 9 - No Change Conflict Event/Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ncen9(&mut self) -> NCEN9_W<9> {
        NCEN9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Conflict Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conen](index.html) module"]
pub struct CONEN_SPEC;
impl crate::RegisterSpec for CONEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conen::R](R) reader structure"]
impl crate::Readable for CONEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conen::W](W) writer structure"]
impl crate::Writable for CONEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONEN to value 0"]
impl crate::Resettable for CONEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
