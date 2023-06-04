#[doc = "Register `RISE` reader"]
pub struct R(crate::R<RISE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RISE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RISE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RISE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RISE` writer"]
pub struct W(crate::W<RISE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RISE_SPEC>;
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
impl From<crate::W<RISE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RISE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDET` reader - Rising edge detect"]
pub type RDET_R = crate::FieldReader<u8, RDET_A>;
#[doc = "Rising edge detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RDET_A {
    #[doc = "0: Read 0- No rising edge has been detected on this pin since Reset or the last time a one was written to this bit, Write 0- no operation"]
    RDET_0 = 0,
    #[doc = "1: Read 1- a rising edge has been detected since Reset or the last time a one was written to this bit, Write 1- clear rising edge detection for this pin"]
    RDET_1 = 1,
}
impl From<RDET_A> for u8 {
    #[inline(always)]
    fn from(variant: RDET_A) -> Self {
        variant as _
    }
}
impl RDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RDET_A> {
        match self.bits {
            0 => Some(RDET_A::RDET_0),
            1 => Some(RDET_A::RDET_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RDET_0`"]
    #[inline(always)]
    pub fn is_rdet_0(&self) -> bool {
        *self == RDET_A::RDET_0
    }
    #[doc = "Checks if the value of the field is `RDET_1`"]
    #[inline(always)]
    pub fn is_rdet_1(&self) -> bool {
        *self == RDET_A::RDET_1
    }
}
#[doc = "Field `RDET` writer - Rising edge detect"]
pub type RDET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RISE_SPEC, u8, RDET_A, 8, O>;
impl<'a, const O: u8> RDET_W<'a, O> {
    #[doc = "Read 0- No rising edge has been detected on this pin since Reset or the last time a one was written to this bit, Write 0- no operation"]
    #[inline(always)]
    pub fn rdet_0(self) -> &'a mut W {
        self.variant(RDET_A::RDET_0)
    }
    #[doc = "Read 1- a rising edge has been detected since Reset or the last time a one was written to this bit, Write 1- clear rising edge detection for this pin"]
    #[inline(always)]
    pub fn rdet_1(self) -> &'a mut W {
        self.variant(RDET_A::RDET_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Rising edge detect"]
    #[inline(always)]
    pub fn rdet(&self) -> RDET_R {
        RDET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Rising edge detect"]
    #[inline(always)]
    #[must_use]
    pub fn rdet(&mut self) -> RDET_W<0> {
        RDET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Interrupt Rising Edge\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rise](index.html) module"]
pub struct RISE_SPEC;
impl crate::RegisterSpec for RISE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rise::R](R) reader structure"]
impl crate::Readable for RISE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rise::W](W) writer structure"]
impl crate::Writable for RISE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RISE to value 0"]
impl crate::Resettable for RISE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
