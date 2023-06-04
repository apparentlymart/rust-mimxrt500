#[doc = "Register `SHIFTBUFEOS[%s]` reader"]
pub struct R(crate::R<SHIFTBUFEOS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTBUFEOS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTBUFEOS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTBUFEOS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTBUFEOS[%s]` writer"]
pub struct W(crate::W<SHIFTBUFEOS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTBUFEOS_SPEC>;
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
impl From<crate::W<SHIFTBUFEOS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTBUFEOS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHIFTBUFEOS` reader - Shift Buffer"]
pub type SHIFTBUFEOS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SHIFTBUFEOS` writer - Shift Buffer"]
pub type SHIFTBUFEOS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHIFTBUFEOS_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    pub fn shiftbufeos(&self) -> SHIFTBUFEOS_R {
        SHIFTBUFEOS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn shiftbufeos(&mut self) -> SHIFTBUFEOS_W<0> {
        SHIFTBUFEOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Buffer N Even Odd Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufeos](index.html) module"]
pub struct SHIFTBUFEOS_SPEC;
impl crate::RegisterSpec for SHIFTBUFEOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shiftbufeos::R](R) reader structure"]
impl crate::Readable for SHIFTBUFEOS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shiftbufeos::W](W) writer structure"]
impl crate::Writable for SHIFTBUFEOS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHIFTBUFEOS[%s]
to value 0"]
impl crate::Resettable for SHIFTBUFEOS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
