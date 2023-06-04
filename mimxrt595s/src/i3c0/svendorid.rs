#[doc = "Register `SVENDORID` reader"]
pub struct R(crate::R<SVENDORID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SVENDORID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SVENDORID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SVENDORID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SVENDORID` writer"]
pub struct W(crate::W<SVENDORID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SVENDORID_SPEC>;
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
impl From<crate::W<SVENDORID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SVENDORID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VID` reader - Vendor ID"]
pub type VID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VID` writer - Vendor ID"]
pub type VID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SVENDORID_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - Vendor ID"]
    #[inline(always)]
    pub fn vid(&self) -> VID_R {
        VID_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Vendor ID"]
    #[inline(always)]
    #[must_use]
    pub fn vid(&mut self) -> VID_W<0> {
        VID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Vendor ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [svendorid](index.html) module"]
pub struct SVENDORID_SPEC;
impl crate::RegisterSpec for SVENDORID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [svendorid::R](R) reader structure"]
impl crate::Readable for SVENDORID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [svendorid::W](W) writer structure"]
impl crate::Writable for SVENDORID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SVENDORID to value 0x011b"]
impl crate::Resettable for SVENDORID_SPEC {
    const RESET_VALUE: Self::Ux = 0x011b;
}
