#[doc = "Register `SIDEXT` reader"]
pub struct R(crate::R<SIDEXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIDEXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIDEXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIDEXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIDEXT` writer"]
pub struct W(crate::W<SIDEXT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIDEXT_SPEC>;
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
impl From<crate::W<SIDEXT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIDEXT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCR` reader - Device Characteristic Register"]
pub type DCR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCR` writer - Device Characteristic Register"]
pub type DCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SIDEXT_SPEC, u8, u8, 8, O>;
#[doc = "Field `BCR` reader - Bus Characteristics Register"]
pub type BCR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BCR` writer - Bus Characteristics Register"]
pub type BCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SIDEXT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 8:15 - Device Characteristic Register"]
    #[inline(always)]
    pub fn dcr(&self) -> DCR_R {
        DCR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Bus Characteristics Register"]
    #[inline(always)]
    pub fn bcr(&self) -> BCR_R {
        BCR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Device Characteristic Register"]
    #[inline(always)]
    #[must_use]
    pub fn dcr(&mut self) -> DCR_W<8> {
        DCR_W::new(self)
    }
    #[doc = "Bits 16:23 - Bus Characteristics Register"]
    #[inline(always)]
    #[must_use]
    pub fn bcr(&mut self) -> BCR_W<16> {
        BCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave ID Extension Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sidext](index.html) module"]
pub struct SIDEXT_SPEC;
impl crate::RegisterSpec for SIDEXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sidext::R](R) reader structure"]
impl crate::Readable for SIDEXT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sidext::W](W) writer structure"]
impl crate::Writable for SIDEXT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIDEXT to value 0"]
impl crate::Resettable for SIDEXT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
