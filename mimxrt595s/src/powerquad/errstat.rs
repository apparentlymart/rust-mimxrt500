#[doc = "Register `ERRSTAT` reader"]
pub struct R(crate::R<ERRSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERRSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERRSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERRSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERRSTAT` writer"]
pub struct W(crate::W<ERRSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERRSTAT_SPEC>;
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
impl From<crate::W<ERRSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERRSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVERFLOW` reader - Floating point overflow"]
pub type OVERFLOW_R = crate::BitReader<OVERFLOW_A>;
#[doc = "Floating point overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVERFLOW_A {
    #[doc = "0: No error"]
    NO_ERROR = 0,
    #[doc = "1: Error on floating point overflow"]
    ERROR = 1,
}
impl From<OVERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: OVERFLOW_A) -> Self {
        variant as u8 != 0
    }
}
impl OVERFLOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERFLOW_A {
        match self.bits {
            false => OVERFLOW_A::NO_ERROR,
            true => OVERFLOW_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == OVERFLOW_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == OVERFLOW_A::ERROR
    }
}
#[doc = "Field `OVERFLOW` writer - Floating point overflow"]
pub type OVERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRSTAT_SPEC, OVERFLOW_A, O>;
impl<'a, const O: u8> OVERFLOW_W<'a, O> {
    #[doc = "No error"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(OVERFLOW_A::NO_ERROR)
    }
    #[doc = "Error on floating point overflow"]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(OVERFLOW_A::ERROR)
    }
}
#[doc = "Field `NAN` reader - Floating Point NaN"]
pub type NAN_R = crate::BitReader<NAN_A>;
#[doc = "Floating Point NaN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NAN_A {
    #[doc = "0: No error"]
    NO_ERROR = 0,
    #[doc = "1: Error on Floating Point NaN"]
    ERROR = 1,
}
impl From<NAN_A> for bool {
    #[inline(always)]
    fn from(variant: NAN_A) -> Self {
        variant as u8 != 0
    }
}
impl NAN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NAN_A {
        match self.bits {
            false => NAN_A::NO_ERROR,
            true => NAN_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == NAN_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == NAN_A::ERROR
    }
}
#[doc = "Field `NAN` writer - Floating Point NaN"]
pub type NAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRSTAT_SPEC, NAN_A, O>;
impl<'a, const O: u8> NAN_W<'a, O> {
    #[doc = "No error"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(NAN_A::NO_ERROR)
    }
    #[doc = "Error on Floating Point NaN"]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(NAN_A::ERROR)
    }
}
#[doc = "Field `FIXEDOVERFLOW` reader - Fixed point overflow"]
pub type FIXEDOVERFLOW_R = crate::BitReader<FIXEDOVERFLOW_A>;
#[doc = "Fixed point overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIXEDOVERFLOW_A {
    #[doc = "0: No error"]
    NO_ERROR = 0,
    #[doc = "1: Error on fixed point overflow"]
    ERROR = 1,
}
impl From<FIXEDOVERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: FIXEDOVERFLOW_A) -> Self {
        variant as u8 != 0
    }
}
impl FIXEDOVERFLOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIXEDOVERFLOW_A {
        match self.bits {
            false => FIXEDOVERFLOW_A::NO_ERROR,
            true => FIXEDOVERFLOW_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == FIXEDOVERFLOW_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FIXEDOVERFLOW_A::ERROR
    }
}
#[doc = "Field `FIXEDOVERFLOW` writer - Fixed point overflow"]
pub type FIXEDOVERFLOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ERRSTAT_SPEC, FIXEDOVERFLOW_A, O>;
impl<'a, const O: u8> FIXEDOVERFLOW_W<'a, O> {
    #[doc = "No error"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(FIXEDOVERFLOW_A::NO_ERROR)
    }
    #[doc = "Error on fixed point overflow"]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(FIXEDOVERFLOW_A::ERROR)
    }
}
#[doc = "Field `UNDERFLOW` reader - Underflow"]
pub type UNDERFLOW_R = crate::BitReader<UNDERFLOW_A>;
#[doc = "Underflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNDERFLOW_A {
    #[doc = "0: No error"]
    NO_ERROR = 0,
    #[doc = "1: Error on underflow"]
    ERROR = 1,
}
impl From<UNDERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: UNDERFLOW_A) -> Self {
        variant as u8 != 0
    }
}
impl UNDERFLOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNDERFLOW_A {
        match self.bits {
            false => UNDERFLOW_A::NO_ERROR,
            true => UNDERFLOW_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == UNDERFLOW_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == UNDERFLOW_A::ERROR
    }
}
#[doc = "Field `UNDERFLOW` writer - Underflow"]
pub type UNDERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRSTAT_SPEC, UNDERFLOW_A, O>;
impl<'a, const O: u8> UNDERFLOW_W<'a, O> {
    #[doc = "No error"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(UNDERFLOW_A::NO_ERROR)
    }
    #[doc = "Error on underflow"]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(UNDERFLOW_A::ERROR)
    }
}
#[doc = "Field `BUSERROR` reader - Bus error"]
pub type BUSERROR_R = crate::BitReader<BUSERROR_A>;
#[doc = "Bus error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSERROR_A {
    #[doc = "0: No error"]
    NO_ERROR = 0,
    #[doc = "1: Error on bus"]
    ERROR = 1,
}
impl From<BUSERROR_A> for bool {
    #[inline(always)]
    fn from(variant: BUSERROR_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSERROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSERROR_A {
        match self.bits {
            false => BUSERROR_A::NO_ERROR,
            true => BUSERROR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == BUSERROR_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == BUSERROR_A::ERROR
    }
}
#[doc = "Field `BUSERROR` writer - Bus error"]
pub type BUSERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRSTAT_SPEC, BUSERROR_A, O>;
impl<'a, const O: u8> BUSERROR_W<'a, O> {
    #[doc = "No error"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(BUSERROR_A::NO_ERROR)
    }
    #[doc = "Error on bus"]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(BUSERROR_A::ERROR)
    }
}
impl R {
    #[doc = "Bit 0 - Floating point overflow"]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Floating Point NaN"]
    #[inline(always)]
    pub fn nan(&self) -> NAN_R {
        NAN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fixed point overflow"]
    #[inline(always)]
    pub fn fixedoverflow(&self) -> FIXEDOVERFLOW_R {
        FIXEDOVERFLOW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Underflow"]
    #[inline(always)]
    pub fn underflow(&self) -> UNDERFLOW_R {
        UNDERFLOW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bus error"]
    #[inline(always)]
    pub fn buserror(&self) -> BUSERROR_R {
        BUSERROR_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Floating point overflow"]
    #[inline(always)]
    #[must_use]
    pub fn overflow(&mut self) -> OVERFLOW_W<0> {
        OVERFLOW_W::new(self)
    }
    #[doc = "Bit 1 - Floating Point NaN"]
    #[inline(always)]
    #[must_use]
    pub fn nan(&mut self) -> NAN_W<1> {
        NAN_W::new(self)
    }
    #[doc = "Bit 2 - Fixed point overflow"]
    #[inline(always)]
    #[must_use]
    pub fn fixedoverflow(&mut self) -> FIXEDOVERFLOW_W<2> {
        FIXEDOVERFLOW_W::new(self)
    }
    #[doc = "Bit 3 - Underflow"]
    #[inline(always)]
    #[must_use]
    pub fn underflow(&mut self) -> UNDERFLOW_W<3> {
        UNDERFLOW_W::new(self)
    }
    #[doc = "Bit 4 - Bus error"]
    #[inline(always)]
    #[must_use]
    pub fn buserror(&mut self) -> BUSERROR_W<4> {
        BUSERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errstat](index.html) module"]
pub struct ERRSTAT_SPEC;
impl crate::RegisterSpec for ERRSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [errstat::R](R) reader structure"]
impl crate::Readable for ERRSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [errstat::W](W) writer structure"]
impl crate::Writable for ERRSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERRSTAT to value 0"]
impl crate::Resettable for ERRSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
