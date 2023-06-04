#[doc = "Register `DPHY_PD_DPHY` reader"]
pub struct R(crate::R<DPHY_PD_DPHY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPHY_PD_DPHY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPHY_PD_DPHY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPHY_PD_DPHY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPHY_PD_DPHY` writer"]
pub struct W(crate::W<DPHY_PD_DPHY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPHY_PD_DPHY_SPEC>;
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
impl From<crate::W<DPHY_PD_DPHY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPHY_PD_DPHY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dphy_pd_dphy` reader - DPHY PD_DPHY input control, see DPHY datasheet"]
pub type DPHY_PD_DPHY_R = crate::BitReader<bool>;
#[doc = "Field `dphy_pd_dphy` writer - DPHY PD_DPHY input control, see DPHY datasheet"]
pub type DPHY_PD_DPHY_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPHY_PD_DPHY_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DPHY PD_DPHY input control, see DPHY datasheet"]
    #[inline(always)]
    pub fn dphy_pd_dphy(&self) -> DPHY_PD_DPHY_R {
        DPHY_PD_DPHY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DPHY PD_DPHY input control, see DPHY datasheet"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_pd_dphy(&mut self) -> DPHY_PD_DPHY_W<0> {
        DPHY_PD_DPHY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dphy_pd_dphy](index.html) module"]
pub struct DPHY_PD_DPHY_SPEC;
impl crate::RegisterSpec for DPHY_PD_DPHY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dphy_pd_dphy::R](R) reader structure"]
impl crate::Readable for DPHY_PD_DPHY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dphy_pd_dphy::W](W) writer structure"]
impl crate::Writable for DPHY_PD_DPHY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPHY_PD_DPHY to value 0x01"]
impl crate::Resettable for DPHY_PD_DPHY_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
