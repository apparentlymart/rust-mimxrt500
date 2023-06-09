#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAITING` reader - Interrupt When Waiting for Data Input"]
pub type WAITING_R = crate::BitReader<WAITING_A>;
#[doc = "Interrupt When Waiting for Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAITING_A {
    #[doc = "0: Interrupt not enabled when waiting"]
    NO_INTERRUPT = 0,
    #[doc = "1: Interrupt is enabled when waiting"]
    INTERRUPT = 1,
}
impl From<WAITING_A> for bool {
    #[inline(always)]
    fn from(variant: WAITING_A) -> Self {
        variant as u8 != 0
    }
}
impl WAITING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITING_A {
        match self.bits {
            false => WAITING_A::NO_INTERRUPT,
            true => WAITING_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == WAITING_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == WAITING_A::INTERRUPT
    }
}
#[doc = "Field `WAITING` writer - Interrupt When Waiting for Data Input"]
pub type WAITING_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, WAITING_A, O>;
impl<'a, const O: u8> WAITING_W<'a, O> {
    #[doc = "Interrupt not enabled when waiting"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(WAITING_A::NO_INTERRUPT)
    }
    #[doc = "Interrupt is enabled when waiting"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(WAITING_A::INTERRUPT)
    }
}
#[doc = "Field `DIGEST` reader - Digest/Outdata"]
pub type DIGEST_R = crate::BitReader<DIGEST_A>;
#[doc = "Digest/Outdata\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIGEST_A {
    #[doc = "0: Interrupt not enabled when Digest is ready"]
    NO_INTERRUPT = 0,
    #[doc = "1: Interrupt is enabled when Digest is ready. Interrupt cleared by writing more data, starting a new Hash, or disabling (done)."]
    INTERRUPT = 1,
}
impl From<DIGEST_A> for bool {
    #[inline(always)]
    fn from(variant: DIGEST_A) -> Self {
        variant as u8 != 0
    }
}
impl DIGEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIGEST_A {
        match self.bits {
            false => DIGEST_A::NO_INTERRUPT,
            true => DIGEST_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == DIGEST_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == DIGEST_A::INTERRUPT
    }
}
#[doc = "Field `DIGEST` writer - Digest/Outdata"]
pub type DIGEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, DIGEST_A, O>;
impl<'a, const O: u8> DIGEST_W<'a, O> {
    #[doc = "Interrupt not enabled when Digest is ready"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(DIGEST_A::NO_INTERRUPT)
    }
    #[doc = "Interrupt is enabled when Digest is ready. Interrupt cleared by writing more data, starting a new Hash, or disabling (done)."]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(DIGEST_A::INTERRUPT)
    }
}
#[doc = "Field `ERROR` reader - Interrupt on Error"]
pub type ERROR_R = crate::BitReader<ERROR_A>;
#[doc = "Interrupt on Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERROR_A {
    #[doc = "0: Interrupt not enabled on Error."]
    NOT_INTERRUPT = 0,
    #[doc = "1: Interrupt is enabled on Error (until cleared)."]
    INTERRUPT = 1,
}
impl From<ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        variant as u8 != 0
    }
}
impl ERROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_A {
        match self.bits {
            false => ERROR_A::NOT_INTERRUPT,
            true => ERROR_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INTERRUPT`"]
    #[inline(always)]
    pub fn is_not_interrupt(&self) -> bool {
        *self == ERROR_A::NOT_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == ERROR_A::INTERRUPT
    }
}
#[doc = "Field `ERROR` writer - Interrupt on Error"]
pub type ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, ERROR_A, O>;
impl<'a, const O: u8> ERROR_W<'a, O> {
    #[doc = "Interrupt not enabled on Error."]
    #[inline(always)]
    pub fn not_interrupt(self) -> &'a mut W {
        self.variant(ERROR_A::NOT_INTERRUPT)
    }
    #[doc = "Interrupt is enabled on Error (until cleared)."]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(ERROR_A::INTERRUPT)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt When Waiting for Data Input"]
    #[inline(always)]
    pub fn waiting(&self) -> WAITING_R {
        WAITING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Digest/Outdata"]
    #[inline(always)]
    pub fn digest(&self) -> DIGEST_R {
        DIGEST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt on Error"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt When Waiting for Data Input"]
    #[inline(always)]
    #[must_use]
    pub fn waiting(&mut self) -> WAITING_W<0> {
        WAITING_W::new(self)
    }
    #[doc = "Bit 1 - Digest/Outdata"]
    #[inline(always)]
    #[must_use]
    pub fn digest(&mut self) -> DIGEST_W<1> {
        DIGEST_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt on Error"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<2> {
        ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
