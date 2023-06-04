#[doc = "Register `DECRESET` reader"]
pub struct R(crate::R<DECRESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DECRESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DECRESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DECRESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DECRESET` writer"]
pub struct W(crate::W<DECRESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DECRESET_SPEC>;
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
impl From<crate::W<DECRESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DECRESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DECRESET` reader - Decimator reset"]
pub type DECRESET_R = crate::FieldReader<u8, DECRESET_A>;
#[doc = "Decimator reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DECRESET_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<DECRESET_A> for u8 {
    #[inline(always)]
    fn from(variant: DECRESET_A) -> Self {
        variant as _
    }
}
impl DECRESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DECRESET_A> {
        match self.bits {
            0 => Some(DECRESET_A::DISABLE),
            1 => Some(DECRESET_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DECRESET_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DECRESET_A::ENABLE
    }
}
#[doc = "Field `DECRESET` writer - Decimator reset"]
pub type DECRESET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DECRESET_SPEC, u8, DECRESET_A, 8, O>;
impl<'a, const O: u8> DECRESET_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DECRESET_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DECRESET_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:7 - Decimator reset"]
    #[inline(always)]
    pub fn decreset(&self) -> DECRESET_R {
        DECRESET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Decimator reset"]
    #[inline(always)]
    #[must_use]
    pub fn decreset(&mut self) -> DECRESET_W<0> {
        DECRESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMIC decimator reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [decreset](index.html) module"]
pub struct DECRESET_SPEC;
impl crate::RegisterSpec for DECRESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [decreset::R](R) reader structure"]
impl crate::Readable for DECRESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [decreset::W](W) writer structure"]
impl crate::Writable for DECRESET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DECRESET to value 0"]
impl crate::Resettable for DECRESET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
