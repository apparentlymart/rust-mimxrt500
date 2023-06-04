#[doc = "Register `SGPIO_PSYNC` reader"]
pub struct R(crate::R<SGPIO_PSYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SGPIO_PSYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SGPIO_PSYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SGPIO_PSYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SGPIO_PSYNC` writer"]
pub struct W(crate::W<SGPIO_PSYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SGPIO_PSYNC_SPEC>;
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
impl From<crate::W<SGPIO_PSYNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SGPIO_PSYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSYNC` reader - Synchronization Stage Setting:"]
pub type PSYNC_R = crate::BitReader<PSYNC_A>;
#[doc = "Synchronization Stage Setting:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSYNC_A {
    #[doc = "0: 2-Stage Sync"]
    PSYNC_0 = 0,
    #[doc = "1: 1-Stage Sync"]
    PSYNC_1 = 1,
}
impl From<PSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: PSYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl PSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSYNC_A {
        match self.bits {
            false => PSYNC_A::PSYNC_0,
            true => PSYNC_A::PSYNC_1,
        }
    }
    #[doc = "Checks if the value of the field is `PSYNC_0`"]
    #[inline(always)]
    pub fn is_psync_0(&self) -> bool {
        *self == PSYNC_A::PSYNC_0
    }
    #[doc = "Checks if the value of the field is `PSYNC_1`"]
    #[inline(always)]
    pub fn is_psync_1(&self) -> bool {
        *self == PSYNC_A::PSYNC_1
    }
}
#[doc = "Field `PSYNC` writer - Synchronization Stage Setting:"]
pub type PSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SGPIO_PSYNC_SPEC, PSYNC_A, O>;
impl<'a, const O: u8> PSYNC_W<'a, O> {
    #[doc = "2-Stage Sync"]
    #[inline(always)]
    pub fn psync_0(self) -> &'a mut W {
        self.variant(PSYNC_A::PSYNC_0)
    }
    #[doc = "1-Stage Sync"]
    #[inline(always)]
    pub fn psync_1(self) -> &'a mut W {
        self.variant(PSYNC_A::PSYNC_1)
    }
}
impl R {
    #[doc = "Bit 0 - Synchronization Stage Setting:"]
    #[inline(always)]
    pub fn psync(&self) -> PSYNC_R {
        PSYNC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronization Stage Setting:"]
    #[inline(always)]
    #[must_use]
    pub fn psync(&mut self) -> PSYNC_W<0> {
        PSYNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Secure GPIO PSYNC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sgpio_psync](index.html) module"]
pub struct SGPIO_PSYNC_SPEC;
impl crate::RegisterSpec for SGPIO_PSYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sgpio_psync::R](R) reader structure"]
impl crate::Readable for SGPIO_PSYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sgpio_psync::W](W) writer structure"]
impl crate::Writable for SGPIO_PSYNC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SGPIO_PSYNC to value 0"]
impl crate::Resettable for SGPIO_PSYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
