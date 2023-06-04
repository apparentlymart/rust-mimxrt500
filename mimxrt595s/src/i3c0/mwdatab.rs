#[doc = "Register `MWDATAB` writer"]
pub struct W(crate::W<MWDATAB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MWDATAB_SPEC>;
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
impl From<crate::W<MWDATAB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MWDATAB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` writer - Data byte"]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MWDATAB_SPEC, u8, u8, 8, O>;
#[doc = "Field `END` writer - End of message"]
pub type END_W<'a, const O: u8> = crate::BitWriter<'a, u32, MWDATAB_SPEC, bool, O>;
#[doc = "Field `END_ALSO` writer - End of message also"]
pub type END_ALSO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MWDATAB_SPEC, bool, O>;
impl W {
    #[doc = "Bits 0:7 - Data byte"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<0> {
        VALUE_W::new(self)
    }
    #[doc = "Bit 8 - End of message"]
    #[inline(always)]
    #[must_use]
    pub fn end(&mut self) -> END_W<8> {
        END_W::new(self)
    }
    #[doc = "Bit 16 - End of message also"]
    #[inline(always)]
    #[must_use]
    pub fn end_also(&mut self) -> END_ALSO_W<16> {
        END_ALSO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Write Data Byte Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mwdatab](index.html) module"]
pub struct MWDATAB_SPEC;
impl crate::RegisterSpec for MWDATAB_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [mwdatab::W](W) writer structure"]
impl crate::Writable for MWDATAB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MWDATAB to value 0"]
impl crate::Resettable for MWDATAB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
