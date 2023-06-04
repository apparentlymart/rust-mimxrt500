#[doc = "Register `TRIM_OVERRIDE_EN` reader"]
pub struct R(crate::R<TRIM_OVERRIDE_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIM_OVERRIDE_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIM_OVERRIDE_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIM_OVERRIDE_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIM_OVERRIDE_EN` writer"]
pub struct W(crate::W<TRIM_OVERRIDE_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIM_OVERRIDE_EN_SPEC>;
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
impl From<crate::W<TRIM_OVERRIDE_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIM_OVERRIDE_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV_SEL_OVERRIDE` reader - DIV_SEL_OVERRIDE"]
pub type DIV_SEL_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `DIV_SEL_OVERRIDE` writer - DIV_SEL_OVERRIDE"]
pub type DIV_SEL_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TRIM_OVERRIDE_EN_SPEC, bool, O>;
#[doc = "Field `ENV_TAIL_ADJ_VD_OVERRIDE` reader - ENV_TAIL_ADJ_VD_OVERRIDE"]
pub type ENV_TAIL_ADJ_VD_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `ENV_TAIL_ADJ_VD_OVERRIDE` writer - ENV_TAIL_ADJ_VD_OVERRIDE"]
pub type ENV_TAIL_ADJ_VD_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TRIM_OVERRIDE_EN_SPEC, bool, O>;
#[doc = "Field `TX_D_CAL_OVERRIDE` reader - TX_D_CAL_OVERRIDE"]
pub type TX_D_CAL_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `TX_D_CAL_OVERRIDE` writer - TX_D_CAL_OVERRIDE"]
pub type TX_D_CAL_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TRIM_OVERRIDE_EN_SPEC, bool, O>;
#[doc = "Field `TX_CAL45DP_OVERRIDE` reader - TX_CAL45DP_OVERRIDE"]
pub type TX_CAL45DP_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `TX_CAL45DP_OVERRIDE` writer - TX_CAL45DP_OVERRIDE"]
pub type TX_CAL45DP_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TRIM_OVERRIDE_EN_SPEC, bool, O>;
#[doc = "Field `TX_CAL45DM_OVERRIDE` reader - TX_CAL45DM_OVERRIDE"]
pub type TX_CAL45DM_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `TX_CAL45DM_OVERRIDE` writer - TX_CAL45DM_OVERRIDE"]
pub type TX_CAL45DM_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TRIM_OVERRIDE_EN_SPEC, bool, O>;
#[doc = "Field `REFBIAS_VBGADJ_OVERRIDE` reader - Override enable for bandgap adjustment."]
pub type REFBIAS_VBGADJ_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `REFBIAS_VBGADJ_OVERRIDE` writer - Override enable for bandgap adjustment."]
pub type REFBIAS_VBGADJ_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TRIM_OVERRIDE_EN_SPEC, bool, O>;
#[doc = "Field `REFBIAS_TST_OVERRIDE` reader - Override enable for bias current control."]
pub type REFBIAS_TST_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `REFBIAS_TST_OVERRIDE` writer - Override enable for bias current control."]
pub type REFBIAS_TST_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TRIM_OVERRIDE_EN_SPEC, bool, O>;
#[doc = "Field `USB2_REFBIAS_VBGADJ` reader - Adjustment bits for bandgap"]
pub type USB2_REFBIAS_VBGADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB2_REFBIAS_TST` reader - Bias current control for usb2_phy and usb_PLL"]
pub type USB2_REFBIAS_TST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL_CTRL0_DIV_SEL` reader - Default value of PLL_DIV_SEL."]
pub type PLL_CTRL0_DIV_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_REG_ENV_TAIL_ADJ_VD` reader - Default value of ENV_TAIL_ADJ."]
pub type USB_REG_ENV_TAIL_ADJ_VD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBPHY_TX_D_CAL` reader - Default value of TX_D_CAL."]
pub type USBPHY_TX_D_CAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBPHY_TX_CAL45DP` reader - Default value of TX_CAL45DP."]
pub type USBPHY_TX_CAL45DP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBPHY_TX_CAL45DN` reader - Default value of TX_CAL45DM."]
pub type USBPHY_TX_CAL45DN_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - DIV_SEL_OVERRIDE"]
    #[inline(always)]
    pub fn div_sel_override(&self) -> DIV_SEL_OVERRIDE_R {
        DIV_SEL_OVERRIDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ENV_TAIL_ADJ_VD_OVERRIDE"]
    #[inline(always)]
    pub fn env_tail_adj_vd_override(&self) -> ENV_TAIL_ADJ_VD_OVERRIDE_R {
        ENV_TAIL_ADJ_VD_OVERRIDE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX_D_CAL_OVERRIDE"]
    #[inline(always)]
    pub fn tx_d_cal_override(&self) -> TX_D_CAL_OVERRIDE_R {
        TX_D_CAL_OVERRIDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX_CAL45DP_OVERRIDE"]
    #[inline(always)]
    pub fn tx_cal45dp_override(&self) -> TX_CAL45DP_OVERRIDE_R {
        TX_CAL45DP_OVERRIDE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX_CAL45DM_OVERRIDE"]
    #[inline(always)]
    pub fn tx_cal45dm_override(&self) -> TX_CAL45DM_OVERRIDE_R {
        TX_CAL45DM_OVERRIDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Override enable for bandgap adjustment."]
    #[inline(always)]
    pub fn refbias_vbgadj_override(&self) -> REFBIAS_VBGADJ_OVERRIDE_R {
        REFBIAS_VBGADJ_OVERRIDE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Override enable for bias current control."]
    #[inline(always)]
    pub fn refbias_tst_override(&self) -> REFBIAS_TST_OVERRIDE_R {
        REFBIAS_TST_OVERRIDE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Adjustment bits for bandgap"]
    #[inline(always)]
    pub fn usb2_refbias_vbgadj(&self) -> USB2_REFBIAS_VBGADJ_R {
        USB2_REFBIAS_VBGADJ_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:14 - Bias current control for usb2_phy and usb_PLL"]
    #[inline(always)]
    pub fn usb2_refbias_tst(&self) -> USB2_REFBIAS_TST_R {
        USB2_REFBIAS_TST_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:17 - Default value of PLL_DIV_SEL."]
    #[inline(always)]
    pub fn pll_ctrl0_div_sel(&self) -> PLL_CTRL0_DIV_SEL_R {
        PLL_CTRL0_DIV_SEL_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:19 - Default value of ENV_TAIL_ADJ."]
    #[inline(always)]
    pub fn usb_reg_env_tail_adj_vd(&self) -> USB_REG_ENV_TAIL_ADJ_VD_R {
        USB_REG_ENV_TAIL_ADJ_VD_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:23 - Default value of TX_D_CAL."]
    #[inline(always)]
    pub fn usbphy_tx_d_cal(&self) -> USBPHY_TX_D_CAL_R {
        USBPHY_TX_D_CAL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Default value of TX_CAL45DP."]
    #[inline(always)]
    pub fn usbphy_tx_cal45dp(&self) -> USBPHY_TX_CAL45DP_R {
        USBPHY_TX_CAL45DP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Default value of TX_CAL45DM."]
    #[inline(always)]
    pub fn usbphy_tx_cal45dn(&self) -> USBPHY_TX_CAL45DN_R {
        USBPHY_TX_CAL45DN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DIV_SEL_OVERRIDE"]
    #[inline(always)]
    #[must_use]
    pub fn div_sel_override(&mut self) -> DIV_SEL_OVERRIDE_W<0> {
        DIV_SEL_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 1 - ENV_TAIL_ADJ_VD_OVERRIDE"]
    #[inline(always)]
    #[must_use]
    pub fn env_tail_adj_vd_override(&mut self) -> ENV_TAIL_ADJ_VD_OVERRIDE_W<1> {
        ENV_TAIL_ADJ_VD_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 2 - TX_D_CAL_OVERRIDE"]
    #[inline(always)]
    #[must_use]
    pub fn tx_d_cal_override(&mut self) -> TX_D_CAL_OVERRIDE_W<2> {
        TX_D_CAL_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 3 - TX_CAL45DP_OVERRIDE"]
    #[inline(always)]
    #[must_use]
    pub fn tx_cal45dp_override(&mut self) -> TX_CAL45DP_OVERRIDE_W<3> {
        TX_CAL45DP_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 4 - TX_CAL45DM_OVERRIDE"]
    #[inline(always)]
    #[must_use]
    pub fn tx_cal45dm_override(&mut self) -> TX_CAL45DM_OVERRIDE_W<4> {
        TX_CAL45DM_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 5 - Override enable for bandgap adjustment."]
    #[inline(always)]
    #[must_use]
    pub fn refbias_vbgadj_override(&mut self) -> REFBIAS_VBGADJ_OVERRIDE_W<5> {
        REFBIAS_VBGADJ_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 6 - Override enable for bias current control."]
    #[inline(always)]
    #[must_use]
    pub fn refbias_tst_override(&mut self) -> REFBIAS_TST_OVERRIDE_W<6> {
        REFBIAS_TST_OVERRIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trim Override Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_override_en](index.html) module"]
pub struct TRIM_OVERRIDE_EN_SPEC;
impl crate::RegisterSpec for TRIM_OVERRIDE_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trim_override_en::R](R) reader structure"]
impl crate::Readable for TRIM_OVERRIDE_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trim_override_en::W](W) writer structure"]
impl crate::Writable for TRIM_OVERRIDE_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIM_OVERRIDE_EN to value 0x7f"]
impl crate::Resettable for TRIM_OVERRIDE_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
