#[doc = "Register `SYSCPUAHBCLKDIV` reader"]
pub struct R(crate::R<SYSCPUAHBCLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCPUAHBCLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCPUAHBCLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCPUAHBCLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCPUAHBCLKDIV` writer"]
pub struct W(crate::W<SYSCPUAHBCLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCPUAHBCLKDIV_SPEC>;
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
impl From<crate::W<SYSCPUAHBCLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCPUAHBCLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - Clock Divider Value Selection"]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - Clock Divider Value Selection"]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSCPUAHBCLKDIV_SPEC, u8, u8, 8, O>;
#[doc = "Field `REQFLAG` reader - Divider status flag"]
pub type REQFLAG_R = crate::BitReader<REQFLAG_A>;
#[doc = "Divider status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REQFLAG_A {
    #[doc = "0: The change to the divider value has finished"]
    DIVIDER_READY = 0,
    #[doc = "1: A change is being made to the divider value"]
    DIVIDER_NOT_READY = 1,
}
impl From<REQFLAG_A> for bool {
    #[inline(always)]
    fn from(variant: REQFLAG_A) -> Self {
        variant as u8 != 0
    }
}
impl REQFLAG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQFLAG_A {
        match self.bits {
            false => REQFLAG_A::DIVIDER_READY,
            true => REQFLAG_A::DIVIDER_NOT_READY,
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDER_READY`"]
    #[inline(always)]
    pub fn is_divider_ready(&self) -> bool {
        *self == REQFLAG_A::DIVIDER_READY
    }
    #[doc = "Checks if the value of the field is `DIVIDER_NOT_READY`"]
    #[inline(always)]
    pub fn is_divider_not_ready(&self) -> bool {
        *self == REQFLAG_A::DIVIDER_NOT_READY
    }
}
#[doc = "Field `REQFLAG` writer - Divider status flag"]
pub type REQFLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCPUAHBCLKDIV_SPEC, REQFLAG_A, O>;
impl<'a, const O: u8> REQFLAG_W<'a, O> {
    #[doc = "The change to the divider value has finished"]
    #[inline(always)]
    pub fn divider_ready(self) -> &'a mut W {
        self.variant(REQFLAG_A::DIVIDER_READY)
    }
    #[doc = "A change is being made to the divider value"]
    #[inline(always)]
    pub fn divider_not_ready(self) -> &'a mut W {
        self.variant(REQFLAG_A::DIVIDER_NOT_READY)
    }
}
impl R {
    #[doc = "Bits 0:7 - Clock Divider Value Selection"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - Divider status flag"]
    #[inline(always)]
    pub fn reqflag(&self) -> REQFLAG_R {
        REQFLAG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Divider Value Selection"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<0> {
        DIV_W::new(self)
    }
    #[doc = "Bit 31 - Divider status flag"]
    #[inline(always)]
    #[must_use]
    pub fn reqflag(&mut self) -> REQFLAG_W<31> {
        REQFLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System CPU AHB Clock Divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscpuahbclkdiv](index.html) module"]
pub struct SYSCPUAHBCLKDIV_SPEC;
impl crate::RegisterSpec for SYSCPUAHBCLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscpuahbclkdiv::R](R) reader structure"]
impl crate::Readable for SYSCPUAHBCLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscpuahbclkdiv::W](W) writer structure"]
impl crate::Writable for SYSCPUAHBCLKDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCPUAHBCLKDIV to value 0"]
impl crate::Resettable for SYSCPUAHBCLKDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
