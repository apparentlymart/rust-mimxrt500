#[doc = "Register `PINOUTDIS` reader"]
pub struct R(crate::R<PINOUTDIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINOUTDIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINOUTDIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINOUTDIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINOUTDIS` writer"]
pub struct W(crate::W<PINOUTDIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINOUTDIS_SPEC>;
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
impl From<crate::W<PINOUTDIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINOUTDIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTDIS` reader - Output Disable"]
pub type OUTDIS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OUTDIS` writer - Output Disable"]
pub type OUTDIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PINOUTDIS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Output Disable"]
    #[inline(always)]
    pub fn outdis(&self) -> OUTDIS_R {
        OUTDIS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output Disable"]
    #[inline(always)]
    #[must_use]
    pub fn outdis(&mut self) -> OUTDIS_W<0> {
        OUTDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Output Disable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinoutdis](index.html) module"]
pub struct PINOUTDIS_SPEC;
impl crate::RegisterSpec for PINOUTDIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinoutdis::R](R) reader structure"]
impl crate::Readable for PINOUTDIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinoutdis::W](W) writer structure"]
impl crate::Writable for PINOUTDIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINOUTDIS to value 0"]
impl crate::Resettable for PINOUTDIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
