#[doc = "Register `IENR` reader"]
pub struct R(crate::R<IENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IENR` writer"]
pub struct W(crate::W<IENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IENR_SPEC>;
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
impl From<crate::W<IENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENRL` reader - Enable Interrupt"]
pub type ENRL_R = crate::FieldReader<u8, ENRL_A>;
#[doc = "Enable Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENRL_A {
    #[doc = "0: Disable rising edge or level interrupt"]
    ENRL_0 = 0,
    #[doc = "1: Enable rising edge or level interrupt"]
    ENRL_1 = 1,
}
impl From<ENRL_A> for u8 {
    #[inline(always)]
    fn from(variant: ENRL_A) -> Self {
        variant as _
    }
}
impl ENRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ENRL_A> {
        match self.bits {
            0 => Some(ENRL_A::ENRL_0),
            1 => Some(ENRL_A::ENRL_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENRL_0`"]
    #[inline(always)]
    pub fn is_enrl_0(&self) -> bool {
        *self == ENRL_A::ENRL_0
    }
    #[doc = "Checks if the value of the field is `ENRL_1`"]
    #[inline(always)]
    pub fn is_enrl_1(&self) -> bool {
        *self == ENRL_A::ENRL_1
    }
}
#[doc = "Field `ENRL` writer - Enable Interrupt"]
pub type ENRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IENR_SPEC, u8, ENRL_A, 8, O>;
impl<'a, const O: u8> ENRL_W<'a, O> {
    #[doc = "Disable rising edge or level interrupt"]
    #[inline(always)]
    pub fn enrl_0(self) -> &'a mut W {
        self.variant(ENRL_A::ENRL_0)
    }
    #[doc = "Enable rising edge or level interrupt"]
    #[inline(always)]
    pub fn enrl_1(self) -> &'a mut W {
        self.variant(ENRL_A::ENRL_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Enable Interrupt"]
    #[inline(always)]
    pub fn enrl(&self) -> ENRL_R {
        ENRL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Enable Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enrl(&mut self) -> ENRL_W<0> {
        ENRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Interrupt Level or Rising Edge Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ienr](index.html) module"]
pub struct IENR_SPEC;
impl crate::RegisterSpec for IENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ienr::R](R) reader structure"]
impl crate::Readable for IENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ienr::W](W) writer structure"]
impl crate::Writable for IENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IENR to value 0"]
impl crate::Resettable for IENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
