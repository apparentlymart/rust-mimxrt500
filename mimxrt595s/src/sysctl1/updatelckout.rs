#[doc = "Register `UPDATELCKOUT` reader"]
pub struct R(crate::R<UPDATELCKOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UPDATELCKOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UPDATELCKOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UPDATELCKOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UPDATELCKOUT` writer"]
pub struct W(crate::W<UPDATELCKOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPDATELCKOUT_SPEC>;
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
impl From<crate::W<UPDATELCKOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UPDATELCKOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPDATELCKOUT` reader - Update Clock Lockout"]
pub type UPDATELCKOUT_R = crate::BitReader<UPDATELCKOUT_A>;
#[doc = "Update Clock Lockout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPDATELCKOUT_A {
    #[doc = "0: Normal Mode"]
    UPDATELCKOUT_0 = 0,
    #[doc = "1: Protected Mode. Cannot be written to. Currently this register does not lock anything"]
    UPDATELCKOUT_1 = 1,
}
impl From<UPDATELCKOUT_A> for bool {
    #[inline(always)]
    fn from(variant: UPDATELCKOUT_A) -> Self {
        variant as u8 != 0
    }
}
impl UPDATELCKOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDATELCKOUT_A {
        match self.bits {
            false => UPDATELCKOUT_A::UPDATELCKOUT_0,
            true => UPDATELCKOUT_A::UPDATELCKOUT_1,
        }
    }
    #[doc = "Checks if the value of the field is `UPDATELCKOUT_0`"]
    #[inline(always)]
    pub fn is_updatelckout_0(&self) -> bool {
        *self == UPDATELCKOUT_A::UPDATELCKOUT_0
    }
    #[doc = "Checks if the value of the field is `UPDATELCKOUT_1`"]
    #[inline(always)]
    pub fn is_updatelckout_1(&self) -> bool {
        *self == UPDATELCKOUT_A::UPDATELCKOUT_1
    }
}
#[doc = "Field `UPDATELCKOUT` writer - Update Clock Lockout"]
pub type UPDATELCKOUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UPDATELCKOUT_SPEC, UPDATELCKOUT_A, O>;
impl<'a, const O: u8> UPDATELCKOUT_W<'a, O> {
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn updatelckout_0(self) -> &'a mut W {
        self.variant(UPDATELCKOUT_A::UPDATELCKOUT_0)
    }
    #[doc = "Protected Mode. Cannot be written to. Currently this register does not lock anything"]
    #[inline(always)]
    pub fn updatelckout_1(self) -> &'a mut W {
        self.variant(UPDATELCKOUT_A::UPDATELCKOUT_1)
    }
}
impl R {
    #[doc = "Bit 0 - Update Clock Lockout"]
    #[inline(always)]
    pub fn updatelckout(&self) -> UPDATELCKOUT_R {
        UPDATELCKOUT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update Clock Lockout"]
    #[inline(always)]
    #[must_use]
    pub fn updatelckout(&mut self) -> UPDATELCKOUT_W<0> {
        UPDATELCKOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Update Clock Lockout\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [updatelckout](index.html) module"]
pub struct UPDATELCKOUT_SPEC;
impl crate::RegisterSpec for UPDATELCKOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [updatelckout::R](R) reader structure"]
impl crate::Readable for UPDATELCKOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [updatelckout::W](W) writer structure"]
impl crate::Writable for UPDATELCKOUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UPDATELCKOUT to value 0"]
impl crate::Resettable for UPDATELCKOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
