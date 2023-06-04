#[doc = "Register `DPHY_M_PRG_HS_ZERO` reader"]
pub struct R(crate::R<DPHY_M_PRG_HS_ZERO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPHY_M_PRG_HS_ZERO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPHY_M_PRG_HS_ZERO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPHY_M_PRG_HS_ZERO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPHY_M_PRG_HS_ZERO` writer"]
pub struct W(crate::W<DPHY_M_PRG_HS_ZERO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPHY_M_PRG_HS_ZERO_SPEC>;
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
impl From<crate::W<DPHY_M_PRG_HS_ZERO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPHY_M_PRG_HS_ZERO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dphy_m_prg_hs_zero` reader - DPHY m_PRG_HS_ZERO input, see DPHY datasheet"]
pub type DPHY_M_PRG_HS_ZERO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dphy_m_prg_hs_zero` writer - DPHY m_PRG_HS_ZERO input, see DPHY datasheet"]
pub type DPHY_M_PRG_HS_ZERO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DPHY_M_PRG_HS_ZERO_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - DPHY m_PRG_HS_ZERO input, see DPHY datasheet"]
    #[inline(always)]
    pub fn dphy_m_prg_hs_zero(&self) -> DPHY_M_PRG_HS_ZERO_R {
        DPHY_M_PRG_HS_ZERO_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DPHY m_PRG_HS_ZERO input, see DPHY datasheet"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_m_prg_hs_zero(&mut self) -> DPHY_M_PRG_HS_ZERO_W<0> {
        DPHY_M_PRG_HS_ZERO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dphy_m_prg_hs_zero](index.html) module"]
pub struct DPHY_M_PRG_HS_ZERO_SPEC;
impl crate::RegisterSpec for DPHY_M_PRG_HS_ZERO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dphy_m_prg_hs_zero::R](R) reader structure"]
impl crate::Readable for DPHY_M_PRG_HS_ZERO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dphy_m_prg_hs_zero::W](W) writer structure"]
impl crate::Writable for DPHY_M_PRG_HS_ZERO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPHY_M_PRG_HS_ZERO to value 0"]
impl crate::Resettable for DPHY_M_PRG_HS_ZERO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
