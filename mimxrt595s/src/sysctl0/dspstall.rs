#[doc = "Register `DSPSTALL` reader"]
pub struct R(crate::R<DSPSTALL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSPSTALL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSPSTALL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSPSTALL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSPSTALL` writer"]
pub struct W(crate::W<DSPSTALL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSPSTALL_SPEC>;
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
impl From<crate::W<DSPSTALL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSPSTALL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSPSTALL` reader - DSPSTALL"]
pub type DSPSTALL_R = crate::BitReader<DSPSTALL_A>;
#[doc = "DSPSTALL\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSPSTALL_A {
    #[doc = "0: Run(Normal mode)"]
    DSPSTALL_0 = 0,
    #[doc = "1: Stall mode"]
    DSPSTALL_1 = 1,
}
impl From<DSPSTALL_A> for bool {
    #[inline(always)]
    fn from(variant: DSPSTALL_A) -> Self {
        variant as u8 != 0
    }
}
impl DSPSTALL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSPSTALL_A {
        match self.bits {
            false => DSPSTALL_A::DSPSTALL_0,
            true => DSPSTALL_A::DSPSTALL_1,
        }
    }
    #[doc = "Checks if the value of the field is `DSPSTALL_0`"]
    #[inline(always)]
    pub fn is_dspstall_0(&self) -> bool {
        *self == DSPSTALL_A::DSPSTALL_0
    }
    #[doc = "Checks if the value of the field is `DSPSTALL_1`"]
    #[inline(always)]
    pub fn is_dspstall_1(&self) -> bool {
        *self == DSPSTALL_A::DSPSTALL_1
    }
}
#[doc = "Field `DSPSTALL` writer - DSPSTALL"]
pub type DSPSTALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSPSTALL_SPEC, DSPSTALL_A, O>;
impl<'a, const O: u8> DSPSTALL_W<'a, O> {
    #[doc = "Run(Normal mode)"]
    #[inline(always)]
    pub fn dspstall_0(self) -> &'a mut W {
        self.variant(DSPSTALL_A::DSPSTALL_0)
    }
    #[doc = "Stall mode"]
    #[inline(always)]
    pub fn dspstall_1(self) -> &'a mut W {
        self.variant(DSPSTALL_A::DSPSTALL_1)
    }
}
impl R {
    #[doc = "Bit 0 - DSPSTALL"]
    #[inline(always)]
    pub fn dspstall(&self) -> DSPSTALL_R {
        DSPSTALL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DSPSTALL"]
    #[inline(always)]
    #[must_use]
    pub fn dspstall(&mut self) -> DSPSTALL_W<0> {
        DSPSTALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSP Stall Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dspstall](index.html) module"]
pub struct DSPSTALL_SPEC;
impl crate::RegisterSpec for DSPSTALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dspstall::R](R) reader structure"]
impl crate::Readable for DSPSTALL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dspstall::W](W) writer structure"]
impl crate::Writable for DSPSTALL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSPSTALL to value 0x01"]
impl crate::Resettable for DSPSTALL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
