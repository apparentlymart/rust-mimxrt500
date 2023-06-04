#[doc = "Register `TX` reader"]
pub struct R(crate::R<TX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX` writer"]
pub struct W(crate::W<TX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_SPEC>;
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
impl From<crate::W<TX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D_CAL` reader - Current Trim decode."]
pub type D_CAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `D_CAL` writer - Current Trim decode."]
pub type D_CAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_SPEC, u8, u8, 4, O>;
#[doc = "Field `TXCAL45DM` reader - DM series termination resistance trim."]
pub type TXCAL45DM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXCAL45DM` writer - DM series termination resistance trim."]
pub type TXCAL45DM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_SPEC, u8, u8, 4, O>;
#[doc = "Field `TXENCAL45DN` reader - DN series Resistance calibration."]
pub type TXENCAL45DN_R = crate::BitReader<bool>;
#[doc = "Field `TXENCAL45DN` writer - DN series Resistance calibration."]
pub type TXENCAL45DN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_SPEC, bool, O>;
#[doc = "Field `TXCAL45DP` reader - DP series termination resistance trim."]
pub type TXCAL45DP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXCAL45DP` writer - DP series termination resistance trim."]
pub type TXCAL45DP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_SPEC, u8, u8, 4, O>;
#[doc = "Field `TXENCAL45DP` reader - DP series resistance calibration"]
pub type TXENCAL45DP_R = crate::BitReader<bool>;
#[doc = "Field `TXENCAL45DP` writer - DP series resistance calibration"]
pub type TXENCAL45DP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Current Trim decode."]
    #[inline(always)]
    pub fn d_cal(&self) -> D_CAL_R {
        D_CAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DM series termination resistance trim."]
    #[inline(always)]
    pub fn txcal45dm(&self) -> TXCAL45DM_R {
        TXCAL45DM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - DN series Resistance calibration."]
    #[inline(always)]
    pub fn txencal45dn(&self) -> TXENCAL45DN_R {
        TXENCAL45DN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:19 - DP series termination resistance trim."]
    #[inline(always)]
    pub fn txcal45dp(&self) -> TXCAL45DP_R {
        TXCAL45DP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - DP series resistance calibration"]
    #[inline(always)]
    pub fn txencal45dp(&self) -> TXENCAL45DP_R {
        TXENCAL45DP_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Current Trim decode."]
    #[inline(always)]
    #[must_use]
    pub fn d_cal(&mut self) -> D_CAL_W<0> {
        D_CAL_W::new(self)
    }
    #[doc = "Bits 8:11 - DM series termination resistance trim."]
    #[inline(always)]
    #[must_use]
    pub fn txcal45dm(&mut self) -> TXCAL45DM_W<8> {
        TXCAL45DM_W::new(self)
    }
    #[doc = "Bit 13 - DN series Resistance calibration."]
    #[inline(always)]
    #[must_use]
    pub fn txencal45dn(&mut self) -> TXENCAL45DN_W<13> {
        TXENCAL45DN_W::new(self)
    }
    #[doc = "Bits 16:19 - DP series termination resistance trim."]
    #[inline(always)]
    #[must_use]
    pub fn txcal45dp(&mut self) -> TXCAL45DP_W<16> {
        TXCAL45DP_W::new(self)
    }
    #[doc = "Bit 21 - DP series resistance calibration"]
    #[inline(always)]
    #[must_use]
    pub fn txencal45dp(&mut self) -> TXENCAL45DP_W<21> {
        TXENCAL45DP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx](index.html) module"]
pub struct TX_SPEC;
impl crate::RegisterSpec for TX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx::R](R) reader structure"]
impl crate::Readable for TX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx::W](W) writer structure"]
impl crate::Writable for TX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX to value 0x0007_0707"]
impl crate::Resettable for TX_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_0707;
}
