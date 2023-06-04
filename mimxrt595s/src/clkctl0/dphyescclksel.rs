#[doc = "Register `DPHYESCCLKSEL` reader"]
pub struct R(crate::R<DPHYESCCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPHYESCCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPHYESCCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPHYESCCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPHYESCCLKSEL` writer"]
pub struct W(crate::W<DPHYESCCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPHYESCCLKSEL_SPEC>;
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
impl From<crate::W<DPHYESCCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPHYESCCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Select Clock Source"]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "Select Clock Source\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: FRO_DIV1 clock"]
    FRO_DIV1 = 0,
    #[doc = "1: FRO_DIV16 Clock"]
    FRO_DIV16 = 1,
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
            0 => Some(SEL_A::FRO_DIV1),
            1 => Some(SEL_A::FRO_DIV16),
            7 => Some(SEL_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FRO_DIV1`"]
    #[inline(always)]
    pub fn is_fro_div1(&self) -> bool {
        *self == SEL_A::FRO_DIV1
    }
    #[doc = "Checks if the value of the field is `FRO_DIV16`"]
    #[inline(always)]
    pub fn is_fro_div16(&self) -> bool {
        *self == SEL_A::FRO_DIV16
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SEL_A::NONE
    }
}
#[doc = "Field `SEL` writer - Select Clock Source"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DPHYESCCLKSEL_SPEC, u8, SEL_A, 3, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "FRO_DIV1 clock"]
    #[inline(always)]
    pub fn fro_div1(self) -> &'a mut W {
        self.variant(SEL_A::FRO_DIV1)
    }
    #[doc = "FRO_DIV16 Clock"]
    #[inline(always)]
    pub fn fro_div16(self) -> &'a mut W {
        self.variant(SEL_A::FRO_DIV16)
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
#[doc = "MIPI-DSI DPHY Escape Mode Clock Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dphyescclksel](index.html) module"]
pub struct DPHYESCCLKSEL_SPEC;
impl crate::RegisterSpec for DPHYESCCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dphyescclksel::R](R) reader structure"]
impl crate::Readable for DPHYESCCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dphyescclksel::W](W) writer structure"]
impl crate::Writable for DPHYESCCLKSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPHYESCCLKSEL to value 0x07"]
impl crate::Resettable for DPHYESCCLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
