#[doc = "Register `HWVADGAIN` reader"]
pub struct R(crate::R<HWVADGAIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWVADGAIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWVADGAIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWVADGAIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWVADGAIN` writer"]
pub struct W(crate::W<HWVADGAIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWVADGAIN_SPEC>;
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
impl From<crate::W<HWVADGAIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWVADGAIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPUTGAIN` reader - Input Gain"]
pub type INPUTGAIN_R = crate::FieldReader<u8, INPUTGAIN_A>;
#[doc = "Input Gain\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INPUTGAIN_A {
    #[doc = "0: -10 bits"]
    MINUS10BITS = 0,
    #[doc = "1: -8 bits"]
    MINUS8BITS = 1,
    #[doc = "2: -6 bits"]
    MINUS6BITS = 2,
    #[doc = "3: -4 bits"]
    MINUS4BITS = 3,
    #[doc = "4: -2 bits"]
    MINUS2BITS = 4,
    #[doc = "5: 0 bits (default)"]
    ZEROBITS = 5,
    #[doc = "6: +2 bits"]
    PLUS2BITS = 6,
    #[doc = "7: +4 bits"]
    PLUS4BITS = 7,
    #[doc = "8: +6 bits"]
    PLUS6BITS = 8,
    #[doc = "9: +8 bits"]
    PLUS8BITS = 9,
    #[doc = "10: +10 bits"]
    PLUS10BITS = 10,
    #[doc = "11: +12 bits"]
    PLUS12BITS = 11,
    #[doc = "12: +14 bits"]
    PLUS14BITS = 12,
}
impl From<INPUTGAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUTGAIN_A) -> Self {
        variant as _
    }
}
impl INPUTGAIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INPUTGAIN_A> {
        match self.bits {
            0 => Some(INPUTGAIN_A::MINUS10BITS),
            1 => Some(INPUTGAIN_A::MINUS8BITS),
            2 => Some(INPUTGAIN_A::MINUS6BITS),
            3 => Some(INPUTGAIN_A::MINUS4BITS),
            4 => Some(INPUTGAIN_A::MINUS2BITS),
            5 => Some(INPUTGAIN_A::ZEROBITS),
            6 => Some(INPUTGAIN_A::PLUS2BITS),
            7 => Some(INPUTGAIN_A::PLUS4BITS),
            8 => Some(INPUTGAIN_A::PLUS6BITS),
            9 => Some(INPUTGAIN_A::PLUS8BITS),
            10 => Some(INPUTGAIN_A::PLUS10BITS),
            11 => Some(INPUTGAIN_A::PLUS12BITS),
            12 => Some(INPUTGAIN_A::PLUS14BITS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MINUS10BITS`"]
    #[inline(always)]
    pub fn is_minus10bits(&self) -> bool {
        *self == INPUTGAIN_A::MINUS10BITS
    }
    #[doc = "Checks if the value of the field is `MINUS8BITS`"]
    #[inline(always)]
    pub fn is_minus8bits(&self) -> bool {
        *self == INPUTGAIN_A::MINUS8BITS
    }
    #[doc = "Checks if the value of the field is `MINUS6BITS`"]
    #[inline(always)]
    pub fn is_minus6bits(&self) -> bool {
        *self == INPUTGAIN_A::MINUS6BITS
    }
    #[doc = "Checks if the value of the field is `MINUS4BITS`"]
    #[inline(always)]
    pub fn is_minus4bits(&self) -> bool {
        *self == INPUTGAIN_A::MINUS4BITS
    }
    #[doc = "Checks if the value of the field is `MINUS2BITS`"]
    #[inline(always)]
    pub fn is_minus2bits(&self) -> bool {
        *self == INPUTGAIN_A::MINUS2BITS
    }
    #[doc = "Checks if the value of the field is `ZEROBITS`"]
    #[inline(always)]
    pub fn is_zerobits(&self) -> bool {
        *self == INPUTGAIN_A::ZEROBITS
    }
    #[doc = "Checks if the value of the field is `PLUS2BITS`"]
    #[inline(always)]
    pub fn is_plus2bits(&self) -> bool {
        *self == INPUTGAIN_A::PLUS2BITS
    }
    #[doc = "Checks if the value of the field is `PLUS4BITS`"]
    #[inline(always)]
    pub fn is_plus4bits(&self) -> bool {
        *self == INPUTGAIN_A::PLUS4BITS
    }
    #[doc = "Checks if the value of the field is `PLUS6BITS`"]
    #[inline(always)]
    pub fn is_plus6bits(&self) -> bool {
        *self == INPUTGAIN_A::PLUS6BITS
    }
    #[doc = "Checks if the value of the field is `PLUS8BITS`"]
    #[inline(always)]
    pub fn is_plus8bits(&self) -> bool {
        *self == INPUTGAIN_A::PLUS8BITS
    }
    #[doc = "Checks if the value of the field is `PLUS10BITS`"]
    #[inline(always)]
    pub fn is_plus10bits(&self) -> bool {
        *self == INPUTGAIN_A::PLUS10BITS
    }
    #[doc = "Checks if the value of the field is `PLUS12BITS`"]
    #[inline(always)]
    pub fn is_plus12bits(&self) -> bool {
        *self == INPUTGAIN_A::PLUS12BITS
    }
    #[doc = "Checks if the value of the field is `PLUS14BITS`"]
    #[inline(always)]
    pub fn is_plus14bits(&self) -> bool {
        *self == INPUTGAIN_A::PLUS14BITS
    }
}
#[doc = "Field `INPUTGAIN` writer - Input Gain"]
pub type INPUTGAIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HWVADGAIN_SPEC, u8, INPUTGAIN_A, 4, O>;
impl<'a, const O: u8> INPUTGAIN_W<'a, O> {
    #[doc = "-10 bits"]
    #[inline(always)]
    pub fn minus10bits(self) -> &'a mut W {
        self.variant(INPUTGAIN_A::MINUS10BITS)
    }
    #[doc = "-8 bits"]
    #[inline(always)]
    pub fn minus8bits(self) -> &'a mut W {
        self.variant(INPUTGAIN_A::MINUS8BITS)
    }
    #[doc = "-6 bits"]
    #[inline(always)]
    pub fn minus6bits(self) -> &'a mut W {
        self.variant(INPUTGAIN_A::MINUS6BITS)
    }
    #[doc = "-4 bits"]
    #[inline(always)]
    pub fn minus4bits(self) -> &'a mut W {
        self.variant(INPUTGAIN_A::MINUS4BITS)
    }
    #[doc = "-2 bits"]
    #[inline(always)]
    pub fn minus2bits(self) -> &'a mut W {
        self.variant(INPUTGAIN_A::MINUS2BITS)
    }
    #[doc = "0 bits (default)"]
    #[inline(always)]
    pub fn zerobits(self) -> &'a mut W {
        self.variant(INPUTGAIN_A::ZEROBITS)
    }
    #[doc = "+2 bits"]
    #[inline(always)]
    pub fn plus2bits(self) -> &'a mut W {
        self.variant(INPUTGAIN_A::PLUS2BITS)
    }
    #[doc = "+4 bits"]
    #[inline(always)]
    pub fn plus4bits(self) -> &'a mut W {
        self.variant(INPUTGAIN_A::PLUS4BITS)
    }
    #[doc = "+6 bits"]
    #[inline(always)]
    pub fn plus6bits(self) -> &'a mut W {
        self.variant(INPUTGAIN_A::PLUS6BITS)
    }
    #[doc = "+8 bits"]
    #[inline(always)]
    pub fn plus8bits(self) -> &'a mut W {
        self.variant(INPUTGAIN_A::PLUS8BITS)
    }
    #[doc = "+10 bits"]
    #[inline(always)]
    pub fn plus10bits(self) -> &'a mut W {
        self.variant(INPUTGAIN_A::PLUS10BITS)
    }
    #[doc = "+12 bits"]
    #[inline(always)]
    pub fn plus12bits(self) -> &'a mut W {
        self.variant(INPUTGAIN_A::PLUS12BITS)
    }
    #[doc = "+14 bits"]
    #[inline(always)]
    pub fn plus14bits(self) -> &'a mut W {
        self.variant(INPUTGAIN_A::PLUS14BITS)
    }
}
impl R {
    #[doc = "Bits 0:3 - Input Gain"]
    #[inline(always)]
    pub fn inputgain(&self) -> INPUTGAIN_R {
        INPUTGAIN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Input Gain"]
    #[inline(always)]
    #[must_use]
    pub fn inputgain(&mut self) -> INPUTGAIN_W<0> {
        INPUTGAIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HWVAD Input Gain\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwvadgain](index.html) module"]
pub struct HWVADGAIN_SPEC;
impl crate::RegisterSpec for HWVADGAIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwvadgain::R](R) reader structure"]
impl crate::Readable for HWVADGAIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwvadgain::W](W) writer structure"]
impl crate::Writable for HWVADGAIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HWVADGAIN to value 0x05"]
impl crate::Resettable for HWVADGAIN_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
