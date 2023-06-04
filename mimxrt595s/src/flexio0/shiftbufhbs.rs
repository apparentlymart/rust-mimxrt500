#[doc = "Register `SHIFTBUFHBS[%s]` reader"]
pub struct R(crate::R<SHIFTBUFHBS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTBUFHBS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTBUFHBS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTBUFHBS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTBUFHBS[%s]` writer"]
pub struct W(crate::W<SHIFTBUFHBS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTBUFHBS_SPEC>;
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
impl From<crate::W<SHIFTBUFHBS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTBUFHBS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHIFTBUFHBS` reader - Shift Buffer"]
pub type SHIFTBUFHBS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SHIFTBUFHBS` writer - Shift Buffer"]
pub type SHIFTBUFHBS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHIFTBUFHBS_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    pub fn shiftbufhbs(&self) -> SHIFTBUFHBS_R {
        SHIFTBUFHBS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn shiftbufhbs(&mut self) -> SHIFTBUFHBS_W<0> {
        SHIFTBUFHBS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Buffer N Halfword Byte Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufhbs](index.html) module"]
pub struct SHIFTBUFHBS_SPEC;
impl crate::RegisterSpec for SHIFTBUFHBS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shiftbufhbs::R](R) reader structure"]
impl crate::Readable for SHIFTBUFHBS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shiftbufhbs::W](W) writer structure"]
impl crate::Writable for SHIFTBUFHBS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHIFTBUFHBS[%s]
to value 0"]
impl crate::Resettable for SHIFTBUFHBS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
