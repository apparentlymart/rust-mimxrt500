#[doc = "Register `SYSOSCBYPASS` reader"]
pub struct R(crate::R<SYSOSCBYPASS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSOSCBYPASS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSOSCBYPASS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSOSCBYPASS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSOSCBYPASS` writer"]
pub struct W(crate::W<SYSOSCBYPASS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSOSCBYPASS_SPEC>;
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
impl From<crate::W<SYSOSCBYPASS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSOSCBYPASS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Select SYSOSC Bypass"]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "Select SYSOSC Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: Select OSC Clock"]
    SYOSC_CLOCK = 0,
    #[doc = "1: Select Clock IN clock"]
    CLOCK_IN = 1,
    #[doc = "7: None; this may be selected to reduce power when no output is needed"]
    NONE = 7,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEL_A> {
        match self.bits {
            0 => Some(SEL_A::SYOSC_CLOCK),
            1 => Some(SEL_A::CLOCK_IN),
            7 => Some(SEL_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYOSC_CLOCK`"]
    #[inline(always)]
    pub fn is_syosc_clock(&self) -> bool {
        *self == SEL_A::SYOSC_CLOCK
    }
    #[doc = "Checks if the value of the field is `CLOCK_IN`"]
    #[inline(always)]
    pub fn is_clock_in(&self) -> bool {
        *self == SEL_A::CLOCK_IN
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SEL_A::NONE
    }
}
#[doc = "Field `SEL` writer - Select SYSOSC Bypass"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSOSCBYPASS_SPEC, u8, SEL_A, 3, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "Select OSC Clock"]
    #[inline(always)]
    pub fn syosc_clock(self) -> &'a mut W {
        self.variant(SEL_A::SYOSC_CLOCK)
    }
    #[doc = "Select Clock IN clock"]
    #[inline(always)]
    pub fn clock_in(self) -> &'a mut W {
        self.variant(SEL_A::CLOCK_IN)
    }
    #[doc = "None; this may be selected to reduce power when no output is needed"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SEL_A::NONE)
    }
}
impl R {
    #[doc = "Bits 0:2 - Select SYSOSC Bypass"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select SYSOSC Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<0> {
        SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OSC Clock Source Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysoscbypass](index.html) module"]
pub struct SYSOSCBYPASS_SPEC;
impl crate::RegisterSpec for SYSOSCBYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysoscbypass::R](R) reader structure"]
impl crate::Readable for SYSOSCBYPASS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysoscbypass::W](W) writer structure"]
impl crate::Writable for SYSOSCBYPASS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSOSCBYPASS to value 0"]
impl crate::Resettable for SYSOSCBYPASS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
