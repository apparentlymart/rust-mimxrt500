#[doc = "Register `BOOTSTATELOCK` reader"]
pub struct R(crate::R<BOOTSTATELOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOTSTATELOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOTSTATELOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOTSTATELOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOOTSTATELOCK` writer"]
pub struct W(crate::W<BOOTSTATELOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOOTSTATELOCK_SPEC>;
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
impl From<crate::W<BOOTSTATELOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOOTSTATELOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOOTSTATESEEDLOCK` reader - Boot State Seed Lockout"]
pub type BOOTSTATESEEDLOCK_R = crate::BitReader<BOOTSTATESEEDLOCK_A>;
#[doc = "Boot State Seed Lockout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOTSTATESEEDLOCK_A {
    #[doc = "0: BOOTSTATESEED\\[0:7\\]
can be changed"]
    BOOTSTATESEEDLOCK_0 = 0,
    #[doc = "1: BOOTSTATESEED\\[0:7\\]
cannot be changed"]
    BOOTSTATESEEDLOCK_1 = 1,
}
impl From<BOOTSTATESEEDLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: BOOTSTATESEEDLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl BOOTSTATESEEDLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOTSTATESEEDLOCK_A {
        match self.bits {
            false => BOOTSTATESEEDLOCK_A::BOOTSTATESEEDLOCK_0,
            true => BOOTSTATESEEDLOCK_A::BOOTSTATESEEDLOCK_1,
        }
    }
    #[doc = "Checks if the value of the field is `BOOTSTATESEEDLOCK_0`"]
    #[inline(always)]
    pub fn is_bootstateseedlock_0(&self) -> bool {
        *self == BOOTSTATESEEDLOCK_A::BOOTSTATESEEDLOCK_0
    }
    #[doc = "Checks if the value of the field is `BOOTSTATESEEDLOCK_1`"]
    #[inline(always)]
    pub fn is_bootstateseedlock_1(&self) -> bool {
        *self == BOOTSTATESEEDLOCK_A::BOOTSTATESEEDLOCK_1
    }
}
#[doc = "Field `BOOTSTATESEEDLOCK` writer - Boot State Seed Lockout"]
pub type BOOTSTATESEEDLOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BOOTSTATELOCK_SPEC, BOOTSTATESEEDLOCK_A, O>;
impl<'a, const O: u8> BOOTSTATESEEDLOCK_W<'a, O> {
    #[doc = "BOOTSTATESEED\\[0:7\\]
can be changed"]
    #[inline(always)]
    pub fn bootstateseedlock_0(self) -> &'a mut W {
        self.variant(BOOTSTATESEEDLOCK_A::BOOTSTATESEEDLOCK_0)
    }
    #[doc = "BOOTSTATESEED\\[0:7\\]
cannot be changed"]
    #[inline(always)]
    pub fn bootstateseedlock_1(self) -> &'a mut W {
        self.variant(BOOTSTATESEEDLOCK_A::BOOTSTATESEEDLOCK_1)
    }
}
#[doc = "Field `BOOTSTATEHMACLOCK` reader - Boot State HMA Lockout"]
pub type BOOTSTATEHMACLOCK_R = crate::BitReader<BOOTSTATEHMACLOCK_A>;
#[doc = "Boot State HMA Lockout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOTSTATEHMACLOCK_A {
    #[doc = "0: BOOTSTATEHMAC\\[0:7\\]
can be changed"]
    BOOTSTATEHMACLOCK_0 = 0,
    #[doc = "1: BOOTSTATEHMAC\\[0:7\\]
cannot be changed"]
    BOOTSTATEHMACLOCK_1 = 1,
}
impl From<BOOTSTATEHMACLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: BOOTSTATEHMACLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl BOOTSTATEHMACLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOTSTATEHMACLOCK_A {
        match self.bits {
            false => BOOTSTATEHMACLOCK_A::BOOTSTATEHMACLOCK_0,
            true => BOOTSTATEHMACLOCK_A::BOOTSTATEHMACLOCK_1,
        }
    }
    #[doc = "Checks if the value of the field is `BOOTSTATEHMACLOCK_0`"]
    #[inline(always)]
    pub fn is_bootstatehmaclock_0(&self) -> bool {
        *self == BOOTSTATEHMACLOCK_A::BOOTSTATEHMACLOCK_0
    }
    #[doc = "Checks if the value of the field is `BOOTSTATEHMACLOCK_1`"]
    #[inline(always)]
    pub fn is_bootstatehmaclock_1(&self) -> bool {
        *self == BOOTSTATEHMACLOCK_A::BOOTSTATEHMACLOCK_1
    }
}
#[doc = "Field `BOOTSTATEHMACLOCK` writer - Boot State HMA Lockout"]
pub type BOOTSTATEHMACLOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BOOTSTATELOCK_SPEC, BOOTSTATEHMACLOCK_A, O>;
impl<'a, const O: u8> BOOTSTATEHMACLOCK_W<'a, O> {
    #[doc = "BOOTSTATEHMAC\\[0:7\\]
can be changed"]
    #[inline(always)]
    pub fn bootstatehmaclock_0(self) -> &'a mut W {
        self.variant(BOOTSTATEHMACLOCK_A::BOOTSTATEHMACLOCK_0)
    }
    #[doc = "BOOTSTATEHMAC\\[0:7\\]
cannot be changed"]
    #[inline(always)]
    pub fn bootstatehmaclock_1(self) -> &'a mut W {
        self.variant(BOOTSTATEHMACLOCK_A::BOOTSTATEHMACLOCK_1)
    }
}
impl R {
    #[doc = "Bit 0 - Boot State Seed Lockout"]
    #[inline(always)]
    pub fn bootstateseedlock(&self) -> BOOTSTATESEEDLOCK_R {
        BOOTSTATESEEDLOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Boot State HMA Lockout"]
    #[inline(always)]
    pub fn bootstatehmaclock(&self) -> BOOTSTATEHMACLOCK_R {
        BOOTSTATEHMACLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Boot State Seed Lockout"]
    #[inline(always)]
    #[must_use]
    pub fn bootstateseedlock(&mut self) -> BOOTSTATESEEDLOCK_W<0> {
        BOOTSTATESEEDLOCK_W::new(self)
    }
    #[doc = "Bit 1 - Boot State HMA Lockout"]
    #[inline(always)]
    #[must_use]
    pub fn bootstatehmaclock(&mut self) -> BOOTSTATEHMACLOCK_W<1> {
        BOOTSTATEHMACLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Boot State Lock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bootstatelock](index.html) module"]
pub struct BOOTSTATELOCK_SPEC;
impl crate::RegisterSpec for BOOTSTATELOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bootstatelock::R](R) reader structure"]
impl crate::Readable for BOOTSTATELOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bootstatelock::W](W) writer structure"]
impl crate::Writable for BOOTSTATELOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOOTSTATELOCK to value 0"]
impl crate::Resettable for BOOTSTATELOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
