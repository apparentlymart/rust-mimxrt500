#[doc = "Register `I3C01FCLKSTSTCLKSEL` reader"]
pub struct R(crate::R<I3C01FCLKSTSTCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I3C01FCLKSTSTCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I3C01FCLKSTSTCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I3C01FCLKSTSTCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I3C01FCLKSTSTCLKSEL` writer"]
pub struct W(crate::W<I3C01FCLKSTSTCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I3C01FCLKSTSTCLKSEL_SPEC>;
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
impl From<crate::W<I3C01FCLKSTSTCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I3C01FCLKSTSTCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - I3C0, I3C1 FCLK Test Clock Source"]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "I3C0, I3C1 FCLK Test Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: Low Power Oscillator Clock (LPOSC)"]
    LPOSC = 0,
    #[doc = "7: None, output gated to reduce power"]
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
            0 => Some(SEL_A::LPOSC),
            7 => Some(SEL_A::NONE),
            _ => None,
        }
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
#[doc = "Field `SEL` writer - I3C0, I3C1 FCLK Test Clock Source"]
pub type SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I3C01FCLKSTSTCLKSEL_SPEC, u8, SEL_A, 3, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "Low Power Oscillator Clock (LPOSC)"]
    #[inline(always)]
    pub fn lposc(self) -> &'a mut W {
        self.variant(SEL_A::LPOSC)
    }
    #[doc = "None, output gated to reduce power"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SEL_A::NONE)
    }
}
impl R {
    #[doc = "Bits 0:2 - I3C0, I3C1 FCLK Test Clock Source"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - I3C0, I3C1 FCLK Test Clock Source"]
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
#[doc = "I3C01 Functional Clock Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i3c01fclkststclksel](index.html) module"]
pub struct I3C01FCLKSTSTCLKSEL_SPEC;
impl crate::RegisterSpec for I3C01FCLKSTSTCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i3c01fclkststclksel::R](R) reader structure"]
impl crate::Readable for I3C01FCLKSTSTCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i3c01fclkststclksel::W](W) writer structure"]
impl crate::Writable for I3C01FCLKSTSTCLKSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I3C01FCLKSTSTCLKSEL to value 0"]
impl crate::Resettable for I3C01FCLKSTSTCLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
