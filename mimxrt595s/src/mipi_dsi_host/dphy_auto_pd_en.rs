#[doc = "Register `DPHY_AUTO_PD_EN` reader"]
pub struct R(crate::R<DPHY_AUTO_PD_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPHY_AUTO_PD_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPHY_AUTO_PD_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPHY_AUTO_PD_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPHY_AUTO_PD_EN` writer"]
pub struct W(crate::W<DPHY_AUTO_PD_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPHY_AUTO_PD_EN_SPEC>;
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
impl From<crate::W<DPHY_AUTO_PD_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPHY_AUTO_PD_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dphy_auto_pd_en` reader - DPHY AUTO_PD_EN input, see DPHY datasheet"]
pub type DPHY_AUTO_PD_EN_R = crate::BitReader<bool>;
#[doc = "Field `dphy_auto_pd_en` writer - DPHY AUTO_PD_EN input, see DPHY datasheet"]
pub type DPHY_AUTO_PD_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DPHY_AUTO_PD_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DPHY AUTO_PD_EN input, see DPHY datasheet"]
    #[inline(always)]
    pub fn dphy_auto_pd_en(&self) -> DPHY_AUTO_PD_EN_R {
        DPHY_AUTO_PD_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DPHY AUTO_PD_EN input, see DPHY datasheet"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_auto_pd_en(&mut self) -> DPHY_AUTO_PD_EN_W<0> {
        DPHY_AUTO_PD_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dphy_auto_pd_en](index.html) module"]
pub struct DPHY_AUTO_PD_EN_SPEC;
impl crate::RegisterSpec for DPHY_AUTO_PD_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dphy_auto_pd_en::R](R) reader structure"]
impl crate::Readable for DPHY_AUTO_PD_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dphy_auto_pd_en::W](W) writer structure"]
impl crate::Writable for DPHY_AUTO_PD_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPHY_AUTO_PD_EN to value 0"]
impl crate::Resettable for DPHY_AUTO_PD_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
