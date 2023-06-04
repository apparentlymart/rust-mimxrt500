#[doc = "Register `ISO_PTD_BASE_ADDRESS` reader"]
pub struct R(crate::R<ISO_PTD_BASE_ADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISO_PTD_BASE_ADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISO_PTD_BASE_ADDRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISO_PTD_BASE_ADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISO_PTD_BASE_ADDRESS` writer"]
pub struct W(crate::W<ISO_PTD_BASE_ADDRESS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISO_PTD_BASE_ADDRESS_SPEC>;
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
impl From<crate::W<ISO_PTD_BASE_ADDRESS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISO_PTD_BASE_ADDRESS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISO_FIRST` reader - First PTD in the ISO list"]
pub type ISO_FIRST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISO_FIRST` writer - First PTD in the ISO list"]
pub type ISO_FIRST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ISO_PTD_BASE_ADDRESS_SPEC, u8, u8, 5, O>;
#[doc = "Field `ISO_BASE` reader - Start of ISO PTD list"]
pub type ISO_BASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ISO_BASE` writer - Start of ISO PTD list"]
pub type ISO_BASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ISO_PTD_BASE_ADDRESS_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 5:9 - First PTD in the ISO list"]
    #[inline(always)]
    pub fn iso_first(&self) -> ISO_FIRST_R {
        ISO_FIRST_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:31 - Start of ISO PTD list"]
    #[inline(always)]
    pub fn iso_base(&self) -> ISO_BASE_R {
        ISO_BASE_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 5:9 - First PTD in the ISO list"]
    #[inline(always)]
    #[must_use]
    pub fn iso_first(&mut self) -> ISO_FIRST_W<5> {
        ISO_FIRST_W::new(self)
    }
    #[doc = "Bits 10:31 - Start of ISO PTD list"]
    #[inline(always)]
    #[must_use]
    pub fn iso_base(&mut self) -> ISO_BASE_W<10> {
        ISO_BASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISO PTD Base Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iso_ptd_base_address](index.html) module"]
pub struct ISO_PTD_BASE_ADDRESS_SPEC;
impl crate::RegisterSpec for ISO_PTD_BASE_ADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iso_ptd_base_address::R](R) reader structure"]
impl crate::Readable for ISO_PTD_BASE_ADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iso_ptd_base_address::W](W) writer structure"]
impl crate::Writable for ISO_PTD_BASE_ADDRESS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISO_PTD_BASE_ADDRESS to value 0"]
impl crate::Resettable for ISO_PTD_BASE_ADDRESS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
