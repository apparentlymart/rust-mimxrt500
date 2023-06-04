#[doc = "Register `DPHY_RTERM_SEL` reader"]
pub struct R(crate::R<DPHY_RTERM_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPHY_RTERM_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPHY_RTERM_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPHY_RTERM_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPHY_RTERM_SEL` writer"]
pub struct W(crate::W<DPHY_RTERM_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPHY_RTERM_SEL_SPEC>;
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
impl From<crate::W<DPHY_RTERM_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPHY_RTERM_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dphy_rterm_sel` reader - DPHY RTERM_SEL input, see DPHY datasheet"]
pub type DPHY_RTERM_SEL_R = crate::BitReader<bool>;
#[doc = "Field `dphy_rterm_sel` writer - DPHY RTERM_SEL input, see DPHY datasheet"]
pub type DPHY_RTERM_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DPHY_RTERM_SEL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DPHY RTERM_SEL input, see DPHY datasheet"]
    #[inline(always)]
    pub fn dphy_rterm_sel(&self) -> DPHY_RTERM_SEL_R {
        DPHY_RTERM_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DPHY RTERM_SEL input, see DPHY datasheet"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_rterm_sel(&mut self) -> DPHY_RTERM_SEL_W<0> {
        DPHY_RTERM_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dphy_rterm_sel](index.html) module"]
pub struct DPHY_RTERM_SEL_SPEC;
impl crate::RegisterSpec for DPHY_RTERM_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dphy_rterm_sel::R](R) reader structure"]
impl crate::Readable for DPHY_RTERM_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dphy_rterm_sel::W](W) writer structure"]
impl crate::Writable for DPHY_RTERM_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPHY_RTERM_SEL to value 0"]
impl crate::Resettable for DPHY_RTERM_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
