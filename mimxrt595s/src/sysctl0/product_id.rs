#[doc = "Register `PRODUCT_ID` reader"]
pub struct R(crate::R<PRODUCT_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRODUCT_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRODUCT_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRODUCT_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRODUCT_ID` writer"]
pub struct W(crate::W<PRODUCT_ID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRODUCT_ID_SPEC>;
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
impl From<crate::W<PRODUCT_ID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRODUCT_ID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRODUCT_ID` reader - PRODUCT ID"]
pub type PRODUCT_ID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRODUCT_ID` writer - PRODUCT ID"]
pub type PRODUCT_ID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRODUCT_ID_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - PRODUCT ID"]
    #[inline(always)]
    pub fn product_id(&self) -> PRODUCT_ID_R {
        PRODUCT_ID_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PRODUCT ID"]
    #[inline(always)]
    #[must_use]
    pub fn product_id(&mut self) -> PRODUCT_ID_W<0> {
        PRODUCT_ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Product ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [product_id](index.html) module"]
pub struct PRODUCT_ID_SPEC;
impl crate::RegisterSpec for PRODUCT_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [product_id::R](R) reader structure"]
impl crate::Readable for PRODUCT_ID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [product_id::W](W) writer structure"]
impl crate::Writable for PRODUCT_ID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRODUCT_ID to value 0xcafe"]
impl crate::Resettable for PRODUCT_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0xcafe;
}
