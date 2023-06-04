#[doc = "Register `RSTGT_W` writer"]
pub struct W(crate::W<RSTGT_RSTGT_W_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTGT_RSTGT_W_SPEC>;
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
impl From<crate::W<RSTGT_RSTGT_W_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTGT_RSTGT_W_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSTGTN` writer - Reset Gate Number"]
pub type RSTGTN_W<'a, const O: u8> = crate::FieldWriter<'a, u16, RSTGT_RSTGT_W_SPEC, u8, u8, 8, O>;
#[doc = "Field `RSTGDP` writer - Reset Gate Data Pattern"]
pub type RSTGDP_W<'a, const O: u8> = crate::FieldWriter<'a, u16, RSTGT_RSTGT_W_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Reset Gate Number"]
    #[inline(always)]
    #[must_use]
    pub fn rstgtn(&mut self) -> RSTGTN_W<0> {
        RSTGTN_W::new(self)
    }
    #[doc = "Bits 8:15 - Reset Gate Data Pattern"]
    #[inline(always)]
    #[must_use]
    pub fn rstgdp(&mut self) -> RSTGDP_W<8> {
        RSTGDP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Gate Write\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstgt_rstgt_w](index.html) module"]
pub struct RSTGT_RSTGT_W_SPEC;
impl crate::RegisterSpec for RSTGT_RSTGT_W_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [rstgt_rstgt_w::W](W) writer structure"]
impl crate::Writable for RSTGT_RSTGT_W_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSTGT_W to value 0"]
impl crate::Resettable for RSTGT_RSTGT_W_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
