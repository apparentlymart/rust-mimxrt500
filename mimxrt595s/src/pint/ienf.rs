#[doc = "Register `IENF` reader"]
pub struct R(crate::R<IENF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IENF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IENF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IENF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IENF` writer"]
pub struct W(crate::W<IENF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IENF_SPEC>;
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
impl From<crate::W<IENF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IENF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENAF` reader - Enable Interrupt"]
pub type ENAF_R = crate::FieldReader<u8, ENAF_A>;
#[doc = "Enable Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENAF_A {
    #[doc = "0: Disable falling edge interrupt or set active interrupt level LOW"]
    ENAF_0 = 0,
    #[doc = "1: Enable falling edge interrupt enabled or set active interrupt level HIGH"]
    ENAF_1 = 1,
}
impl From<ENAF_A> for u8 {
    #[inline(always)]
    fn from(variant: ENAF_A) -> Self {
        variant as _
    }
}
impl ENAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ENAF_A> {
        match self.bits {
            0 => Some(ENAF_A::ENAF_0),
            1 => Some(ENAF_A::ENAF_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENAF_0`"]
    #[inline(always)]
    pub fn is_enaf_0(&self) -> bool {
        *self == ENAF_A::ENAF_0
    }
    #[doc = "Checks if the value of the field is `ENAF_1`"]
    #[inline(always)]
    pub fn is_enaf_1(&self) -> bool {
        *self == ENAF_A::ENAF_1
    }
}
#[doc = "Field `ENAF` writer - Enable Interrupt"]
pub type ENAF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IENF_SPEC, u8, ENAF_A, 8, O>;
impl<'a, const O: u8> ENAF_W<'a, O> {
    #[doc = "Disable falling edge interrupt or set active interrupt level LOW"]
    #[inline(always)]
    pub fn enaf_0(self) -> &'a mut W {
        self.variant(ENAF_A::ENAF_0)
    }
    #[doc = "Enable falling edge interrupt enabled or set active interrupt level HIGH"]
    #[inline(always)]
    pub fn enaf_1(self) -> &'a mut W {
        self.variant(ENAF_A::ENAF_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Enable Interrupt"]
    #[inline(always)]
    pub fn enaf(&self) -> ENAF_R {
        ENAF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Enable Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enaf(&mut self) -> ENAF_W<0> {
        ENAF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Interrupt Active Level or Falling Edge Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ienf](index.html) module"]
pub struct IENF_SPEC;
impl crate::RegisterSpec for IENF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ienf::R](R) reader structure"]
impl crate::Readable for IENF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ienf::W](W) writer structure"]
impl crate::Writable for IENF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IENF to value 0"]
impl crate::Resettable for IENF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
