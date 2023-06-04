#[doc = "Register `UUID[%s]` reader"]
pub struct R(crate::R<UUID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UUID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UUID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UUID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UUID[%s]` writer"]
pub struct W(crate::W<UUID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UUID_SPEC>;
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
impl From<crate::W<UUID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UUID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UUIDN` reader - UUIDn 32-Bit Data Register"]
pub type UUIDN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `UUIDN` writer - UUIDn 32-Bit Data Register"]
pub type UUIDN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UUID_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - UUIDn 32-Bit Data Register"]
    #[inline(always)]
    pub fn uuidn(&self) -> UUIDN_R {
        UUIDN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - UUIDn 32-Bit Data Register"]
    #[inline(always)]
    #[must_use]
    pub fn uuidn(&mut self) -> UUIDN_W<0> {
        UUIDN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UUID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uuid](index.html) module"]
pub struct UUID_SPEC;
impl crate::RegisterSpec for UUID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uuid::R](R) reader structure"]
impl crate::Readable for UUID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uuid::W](W) writer structure"]
impl crate::Writable for UUID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UUID[%s]
to value 0"]
impl crate::Resettable for UUID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
