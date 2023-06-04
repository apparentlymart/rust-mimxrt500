#[doc = "Register `INTVAL` reader"]
pub struct R(crate::R<INTVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTVAL` writer"]
pub struct W(crate::W<INTVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTVAL_SPEC>;
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
impl From<crate::W<INTVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IVALUE` reader - Time Interval Load Value."]
pub type IVALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IVALUE` writer - Time Interval Load Value."]
pub type IVALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTVAL_SPEC, u32, u32, 24, O>;
#[doc = "Field `LOAD` reader - Force Load Enable"]
pub type LOAD_R = crate::BitReader<LOAD_A>;
#[doc = "Force Load Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOAD_A {
    #[doc = "0: No force load"]
    NO_FORCE_LOAD = 0,
    #[doc = "1: Force load"]
    FORCE_LOAD = 1,
}
impl From<LOAD_A> for bool {
    #[inline(always)]
    fn from(variant: LOAD_A) -> Self {
        variant as u8 != 0
    }
}
impl LOAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOAD_A {
        match self.bits {
            false => LOAD_A::NO_FORCE_LOAD,
            true => LOAD_A::FORCE_LOAD,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FORCE_LOAD`"]
    #[inline(always)]
    pub fn is_no_force_load(&self) -> bool {
        *self == LOAD_A::NO_FORCE_LOAD
    }
    #[doc = "Checks if the value of the field is `FORCE_LOAD`"]
    #[inline(always)]
    pub fn is_force_load(&self) -> bool {
        *self == LOAD_A::FORCE_LOAD
    }
}
#[doc = "Field `LOAD` writer - Force Load Enable"]
pub type LOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTVAL_SPEC, LOAD_A, O>;
impl<'a, const O: u8> LOAD_W<'a, O> {
    #[doc = "No force load"]
    #[inline(always)]
    pub fn no_force_load(self) -> &'a mut W {
        self.variant(LOAD_A::NO_FORCE_LOAD)
    }
    #[doc = "Force load"]
    #[inline(always)]
    pub fn force_load(self) -> &'a mut W {
        self.variant(LOAD_A::FORCE_LOAD)
    }
}
impl R {
    #[doc = "Bits 0:23 - Time Interval Load Value."]
    #[inline(always)]
    pub fn ivalue(&self) -> IVALUE_R {
        IVALUE_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 31 - Force Load Enable"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Time Interval Load Value."]
    #[inline(always)]
    #[must_use]
    pub fn ivalue(&mut self) -> IVALUE_W<0> {
        IVALUE_W::new(self)
    }
    #[doc = "Bit 31 - Force Load Enable"]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LOAD_W<31> {
        LOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time Interval Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intval](index.html) module"]
pub struct INTVAL_SPEC;
impl crate::RegisterSpec for INTVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intval::R](R) reader structure"]
impl crate::Readable for INTVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intval::W](W) writer structure"]
impl crate::Writable for INTVAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTVAL to value 0"]
impl crate::Resettable for INTVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
