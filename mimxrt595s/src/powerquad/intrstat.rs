#[doc = "Register `INTRSTAT` reader"]
pub struct R(crate::R<INTRSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTRSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTRSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTRSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTRSTAT` writer"]
pub struct W(crate::W<INTRSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTRSTAT_SPEC>;
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
impl From<crate::W<INTRSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTRSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTR_STAT` reader - Interrupt Status"]
pub type INTR_STAT_R = crate::BitReader<INTR_STAT_A>;
#[doc = "Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTR_STAT_A {
    #[doc = "0: No new interrupt"]
    DISABLE = 0,
    #[doc = "1: Interrupt captured"]
    ENABLE = 1,
}
impl From<INTR_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: INTR_STAT_A) -> Self {
        variant as u8 != 0
    }
}
impl INTR_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTR_STAT_A {
        match self.bits {
            false => INTR_STAT_A::DISABLE,
            true => INTR_STAT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INTR_STAT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == INTR_STAT_A::ENABLE
    }
}
#[doc = "Field `INTR_STAT` writer - Interrupt Status"]
pub type INTR_STAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRSTAT_SPEC, INTR_STAT_A, O>;
impl<'a, const O: u8> INTR_STAT_W<'a, O> {
    #[doc = "No new interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INTR_STAT_A::DISABLE)
    }
    #[doc = "Interrupt captured"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(INTR_STAT_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Status"]
    #[inline(always)]
    pub fn intr_stat(&self) -> INTR_STAT_R {
        INTR_STAT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn intr_stat(&mut self) -> INTR_STAT_W<0> {
        INTR_STAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intrstat](index.html) module"]
pub struct INTRSTAT_SPEC;
impl crate::RegisterSpec for INTRSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intrstat::R](R) reader structure"]
impl crate::Readable for INTRSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intrstat::W](W) writer structure"]
impl crate::Writable for INTRSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTRSTAT to value 0"]
impl crate::Resettable for INTRSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
