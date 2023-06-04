#[doc = "Register `CS_PROTCPU0` reader"]
pub struct R(crate::R<CS_PROTCPU0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CS_PROTCPU0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CS_PROTCPU0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CS_PROTCPU0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CS_PROTCPU0` writer"]
pub struct W(crate::W<CS_PROTCPU0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CS_PROTCPU0_SPEC>;
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
impl From<crate::W<CS_PROTCPU0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CS_PROTCPU0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS_PROTCPU0` reader - Controls M33 AP Enable"]
pub type CS_PROTCPU0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CS_PROTCPU0` writer - Controls M33 AP Enable"]
pub type CS_PROTCPU0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CS_PROTCPU0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Controls M33 AP Enable"]
    #[inline(always)]
    pub fn cs_protcpu0(&self) -> CS_PROTCPU0_R {
        CS_PROTCPU0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Controls M33 AP Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs_protcpu0(&mut self) -> CS_PROTCPU0_W<0> {
        CS_PROTCPU0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Code Security for CPU0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs_protcpu0](index.html) module"]
pub struct CS_PROTCPU0_SPEC;
impl crate::RegisterSpec for CS_PROTCPU0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cs_protcpu0::R](R) reader structure"]
impl crate::Readable for CS_PROTCPU0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cs_protcpu0::W](W) writer structure"]
impl crate::Writable for CS_PROTCPU0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CS_PROTCPU0 to value 0"]
impl crate::Resettable for CS_PROTCPU0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
