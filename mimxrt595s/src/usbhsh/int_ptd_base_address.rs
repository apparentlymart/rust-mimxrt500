#[doc = "Register `INT_PTD_BASE_ADDRESS` reader"]
pub struct R(crate::R<INT_PTD_BASE_ADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_PTD_BASE_ADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_PTD_BASE_ADDRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_PTD_BASE_ADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_PTD_BASE_ADDRESS` writer"]
pub struct W(crate::W<INT_PTD_BASE_ADDRESS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_PTD_BASE_ADDRESS_SPEC>;
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
impl From<crate::W<INT_PTD_BASE_ADDRESS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_PTD_BASE_ADDRESS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_FIRST` reader - First PTD in the INT list"]
pub type INT_FIRST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_FIRST` writer - First PTD in the INT list"]
pub type INT_FIRST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INT_PTD_BASE_ADDRESS_SPEC, u8, u8, 5, O>;
#[doc = "Field `INT_BASE` reader - Start of INT PTD list"]
pub type INT_BASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INT_BASE` writer - Start of INT PTD list"]
pub type INT_BASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INT_PTD_BASE_ADDRESS_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 5:9 - First PTD in the INT list"]
    #[inline(always)]
    pub fn int_first(&self) -> INT_FIRST_R {
        INT_FIRST_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:31 - Start of INT PTD list"]
    #[inline(always)]
    pub fn int_base(&self) -> INT_BASE_R {
        INT_BASE_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 5:9 - First PTD in the INT list"]
    #[inline(always)]
    #[must_use]
    pub fn int_first(&mut self) -> INT_FIRST_W<5> {
        INT_FIRST_W::new(self)
    }
    #[doc = "Bits 10:31 - Start of INT PTD list"]
    #[inline(always)]
    #[must_use]
    pub fn int_base(&mut self) -> INT_BASE_W<10> {
        INT_BASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INT PTD Base Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ptd_base_address](index.html) module"]
pub struct INT_PTD_BASE_ADDRESS_SPEC;
impl crate::RegisterSpec for INT_PTD_BASE_ADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ptd_base_address::R](R) reader structure"]
impl crate::Readable for INT_PTD_BASE_ADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ptd_base_address::W](W) writer structure"]
impl crate::Writable for INT_PTD_BASE_ADDRESS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_PTD_BASE_ADDRESS to value 0"]
impl crate::Resettable for INT_PTD_BASE_ADDRESS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
