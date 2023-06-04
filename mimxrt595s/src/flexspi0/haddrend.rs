#[doc = "Register `HADDREND` reader"]
pub struct R(crate::R<HADDREND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HADDREND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HADDREND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HADDREND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HADDREND` writer"]
pub struct W(crate::W<HADDREND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HADDREND_SPEC>;
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
impl From<crate::W<HADDREND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HADDREND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENDSTART` reader - HADDR remap range's end address, 4K aligned"]
pub type ENDSTART_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ENDSTART` writer - HADDR remap range's end address, 4K aligned"]
pub type ENDSTART_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HADDREND_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 12:31 - HADDR remap range's end address, 4K aligned"]
    #[inline(always)]
    pub fn endstart(&self) -> ENDSTART_R {
        ENDSTART_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - HADDR remap range's end address, 4K aligned"]
    #[inline(always)]
    #[must_use]
    pub fn endstart(&mut self) -> ENDSTART_W<12> {
        ENDSTART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HADDR REMAP END ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [haddrend](index.html) module"]
pub struct HADDREND_SPEC;
impl crate::RegisterSpec for HADDREND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [haddrend::R](R) reader structure"]
impl crate::Readable for HADDREND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [haddrend::W](W) writer structure"]
impl crate::Writable for HADDREND_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HADDREND to value 0"]
impl crate::Resettable for HADDREND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
