#[doc = "Register `DATA_PAYLOAD_BASE_ADDRESS` reader"]
pub struct R(crate::R<DATA_PAYLOAD_BASE_ADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_PAYLOAD_BASE_ADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_PAYLOAD_BASE_ADDRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_PAYLOAD_BASE_ADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA_PAYLOAD_BASE_ADDRESS` writer"]
pub struct W(crate::W<DATA_PAYLOAD_BASE_ADDRESS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_PAYLOAD_BASE_ADDRESS_SPEC>;
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
impl From<crate::W<DATA_PAYLOAD_BASE_ADDRESS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_PAYLOAD_BASE_ADDRESS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAT_BASE` reader - Data Payload Section Base Address"]
pub type DAT_BASE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DAT_BASE` writer - Data Payload Section Base Address"]
pub type DAT_BASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATA_PAYLOAD_BASE_ADDRESS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 16:31 - Data Payload Section Base Address"]
    #[inline(always)]
    pub fn dat_base(&self) -> DAT_BASE_R {
        DAT_BASE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Data Payload Section Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn dat_base(&mut self) -> DAT_BASE_W<16> {
        DAT_BASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DATA PAYLOAD Base Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_payload_base_address](index.html) module"]
pub struct DATA_PAYLOAD_BASE_ADDRESS_SPEC;
impl crate::RegisterSpec for DATA_PAYLOAD_BASE_ADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_payload_base_address::R](R) reader structure"]
impl crate::Readable for DATA_PAYLOAD_BASE_ADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_payload_base_address::W](W) writer structure"]
impl crate::Writable for DATA_PAYLOAD_BASE_ADDRESS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA_PAYLOAD_BASE_ADDRESS to value 0"]
impl crate::Resettable for DATA_PAYLOAD_BASE_ADDRESS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
