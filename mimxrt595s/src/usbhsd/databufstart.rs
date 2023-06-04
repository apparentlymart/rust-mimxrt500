#[doc = "Register `DATABUFSTART` reader"]
pub struct R(crate::R<DATABUFSTART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATABUFSTART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATABUFSTART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATABUFSTART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATABUFSTART` writer"]
pub struct W(crate::W<DATABUFSTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATABUFSTART_SPEC>;
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
impl From<crate::W<DATABUFSTART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATABUFSTART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DA_BUF_FIXED` reader - Fixed portion of the data buffer start address."]
pub type DA_BUF_FIXED_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DA_BUF` reader - Programmable portion of the the data buffer start address."]
pub type DA_BUF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DA_BUF` writer - Programmable portion of the the data buffer start address."]
pub type DA_BUF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATABUFSTART_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:17 - Fixed portion of the data buffer start address."]
    #[inline(always)]
    pub fn da_buf_fixed(&self) -> DA_BUF_FIXED_R {
        DA_BUF_FIXED_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:31 - Programmable portion of the the data buffer start address."]
    #[inline(always)]
    pub fn da_buf(&self) -> DA_BUF_R {
        DA_BUF_R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 18:31 - Programmable portion of the the data buffer start address."]
    #[inline(always)]
    #[must_use]
    pub fn da_buf(&mut self) -> DA_BUF_W<18> {
        DA_BUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Data Buffer List Start Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [databufstart](index.html) module"]
pub struct DATABUFSTART_SPEC;
impl crate::RegisterSpec for DATABUFSTART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [databufstart::R](R) reader structure"]
impl crate::Readable for DATABUFSTART_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [databufstart::W](W) writer structure"]
impl crate::Writable for DATABUFSTART_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATABUFSTART to value 0"]
impl crate::Resettable for DATABUFSTART_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
