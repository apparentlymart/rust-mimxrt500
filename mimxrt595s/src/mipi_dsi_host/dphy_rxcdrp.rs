#[doc = "Register `DPHY_RXCDRP` reader"]
pub struct R(crate::R<DPHY_RXCDRP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPHY_RXCDRP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPHY_RXCDRP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPHY_RXCDRP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPHY_RXCDRP` writer"]
pub struct W(crate::W<DPHY_RXCDRP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPHY_RXCDRP_SPEC>;
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
impl From<crate::W<DPHY_RXCDRP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPHY_RXCDRP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dphy_rxcdrp` reader - DPHY RXCDRP input, see DPHY datasheet"]
pub type DPHY_RXCDRP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dphy_rxcdrp` writer - DPHY RXCDRP input, see DPHY datasheet"]
pub type DPHY_RXCDRP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DPHY_RXCDRP_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - DPHY RXCDRP input, see DPHY datasheet"]
    #[inline(always)]
    pub fn dphy_rxcdrp(&self) -> DPHY_RXCDRP_R {
        DPHY_RXCDRP_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DPHY RXCDRP input, see DPHY datasheet"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_rxcdrp(&mut self) -> DPHY_RXCDRP_W<0> {
        DPHY_RXCDRP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dphy_rxcdrp](index.html) module"]
pub struct DPHY_RXCDRP_SPEC;
impl crate::RegisterSpec for DPHY_RXCDRP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dphy_rxcdrp::R](R) reader structure"]
impl crate::Readable for DPHY_RXCDRP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dphy_rxcdrp::W](W) writer structure"]
impl crate::Writable for DPHY_RXCDRP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPHY_RXCDRP to value 0"]
impl crate::Resettable for DPHY_RXCDRP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
