#[doc = "Register `OSR` reader"]
pub struct R(crate::R<OSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSR` writer"]
pub struct W(crate::W<OSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSR_SPEC>;
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
impl From<crate::W<OSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSRVAL` reader - Oversample Selection Value"]
pub type OSRVAL_R = crate::FieldReader<u8, OSRVAL_A>;
#[doc = "Oversample Selection Value\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSRVAL_A {
    #[doc = "0: Not supported"]
    ZERO = 0,
    #[doc = "1: Not supported"]
    ONE = 1,
    #[doc = "2: Not supported"]
    TWO = 2,
    #[doc = "3: Not supported"]
    THREE = 3,
    #[doc = "4: 5 function clocks are used to transmit and receive each data bit."]
    FOUR = 4,
    #[doc = "5: 6 function clocks are used to transmit and receive each data bit."]
    FIVE = 5,
    #[doc = "15: 16 function clocks are used to transmit and receive each data bit."]
    SIXTEEN = 15,
}
impl From<OSRVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: OSRVAL_A) -> Self {
        variant as _
    }
}
impl OSRVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OSRVAL_A> {
        match self.bits {
            0 => Some(OSRVAL_A::ZERO),
            1 => Some(OSRVAL_A::ONE),
            2 => Some(OSRVAL_A::TWO),
            3 => Some(OSRVAL_A::THREE),
            4 => Some(OSRVAL_A::FOUR),
            5 => Some(OSRVAL_A::FIVE),
            15 => Some(OSRVAL_A::SIXTEEN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == OSRVAL_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == OSRVAL_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == OSRVAL_A::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == OSRVAL_A::THREE
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == OSRVAL_A::FOUR
    }
    #[doc = "Checks if the value of the field is `FIVE`"]
    #[inline(always)]
    pub fn is_five(&self) -> bool {
        *self == OSRVAL_A::FIVE
    }
    #[doc = "Checks if the value of the field is `SIXTEEN`"]
    #[inline(always)]
    pub fn is_sixteen(&self) -> bool {
        *self == OSRVAL_A::SIXTEEN
    }
}
#[doc = "Field `OSRVAL` writer - Oversample Selection Value"]
pub type OSRVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSR_SPEC, u8, OSRVAL_A, 4, O>;
impl<'a, const O: u8> OSRVAL_W<'a, O> {
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(OSRVAL_A::ZERO)
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(OSRVAL_A::ONE)
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(OSRVAL_A::TWO)
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(OSRVAL_A::THREE)
    }
    #[doc = "5 function clocks are used to transmit and receive each data bit."]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(OSRVAL_A::FOUR)
    }
    #[doc = "6 function clocks are used to transmit and receive each data bit."]
    #[inline(always)]
    pub fn five(self) -> &'a mut W {
        self.variant(OSRVAL_A::FIVE)
    }
    #[doc = "16 function clocks are used to transmit and receive each data bit."]
    #[inline(always)]
    pub fn sixteen(self) -> &'a mut W {
        self.variant(OSRVAL_A::SIXTEEN)
    }
}
impl R {
    #[doc = "Bits 0:3 - Oversample Selection Value"]
    #[inline(always)]
    pub fn osrval(&self) -> OSRVAL_R {
        OSRVAL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Oversample Selection Value"]
    #[inline(always)]
    #[must_use]
    pub fn osrval(&mut self) -> OSRVAL_W<0> {
        OSRVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oversample Selection Register for Asynchronous Communication\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osr](index.html) module"]
pub struct OSR_SPEC;
impl crate::RegisterSpec for OSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osr::R](R) reader structure"]
impl crate::Readable for OSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osr::W](W) writer structure"]
impl crate::Writable for OSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSR to value 0x0f"]
impl crate::Resettable for OSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
