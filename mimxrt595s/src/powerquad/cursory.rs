#[doc = "Register `CURSORY` reader"]
pub struct R(crate::R<CURSORY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CURSORY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CURSORY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CURSORY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CURSORY` writer"]
pub struct W(crate::W<CURSORY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CURSORY_SPEC>;
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
impl From<crate::W<CURSORY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CURSORY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURSORY` reader - Cursory Mode"]
pub type CURSORY_R = crate::BitReader<CURSORY_A>;
#[doc = "Cursory Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CURSORY_A {
    #[doc = "0: Disable cursory mode"]
    DISABLE = 0,
    #[doc = "1: Enable cursory Mode"]
    ENABLE = 1,
}
impl From<CURSORY_A> for bool {
    #[inline(always)]
    fn from(variant: CURSORY_A) -> Self {
        variant as u8 != 0
    }
}
impl CURSORY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CURSORY_A {
        match self.bits {
            false => CURSORY_A::DISABLE,
            true => CURSORY_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CURSORY_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CURSORY_A::ENABLE
    }
}
#[doc = "Field `CURSORY` writer - Cursory Mode"]
pub type CURSORY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CURSORY_SPEC, CURSORY_A, O>;
impl<'a, const O: u8> CURSORY_W<'a, O> {
    #[doc = "Disable cursory mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CURSORY_A::DISABLE)
    }
    #[doc = "Enable cursory Mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CURSORY_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Cursory Mode"]
    #[inline(always)]
    pub fn cursory(&self) -> CURSORY_R {
        CURSORY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cursory Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cursory(&mut self) -> CURSORY_W<0> {
        CURSORY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cursory\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cursory](index.html) module"]
pub struct CURSORY_SPEC;
impl crate::RegisterSpec for CURSORY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cursory::R](R) reader structure"]
impl crate::Readable for CURSORY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cursory::W](W) writer structure"]
impl crate::Writable for CURSORY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CURSORY to value 0"]
impl crate::Resettable for CURSORY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
