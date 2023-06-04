#[doc = "Register `SLEEPCTRL` reader"]
pub struct R(crate::R<SLEEPCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLEEPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLEEPCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLEEPCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLEEPCTRL` writer"]
pub struct W(crate::W<SLEEPCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLEEPCTRL_SPEC>;
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
impl From<crate::W<SLEEPCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLEEPCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORELVL` reader - Vddcore voltage value when using on-chip regulator and SYSCTL is in sleep mode."]
pub type CORELVL_R = crate::FieldReader<u8, CORELVL_A>;
#[doc = "Vddcore voltage value when using on-chip regulator and SYSCTL is in sleep mode.\n\nValue on reset: 38"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CORELVL_A {
    #[doc = "0: 0.595833V"]
    VALUE_0X00 = 0,
    #[doc = "38: 1.007498V = 0.595833 + 0x26 10.8333mV"]
    VALUE_0X26 = 38,
    #[doc = "50: 1.138V"]
    VALUE_0X32 = 50,
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
            0 => Some(CORELVL_A::VALUE_0X00),
            38 => Some(CORELVL_A::VALUE_0X26),
            50 => Some(CORELVL_A::VALUE_0X32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0X00`"]
    #[inline(always)]
    pub fn is_value_0x00(&self) -> bool {
        *self == CORELVL_A::VALUE_0X00
    }
    #[doc = "Checks if the value of the field is `VALUE_0X26`"]
    #[inline(always)]
    pub fn is_value_0x26(&self) -> bool {
        *self == CORELVL_A::VALUE_0X26
    }
    #[doc = "Checks if the value of the field is `VALUE_0X32`"]
    #[inline(always)]
    pub fn is_value_0x32(&self) -> bool {
        *self == CORELVL_A::VALUE_0X32
    }
}
#[doc = "Field `CORELVL` writer - Vddcore voltage value when using on-chip regulator and SYSCTL is in sleep mode."]
pub type CORELVL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SLEEPCTRL_SPEC, u8, CORELVL_A, 6, O>;
impl<'a, const O: u8> CORELVL_W<'a, O> {
    #[doc = "0.595833V"]
    #[inline(always)]
    pub fn value_0x00(self) -> &'a mut W {
        self.variant(CORELVL_A::VALUE_0X00)
    }
    #[doc = "1.007498V = 0.595833 + 0x26 10.8333mV"]
    #[inline(always)]
    pub fn value_0x26(self) -> &'a mut W {
        self.variant(CORELVL_A::VALUE_0X26)
    }
    #[doc = "1.138V"]
    #[inline(always)]
    pub fn value_0x32(self) -> &'a mut W {
        self.variant(CORELVL_A::VALUE_0X32)
    }
}
impl R {
    #[doc = "Bits 0:5 - Vddcore voltage value when using on-chip regulator and SYSCTL is in sleep mode."]
    #[inline(always)]
    pub fn corelvl(&self) -> CORELVL_R {
        CORELVL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Vddcore voltage value when using on-chip regulator and SYSCTL is in sleep mode."]
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
#[doc = "PMC controls used during deep sleep mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sleepctrl](index.html) module"]
pub struct SLEEPCTRL_SPEC;
impl crate::RegisterSpec for SLEEPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sleepctrl::R](R) reader structure"]
impl crate::Readable for SLEEPCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sleepctrl::W](W) writer structure"]
impl crate::Writable for SLEEPCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLEEPCTRL to value 0x26"]
impl crate::Resettable for SLEEPCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x26;
}
