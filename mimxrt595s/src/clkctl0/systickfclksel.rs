#[doc = "Register `SYSTICKFCLKSEL` reader"]
pub struct R(crate::R<SYSTICKFCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSTICKFCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSTICKFCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSTICKFCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSTICKFCLKSEL` writer"]
pub struct W(crate::W<SYSTICKFCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSTICKFCLKSEL_SPEC>;
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
impl From<crate::W<SYSTICKFCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSTICKFCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Select Clock Source"]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "Select Clock Source\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: Systick Divider Output Clock"]
    SYSTICK_DIV_OUTPUT = 0,
    #[doc = "1: Low Power Oscillator Clock (LPOSC)"]
    LPOSC = 1,
    #[doc = "2: 32 KHz RTC Clock"]
    A32KHZ_RTC = 2,
    #[doc = "7: None; this may be selected to reduce power when no output is needed."]
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
            0 => Some(SEL_A::SYSTICK_DIV_OUTPUT),
            1 => Some(SEL_A::LPOSC),
            2 => Some(SEL_A::A32KHZ_RTC),
            7 => Some(SEL_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSTICK_DIV_OUTPUT`"]
    #[inline(always)]
    pub fn is_systick_div_output(&self) -> bool {
        *self == SEL_A::SYSTICK_DIV_OUTPUT
    }
    #[doc = "Checks if the value of the field is `LPOSC`"]
    #[inline(always)]
    pub fn is_lposc(&self) -> bool {
        *self == SEL_A::LPOSC
    }
    #[doc = "Checks if the value of the field is `A32KHZ_RTC`"]
    #[inline(always)]
    pub fn is_a32khz_rtc(&self) -> bool {
        *self == SEL_A::A32KHZ_RTC
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SEL_A::NONE
    }
}
#[doc = "Field `SEL` writer - Select Clock Source"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSTICKFCLKSEL_SPEC, u8, SEL_A, 3, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "Systick Divider Output Clock"]
    #[inline(always)]
    pub fn systick_div_output(self) -> &'a mut W {
        self.variant(SEL_A::SYSTICK_DIV_OUTPUT)
    }
    #[doc = "Low Power Oscillator Clock (LPOSC)"]
    #[inline(always)]
    pub fn lposc(self) -> &'a mut W {
        self.variant(SEL_A::LPOSC)
    }
    #[doc = "32 KHz RTC Clock"]
    #[inline(always)]
    pub fn a32khz_rtc(self) -> &'a mut W {
        self.variant(SEL_A::A32KHZ_RTC)
    }
    #[doc = "None; this may be selected to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SEL_A::NONE)
    }
}
impl R {
    #[doc = "Bits 0:2 - Select Clock Source"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select Clock Source"]
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
#[doc = "SYSTICK Functional Clock Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systickfclksel](index.html) module"]
pub struct SYSTICKFCLKSEL_SPEC;
impl crate::RegisterSpec for SYSTICKFCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [systickfclksel::R](R) reader structure"]
impl crate::Readable for SYSTICKFCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [systickfclksel::W](W) writer structure"]
impl crate::Writable for SYSTICKFCLKSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSTICKFCLKSEL to value 0x07"]
impl crate::Resettable for SYSTICKFCLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
