#[doc = "Register `WRITE_QOS` reader"]
pub struct R(crate::R<WRITE_QOS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRITE_QOS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRITE_QOS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRITE_QOS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRITE_QOS` writer"]
pub struct W(crate::W<WRITE_QOS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRITE_QOS_SPEC>;
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
impl From<crate::W<WRITE_QOS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRITE_QOS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRITE_QOS` reader - Write channel QoS value"]
pub type WRITE_QOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRITE_QOS` writer - Write channel QoS value"]
pub type WRITE_QOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRITE_QOS_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Write channel QoS value"]
    #[inline(always)]
    pub fn write_qos(&self) -> WRITE_QOS_R {
        WRITE_QOS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Write channel QoS value"]
    #[inline(always)]
    #[must_use]
    pub fn write_qos(&mut self) -> WRITE_QOS_W<0> {
        WRITE_QOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WRITE channel QoS value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [write_qos](index.html) module"]
pub struct WRITE_QOS_SPEC;
impl crate::RegisterSpec for WRITE_QOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [write_qos::R](R) reader structure"]
impl crate::Readable for WRITE_QOS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [write_qos::W](W) writer structure"]
impl crate::Writable for WRITE_QOS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRITE_QOS to value 0x01"]
impl crate::Resettable for WRITE_QOS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
