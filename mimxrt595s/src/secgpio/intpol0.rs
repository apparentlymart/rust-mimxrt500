#[doc = "Register `INTPOL0` reader"]
pub struct R(crate::R<INTPOL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTPOL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTPOL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTPOL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTPOL0` writer"]
pub struct W(crate::W<INTPOL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTPOL0_SPEC>;
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
impl From<crate::W<INTPOL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTPOL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POL_CTL0` reader - Polarity control for each pin"]
pub type POL_CTL0_R = crate::BitReader<POL_CTL0_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL0_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL0_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL0_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL0_A {
        match self.bits {
            false => POL_CTL0_A::HIGH,
            true => POL_CTL0_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL0_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL0_A::LOW
    }
}
#[doc = "Field `POL_CTL0` writer - Polarity control for each pin"]
pub type POL_CTL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL0_A, O>;
impl<'a, const O: u8> POL_CTL0_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL0_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL0_A::LOW)
    }
}
#[doc = "Field `POL_CTL1` reader - Polarity control for each pin"]
pub type POL_CTL1_R = crate::BitReader<POL_CTL1_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL1_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL1_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL1_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL1_A {
        match self.bits {
            false => POL_CTL1_A::HIGH,
            true => POL_CTL1_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL1_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL1_A::LOW
    }
}
#[doc = "Field `POL_CTL1` writer - Polarity control for each pin"]
pub type POL_CTL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL1_A, O>;
impl<'a, const O: u8> POL_CTL1_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL1_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL1_A::LOW)
    }
}
#[doc = "Field `POL_CTL2` reader - Polarity control for each pin"]
pub type POL_CTL2_R = crate::BitReader<POL_CTL2_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL2_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL2_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL2_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL2_A {
        match self.bits {
            false => POL_CTL2_A::HIGH,
            true => POL_CTL2_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL2_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL2_A::LOW
    }
}
#[doc = "Field `POL_CTL2` writer - Polarity control for each pin"]
pub type POL_CTL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL2_A, O>;
impl<'a, const O: u8> POL_CTL2_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL2_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL2_A::LOW)
    }
}
#[doc = "Field `POL_CTL3` reader - Polarity control for each pin"]
pub type POL_CTL3_R = crate::BitReader<POL_CTL3_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL3_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL3_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL3_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL3_A {
        match self.bits {
            false => POL_CTL3_A::HIGH,
            true => POL_CTL3_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL3_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL3_A::LOW
    }
}
#[doc = "Field `POL_CTL3` writer - Polarity control for each pin"]
pub type POL_CTL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL3_A, O>;
impl<'a, const O: u8> POL_CTL3_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL3_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL3_A::LOW)
    }
}
#[doc = "Field `POL_CTL4` reader - Polarity control for each pin"]
pub type POL_CTL4_R = crate::BitReader<POL_CTL4_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL4_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL4_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL4_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL4_A {
        match self.bits {
            false => POL_CTL4_A::HIGH,
            true => POL_CTL4_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL4_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL4_A::LOW
    }
}
#[doc = "Field `POL_CTL4` writer - Polarity control for each pin"]
pub type POL_CTL4_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL4_A, O>;
impl<'a, const O: u8> POL_CTL4_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL4_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL4_A::LOW)
    }
}
#[doc = "Field `POL_CTL5` reader - Polarity control for each pin"]
pub type POL_CTL5_R = crate::BitReader<POL_CTL5_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL5_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL5_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL5_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL5_A {
        match self.bits {
            false => POL_CTL5_A::HIGH,
            true => POL_CTL5_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL5_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL5_A::LOW
    }
}
#[doc = "Field `POL_CTL5` writer - Polarity control for each pin"]
pub type POL_CTL5_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL5_A, O>;
impl<'a, const O: u8> POL_CTL5_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL5_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL5_A::LOW)
    }
}
#[doc = "Field `POL_CTL6` reader - Polarity control for each pin"]
pub type POL_CTL6_R = crate::BitReader<POL_CTL6_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL6_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL6_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL6_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL6_A {
        match self.bits {
            false => POL_CTL6_A::HIGH,
            true => POL_CTL6_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL6_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL6_A::LOW
    }
}
#[doc = "Field `POL_CTL6` writer - Polarity control for each pin"]
pub type POL_CTL6_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL6_A, O>;
impl<'a, const O: u8> POL_CTL6_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL6_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL6_A::LOW)
    }
}
#[doc = "Field `POL_CTL7` reader - Polarity control for each pin"]
pub type POL_CTL7_R = crate::BitReader<POL_CTL7_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL7_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL7_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL7_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL7_A {
        match self.bits {
            false => POL_CTL7_A::HIGH,
            true => POL_CTL7_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL7_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL7_A::LOW
    }
}
#[doc = "Field `POL_CTL7` writer - Polarity control for each pin"]
pub type POL_CTL7_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL7_A, O>;
impl<'a, const O: u8> POL_CTL7_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL7_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL7_A::LOW)
    }
}
#[doc = "Field `POL_CTL8` reader - Polarity control for each pin"]
pub type POL_CTL8_R = crate::BitReader<POL_CTL8_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL8_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL8_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL8_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL8_A {
        match self.bits {
            false => POL_CTL8_A::HIGH,
            true => POL_CTL8_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL8_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL8_A::LOW
    }
}
#[doc = "Field `POL_CTL8` writer - Polarity control for each pin"]
pub type POL_CTL8_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL8_A, O>;
impl<'a, const O: u8> POL_CTL8_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL8_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL8_A::LOW)
    }
}
#[doc = "Field `POL_CTL9` reader - Polarity control for each pin"]
pub type POL_CTL9_R = crate::BitReader<POL_CTL9_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL9_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL9_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL9_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL9_A {
        match self.bits {
            false => POL_CTL9_A::HIGH,
            true => POL_CTL9_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL9_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL9_A::LOW
    }
}
#[doc = "Field `POL_CTL9` writer - Polarity control for each pin"]
pub type POL_CTL9_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL9_A, O>;
impl<'a, const O: u8> POL_CTL9_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL9_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL9_A::LOW)
    }
}
#[doc = "Field `POL_CTL10` reader - Polarity control for each pin"]
pub type POL_CTL10_R = crate::BitReader<POL_CTL10_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL10_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL10_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL10_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL10_A {
        match self.bits {
            false => POL_CTL10_A::HIGH,
            true => POL_CTL10_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL10_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL10_A::LOW
    }
}
#[doc = "Field `POL_CTL10` writer - Polarity control for each pin"]
pub type POL_CTL10_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL10_A, O>;
impl<'a, const O: u8> POL_CTL10_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL10_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL10_A::LOW)
    }
}
#[doc = "Field `POL_CTL11` reader - Polarity control for each pin"]
pub type POL_CTL11_R = crate::BitReader<POL_CTL11_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL11_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL11_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL11_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL11_A {
        match self.bits {
            false => POL_CTL11_A::HIGH,
            true => POL_CTL11_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL11_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL11_A::LOW
    }
}
#[doc = "Field `POL_CTL11` writer - Polarity control for each pin"]
pub type POL_CTL11_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL11_A, O>;
impl<'a, const O: u8> POL_CTL11_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL11_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL11_A::LOW)
    }
}
#[doc = "Field `POL_CTL12` reader - Polarity control for each pin"]
pub type POL_CTL12_R = crate::BitReader<POL_CTL12_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL12_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL12_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL12_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL12_A {
        match self.bits {
            false => POL_CTL12_A::HIGH,
            true => POL_CTL12_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL12_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL12_A::LOW
    }
}
#[doc = "Field `POL_CTL12` writer - Polarity control for each pin"]
pub type POL_CTL12_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL12_A, O>;
impl<'a, const O: u8> POL_CTL12_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL12_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL12_A::LOW)
    }
}
#[doc = "Field `POL_CTL13` reader - Polarity control for each pin"]
pub type POL_CTL13_R = crate::BitReader<POL_CTL13_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL13_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL13_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL13_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL13_A {
        match self.bits {
            false => POL_CTL13_A::HIGH,
            true => POL_CTL13_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL13_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL13_A::LOW
    }
}
#[doc = "Field `POL_CTL13` writer - Polarity control for each pin"]
pub type POL_CTL13_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL13_A, O>;
impl<'a, const O: u8> POL_CTL13_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL13_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL13_A::LOW)
    }
}
#[doc = "Field `POL_CTL14` reader - Polarity control for each pin"]
pub type POL_CTL14_R = crate::BitReader<POL_CTL14_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL14_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL14_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL14_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL14_A {
        match self.bits {
            false => POL_CTL14_A::HIGH,
            true => POL_CTL14_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL14_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL14_A::LOW
    }
}
#[doc = "Field `POL_CTL14` writer - Polarity control for each pin"]
pub type POL_CTL14_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL14_A, O>;
impl<'a, const O: u8> POL_CTL14_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL14_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL14_A::LOW)
    }
}
#[doc = "Field `POL_CTL15` reader - Polarity control for each pin"]
pub type POL_CTL15_R = crate::BitReader<POL_CTL15_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL15_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL15_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL15_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL15_A {
        match self.bits {
            false => POL_CTL15_A::HIGH,
            true => POL_CTL15_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL15_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL15_A::LOW
    }
}
#[doc = "Field `POL_CTL15` writer - Polarity control for each pin"]
pub type POL_CTL15_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL15_A, O>;
impl<'a, const O: u8> POL_CTL15_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL15_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL15_A::LOW)
    }
}
#[doc = "Field `POL_CTL16` reader - Polarity control for each pin"]
pub type POL_CTL16_R = crate::BitReader<POL_CTL16_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL16_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL16_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL16_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL16_A {
        match self.bits {
            false => POL_CTL16_A::HIGH,
            true => POL_CTL16_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL16_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL16_A::LOW
    }
}
#[doc = "Field `POL_CTL16` writer - Polarity control for each pin"]
pub type POL_CTL16_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL16_A, O>;
impl<'a, const O: u8> POL_CTL16_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL16_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL16_A::LOW)
    }
}
#[doc = "Field `POL_CTL17` reader - Polarity control for each pin"]
pub type POL_CTL17_R = crate::BitReader<POL_CTL17_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL17_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL17_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL17_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL17_A {
        match self.bits {
            false => POL_CTL17_A::HIGH,
            true => POL_CTL17_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL17_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL17_A::LOW
    }
}
#[doc = "Field `POL_CTL17` writer - Polarity control for each pin"]
pub type POL_CTL17_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL17_A, O>;
impl<'a, const O: u8> POL_CTL17_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL17_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL17_A::LOW)
    }
}
#[doc = "Field `POL_CTL18` reader - Polarity control for each pin"]
pub type POL_CTL18_R = crate::BitReader<POL_CTL18_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL18_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL18_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL18_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL18_A {
        match self.bits {
            false => POL_CTL18_A::HIGH,
            true => POL_CTL18_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL18_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL18_A::LOW
    }
}
#[doc = "Field `POL_CTL18` writer - Polarity control for each pin"]
pub type POL_CTL18_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL18_A, O>;
impl<'a, const O: u8> POL_CTL18_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL18_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL18_A::LOW)
    }
}
#[doc = "Field `POL_CTL19` reader - Polarity control for each pin"]
pub type POL_CTL19_R = crate::BitReader<POL_CTL19_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL19_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL19_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL19_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL19_A {
        match self.bits {
            false => POL_CTL19_A::HIGH,
            true => POL_CTL19_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL19_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL19_A::LOW
    }
}
#[doc = "Field `POL_CTL19` writer - Polarity control for each pin"]
pub type POL_CTL19_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL19_A, O>;
impl<'a, const O: u8> POL_CTL19_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL19_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL19_A::LOW)
    }
}
#[doc = "Field `POL_CTL20` reader - Polarity control for each pin"]
pub type POL_CTL20_R = crate::BitReader<POL_CTL20_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL20_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL20_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL20_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL20_A {
        match self.bits {
            false => POL_CTL20_A::HIGH,
            true => POL_CTL20_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL20_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL20_A::LOW
    }
}
#[doc = "Field `POL_CTL20` writer - Polarity control for each pin"]
pub type POL_CTL20_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL20_A, O>;
impl<'a, const O: u8> POL_CTL20_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL20_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL20_A::LOW)
    }
}
#[doc = "Field `POL_CTL21` reader - Polarity control for each pin"]
pub type POL_CTL21_R = crate::BitReader<POL_CTL21_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL21_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL21_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL21_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL21_A {
        match self.bits {
            false => POL_CTL21_A::HIGH,
            true => POL_CTL21_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL21_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL21_A::LOW
    }
}
#[doc = "Field `POL_CTL21` writer - Polarity control for each pin"]
pub type POL_CTL21_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL21_A, O>;
impl<'a, const O: u8> POL_CTL21_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL21_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL21_A::LOW)
    }
}
#[doc = "Field `POL_CTL22` reader - Polarity control for each pin"]
pub type POL_CTL22_R = crate::BitReader<POL_CTL22_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL22_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL22_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL22_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL22_A {
        match self.bits {
            false => POL_CTL22_A::HIGH,
            true => POL_CTL22_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL22_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL22_A::LOW
    }
}
#[doc = "Field `POL_CTL22` writer - Polarity control for each pin"]
pub type POL_CTL22_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL22_A, O>;
impl<'a, const O: u8> POL_CTL22_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL22_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL22_A::LOW)
    }
}
#[doc = "Field `POL_CTL23` reader - Polarity control for each pin"]
pub type POL_CTL23_R = crate::BitReader<POL_CTL23_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL23_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL23_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL23_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL23_A {
        match self.bits {
            false => POL_CTL23_A::HIGH,
            true => POL_CTL23_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL23_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL23_A::LOW
    }
}
#[doc = "Field `POL_CTL23` writer - Polarity control for each pin"]
pub type POL_CTL23_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL23_A, O>;
impl<'a, const O: u8> POL_CTL23_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL23_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL23_A::LOW)
    }
}
#[doc = "Field `POL_CTL24` reader - Polarity control for each pin"]
pub type POL_CTL24_R = crate::BitReader<POL_CTL24_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL24_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL24_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL24_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL24_A {
        match self.bits {
            false => POL_CTL24_A::HIGH,
            true => POL_CTL24_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL24_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL24_A::LOW
    }
}
#[doc = "Field `POL_CTL24` writer - Polarity control for each pin"]
pub type POL_CTL24_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL24_A, O>;
impl<'a, const O: u8> POL_CTL24_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL24_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL24_A::LOW)
    }
}
#[doc = "Field `POL_CTL25` reader - Polarity control for each pin"]
pub type POL_CTL25_R = crate::BitReader<POL_CTL25_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL25_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL25_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL25_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL25_A {
        match self.bits {
            false => POL_CTL25_A::HIGH,
            true => POL_CTL25_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL25_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL25_A::LOW
    }
}
#[doc = "Field `POL_CTL25` writer - Polarity control for each pin"]
pub type POL_CTL25_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL25_A, O>;
impl<'a, const O: u8> POL_CTL25_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL25_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL25_A::LOW)
    }
}
#[doc = "Field `POL_CTL26` reader - Polarity control for each pin"]
pub type POL_CTL26_R = crate::BitReader<POL_CTL26_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL26_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL26_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL26_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL26_A {
        match self.bits {
            false => POL_CTL26_A::HIGH,
            true => POL_CTL26_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL26_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL26_A::LOW
    }
}
#[doc = "Field `POL_CTL26` writer - Polarity control for each pin"]
pub type POL_CTL26_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL26_A, O>;
impl<'a, const O: u8> POL_CTL26_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL26_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL26_A::LOW)
    }
}
#[doc = "Field `POL_CTL27` reader - Polarity control for each pin"]
pub type POL_CTL27_R = crate::BitReader<POL_CTL27_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL27_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL27_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL27_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL27_A {
        match self.bits {
            false => POL_CTL27_A::HIGH,
            true => POL_CTL27_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL27_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL27_A::LOW
    }
}
#[doc = "Field `POL_CTL27` writer - Polarity control for each pin"]
pub type POL_CTL27_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL27_A, O>;
impl<'a, const O: u8> POL_CTL27_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL27_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL27_A::LOW)
    }
}
#[doc = "Field `POL_CTL28` reader - Polarity control for each pin"]
pub type POL_CTL28_R = crate::BitReader<POL_CTL28_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL28_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL28_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL28_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL28_A {
        match self.bits {
            false => POL_CTL28_A::HIGH,
            true => POL_CTL28_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL28_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL28_A::LOW
    }
}
#[doc = "Field `POL_CTL28` writer - Polarity control for each pin"]
pub type POL_CTL28_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL28_A, O>;
impl<'a, const O: u8> POL_CTL28_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL28_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL28_A::LOW)
    }
}
#[doc = "Field `POL_CTL29` reader - Polarity control for each pin"]
pub type POL_CTL29_R = crate::BitReader<POL_CTL29_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL29_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL29_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL29_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL29_A {
        match self.bits {
            false => POL_CTL29_A::HIGH,
            true => POL_CTL29_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL29_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL29_A::LOW
    }
}
#[doc = "Field `POL_CTL29` writer - Polarity control for each pin"]
pub type POL_CTL29_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL29_A, O>;
impl<'a, const O: u8> POL_CTL29_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL29_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL29_A::LOW)
    }
}
#[doc = "Field `POL_CTL30` reader - Polarity control for each pin"]
pub type POL_CTL30_R = crate::BitReader<POL_CTL30_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL30_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL30_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL30_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL30_A {
        match self.bits {
            false => POL_CTL30_A::HIGH,
            true => POL_CTL30_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL30_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL30_A::LOW
    }
}
#[doc = "Field `POL_CTL30` writer - Polarity control for each pin"]
pub type POL_CTL30_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL30_A, O>;
impl<'a, const O: u8> POL_CTL30_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL30_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL30_A::LOW)
    }
}
#[doc = "Field `POL_CTL31` reader - Polarity control for each pin"]
pub type POL_CTL31_R = crate::BitReader<POL_CTL31_A>;
#[doc = "Polarity control for each pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_CTL31_A {
    #[doc = "0: High level or rising edge triggered"]
    HIGH = 0,
    #[doc = "1: Low level or falling edge triggered"]
    LOW = 1,
}
impl From<POL_CTL31_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CTL31_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_CTL31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CTL31_A {
        match self.bits {
            false => POL_CTL31_A::HIGH,
            true => POL_CTL31_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_CTL31_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_CTL31_A::LOW
    }
}
#[doc = "Field `POL_CTL31` writer - Polarity control for each pin"]
pub type POL_CTL31_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTPOL0_SPEC, POL_CTL31_A, O>;
impl<'a, const O: u8> POL_CTL31_W<'a, O> {
    #[doc = "High level or rising edge triggered"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_CTL31_A::HIGH)
    }
    #[doc = "Low level or falling edge triggered"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_CTL31_A::LOW)
    }
}
impl R {
    #[doc = "Bit 0 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl0(&self) -> POL_CTL0_R {
        POL_CTL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl1(&self) -> POL_CTL1_R {
        POL_CTL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl2(&self) -> POL_CTL2_R {
        POL_CTL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl3(&self) -> POL_CTL3_R {
        POL_CTL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl4(&self) -> POL_CTL4_R {
        POL_CTL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl5(&self) -> POL_CTL5_R {
        POL_CTL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl6(&self) -> POL_CTL6_R {
        POL_CTL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl7(&self) -> POL_CTL7_R {
        POL_CTL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl8(&self) -> POL_CTL8_R {
        POL_CTL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl9(&self) -> POL_CTL9_R {
        POL_CTL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl10(&self) -> POL_CTL10_R {
        POL_CTL10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl11(&self) -> POL_CTL11_R {
        POL_CTL11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl12(&self) -> POL_CTL12_R {
        POL_CTL12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl13(&self) -> POL_CTL13_R {
        POL_CTL13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl14(&self) -> POL_CTL14_R {
        POL_CTL14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl15(&self) -> POL_CTL15_R {
        POL_CTL15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl16(&self) -> POL_CTL16_R {
        POL_CTL16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl17(&self) -> POL_CTL17_R {
        POL_CTL17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl18(&self) -> POL_CTL18_R {
        POL_CTL18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl19(&self) -> POL_CTL19_R {
        POL_CTL19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl20(&self) -> POL_CTL20_R {
        POL_CTL20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl21(&self) -> POL_CTL21_R {
        POL_CTL21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl22(&self) -> POL_CTL22_R {
        POL_CTL22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl23(&self) -> POL_CTL23_R {
        POL_CTL23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl24(&self) -> POL_CTL24_R {
        POL_CTL24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl25(&self) -> POL_CTL25_R {
        POL_CTL25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl26(&self) -> POL_CTL26_R {
        POL_CTL26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl27(&self) -> POL_CTL27_R {
        POL_CTL27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl28(&self) -> POL_CTL28_R {
        POL_CTL28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl29(&self) -> POL_CTL29_R {
        POL_CTL29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl30(&self) -> POL_CTL30_R {
        POL_CTL30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Polarity control for each pin"]
    #[inline(always)]
    pub fn pol_ctl31(&self) -> POL_CTL31_R {
        POL_CTL31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl0(&mut self) -> POL_CTL0_W<0> {
        POL_CTL0_W::new(self)
    }
    #[doc = "Bit 1 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl1(&mut self) -> POL_CTL1_W<1> {
        POL_CTL1_W::new(self)
    }
    #[doc = "Bit 2 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl2(&mut self) -> POL_CTL2_W<2> {
        POL_CTL2_W::new(self)
    }
    #[doc = "Bit 3 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl3(&mut self) -> POL_CTL3_W<3> {
        POL_CTL3_W::new(self)
    }
    #[doc = "Bit 4 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl4(&mut self) -> POL_CTL4_W<4> {
        POL_CTL4_W::new(self)
    }
    #[doc = "Bit 5 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl5(&mut self) -> POL_CTL5_W<5> {
        POL_CTL5_W::new(self)
    }
    #[doc = "Bit 6 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl6(&mut self) -> POL_CTL6_W<6> {
        POL_CTL6_W::new(self)
    }
    #[doc = "Bit 7 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl7(&mut self) -> POL_CTL7_W<7> {
        POL_CTL7_W::new(self)
    }
    #[doc = "Bit 8 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl8(&mut self) -> POL_CTL8_W<8> {
        POL_CTL8_W::new(self)
    }
    #[doc = "Bit 9 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl9(&mut self) -> POL_CTL9_W<9> {
        POL_CTL9_W::new(self)
    }
    #[doc = "Bit 10 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl10(&mut self) -> POL_CTL10_W<10> {
        POL_CTL10_W::new(self)
    }
    #[doc = "Bit 11 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl11(&mut self) -> POL_CTL11_W<11> {
        POL_CTL11_W::new(self)
    }
    #[doc = "Bit 12 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl12(&mut self) -> POL_CTL12_W<12> {
        POL_CTL12_W::new(self)
    }
    #[doc = "Bit 13 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl13(&mut self) -> POL_CTL13_W<13> {
        POL_CTL13_W::new(self)
    }
    #[doc = "Bit 14 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl14(&mut self) -> POL_CTL14_W<14> {
        POL_CTL14_W::new(self)
    }
    #[doc = "Bit 15 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl15(&mut self) -> POL_CTL15_W<15> {
        POL_CTL15_W::new(self)
    }
    #[doc = "Bit 16 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl16(&mut self) -> POL_CTL16_W<16> {
        POL_CTL16_W::new(self)
    }
    #[doc = "Bit 17 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl17(&mut self) -> POL_CTL17_W<17> {
        POL_CTL17_W::new(self)
    }
    #[doc = "Bit 18 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl18(&mut self) -> POL_CTL18_W<18> {
        POL_CTL18_W::new(self)
    }
    #[doc = "Bit 19 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl19(&mut self) -> POL_CTL19_W<19> {
        POL_CTL19_W::new(self)
    }
    #[doc = "Bit 20 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl20(&mut self) -> POL_CTL20_W<20> {
        POL_CTL20_W::new(self)
    }
    #[doc = "Bit 21 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl21(&mut self) -> POL_CTL21_W<21> {
        POL_CTL21_W::new(self)
    }
    #[doc = "Bit 22 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl22(&mut self) -> POL_CTL22_W<22> {
        POL_CTL22_W::new(self)
    }
    #[doc = "Bit 23 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl23(&mut self) -> POL_CTL23_W<23> {
        POL_CTL23_W::new(self)
    }
    #[doc = "Bit 24 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl24(&mut self) -> POL_CTL24_W<24> {
        POL_CTL24_W::new(self)
    }
    #[doc = "Bit 25 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl25(&mut self) -> POL_CTL25_W<25> {
        POL_CTL25_W::new(self)
    }
    #[doc = "Bit 26 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl26(&mut self) -> POL_CTL26_W<26> {
        POL_CTL26_W::new(self)
    }
    #[doc = "Bit 27 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl27(&mut self) -> POL_CTL27_W<27> {
        POL_CTL27_W::new(self)
    }
    #[doc = "Bit 28 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl28(&mut self) -> POL_CTL28_W<28> {
        POL_CTL28_W::new(self)
    }
    #[doc = "Bit 29 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl29(&mut self) -> POL_CTL29_W<29> {
        POL_CTL29_W::new(self)
    }
    #[doc = "Bit 30 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl30(&mut self) -> POL_CTL30_W<30> {
        POL_CTL30_W::new(self)
    }
    #[doc = "Bit 31 - Polarity control for each pin"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl31(&mut self) -> POL_CTL31_W<31> {
        POL_CTL31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interupt polarity control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intpol0](index.html) module"]
pub struct INTPOL0_SPEC;
impl crate::RegisterSpec for INTPOL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intpol0::R](R) reader structure"]
impl crate::Readable for INTPOL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intpol0::W](W) writer structure"]
impl crate::Writable for INTPOL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTPOL0 to value 0"]
impl crate::Resettable for INTPOL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
