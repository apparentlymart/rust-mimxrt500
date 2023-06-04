#[doc = "Register `CIENR` reader"]
pub struct R(crate::R<CIENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIENR` writer"]
pub struct W(crate::W<CIENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIENR_SPEC>;
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
impl From<crate::W<CIENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CENRL` reader - Clear bits in the IENR"]
pub type CENRL_R = crate::FieldReader<u8, CENRL_A>;
#[doc = "Clear bits in the IENR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CENRL_A {
    #[doc = "0: No operation"]
    CENRL_0 = 0,
    #[doc = "1: Disable rising edge or level interrupt"]
    CENRL_1 = 1,
}
impl From<CENRL_A> for u8 {
    #[inline(always)]
    fn from(variant: CENRL_A) -> Self {
        variant as _
    }
}
impl CENRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CENRL_A> {
        match self.bits {
            0 => Some(CENRL_A::CENRL_0),
            1 => Some(CENRL_A::CENRL_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CENRL_0`"]
    #[inline(always)]
    pub fn is_cenrl_0(&self) -> bool {
        *self == CENRL_A::CENRL_0
    }
    #[doc = "Checks if the value of the field is `CENRL_1`"]
    #[inline(always)]
    pub fn is_cenrl_1(&self) -> bool {
        *self == CENRL_A::CENRL_1
    }
}
#[doc = "Field `CENRL` writer - Clear bits in the IENR"]
pub type CENRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CIENR_SPEC, u8, CENRL_A, 8, O>;
impl<'a, const O: u8> CENRL_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn cenrl_0(self) -> &'a mut W {
        self.variant(CENRL_A::CENRL_0)
    }
    #[doc = "Disable rising edge or level interrupt"]
    #[inline(always)]
    pub fn cenrl_1(self) -> &'a mut W {
        self.variant(CENRL_A::CENRL_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Clear bits in the IENR"]
    #[inline(always)]
    pub fn cenrl(&self) -> CENRL_R {
        CENRL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clear bits in the IENR"]
    #[inline(always)]
    #[must_use]
    pub fn cenrl(&mut self) -> CENRL_W<0> {
        CENRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Interrupt Level (Rising Edge Interrupt) Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cienr](index.html) module"]
pub struct CIENR_SPEC;
impl crate::RegisterSpec for CIENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cienr::R](R) reader structure"]
impl crate::Readable for CIENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cienr::W](W) writer structure"]
impl crate::Writable for CIENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xff;
}
#[doc = "`reset()` method sets CIENR to value 0"]
impl crate::Resettable for CIENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
