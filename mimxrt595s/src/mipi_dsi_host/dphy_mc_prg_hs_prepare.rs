#[doc = "Register `DPHY_MC_PRG_HS_PREPARE` reader"]
pub struct R(crate::R<DPHY_MC_PRG_HS_PREPARE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPHY_MC_PRG_HS_PREPARE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPHY_MC_PRG_HS_PREPARE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPHY_MC_PRG_HS_PREPARE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPHY_MC_PRG_HS_PREPARE` writer"]
pub struct W(crate::W<DPHY_MC_PRG_HS_PREPARE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPHY_MC_PRG_HS_PREPARE_SPEC>;
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
impl From<crate::W<DPHY_MC_PRG_HS_PREPARE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPHY_MC_PRG_HS_PREPARE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dphy_mc_prg_hs_prepare` reader - DPHY mc_PRG_HS_PREPARE input, see DPHY datasheet"]
pub type DPHY_MC_PRG_HS_PREPARE_R = crate::BitReader<bool>;
#[doc = "Field `dphy_mc_prg_hs_prepare` writer - DPHY mc_PRG_HS_PREPARE input, see DPHY datasheet"]
pub type DPHY_MC_PRG_HS_PREPARE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DPHY_MC_PRG_HS_PREPARE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DPHY mc_PRG_HS_PREPARE input, see DPHY datasheet"]
    #[inline(always)]
    pub fn dphy_mc_prg_hs_prepare(&self) -> DPHY_MC_PRG_HS_PREPARE_R {
        DPHY_MC_PRG_HS_PREPARE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DPHY mc_PRG_HS_PREPARE input, see DPHY datasheet"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_mc_prg_hs_prepare(&mut self) -> DPHY_MC_PRG_HS_PREPARE_W<0> {
        DPHY_MC_PRG_HS_PREPARE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dphy_mc_prg_hs_prepare](index.html) module"]
pub struct DPHY_MC_PRG_HS_PREPARE_SPEC;
impl crate::RegisterSpec for DPHY_MC_PRG_HS_PREPARE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dphy_mc_prg_hs_prepare::R](R) reader structure"]
impl crate::Readable for DPHY_MC_PRG_HS_PREPARE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dphy_mc_prg_hs_prepare::W](W) writer structure"]
impl crate::Writable for DPHY_MC_PRG_HS_PREPARE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPHY_MC_PRG_HS_PREPARE to value 0"]
impl crate::Resettable for DPHY_MC_PRG_HS_PREPARE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
