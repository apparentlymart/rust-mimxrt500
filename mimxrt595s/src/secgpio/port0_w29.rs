#[doc = "Register `Port0_W29` reader"]
pub struct R(crate::R<PORT0_W29_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORT0_W29_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORT0_W29_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORT0_W29_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Port0_W29` writer"]
pub struct W(crate::W<PORT0_W29_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORT0_W29_SPEC>;
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
impl From<crate::W<PORT0_W29_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORT0_W29_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWORD` reader - PWORD"]
pub type PWORD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PWORD` writer - PWORD"]
pub type PWORD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PORT0_W29_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - PWORD"]
    #[inline(always)]
    pub fn pword(&self) -> PWORD_R {
        PWORD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PWORD"]
    #[inline(always)]
    #[must_use]
    pub fn pword(&mut self) -> PWORD_W<0> {
        PWORD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Word pin registers for all port GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port0_w29](index.html) module"]
pub struct PORT0_W29_SPEC;
impl crate::RegisterSpec for PORT0_W29_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [port0_w29::R](R) reader structure"]
impl crate::Readable for PORT0_W29_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [port0_w29::W](W) writer structure"]
impl crate::Writable for PORT0_W29_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Port0_W29 to value 0"]
impl crate::Resettable for PORT0_W29_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
