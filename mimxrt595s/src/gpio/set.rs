#[doc = "Register `SET[%s]` reader"]
pub struct R(crate::R<SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SET[%s]` writer"]
pub struct W(crate::W<SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SET_SPEC>;
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
impl From<crate::W<SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETP` reader - Read or set output bits"]
pub type SETP_R = crate::FieldReader<u32, SETP_A>;
#[doc = "Read or set output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SETP_A {
    #[doc = "0: Read- output bit; write- no operation"]
    SETP_0 = 0,
    #[doc = "1: Read- output bit; write- set output bit"]
    SETP_1 = 1,
}
impl From<SETP_A> for u32 {
    #[inline(always)]
    fn from(variant: SETP_A) -> Self {
        variant as _
    }
}
impl SETP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SETP_A> {
        match self.bits {
            0 => Some(SETP_A::SETP_0),
            1 => Some(SETP_A::SETP_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SETP_0`"]
    #[inline(always)]
    pub fn is_setp_0(&self) -> bool {
        *self == SETP_A::SETP_0
    }
    #[doc = "Checks if the value of the field is `SETP_1`"]
    #[inline(always)]
    pub fn is_setp_1(&self) -> bool {
        *self == SETP_A::SETP_1
    }
}
#[doc = "Field `SETP` writer - Read or set output bits"]
pub type SETP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SET_SPEC, u32, SETP_A, 32, O>;
impl<'a, const O: u8> SETP_W<'a, O> {
    #[doc = "Read- output bit; write- no operation"]
    #[inline(always)]
    pub fn setp_0(self) -> &'a mut W {
        self.variant(SETP_A::SETP_0)
    }
    #[doc = "Read- output bit; write- set output bit"]
    #[inline(always)]
    pub fn setp_1(self) -> &'a mut W {
        self.variant(SETP_A::SETP_1)
    }
}
impl R {
    #[doc = "Bits 0:31 - Read or set output bits"]
    #[inline(always)]
    pub fn setp(&self) -> SETP_R {
        SETP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read or set output bits"]
    #[inline(always)]
    #[must_use]
    pub fn setp(&mut self) -> SETP_W<0> {
        SETP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set](index.html) module"]
pub struct SET_SPEC;
impl crate::RegisterSpec for SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [set::R](R) reader structure"]
impl crate::Readable for SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [set::W](W) writer structure"]
impl crate::Writable for SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff_ffff;
}
#[doc = "`reset()` method sets SET[%s]
to value 0"]
impl crate::Resettable for SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
