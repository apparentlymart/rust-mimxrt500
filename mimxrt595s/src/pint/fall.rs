#[doc = "Register `FALL` reader"]
pub struct R(crate::R<FALL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FALL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FALL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FALL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FALL` writer"]
pub struct W(crate::W<FALL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FALL_SPEC>;
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
impl From<crate::W<FALL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FALL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FDET` reader - Falling edge detect"]
pub type FDET_R = crate::FieldReader<u8, FDET_A>;
#[doc = "Falling edge detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FDET_A {
    #[doc = "0: Read 0- No falling edge has been detected on this pin since Reset or the last time a one was written to this bit, Write 0- no operation"]
    FDET_0 = 0,
    #[doc = "1: Read 1- a falling edge has been detected since Reset or the last time a one was written to this bit, Write 1- clear falling edge detection for this bit"]
    FDET_1 = 1,
}
impl From<FDET_A> for u8 {
    #[inline(always)]
    fn from(variant: FDET_A) -> Self {
        variant as _
    }
}
impl FDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FDET_A> {
        match self.bits {
            0 => Some(FDET_A::FDET_0),
            1 => Some(FDET_A::FDET_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FDET_0`"]
    #[inline(always)]
    pub fn is_fdet_0(&self) -> bool {
        *self == FDET_A::FDET_0
    }
    #[doc = "Checks if the value of the field is `FDET_1`"]
    #[inline(always)]
    pub fn is_fdet_1(&self) -> bool {
        *self == FDET_A::FDET_1
    }
}
#[doc = "Field `FDET` writer - Falling edge detect"]
pub type FDET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FALL_SPEC, u8, FDET_A, 8, O>;
impl<'a, const O: u8> FDET_W<'a, O> {
    #[doc = "Read 0- No falling edge has been detected on this pin since Reset or the last time a one was written to this bit, Write 0- no operation"]
    #[inline(always)]
    pub fn fdet_0(self) -> &'a mut W {
        self.variant(FDET_A::FDET_0)
    }
    #[doc = "Read 1- a falling edge has been detected since Reset or the last time a one was written to this bit, Write 1- clear falling edge detection for this bit"]
    #[inline(always)]
    pub fn fdet_1(self) -> &'a mut W {
        self.variant(FDET_A::FDET_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Falling edge detect"]
    #[inline(always)]
    pub fn fdet(&self) -> FDET_R {
        FDET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Falling edge detect"]
    #[inline(always)]
    #[must_use]
    pub fn fdet(&mut self) -> FDET_W<0> {
        FDET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Interrupt Falling Edge\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fall](index.html) module"]
pub struct FALL_SPEC;
impl crate::RegisterSpec for FALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fall::R](R) reader structure"]
impl crate::Readable for FALL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fall::W](W) writer structure"]
impl crate::Writable for FALL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FALL to value 0"]
impl crate::Resettable for FALL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
