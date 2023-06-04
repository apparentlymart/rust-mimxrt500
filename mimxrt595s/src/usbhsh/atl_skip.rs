#[doc = "Register `ATL_SKIP` reader"]
pub struct R(crate::R<ATL_SKIP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATL_SKIP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATL_SKIP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATL_SKIP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATL_SKIP` writer"]
pub struct W(crate::W<ATL_SKIP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATL_SKIP_SPEC>;
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
impl From<crate::W<ATL_SKIP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATL_SKIP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATL_SKIP` reader - ATL PTD Skip Map"]
pub type ATL_SKIP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ATL_SKIP` writer - ATL PTD Skip Map"]
pub type ATL_SKIP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATL_SKIP_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - ATL PTD Skip Map"]
    #[inline(always)]
    pub fn atl_skip(&self) -> ATL_SKIP_R {
        ATL_SKIP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ATL PTD Skip Map"]
    #[inline(always)]
    #[must_use]
    pub fn atl_skip(&mut self) -> ATL_SKIP_W<0> {
        ATL_SKIP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ATL PTD Skip Map\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atl_skip](index.html) module"]
pub struct ATL_SKIP_SPEC;
impl crate::RegisterSpec for ATL_SKIP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [atl_skip::R](R) reader structure"]
impl crate::Readable for ATL_SKIP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atl_skip::W](W) writer structure"]
impl crate::Writable for ATL_SKIP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ATL_SKIP to value 0"]
impl crate::Resettable for ATL_SKIP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
