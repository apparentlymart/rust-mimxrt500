#[doc = "Register `DbiIfReset0` writer"]
pub struct W(crate::W<DBI_IF_RESET0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBI_IF_RESET0_SPEC>;
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
impl From<crate::W<DBI_IF_RESET0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBI_IF_RESET0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBI_IF_LEVEL_RESET` writer - Reset DBI interface to idle state 1=RESET;"]
pub type DBI_IF_LEVEL_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DBI_IF_RESET0_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Reset DBI interface to idle state 1=RESET;"]
    #[inline(always)]
    #[must_use]
    pub fn dbi_if_level_reset(&mut self) -> DBI_IF_LEVEL_RESET_W<0> {
        DBI_IF_LEVEL_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset DBI Interface to Idle State\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbi_if_reset0](index.html) module"]
pub struct DBI_IF_RESET0_SPEC;
impl crate::RegisterSpec for DBI_IF_RESET0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dbi_if_reset0::W](W) writer structure"]
impl crate::Writable for DBI_IF_RESET0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DbiIfReset0 to value 0"]
impl crate::Resettable for DBI_IF_RESET0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
