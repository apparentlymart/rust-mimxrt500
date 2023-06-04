#[doc = "Register `A32KHZWAKECLKSEL` reader"]
pub struct R(crate::R<A32KHZWAKECLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<A32KHZWAKECLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<A32KHZWAKECLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<A32KHZWAKECLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `A32KHZWAKECLKSEL` writer"]
pub struct W(crate::W<A32KHZWAKECLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<A32KHZWAKECLKSEL_SPEC>;
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
impl From<crate::W<A32KHZWAKECLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<A32KHZWAKECLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Select Clock Source"]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "Select Clock Source\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: 32 KHz"]
    A32KHZ = 0,
    #[doc = "1: Low Power Oscillator Clock (LPOSC); divided by 32 by default"]
    LPOSC = 1,
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
            0 => Some(SEL_A::A32KHZ),
            1 => Some(SEL_A::LPOSC),
            7 => Some(SEL_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `A32KHZ`"]
    #[inline(always)]
    pub fn is_a32khz(&self) -> bool {
        *self == SEL_A::A32KHZ
    }
    #[doc = "Checks if the value of the field is `LPOSC`"]
    #[inline(always)]
    pub fn is_lposc(&self) -> bool {
        *self == SEL_A::LPOSC
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SEL_A::NONE
    }
}
#[doc = "Field `SEL` writer - Select Clock Source"]
pub type SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, A32KHZWAKECLKSEL_SPEC, u8, SEL_A, 3, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "32 KHz"]
    #[inline(always)]
    pub fn a32khz(self) -> &'a mut W {
        self.variant(SEL_A::A32KHZ)
    }
    #[doc = "Low Power Oscillator Clock (LPOSC); divided by 32 by default"]
    #[inline(always)]
    pub fn lposc(self) -> &'a mut W {
        self.variant(SEL_A::LPOSC)
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
#[doc = "32 KHz Wake Clock Source Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a32khzwakeclksel](index.html) module"]
pub struct A32KHZWAKECLKSEL_SPEC;
impl crate::RegisterSpec for A32KHZWAKECLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [a32khzwakeclksel::R](R) reader structure"]
impl crate::Readable for A32KHZWAKECLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [a32khzwakeclksel::W](W) writer structure"]
impl crate::Writable for A32KHZWAKECLKSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets A32KHZWAKECLKSEL to value 0x01"]
impl crate::Resettable for A32KHZWAKECLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
