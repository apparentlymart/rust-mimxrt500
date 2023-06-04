#[doc = "Register `OUTPUT` reader"]
pub struct R(crate::R<OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTPUT` writer"]
pub struct W(crate::W<OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTPUT_SPEC>;
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
impl From<crate::W<OUTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT0` reader - Output n"]
pub type OUT0_R = crate::BitReader<OUT0_A>;
#[doc = "Output n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUT0_A {
    #[doc = "0: Writing a 0 forces the corresponding output low"]
    LOW = 0,
    #[doc = "1: Writing a 1 forces the corresponding output high"]
    HIGH = 1,
}
impl From<OUT0_A> for bool {
    #[inline(always)]
    fn from(variant: OUT0_A) -> Self {
        variant as u8 != 0
    }
}
impl OUT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT0_A {
        match self.bits {
            false => OUT0_A::LOW,
            true => OUT0_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OUT0_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OUT0_A::HIGH
    }
}
#[doc = "Field `OUT0` writer - Output n"]
pub type OUT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTPUT_SPEC, OUT0_A, O>;
impl<'a, const O: u8> OUT0_W<'a, O> {
    #[doc = "Writing a 0 forces the corresponding output low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OUT0_A::LOW)
    }
    #[doc = "Writing a 1 forces the corresponding output high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OUT0_A::HIGH)
    }
}
#[doc = "Field `OUT1` reader - Output n"]
pub type OUT1_R = crate::BitReader<OUT1_A>;
#[doc = "Output n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUT1_A {
    #[doc = "0: Writing a 0 forces the corresponding output low"]
    LOW = 0,
    #[doc = "1: Writing a 1 forces the corresponding output high"]
    HIGH = 1,
}
impl From<OUT1_A> for bool {
    #[inline(always)]
    fn from(variant: OUT1_A) -> Self {
        variant as u8 != 0
    }
}
impl OUT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT1_A {
        match self.bits {
            false => OUT1_A::LOW,
            true => OUT1_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OUT1_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OUT1_A::HIGH
    }
}
#[doc = "Field `OUT1` writer - Output n"]
pub type OUT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTPUT_SPEC, OUT1_A, O>;
impl<'a, const O: u8> OUT1_W<'a, O> {
    #[doc = "Writing a 0 forces the corresponding output low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OUT1_A::LOW)
    }
    #[doc = "Writing a 1 forces the corresponding output high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OUT1_A::HIGH)
    }
}
#[doc = "Field `OUT2` reader - Output n"]
pub type OUT2_R = crate::BitReader<OUT2_A>;
#[doc = "Output n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUT2_A {
    #[doc = "0: Writing a 0 forces the corresponding output low"]
    LOW = 0,
    #[doc = "1: Writing a 1 forces the corresponding output high"]
    HIGH = 1,
}
impl From<OUT2_A> for bool {
    #[inline(always)]
    fn from(variant: OUT2_A) -> Self {
        variant as u8 != 0
    }
}
impl OUT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT2_A {
        match self.bits {
            false => OUT2_A::LOW,
            true => OUT2_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OUT2_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OUT2_A::HIGH
    }
}
#[doc = "Field `OUT2` writer - Output n"]
pub type OUT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTPUT_SPEC, OUT2_A, O>;
impl<'a, const O: u8> OUT2_W<'a, O> {
    #[doc = "Writing a 0 forces the corresponding output low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OUT2_A::LOW)
    }
    #[doc = "Writing a 1 forces the corresponding output high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OUT2_A::HIGH)
    }
}
#[doc = "Field `OUT3` reader - Output n"]
pub type OUT3_R = crate::BitReader<OUT3_A>;
#[doc = "Output n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUT3_A {
    #[doc = "0: Writing a 0 forces the corresponding output low"]
    LOW = 0,
    #[doc = "1: Writing a 1 forces the corresponding output high"]
    HIGH = 1,
}
impl From<OUT3_A> for bool {
    #[inline(always)]
    fn from(variant: OUT3_A) -> Self {
        variant as u8 != 0
    }
}
impl OUT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT3_A {
        match self.bits {
            false => OUT3_A::LOW,
            true => OUT3_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OUT3_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OUT3_A::HIGH
    }
}
#[doc = "Field `OUT3` writer - Output n"]
pub type OUT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTPUT_SPEC, OUT3_A, O>;
impl<'a, const O: u8> OUT3_W<'a, O> {
    #[doc = "Writing a 0 forces the corresponding output low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OUT3_A::LOW)
    }
    #[doc = "Writing a 1 forces the corresponding output high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OUT3_A::HIGH)
    }
}
#[doc = "Field `OUT4` reader - Output n"]
pub type OUT4_R = crate::BitReader<OUT4_A>;
#[doc = "Output n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUT4_A {
    #[doc = "0: Writing a 0 forces the corresponding output low"]
    LOW = 0,
    #[doc = "1: Writing a 1 forces the corresponding output high"]
    HIGH = 1,
}
impl From<OUT4_A> for bool {
    #[inline(always)]
    fn from(variant: OUT4_A) -> Self {
        variant as u8 != 0
    }
}
impl OUT4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT4_A {
        match self.bits {
            false => OUT4_A::LOW,
            true => OUT4_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OUT4_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OUT4_A::HIGH
    }
}
#[doc = "Field `OUT4` writer - Output n"]
pub type OUT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTPUT_SPEC, OUT4_A, O>;
impl<'a, const O: u8> OUT4_W<'a, O> {
    #[doc = "Writing a 0 forces the corresponding output low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OUT4_A::LOW)
    }
    #[doc = "Writing a 1 forces the corresponding output high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OUT4_A::HIGH)
    }
}
#[doc = "Field `OUT5` reader - Output n"]
pub type OUT5_R = crate::BitReader<OUT5_A>;
#[doc = "Output n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUT5_A {
    #[doc = "0: Writing a 0 forces the corresponding output low"]
    LOW = 0,
    #[doc = "1: Writing a 1 forces the corresponding output high"]
    HIGH = 1,
}
impl From<OUT5_A> for bool {
    #[inline(always)]
    fn from(variant: OUT5_A) -> Self {
        variant as u8 != 0
    }
}
impl OUT5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT5_A {
        match self.bits {
            false => OUT5_A::LOW,
            true => OUT5_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OUT5_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OUT5_A::HIGH
    }
}
#[doc = "Field `OUT5` writer - Output n"]
pub type OUT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTPUT_SPEC, OUT5_A, O>;
impl<'a, const O: u8> OUT5_W<'a, O> {
    #[doc = "Writing a 0 forces the corresponding output low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OUT5_A::LOW)
    }
    #[doc = "Writing a 1 forces the corresponding output high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OUT5_A::HIGH)
    }
}
#[doc = "Field `OUT6` reader - Output n"]
pub type OUT6_R = crate::BitReader<OUT6_A>;
#[doc = "Output n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUT6_A {
    #[doc = "0: Writing a 0 forces the corresponding output low"]
    LOW = 0,
    #[doc = "1: Writing a 1 forces the corresponding output high"]
    HIGH = 1,
}
impl From<OUT6_A> for bool {
    #[inline(always)]
    fn from(variant: OUT6_A) -> Self {
        variant as u8 != 0
    }
}
impl OUT6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT6_A {
        match self.bits {
            false => OUT6_A::LOW,
            true => OUT6_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OUT6_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OUT6_A::HIGH
    }
}
#[doc = "Field `OUT6` writer - Output n"]
pub type OUT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTPUT_SPEC, OUT6_A, O>;
impl<'a, const O: u8> OUT6_W<'a, O> {
    #[doc = "Writing a 0 forces the corresponding output low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OUT6_A::LOW)
    }
    #[doc = "Writing a 1 forces the corresponding output high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OUT6_A::HIGH)
    }
}
#[doc = "Field `OUT7` reader - Output n"]
pub type OUT7_R = crate::BitReader<OUT7_A>;
#[doc = "Output n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUT7_A {
    #[doc = "0: Writing a 0 forces the corresponding output low"]
    LOW = 0,
    #[doc = "1: Writing a 1 forces the corresponding output high"]
    HIGH = 1,
}
impl From<OUT7_A> for bool {
    #[inline(always)]
    fn from(variant: OUT7_A) -> Self {
        variant as u8 != 0
    }
}
impl OUT7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT7_A {
        match self.bits {
            false => OUT7_A::LOW,
            true => OUT7_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OUT7_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OUT7_A::HIGH
    }
}
#[doc = "Field `OUT7` writer - Output n"]
pub type OUT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTPUT_SPEC, OUT7_A, O>;
impl<'a, const O: u8> OUT7_W<'a, O> {
    #[doc = "Writing a 0 forces the corresponding output low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OUT7_A::LOW)
    }
    #[doc = "Writing a 1 forces the corresponding output high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OUT7_A::HIGH)
    }
}
#[doc = "Field `OUT8` reader - Output n"]
pub type OUT8_R = crate::BitReader<OUT8_A>;
#[doc = "Output n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUT8_A {
    #[doc = "0: Writing a 0 forces the corresponding output low"]
    LOW = 0,
    #[doc = "1: Writing a 1 forces the corresponding output high"]
    HIGH = 1,
}
impl From<OUT8_A> for bool {
    #[inline(always)]
    fn from(variant: OUT8_A) -> Self {
        variant as u8 != 0
    }
}
impl OUT8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT8_A {
        match self.bits {
            false => OUT8_A::LOW,
            true => OUT8_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OUT8_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OUT8_A::HIGH
    }
}
#[doc = "Field `OUT8` writer - Output n"]
pub type OUT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTPUT_SPEC, OUT8_A, O>;
impl<'a, const O: u8> OUT8_W<'a, O> {
    #[doc = "Writing a 0 forces the corresponding output low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OUT8_A::LOW)
    }
    #[doc = "Writing a 1 forces the corresponding output high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OUT8_A::HIGH)
    }
}
#[doc = "Field `OUT9` reader - Output n"]
pub type OUT9_R = crate::BitReader<OUT9_A>;
#[doc = "Output n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUT9_A {
    #[doc = "0: Writing a 0 forces the corresponding output low"]
    LOW = 0,
    #[doc = "1: Writing a 1 forces the corresponding output high"]
    HIGH = 1,
}
impl From<OUT9_A> for bool {
    #[inline(always)]
    fn from(variant: OUT9_A) -> Self {
        variant as u8 != 0
    }
}
impl OUT9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT9_A {
        match self.bits {
            false => OUT9_A::LOW,
            true => OUT9_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OUT9_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OUT9_A::HIGH
    }
}
#[doc = "Field `OUT9` writer - Output n"]
pub type OUT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTPUT_SPEC, OUT9_A, O>;
impl<'a, const O: u8> OUT9_W<'a, O> {
    #[doc = "Writing a 0 forces the corresponding output low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OUT9_A::LOW)
    }
    #[doc = "Writing a 1 forces the corresponding output high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OUT9_A::HIGH)
    }
}
impl R {
    #[doc = "Bit 0 - Output n"]
    #[inline(always)]
    pub fn out0(&self) -> OUT0_R {
        OUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output n"]
    #[inline(always)]
    pub fn out1(&self) -> OUT1_R {
        OUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output n"]
    #[inline(always)]
    pub fn out2(&self) -> OUT2_R {
        OUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output n"]
    #[inline(always)]
    pub fn out3(&self) -> OUT3_R {
        OUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Output n"]
    #[inline(always)]
    pub fn out4(&self) -> OUT4_R {
        OUT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Output n"]
    #[inline(always)]
    pub fn out5(&self) -> OUT5_R {
        OUT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Output n"]
    #[inline(always)]
    pub fn out6(&self) -> OUT6_R {
        OUT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output n"]
    #[inline(always)]
    pub fn out7(&self) -> OUT7_R {
        OUT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Output n"]
    #[inline(always)]
    pub fn out8(&self) -> OUT8_R {
        OUT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output n"]
    #[inline(always)]
    pub fn out9(&self) -> OUT9_R {
        OUT9_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output n"]
    #[inline(always)]
    #[must_use]
    pub fn out0(&mut self) -> OUT0_W<0> {
        OUT0_W::new(self)
    }
    #[doc = "Bit 1 - Output n"]
    #[inline(always)]
    #[must_use]
    pub fn out1(&mut self) -> OUT1_W<1> {
        OUT1_W::new(self)
    }
    #[doc = "Bit 2 - Output n"]
    #[inline(always)]
    #[must_use]
    pub fn out2(&mut self) -> OUT2_W<2> {
        OUT2_W::new(self)
    }
    #[doc = "Bit 3 - Output n"]
    #[inline(always)]
    #[must_use]
    pub fn out3(&mut self) -> OUT3_W<3> {
        OUT3_W::new(self)
    }
    #[doc = "Bit 4 - Output n"]
    #[inline(always)]
    #[must_use]
    pub fn out4(&mut self) -> OUT4_W<4> {
        OUT4_W::new(self)
    }
    #[doc = "Bit 5 - Output n"]
    #[inline(always)]
    #[must_use]
    pub fn out5(&mut self) -> OUT5_W<5> {
        OUT5_W::new(self)
    }
    #[doc = "Bit 6 - Output n"]
    #[inline(always)]
    #[must_use]
    pub fn out6(&mut self) -> OUT6_W<6> {
        OUT6_W::new(self)
    }
    #[doc = "Bit 7 - Output n"]
    #[inline(always)]
    #[must_use]
    pub fn out7(&mut self) -> OUT7_W<7> {
        OUT7_W::new(self)
    }
    #[doc = "Bit 8 - Output n"]
    #[inline(always)]
    #[must_use]
    pub fn out8(&mut self) -> OUT8_W<8> {
        OUT8_W::new(self)
    }
    #[doc = "Bit 9 - Output n"]
    #[inline(always)]
    #[must_use]
    pub fn out9(&mut self) -> OUT9_W<9> {
        OUT9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [output](index.html) module"]
pub struct OUTPUT_SPEC;
impl crate::RegisterSpec for OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [output::R](R) reader structure"]
impl crate::Readable for OUTPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [output::W](W) writer structure"]
impl crate::Writable for OUTPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTPUT to value 0"]
impl crate::Resettable for OUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
