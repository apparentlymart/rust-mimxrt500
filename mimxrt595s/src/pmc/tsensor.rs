#[doc = "Register `TSENSOR` reader"]
pub struct R(crate::R<TSENSOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSENSOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSENSOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSENSOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSENSOR` writer"]
pub struct W(crate::W<TSENSOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSENSOR_SPEC>;
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
impl From<crate::W<TSENSOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSENSOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSENSM` reader - Temperature sensor mode select"]
pub type TSENSM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSENSM` writer - Temperature sensor mode select"]
pub type TSENSM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSENSOR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Temperature sensor mode select"]
    #[inline(always)]
    pub fn tsensm(&self) -> TSENSM_R {
        TSENSM_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Temperature sensor mode select"]
    #[inline(always)]
    #[must_use]
    pub fn tsensm(&mut self) -> TSENSM_W<0> {
        TSENSM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMC Temperature Sensor Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsensor](index.html) module"]
pub struct TSENSOR_SPEC;
impl crate::RegisterSpec for TSENSOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsensor::R](R) reader structure"]
impl crate::Readable for TSENSOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsensor::W](W) writer structure"]
impl crate::Writable for TSENSOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSENSOR to value 0"]
impl crate::Resettable for TSENSOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
