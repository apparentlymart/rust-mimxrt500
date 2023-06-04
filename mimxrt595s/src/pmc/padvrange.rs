#[doc = "Register `PADVRANGE` reader"]
pub struct R(crate::R<PADVRANGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADVRANGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADVRANGE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADVRANGE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADVRANGE` writer"]
pub struct W(crate::W<PADVRANGE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADVRANGE_SPEC>;
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
impl From<crate::W<PADVRANGE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADVRANGE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VDDIO_0RANGE` reader - VDDIO_0RANGE"]
pub type VDDIO_0RANGE_R = crate::FieldReader<u8, VDDIO_0RANGE_A>;
#[doc = "VDDIO_0RANGE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VDDIO_0RANGE_A {
    #[doc = "0: 1.71 - 1.98V. Consumes static current to detect VDDIO_0 level. To reduce power consumption, change this value to 01."]
    DISABLE = 0,
    #[doc = "1: 1.71 - 1.98V, vdde detector off"]
    ENABLE = 1,
    #[doc = "2: Not allowed"]
    VALUE_0B10 = 2,
    #[doc = "3: Not allowed (hardware translates to 10)"]
    VALUE_0B11 = 3,
}
impl From<VDDIO_0RANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: VDDIO_0RANGE_A) -> Self {
        variant as _
    }
}
impl VDDIO_0RANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDIO_0RANGE_A {
        match self.bits {
            0 => VDDIO_0RANGE_A::DISABLE,
            1 => VDDIO_0RANGE_A::ENABLE,
            2 => VDDIO_0RANGE_A::VALUE_0B10,
            3 => VDDIO_0RANGE_A::VALUE_0B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VDDIO_0RANGE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VDDIO_0RANGE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `VALUE_0B10`"]
    #[inline(always)]
    pub fn is_value_0b10(&self) -> bool {
        *self == VDDIO_0RANGE_A::VALUE_0B10
    }
    #[doc = "Checks if the value of the field is `VALUE_0B11`"]
    #[inline(always)]
    pub fn is_value_0b11(&self) -> bool {
        *self == VDDIO_0RANGE_A::VALUE_0B11
    }
}
#[doc = "Field `VDDIO_0RANGE` writer - VDDIO_0RANGE"]
pub type VDDIO_0RANGE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PADVRANGE_SPEC, u8, VDDIO_0RANGE_A, 2, O>;
impl<'a, const O: u8> VDDIO_0RANGE_W<'a, O> {
    #[doc = "1.71 - 1.98V. Consumes static current to detect VDDIO_0 level. To reduce power consumption, change this value to 01."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VDDIO_0RANGE_A::DISABLE)
    }
    #[doc = "1.71 - 1.98V, vdde detector off"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VDDIO_0RANGE_A::ENABLE)
    }
    #[doc = "Not allowed"]
    #[inline(always)]
    pub fn value_0b10(self) -> &'a mut W {
        self.variant(VDDIO_0RANGE_A::VALUE_0B10)
    }
    #[doc = "Not allowed (hardware translates to 10)"]
    #[inline(always)]
    pub fn value_0b11(self) -> &'a mut W {
        self.variant(VDDIO_0RANGE_A::VALUE_0B11)
    }
}
#[doc = "Field `VDDIO_1RANGE` reader - VDDIO1RANGE It is recommended that the user change this value to 01 to reduce power consumption."]
pub type VDDIO_1RANGE_R = crate::FieldReader<u8, VDDIO_1RANGE_A>;
#[doc = "VDDIO1RANGE It is recommended that the user change this value to 01 to reduce power consumption.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VDDIO_1RANGE_A {
    #[doc = "0: 1.71 - 1.98V. Continuous mode. Consumes static current to detect VDDIO_1 level."]
    DISABLE = 0,
    #[doc = "1: 1.71 - 1.98V, vdde detector off"]
    ENABLE = 1,
    #[doc = "2: Not allowed"]
    VALUE_0B10 = 2,
    #[doc = "3: Not allowed (hardware translates to 00 = continuous mode)"]
    VALUE_0B11 = 3,
}
impl From<VDDIO_1RANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: VDDIO_1RANGE_A) -> Self {
        variant as _
    }
}
impl VDDIO_1RANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDIO_1RANGE_A {
        match self.bits {
            0 => VDDIO_1RANGE_A::DISABLE,
            1 => VDDIO_1RANGE_A::ENABLE,
            2 => VDDIO_1RANGE_A::VALUE_0B10,
            3 => VDDIO_1RANGE_A::VALUE_0B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VDDIO_1RANGE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VDDIO_1RANGE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `VALUE_0B10`"]
    #[inline(always)]
    pub fn is_value_0b10(&self) -> bool {
        *self == VDDIO_1RANGE_A::VALUE_0B10
    }
    #[doc = "Checks if the value of the field is `VALUE_0B11`"]
    #[inline(always)]
    pub fn is_value_0b11(&self) -> bool {
        *self == VDDIO_1RANGE_A::VALUE_0B11
    }
}
#[doc = "Field `VDDIO_1RANGE` writer - VDDIO1RANGE It is recommended that the user change this value to 01 to reduce power consumption."]
pub type VDDIO_1RANGE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PADVRANGE_SPEC, u8, VDDIO_1RANGE_A, 2, O>;
impl<'a, const O: u8> VDDIO_1RANGE_W<'a, O> {
    #[doc = "1.71 - 1.98V. Continuous mode. Consumes static current to detect VDDIO_1 level."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VDDIO_1RANGE_A::DISABLE)
    }
    #[doc = "1.71 - 1.98V, vdde detector off"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VDDIO_1RANGE_A::ENABLE)
    }
    #[doc = "Not allowed"]
    #[inline(always)]
    pub fn value_0b10(self) -> &'a mut W {
        self.variant(VDDIO_1RANGE_A::VALUE_0B10)
    }
    #[doc = "Not allowed (hardware translates to 00 = continuous mode)"]
    #[inline(always)]
    pub fn value_0b11(self) -> &'a mut W {
        self.variant(VDDIO_1RANGE_A::VALUE_0B11)
    }
}
#[doc = "Field `VDDIO_2RANGE` reader - VDDIO2RANGE"]
pub type VDDIO_2RANGE_R = crate::FieldReader<u8, VDDIO_2RANGE_A>;
#[doc = "VDDIO2RANGE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VDDIO_2RANGE_A {
    #[doc = "0: 1.71 - 1.98V. Continuous mode. Consumes static current to detect VDDIO_2 level. To reduce power consumption, change this value to 01."]
    DISABLE = 0,
    #[doc = "1: 1.71 - 1.98V, vdde detector off"]
    ENABLE = 1,
    #[doc = "2: Not allowed"]
    VALUE_0B10 = 2,
    #[doc = "3: Not allowed (hardware translates to 00 = continuous mode)"]
    VALUE_0B11 = 3,
}
impl From<VDDIO_2RANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: VDDIO_2RANGE_A) -> Self {
        variant as _
    }
}
impl VDDIO_2RANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDIO_2RANGE_A {
        match self.bits {
            0 => VDDIO_2RANGE_A::DISABLE,
            1 => VDDIO_2RANGE_A::ENABLE,
            2 => VDDIO_2RANGE_A::VALUE_0B10,
            3 => VDDIO_2RANGE_A::VALUE_0B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VDDIO_2RANGE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VDDIO_2RANGE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `VALUE_0B10`"]
    #[inline(always)]
    pub fn is_value_0b10(&self) -> bool {
        *self == VDDIO_2RANGE_A::VALUE_0B10
    }
    #[doc = "Checks if the value of the field is `VALUE_0B11`"]
    #[inline(always)]
    pub fn is_value_0b11(&self) -> bool {
        *self == VDDIO_2RANGE_A::VALUE_0B11
    }
}
#[doc = "Field `VDDIO_2RANGE` writer - VDDIO2RANGE"]
pub type VDDIO_2RANGE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PADVRANGE_SPEC, u8, VDDIO_2RANGE_A, 2, O>;
impl<'a, const O: u8> VDDIO_2RANGE_W<'a, O> {
    #[doc = "1.71 - 1.98V. Continuous mode. Consumes static current to detect VDDIO_2 level. To reduce power consumption, change this value to 01."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VDDIO_2RANGE_A::DISABLE)
    }
    #[doc = "1.71 - 1.98V, vdde detector off"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VDDIO_2RANGE_A::ENABLE)
    }
    #[doc = "Not allowed"]
    #[inline(always)]
    pub fn value_0b10(self) -> &'a mut W {
        self.variant(VDDIO_2RANGE_A::VALUE_0B10)
    }
    #[doc = "Not allowed (hardware translates to 00 = continuous mode)"]
    #[inline(always)]
    pub fn value_0b11(self) -> &'a mut W {
        self.variant(VDDIO_2RANGE_A::VALUE_0B11)
    }
}
#[doc = "Field `VDDIO_3RANGE` reader - VDDIO3RANGE"]
pub type VDDIO_3RANGE_R = crate::FieldReader<u8, VDDIO_3RANGE_A>;
#[doc = "VDDIO3RANGE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VDDIO_3RANGE_A {
    #[doc = "0: 1.71 - 3.6V. Continuous mode. Consumes static current to detect VDDIO_3 level"]
    DISABLE = 0,
    #[doc = "1: 1.71 - 1.98V, vdde detector off"]
    ENABLE = 1,
    #[doc = "2: 3.00 - 3.6V, vdde detector off"]
    VALUE_0B10 = 2,
    #[doc = "3: Not allowed (hardware translates to 00 = continuous mode)"]
    VALUE_0B11 = 3,
}
impl From<VDDIO_3RANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: VDDIO_3RANGE_A) -> Self {
        variant as _
    }
}
impl VDDIO_3RANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDIO_3RANGE_A {
        match self.bits {
            0 => VDDIO_3RANGE_A::DISABLE,
            1 => VDDIO_3RANGE_A::ENABLE,
            2 => VDDIO_3RANGE_A::VALUE_0B10,
            3 => VDDIO_3RANGE_A::VALUE_0B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VDDIO_3RANGE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VDDIO_3RANGE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `VALUE_0B10`"]
    #[inline(always)]
    pub fn is_value_0b10(&self) -> bool {
        *self == VDDIO_3RANGE_A::VALUE_0B10
    }
    #[doc = "Checks if the value of the field is `VALUE_0B11`"]
    #[inline(always)]
    pub fn is_value_0b11(&self) -> bool {
        *self == VDDIO_3RANGE_A::VALUE_0B11
    }
}
#[doc = "Field `VDDIO_3RANGE` writer - VDDIO3RANGE"]
pub type VDDIO_3RANGE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PADVRANGE_SPEC, u8, VDDIO_3RANGE_A, 2, O>;
impl<'a, const O: u8> VDDIO_3RANGE_W<'a, O> {
    #[doc = "1.71 - 3.6V. Continuous mode. Consumes static current to detect VDDIO_3 level"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VDDIO_3RANGE_A::DISABLE)
    }
    #[doc = "1.71 - 1.98V, vdde detector off"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VDDIO_3RANGE_A::ENABLE)
    }
    #[doc = "3.00 - 3.6V, vdde detector off"]
    #[inline(always)]
    pub fn value_0b10(self) -> &'a mut W {
        self.variant(VDDIO_3RANGE_A::VALUE_0B10)
    }
    #[doc = "Not allowed (hardware translates to 00 = continuous mode)"]
    #[inline(always)]
    pub fn value_0b11(self) -> &'a mut W {
        self.variant(VDDIO_3RANGE_A::VALUE_0B11)
    }
}
#[doc = "Field `VDDIO_4RANGE` reader - VDDIO4RANGE"]
pub type VDDIO_4RANGE_R = crate::FieldReader<u8, VDDIO_4RANGE_A>;
#[doc = "VDDIO4RANGE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VDDIO_4RANGE_A {
    #[doc = "0: 1.71 - 1.98V. Continuous mode. Consumes static current to detect VDDIO_4 level. To reduce power consumption, change this value to 01."]
    DISABLE = 0,
    #[doc = "1: 1.71 - 1.98V, vdde detector off"]
    ENABLE = 1,
    #[doc = "2: Not allowed"]
    VALUE_0B10 = 2,
    #[doc = "3: Not allowed (hardware translates to 00 = continuous mode)"]
    VALUE_0B11 = 3,
}
impl From<VDDIO_4RANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: VDDIO_4RANGE_A) -> Self {
        variant as _
    }
}
impl VDDIO_4RANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDIO_4RANGE_A {
        match self.bits {
            0 => VDDIO_4RANGE_A::DISABLE,
            1 => VDDIO_4RANGE_A::ENABLE,
            2 => VDDIO_4RANGE_A::VALUE_0B10,
            3 => VDDIO_4RANGE_A::VALUE_0B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VDDIO_4RANGE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VDDIO_4RANGE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `VALUE_0B10`"]
    #[inline(always)]
    pub fn is_value_0b10(&self) -> bool {
        *self == VDDIO_4RANGE_A::VALUE_0B10
    }
    #[doc = "Checks if the value of the field is `VALUE_0B11`"]
    #[inline(always)]
    pub fn is_value_0b11(&self) -> bool {
        *self == VDDIO_4RANGE_A::VALUE_0B11
    }
}
#[doc = "Field `VDDIO_4RANGE` writer - VDDIO4RANGE"]
pub type VDDIO_4RANGE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PADVRANGE_SPEC, u8, VDDIO_4RANGE_A, 2, O>;
impl<'a, const O: u8> VDDIO_4RANGE_W<'a, O> {
    #[doc = "1.71 - 1.98V. Continuous mode. Consumes static current to detect VDDIO_4 level. To reduce power consumption, change this value to 01."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VDDIO_4RANGE_A::DISABLE)
    }
    #[doc = "1.71 - 1.98V, vdde detector off"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VDDIO_4RANGE_A::ENABLE)
    }
    #[doc = "Not allowed"]
    #[inline(always)]
    pub fn value_0b10(self) -> &'a mut W {
        self.variant(VDDIO_4RANGE_A::VALUE_0B10)
    }
    #[doc = "Not allowed (hardware translates to 00 = continuous mode)"]
    #[inline(always)]
    pub fn value_0b11(self) -> &'a mut W {
        self.variant(VDDIO_4RANGE_A::VALUE_0B11)
    }
}
impl R {
    #[doc = "Bits 0:1 - VDDIO_0RANGE"]
    #[inline(always)]
    pub fn vddio_0range(&self) -> VDDIO_0RANGE_R {
        VDDIO_0RANGE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - VDDIO1RANGE It is recommended that the user change this value to 01 to reduce power consumption."]
    #[inline(always)]
    pub fn vddio_1range(&self) -> VDDIO_1RANGE_R {
        VDDIO_1RANGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - VDDIO2RANGE"]
    #[inline(always)]
    pub fn vddio_2range(&self) -> VDDIO_2RANGE_R {
        VDDIO_2RANGE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - VDDIO3RANGE"]
    #[inline(always)]
    pub fn vddio_3range(&self) -> VDDIO_3RANGE_R {
        VDDIO_3RANGE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - VDDIO4RANGE"]
    #[inline(always)]
    pub fn vddio_4range(&self) -> VDDIO_4RANGE_R {
        VDDIO_4RANGE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - VDDIO_0RANGE"]
    #[inline(always)]
    #[must_use]
    pub fn vddio_0range(&mut self) -> VDDIO_0RANGE_W<0> {
        VDDIO_0RANGE_W::new(self)
    }
    #[doc = "Bits 2:3 - VDDIO1RANGE It is recommended that the user change this value to 01 to reduce power consumption."]
    #[inline(always)]
    #[must_use]
    pub fn vddio_1range(&mut self) -> VDDIO_1RANGE_W<2> {
        VDDIO_1RANGE_W::new(self)
    }
    #[doc = "Bits 4:5 - VDDIO2RANGE"]
    #[inline(always)]
    #[must_use]
    pub fn vddio_2range(&mut self) -> VDDIO_2RANGE_W<4> {
        VDDIO_2RANGE_W::new(self)
    }
    #[doc = "Bits 6:7 - VDDIO3RANGE"]
    #[inline(always)]
    #[must_use]
    pub fn vddio_3range(&mut self) -> VDDIO_3RANGE_W<6> {
        VDDIO_3RANGE_W::new(self)
    }
    #[doc = "Bits 8:9 - VDDIO4RANGE"]
    #[inline(always)]
    #[must_use]
    pub fn vddio_4range(&mut self) -> VDDIO_4RANGE_W<8> {
        VDDIO_4RANGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMC GPIO VDDIO Range Selection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padvrange](index.html) module"]
pub struct PADVRANGE_SPEC;
impl crate::RegisterSpec for PADVRANGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padvrange::R](R) reader structure"]
impl crate::Readable for PADVRANGE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padvrange::W](W) writer structure"]
impl crate::Writable for PADVRANGE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PADVRANGE to value 0"]
impl crate::Resettable for PADVRANGE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
