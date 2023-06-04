#[doc = "Register `CTX_RGD_W0` reader"]
pub struct R(crate::R<CTX_RGD_W0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTX_RGD_W0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTX_RGD_W0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTX_RGD_W0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTX_RGD_W0` writer"]
pub struct W(crate::W<CTX_RGD_W0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTX_RGD_W0_SPEC>;
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
impl From<crate::W<CTX_RGD_W0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTX_RGD_W0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRTADDR` reader - Start Address"]
pub type SRTADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SRTADDR` writer - Start Address"]
pub type SRTADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTX_RGD_W0_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 10:31 - Start Address"]
    #[inline(always)]
    pub fn srtaddr(&self) -> SRTADDR_R {
        SRTADDR_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 10:31 - Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn srtaddr(&mut self) -> SRTADDR_W<10> {
        SRTADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Region Descriptor Word0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctx_rgd_w0](index.html) module"]
pub struct CTX_RGD_W0_SPEC;
impl crate::RegisterSpec for CTX_RGD_W0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctx_rgd_w0::R](R) reader structure"]
impl crate::Readable for CTX_RGD_W0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctx_rgd_w0::W](W) writer structure"]
impl crate::Writable for CTX_RGD_W0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTX_RGD_W0 to value 0"]
impl crate::Resettable for CTX_RGD_W0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
