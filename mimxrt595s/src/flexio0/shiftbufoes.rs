#[doc = "Register `SHIFTBUFOES[%s]` reader"]
pub struct R(crate::R<SHIFTBUFOES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTBUFOES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTBUFOES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTBUFOES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTBUFOES[%s]` writer"]
pub struct W(crate::W<SHIFTBUFOES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTBUFOES_SPEC>;
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
impl From<crate::W<SHIFTBUFOES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTBUFOES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHIFTBUFOES` reader - Shift Buffer"]
pub type SHIFTBUFOES_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SHIFTBUFOES` writer - Shift Buffer"]
pub type SHIFTBUFOES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHIFTBUFOES_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    pub fn shiftbufoes(&self) -> SHIFTBUFOES_R {
        SHIFTBUFOES_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn shiftbufoes(&mut self) -> SHIFTBUFOES_W<0> {
        SHIFTBUFOES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Buffer N Odd Even Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufoes](index.html) module"]
pub struct SHIFTBUFOES_SPEC;
impl crate::RegisterSpec for SHIFTBUFOES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shiftbufoes::R](R) reader structure"]
impl crate::Readable for SHIFTBUFOES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shiftbufoes::W](W) writer structure"]
impl crate::Writable for SHIFTBUFOES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHIFTBUFOES[%s]
to value 0"]
impl crate::Resettable for SHIFTBUFOES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
