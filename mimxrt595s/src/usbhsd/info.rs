#[doc = "Register `INFO` reader"]
pub struct R(crate::R<INFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INFO` writer"]
pub struct W(crate::W<INFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INFO_SPEC>;
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
impl From<crate::W<INFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAME_NR` reader - Frame number."]
pub type FRAME_NR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ERR_CODE` reader - The error code which last occurred"]
pub type ERR_CODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ERR_CODE` writer - The error code which last occurred"]
pub type ERR_CODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INFO_SPEC, u8, u8, 4, O>;
#[doc = "Field `MINREV` reader - Minor revision"]
pub type MINREV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAJREV` reader - Major revision"]
pub type MAJREV_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:10 - Frame number."]
    #[inline(always)]
    pub fn frame_nr(&self) -> FRAME_NR_R {
        FRAME_NR_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - The error code which last occurred"]
    #[inline(always)]
    pub fn err_code(&self) -> ERR_CODE_R {
        ERR_CODE_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Minor revision"]
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Major revision"]
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 11:14 - The error code which last occurred"]
    #[inline(always)]
    #[must_use]
    pub fn err_code(&mut self) -> ERR_CODE_W<11> {
        ERR_CODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Info\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [info](index.html) module"]
pub struct INFO_SPEC;
impl crate::RegisterSpec for INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [info::R](R) reader structure"]
impl crate::Readable for INFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [info::W](W) writer structure"]
impl crate::Writable for INFO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INFO to value 0x2000_0000"]
impl crate::Resettable for INFO_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000_0000;
}
