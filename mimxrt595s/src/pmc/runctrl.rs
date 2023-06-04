#[doc = "Register `RUNCTRL` reader"]
pub struct R(crate::R<RUNCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RUNCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RUNCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RUNCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RUNCTRL` writer"]
pub struct W(crate::W<RUNCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RUNCTRL_SPEC>;
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
impl From<crate::W<RUNCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RUNCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORELVL` reader - Vddcore voltage value when using on-chip regulator and SYSCTL is in run mode."]
pub type CORELVL_R = crate::FieldReader<u8, CORELVL_A>;
#[doc = "Vddcore voltage value when using on-chip regulator and SYSCTL is in run mode.\n\nValue on reset: 38"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CORELVL_A {
    #[doc = "1: 0.6V"]
    VALUE_01 = 1,
    #[doc = "16: 0.7V"]
    VALUE_10 = 16,
    #[doc = "19: 0.8V"]
    VALUE_13 = 19,
    #[doc = "38: 1.0V"]
    VALUE_26 = 38,
    #[doc = "50: 1.138V"]
    VALUE_32 = 50,
}
impl From<CORELVL_A> for u8 {
    #[inline(always)]
    fn from(variant: CORELVL_A) -> Self {
        variant as _
    }
}
impl CORELVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CORELVL_A> {
        match self.bits {
            1 => Some(CORELVL_A::VALUE_01),
            16 => Some(CORELVL_A::VALUE_10),
            19 => Some(CORELVL_A::VALUE_13),
            38 => Some(CORELVL_A::VALUE_26),
            50 => Some(CORELVL_A::VALUE_32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_01`"]
    #[inline(always)]
    pub fn is_value_01(&self) -> bool {
        *self == CORELVL_A::VALUE_01
    }
    #[doc = "Checks if the value of the field is `VALUE_10`"]
    #[inline(always)]
    pub fn is_value_10(&self) -> bool {
        *self == CORELVL_A::VALUE_10
    }
    #[doc = "Checks if the value of the field is `VALUE_13`"]
    #[inline(always)]
    pub fn is_value_13(&self) -> bool {
        *self == CORELVL_A::VALUE_13
    }
    #[doc = "Checks if the value of the field is `VALUE_26`"]
    #[inline(always)]
    pub fn is_value_26(&self) -> bool {
        *self == CORELVL_A::VALUE_26
    }
    #[doc = "Checks if the value of the field is `VALUE_32`"]
    #[inline(always)]
    pub fn is_value_32(&self) -> bool {
        *self == CORELVL_A::VALUE_32
    }
}
#[doc = "Field `CORELVL` writer - Vddcore voltage value when using on-chip regulator and SYSCTL is in run mode."]
pub type CORELVL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RUNCTRL_SPEC, u8, CORELVL_A, 6, O>;
impl<'a, const O: u8> CORELVL_W<'a, O> {
    #[doc = "0.6V"]
    #[inline(always)]
    pub fn value_01(self) -> &'a mut W {
        self.variant(CORELVL_A::VALUE_01)
    }
    #[doc = "0.7V"]
    #[inline(always)]
    pub fn value_10(self) -> &'a mut W {
        self.variant(CORELVL_A::VALUE_10)
    }
    #[doc = "0.8V"]
    #[inline(always)]
    pub fn value_13(self) -> &'a mut W {
        self.variant(CORELVL_A::VALUE_13)
    }
    #[doc = "1.0V"]
    #[inline(always)]
    pub fn value_26(self) -> &'a mut W {
        self.variant(CORELVL_A::VALUE_26)
    }
    #[doc = "1.138V"]
    #[inline(always)]
    pub fn value_32(self) -> &'a mut W {
        self.variant(CORELVL_A::VALUE_32)
    }
}
impl R {
    #[doc = "Bits 0:5 - Vddcore voltage value when using on-chip regulator and SYSCTL is in run mode."]
    #[inline(always)]
    pub fn corelvl(&self) -> CORELVL_R {
        CORELVL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Vddcore voltage value when using on-chip regulator and SYSCTL is in run mode."]
    #[inline(always)]
    #[must_use]
    pub fn corelvl(&mut self) -> CORELVL_W<0> {
        CORELVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMC controls used during run mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [runctrl](index.html) module"]
pub struct RUNCTRL_SPEC;
impl crate::RegisterSpec for RUNCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [runctrl::R](R) reader structure"]
impl crate::Readable for RUNCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [runctrl::W](W) writer structure"]
impl crate::Writable for RUNCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RUNCTRL to value 0x26"]
impl crate::Resettable for RUNCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x26;
}
