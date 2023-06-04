#[doc = "Register `SWTRIG` reader"]
pub struct R(crate::R<SWTRIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWTRIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWTRIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWTRIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWTRIG` writer"]
pub struct W(crate::W<SWTRIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWTRIG_SPEC>;
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
impl From<crate::W<SWTRIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWTRIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWT0` reader - Software Trigger 0"]
pub type SWT0_R = crate::BitReader<SWT0_A>;
#[doc = "Software Trigger 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT0_A {
    #[doc = "0: No trigger 0 event generated."]
    NO_TRIGGER = 0,
    #[doc = "1: Trigger 0 event generated."]
    INITIATE_TRIGGER_0 = 1,
}
impl From<SWT0_A> for bool {
    #[inline(always)]
    fn from(variant: SWT0_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT0_A {
        match self.bits {
            false => SWT0_A::NO_TRIGGER,
            true => SWT0_A::INITIATE_TRIGGER_0,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TRIGGER`"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == SWT0_A::NO_TRIGGER
    }
    #[doc = "Checks if the value of the field is `INITIATE_TRIGGER_0`"]
    #[inline(always)]
    pub fn is_initiate_trigger_0(&self) -> bool {
        *self == SWT0_A::INITIATE_TRIGGER_0
    }
}
#[doc = "Field `SWT0` writer - Software Trigger 0"]
pub type SWT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT0_A, O>;
impl<'a, const O: u8> SWT0_W<'a, O> {
    #[doc = "No trigger 0 event generated."]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut W {
        self.variant(SWT0_A::NO_TRIGGER)
    }
    #[doc = "Trigger 0 event generated."]
    #[inline(always)]
    pub fn initiate_trigger_0(self) -> &'a mut W {
        self.variant(SWT0_A::INITIATE_TRIGGER_0)
    }
}
#[doc = "Field `SWT1` reader - Software Trigger 1"]
pub type SWT1_R = crate::BitReader<SWT1_A>;
#[doc = "Software Trigger 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT1_A {
    #[doc = "0: No trigger 1 event generated."]
    NO_TRIGGER = 0,
    #[doc = "1: Trigger 1 event generated."]
    INITIATE_TRIGGER_1 = 1,
}
impl From<SWT1_A> for bool {
    #[inline(always)]
    fn from(variant: SWT1_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT1_A {
        match self.bits {
            false => SWT1_A::NO_TRIGGER,
            true => SWT1_A::INITIATE_TRIGGER_1,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TRIGGER`"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == SWT1_A::NO_TRIGGER
    }
    #[doc = "Checks if the value of the field is `INITIATE_TRIGGER_1`"]
    #[inline(always)]
    pub fn is_initiate_trigger_1(&self) -> bool {
        *self == SWT1_A::INITIATE_TRIGGER_1
    }
}
#[doc = "Field `SWT1` writer - Software Trigger 1"]
pub type SWT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT1_A, O>;
impl<'a, const O: u8> SWT1_W<'a, O> {
    #[doc = "No trigger 1 event generated."]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut W {
        self.variant(SWT1_A::NO_TRIGGER)
    }
    #[doc = "Trigger 1 event generated."]
    #[inline(always)]
    pub fn initiate_trigger_1(self) -> &'a mut W {
        self.variant(SWT1_A::INITIATE_TRIGGER_1)
    }
}
#[doc = "Field `SWT2` reader - Software Trigger 2"]
pub type SWT2_R = crate::BitReader<SWT2_A>;
#[doc = "Software Trigger 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT2_A {
    #[doc = "0: No trigger 2 event generated."]
    NO_TRIGGER = 0,
    #[doc = "1: Trigger 2 event generated."]
    INITIATE_TRIGGER_2 = 1,
}
impl From<SWT2_A> for bool {
    #[inline(always)]
    fn from(variant: SWT2_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT2_A {
        match self.bits {
            false => SWT2_A::NO_TRIGGER,
            true => SWT2_A::INITIATE_TRIGGER_2,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TRIGGER`"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == SWT2_A::NO_TRIGGER
    }
    #[doc = "Checks if the value of the field is `INITIATE_TRIGGER_2`"]
    #[inline(always)]
    pub fn is_initiate_trigger_2(&self) -> bool {
        *self == SWT2_A::INITIATE_TRIGGER_2
    }
}
#[doc = "Field `SWT2` writer - Software Trigger 2"]
pub type SWT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT2_A, O>;
impl<'a, const O: u8> SWT2_W<'a, O> {
    #[doc = "No trigger 2 event generated."]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut W {
        self.variant(SWT2_A::NO_TRIGGER)
    }
    #[doc = "Trigger 2 event generated."]
    #[inline(always)]
    pub fn initiate_trigger_2(self) -> &'a mut W {
        self.variant(SWT2_A::INITIATE_TRIGGER_2)
    }
}
#[doc = "Field `SWT3` reader - Software Trigger 3"]
pub type SWT3_R = crate::BitReader<SWT3_A>;
#[doc = "Software Trigger 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT3_A {
    #[doc = "0: No trigger 3 event generated."]
    NO_TRIGGER = 0,
    #[doc = "1: Trigger 3 event generated."]
    INITIATE_TRIGGER_3 = 1,
}
impl From<SWT3_A> for bool {
    #[inline(always)]
    fn from(variant: SWT3_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT3_A {
        match self.bits {
            false => SWT3_A::NO_TRIGGER,
            true => SWT3_A::INITIATE_TRIGGER_3,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TRIGGER`"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == SWT3_A::NO_TRIGGER
    }
    #[doc = "Checks if the value of the field is `INITIATE_TRIGGER_3`"]
    #[inline(always)]
    pub fn is_initiate_trigger_3(&self) -> bool {
        *self == SWT3_A::INITIATE_TRIGGER_3
    }
}
#[doc = "Field `SWT3` writer - Software Trigger 3"]
pub type SWT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT3_A, O>;
impl<'a, const O: u8> SWT3_W<'a, O> {
    #[doc = "No trigger 3 event generated."]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut W {
        self.variant(SWT3_A::NO_TRIGGER)
    }
    #[doc = "Trigger 3 event generated."]
    #[inline(always)]
    pub fn initiate_trigger_3(self) -> &'a mut W {
        self.variant(SWT3_A::INITIATE_TRIGGER_3)
    }
}
#[doc = "Field `SWT4` reader - Software Trigger 4"]
pub type SWT4_R = crate::BitReader<SWT4_A>;
#[doc = "Software Trigger 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT4_A {
    #[doc = "0: No trigger 4 event generated."]
    NO_TRIGGER = 0,
    #[doc = "1: Trigger 4 event generated."]
    INITIATE_TRIGGER_4 = 1,
}
impl From<SWT4_A> for bool {
    #[inline(always)]
    fn from(variant: SWT4_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT4_A {
        match self.bits {
            false => SWT4_A::NO_TRIGGER,
            true => SWT4_A::INITIATE_TRIGGER_4,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TRIGGER`"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == SWT4_A::NO_TRIGGER
    }
    #[doc = "Checks if the value of the field is `INITIATE_TRIGGER_4`"]
    #[inline(always)]
    pub fn is_initiate_trigger_4(&self) -> bool {
        *self == SWT4_A::INITIATE_TRIGGER_4
    }
}
#[doc = "Field `SWT4` writer - Software Trigger 4"]
pub type SWT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT4_A, O>;
impl<'a, const O: u8> SWT4_W<'a, O> {
    #[doc = "No trigger 4 event generated."]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut W {
        self.variant(SWT4_A::NO_TRIGGER)
    }
    #[doc = "Trigger 4 event generated."]
    #[inline(always)]
    pub fn initiate_trigger_4(self) -> &'a mut W {
        self.variant(SWT4_A::INITIATE_TRIGGER_4)
    }
}
#[doc = "Field `SWT5` reader - Software Trigger 5"]
pub type SWT5_R = crate::BitReader<SWT5_A>;
#[doc = "Software Trigger 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT5_A {
    #[doc = "0: No trigger 5 event generated."]
    NO_TRIGGER = 0,
    #[doc = "1: Trigger 5 event generated."]
    INITIATE_TRIGGER_5 = 1,
}
impl From<SWT5_A> for bool {
    #[inline(always)]
    fn from(variant: SWT5_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT5_A {
        match self.bits {
            false => SWT5_A::NO_TRIGGER,
            true => SWT5_A::INITIATE_TRIGGER_5,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TRIGGER`"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == SWT5_A::NO_TRIGGER
    }
    #[doc = "Checks if the value of the field is `INITIATE_TRIGGER_5`"]
    #[inline(always)]
    pub fn is_initiate_trigger_5(&self) -> bool {
        *self == SWT5_A::INITIATE_TRIGGER_5
    }
}
#[doc = "Field `SWT5` writer - Software Trigger 5"]
pub type SWT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT5_A, O>;
impl<'a, const O: u8> SWT5_W<'a, O> {
    #[doc = "No trigger 5 event generated."]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut W {
        self.variant(SWT5_A::NO_TRIGGER)
    }
    #[doc = "Trigger 5 event generated."]
    #[inline(always)]
    pub fn initiate_trigger_5(self) -> &'a mut W {
        self.variant(SWT5_A::INITIATE_TRIGGER_5)
    }
}
#[doc = "Field `SWT6` reader - Software Trigger 6"]
pub type SWT6_R = crate::BitReader<SWT6_A>;
#[doc = "Software Trigger 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT6_A {
    #[doc = "0: No trigger 6 event generated."]
    NO_TRIGGER = 0,
    #[doc = "1: Trigger 6 event generated."]
    INITIATE_TRIGGER_6 = 1,
}
impl From<SWT6_A> for bool {
    #[inline(always)]
    fn from(variant: SWT6_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT6_A {
        match self.bits {
            false => SWT6_A::NO_TRIGGER,
            true => SWT6_A::INITIATE_TRIGGER_6,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TRIGGER`"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == SWT6_A::NO_TRIGGER
    }
    #[doc = "Checks if the value of the field is `INITIATE_TRIGGER_6`"]
    #[inline(always)]
    pub fn is_initiate_trigger_6(&self) -> bool {
        *self == SWT6_A::INITIATE_TRIGGER_6
    }
}
#[doc = "Field `SWT6` writer - Software Trigger 6"]
pub type SWT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT6_A, O>;
impl<'a, const O: u8> SWT6_W<'a, O> {
    #[doc = "No trigger 6 event generated."]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut W {
        self.variant(SWT6_A::NO_TRIGGER)
    }
    #[doc = "Trigger 6 event generated."]
    #[inline(always)]
    pub fn initiate_trigger_6(self) -> &'a mut W {
        self.variant(SWT6_A::INITIATE_TRIGGER_6)
    }
}
#[doc = "Field `SWT7` reader - Software Trigger 7"]
pub type SWT7_R = crate::BitReader<SWT7_A>;
#[doc = "Software Trigger 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT7_A {
    #[doc = "0: No trigger 7 event generated."]
    NO_TRIGGER = 0,
    #[doc = "1: Trigger 7 event generated."]
    INITIATE_TRIGGER_7 = 1,
}
impl From<SWT7_A> for bool {
    #[inline(always)]
    fn from(variant: SWT7_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT7_A {
        match self.bits {
            false => SWT7_A::NO_TRIGGER,
            true => SWT7_A::INITIATE_TRIGGER_7,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TRIGGER`"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == SWT7_A::NO_TRIGGER
    }
    #[doc = "Checks if the value of the field is `INITIATE_TRIGGER_7`"]
    #[inline(always)]
    pub fn is_initiate_trigger_7(&self) -> bool {
        *self == SWT7_A::INITIATE_TRIGGER_7
    }
}
#[doc = "Field `SWT7` writer - Software Trigger 7"]
pub type SWT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT7_A, O>;
impl<'a, const O: u8> SWT7_W<'a, O> {
    #[doc = "No trigger 7 event generated."]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut W {
        self.variant(SWT7_A::NO_TRIGGER)
    }
    #[doc = "Trigger 7 event generated."]
    #[inline(always)]
    pub fn initiate_trigger_7(self) -> &'a mut W {
        self.variant(SWT7_A::INITIATE_TRIGGER_7)
    }
}
#[doc = "Field `SWT8` reader - Software Trigger 8"]
pub type SWT8_R = crate::BitReader<SWT8_A>;
#[doc = "Software Trigger 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT8_A {
    #[doc = "0: No trigger 8 event generated."]
    NO_TRIGGER = 0,
    #[doc = "1: Trigger 8 event generated."]
    INITIATE_TRIGGER_8 = 1,
}
impl From<SWT8_A> for bool {
    #[inline(always)]
    fn from(variant: SWT8_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT8_A {
        match self.bits {
            false => SWT8_A::NO_TRIGGER,
            true => SWT8_A::INITIATE_TRIGGER_8,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TRIGGER`"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == SWT8_A::NO_TRIGGER
    }
    #[doc = "Checks if the value of the field is `INITIATE_TRIGGER_8`"]
    #[inline(always)]
    pub fn is_initiate_trigger_8(&self) -> bool {
        *self == SWT8_A::INITIATE_TRIGGER_8
    }
}
#[doc = "Field `SWT8` writer - Software Trigger 8"]
pub type SWT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT8_A, O>;
impl<'a, const O: u8> SWT8_W<'a, O> {
    #[doc = "No trigger 8 event generated."]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut W {
        self.variant(SWT8_A::NO_TRIGGER)
    }
    #[doc = "Trigger 8 event generated."]
    #[inline(always)]
    pub fn initiate_trigger_8(self) -> &'a mut W {
        self.variant(SWT8_A::INITIATE_TRIGGER_8)
    }
}
#[doc = "Field `SWT9` reader - Software Trigger 9"]
pub type SWT9_R = crate::BitReader<SWT9_A>;
#[doc = "Software Trigger 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT9_A {
    #[doc = "0: No trigger 9 event generated."]
    NO_TRIGGER = 0,
    #[doc = "1: Trigger 9 event generated."]
    INITIATE_TRIGGER_9 = 1,
}
impl From<SWT9_A> for bool {
    #[inline(always)]
    fn from(variant: SWT9_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT9_A {
        match self.bits {
            false => SWT9_A::NO_TRIGGER,
            true => SWT9_A::INITIATE_TRIGGER_9,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TRIGGER`"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == SWT9_A::NO_TRIGGER
    }
    #[doc = "Checks if the value of the field is `INITIATE_TRIGGER_9`"]
    #[inline(always)]
    pub fn is_initiate_trigger_9(&self) -> bool {
        *self == SWT9_A::INITIATE_TRIGGER_9
    }
}
#[doc = "Field `SWT9` writer - Software Trigger 9"]
pub type SWT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT9_A, O>;
impl<'a, const O: u8> SWT9_W<'a, O> {
    #[doc = "No trigger 9 event generated."]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut W {
        self.variant(SWT9_A::NO_TRIGGER)
    }
    #[doc = "Trigger 9 event generated."]
    #[inline(always)]
    pub fn initiate_trigger_9(self) -> &'a mut W {
        self.variant(SWT9_A::INITIATE_TRIGGER_9)
    }
}
#[doc = "Field `SWT10` reader - Software Trigger 10"]
pub type SWT10_R = crate::BitReader<SWT10_A>;
#[doc = "Software Trigger 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT10_A {
    #[doc = "0: No trigger 10 event generated."]
    NO_TRIGGER = 0,
    #[doc = "1: Trigger 10 event generated."]
    INITIATE_TRIGGER_10 = 1,
}
impl From<SWT10_A> for bool {
    #[inline(always)]
    fn from(variant: SWT10_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT10_A {
        match self.bits {
            false => SWT10_A::NO_TRIGGER,
            true => SWT10_A::INITIATE_TRIGGER_10,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TRIGGER`"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == SWT10_A::NO_TRIGGER
    }
    #[doc = "Checks if the value of the field is `INITIATE_TRIGGER_10`"]
    #[inline(always)]
    pub fn is_initiate_trigger_10(&self) -> bool {
        *self == SWT10_A::INITIATE_TRIGGER_10
    }
}
#[doc = "Field `SWT10` writer - Software Trigger 10"]
pub type SWT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT10_A, O>;
impl<'a, const O: u8> SWT10_W<'a, O> {
    #[doc = "No trigger 10 event generated."]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut W {
        self.variant(SWT10_A::NO_TRIGGER)
    }
    #[doc = "Trigger 10 event generated."]
    #[inline(always)]
    pub fn initiate_trigger_10(self) -> &'a mut W {
        self.variant(SWT10_A::INITIATE_TRIGGER_10)
    }
}
#[doc = "Field `SWT11` reader - Software Trigger 11"]
pub type SWT11_R = crate::BitReader<SWT11_A>;
#[doc = "Software Trigger 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT11_A {
    #[doc = "0: No trigger 11 event generated."]
    NO_TRIGGER = 0,
    #[doc = "1: Trigger 11 event generated."]
    INITIATE_TRIGGER_11 = 1,
}
impl From<SWT11_A> for bool {
    #[inline(always)]
    fn from(variant: SWT11_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT11_A {
        match self.bits {
            false => SWT11_A::NO_TRIGGER,
            true => SWT11_A::INITIATE_TRIGGER_11,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TRIGGER`"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == SWT11_A::NO_TRIGGER
    }
    #[doc = "Checks if the value of the field is `INITIATE_TRIGGER_11`"]
    #[inline(always)]
    pub fn is_initiate_trigger_11(&self) -> bool {
        *self == SWT11_A::INITIATE_TRIGGER_11
    }
}
#[doc = "Field `SWT11` writer - Software Trigger 11"]
pub type SWT11_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT11_A, O>;
impl<'a, const O: u8> SWT11_W<'a, O> {
    #[doc = "No trigger 11 event generated."]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut W {
        self.variant(SWT11_A::NO_TRIGGER)
    }
    #[doc = "Trigger 11 event generated."]
    #[inline(always)]
    pub fn initiate_trigger_11(self) -> &'a mut W {
        self.variant(SWT11_A::INITIATE_TRIGGER_11)
    }
}
#[doc = "Field `SWT12` reader - Software Trigger 12"]
pub type SWT12_R = crate::BitReader<SWT12_A>;
#[doc = "Software Trigger 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT12_A {
    #[doc = "0: No trigger 12 event generated."]
    NO_TRIGGER = 0,
    #[doc = "1: Trigger 12 event generated."]
    INITIATE_TRIGGER_12 = 1,
}
impl From<SWT12_A> for bool {
    #[inline(always)]
    fn from(variant: SWT12_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT12_A {
        match self.bits {
            false => SWT12_A::NO_TRIGGER,
            true => SWT12_A::INITIATE_TRIGGER_12,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TRIGGER`"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == SWT12_A::NO_TRIGGER
    }
    #[doc = "Checks if the value of the field is `INITIATE_TRIGGER_12`"]
    #[inline(always)]
    pub fn is_initiate_trigger_12(&self) -> bool {
        *self == SWT12_A::INITIATE_TRIGGER_12
    }
}
#[doc = "Field `SWT12` writer - Software Trigger 12"]
pub type SWT12_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT12_A, O>;
impl<'a, const O: u8> SWT12_W<'a, O> {
    #[doc = "No trigger 12 event generated."]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut W {
        self.variant(SWT12_A::NO_TRIGGER)
    }
    #[doc = "Trigger 12 event generated."]
    #[inline(always)]
    pub fn initiate_trigger_12(self) -> &'a mut W {
        self.variant(SWT12_A::INITIATE_TRIGGER_12)
    }
}
#[doc = "Field `SWT13` reader - Software Trigger 13"]
pub type SWT13_R = crate::BitReader<SWT13_A>;
#[doc = "Software Trigger 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT13_A {
    #[doc = "0: No trigger 13 event generated."]
    NO_TRIGGER = 0,
    #[doc = "1: Trigger 13 event generated."]
    INITIATE_TRIGGER_13 = 1,
}
impl From<SWT13_A> for bool {
    #[inline(always)]
    fn from(variant: SWT13_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT13_A {
        match self.bits {
            false => SWT13_A::NO_TRIGGER,
            true => SWT13_A::INITIATE_TRIGGER_13,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TRIGGER`"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == SWT13_A::NO_TRIGGER
    }
    #[doc = "Checks if the value of the field is `INITIATE_TRIGGER_13`"]
    #[inline(always)]
    pub fn is_initiate_trigger_13(&self) -> bool {
        *self == SWT13_A::INITIATE_TRIGGER_13
    }
}
#[doc = "Field `SWT13` writer - Software Trigger 13"]
pub type SWT13_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT13_A, O>;
impl<'a, const O: u8> SWT13_W<'a, O> {
    #[doc = "No trigger 13 event generated."]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut W {
        self.variant(SWT13_A::NO_TRIGGER)
    }
    #[doc = "Trigger 13 event generated."]
    #[inline(always)]
    pub fn initiate_trigger_13(self) -> &'a mut W {
        self.variant(SWT13_A::INITIATE_TRIGGER_13)
    }
}
#[doc = "Field `SWT14` reader - Software Trigger 14"]
pub type SWT14_R = crate::BitReader<SWT14_A>;
#[doc = "Software Trigger 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT14_A {
    #[doc = "0: No trigger 14 event generated."]
    NO_TRIGGER = 0,
    #[doc = "1: Trigger 14 event generated."]
    INITIATE_TRIGGER_14 = 1,
}
impl From<SWT14_A> for bool {
    #[inline(always)]
    fn from(variant: SWT14_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT14_A {
        match self.bits {
            false => SWT14_A::NO_TRIGGER,
            true => SWT14_A::INITIATE_TRIGGER_14,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TRIGGER`"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == SWT14_A::NO_TRIGGER
    }
    #[doc = "Checks if the value of the field is `INITIATE_TRIGGER_14`"]
    #[inline(always)]
    pub fn is_initiate_trigger_14(&self) -> bool {
        *self == SWT14_A::INITIATE_TRIGGER_14
    }
}
#[doc = "Field `SWT14` writer - Software Trigger 14"]
pub type SWT14_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT14_A, O>;
impl<'a, const O: u8> SWT14_W<'a, O> {
    #[doc = "No trigger 14 event generated."]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut W {
        self.variant(SWT14_A::NO_TRIGGER)
    }
    #[doc = "Trigger 14 event generated."]
    #[inline(always)]
    pub fn initiate_trigger_14(self) -> &'a mut W {
        self.variant(SWT14_A::INITIATE_TRIGGER_14)
    }
}
#[doc = "Field `SWT15` reader - Software Trigger 15"]
pub type SWT15_R = crate::BitReader<SWT15_A>;
#[doc = "Software Trigger 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT15_A {
    #[doc = "0: No trigger 15 event generated."]
    NO_TRIGGER = 0,
    #[doc = "1: Trigger 15 event generated."]
    INITIATE_TRIGGER_15 = 1,
}
impl From<SWT15_A> for bool {
    #[inline(always)]
    fn from(variant: SWT15_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT15_A {
        match self.bits {
            false => SWT15_A::NO_TRIGGER,
            true => SWT15_A::INITIATE_TRIGGER_15,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TRIGGER`"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == SWT15_A::NO_TRIGGER
    }
    #[doc = "Checks if the value of the field is `INITIATE_TRIGGER_15`"]
    #[inline(always)]
    pub fn is_initiate_trigger_15(&self) -> bool {
        *self == SWT15_A::INITIATE_TRIGGER_15
    }
}
#[doc = "Field `SWT15` writer - Software Trigger 15"]
pub type SWT15_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIG_SPEC, SWT15_A, O>;
impl<'a, const O: u8> SWT15_W<'a, O> {
    #[doc = "No trigger 15 event generated."]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut W {
        self.variant(SWT15_A::NO_TRIGGER)
    }
    #[doc = "Trigger 15 event generated."]
    #[inline(always)]
    pub fn initiate_trigger_15(self) -> &'a mut W {
        self.variant(SWT15_A::INITIATE_TRIGGER_15)
    }
}
impl R {
    #[doc = "Bit 0 - Software Trigger 0"]
    #[inline(always)]
    pub fn swt0(&self) -> SWT0_R {
        SWT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Trigger 1"]
    #[inline(always)]
    pub fn swt1(&self) -> SWT1_R {
        SWT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Trigger 2"]
    #[inline(always)]
    pub fn swt2(&self) -> SWT2_R {
        SWT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software Trigger 3"]
    #[inline(always)]
    pub fn swt3(&self) -> SWT3_R {
        SWT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software Trigger 4"]
    #[inline(always)]
    pub fn swt4(&self) -> SWT4_R {
        SWT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software Trigger 5"]
    #[inline(always)]
    pub fn swt5(&self) -> SWT5_R {
        SWT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software Trigger 6"]
    #[inline(always)]
    pub fn swt6(&self) -> SWT6_R {
        SWT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software Trigger 7"]
    #[inline(always)]
    pub fn swt7(&self) -> SWT7_R {
        SWT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software Trigger 8"]
    #[inline(always)]
    pub fn swt8(&self) -> SWT8_R {
        SWT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software Trigger 9"]
    #[inline(always)]
    pub fn swt9(&self) -> SWT9_R {
        SWT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software Trigger 10"]
    #[inline(always)]
    pub fn swt10(&self) -> SWT10_R {
        SWT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software Trigger 11"]
    #[inline(always)]
    pub fn swt11(&self) -> SWT11_R {
        SWT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Software Trigger 12"]
    #[inline(always)]
    pub fn swt12(&self) -> SWT12_R {
        SWT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Software Trigger 13"]
    #[inline(always)]
    pub fn swt13(&self) -> SWT13_R {
        SWT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Software Trigger 14"]
    #[inline(always)]
    pub fn swt14(&self) -> SWT14_R {
        SWT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Software Trigger 15"]
    #[inline(always)]
    pub fn swt15(&self) -> SWT15_R {
        SWT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Trigger 0"]
    #[inline(always)]
    #[must_use]
    pub fn swt0(&mut self) -> SWT0_W<0> {
        SWT0_W::new(self)
    }
    #[doc = "Bit 1 - Software Trigger 1"]
    #[inline(always)]
    #[must_use]
    pub fn swt1(&mut self) -> SWT1_W<1> {
        SWT1_W::new(self)
    }
    #[doc = "Bit 2 - Software Trigger 2"]
    #[inline(always)]
    #[must_use]
    pub fn swt2(&mut self) -> SWT2_W<2> {
        SWT2_W::new(self)
    }
    #[doc = "Bit 3 - Software Trigger 3"]
    #[inline(always)]
    #[must_use]
    pub fn swt3(&mut self) -> SWT3_W<3> {
        SWT3_W::new(self)
    }
    #[doc = "Bit 4 - Software Trigger 4"]
    #[inline(always)]
    #[must_use]
    pub fn swt4(&mut self) -> SWT4_W<4> {
        SWT4_W::new(self)
    }
    #[doc = "Bit 5 - Software Trigger 5"]
    #[inline(always)]
    #[must_use]
    pub fn swt5(&mut self) -> SWT5_W<5> {
        SWT5_W::new(self)
    }
    #[doc = "Bit 6 - Software Trigger 6"]
    #[inline(always)]
    #[must_use]
    pub fn swt6(&mut self) -> SWT6_W<6> {
        SWT6_W::new(self)
    }
    #[doc = "Bit 7 - Software Trigger 7"]
    #[inline(always)]
    #[must_use]
    pub fn swt7(&mut self) -> SWT7_W<7> {
        SWT7_W::new(self)
    }
    #[doc = "Bit 8 - Software Trigger 8"]
    #[inline(always)]
    #[must_use]
    pub fn swt8(&mut self) -> SWT8_W<8> {
        SWT8_W::new(self)
    }
    #[doc = "Bit 9 - Software Trigger 9"]
    #[inline(always)]
    #[must_use]
    pub fn swt9(&mut self) -> SWT9_W<9> {
        SWT9_W::new(self)
    }
    #[doc = "Bit 10 - Software Trigger 10"]
    #[inline(always)]
    #[must_use]
    pub fn swt10(&mut self) -> SWT10_W<10> {
        SWT10_W::new(self)
    }
    #[doc = "Bit 11 - Software Trigger 11"]
    #[inline(always)]
    #[must_use]
    pub fn swt11(&mut self) -> SWT11_W<11> {
        SWT11_W::new(self)
    }
    #[doc = "Bit 12 - Software Trigger 12"]
    #[inline(always)]
    #[must_use]
    pub fn swt12(&mut self) -> SWT12_W<12> {
        SWT12_W::new(self)
    }
    #[doc = "Bit 13 - Software Trigger 13"]
    #[inline(always)]
    #[must_use]
    pub fn swt13(&mut self) -> SWT13_W<13> {
        SWT13_W::new(self)
    }
    #[doc = "Bit 14 - Software Trigger 14"]
    #[inline(always)]
    #[must_use]
    pub fn swt14(&mut self) -> SWT14_W<14> {
        SWT14_W::new(self)
    }
    #[doc = "Bit 15 - Software Trigger 15"]
    #[inline(always)]
    #[must_use]
    pub fn swt15(&mut self) -> SWT15_W<15> {
        SWT15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Trigger Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swtrig](index.html) module"]
pub struct SWTRIG_SPEC;
impl crate::RegisterSpec for SWTRIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swtrig::R](R) reader structure"]
impl crate::Readable for SWTRIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swtrig::W](W) writer structure"]
impl crate::Writable for SWTRIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWTRIG to value 0"]
impl crate::Resettable for SWTRIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
