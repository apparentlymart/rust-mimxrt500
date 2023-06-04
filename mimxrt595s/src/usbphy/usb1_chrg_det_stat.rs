#[doc = "Register `USB1_CHRG_DET_STAT` reader"]
pub struct R(crate::R<USB1_CHRG_DET_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB1_CHRG_DET_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB1_CHRG_DET_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB1_CHRG_DET_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PLUG_CONTACT` reader - Battery Charging Data Contact Detection phase output"]
pub type PLUG_CONTACT_R = crate::BitReader<PLUG_CONTACT_A>;
#[doc = "Battery Charging Data Contact Detection phase output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLUG_CONTACT_A {
    #[doc = "0: No USB cable attachment has been detected"]
    NO_ATTACH = 0,
    #[doc = "1: A USB cable attachment between the device and host has been detected"]
    ATTACH = 1,
}
impl From<PLUG_CONTACT_A> for bool {
    #[inline(always)]
    fn from(variant: PLUG_CONTACT_A) -> Self {
        variant as u8 != 0
    }
}
impl PLUG_CONTACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLUG_CONTACT_A {
        match self.bits {
            false => PLUG_CONTACT_A::NO_ATTACH,
            true => PLUG_CONTACT_A::ATTACH,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ATTACH`"]
    #[inline(always)]
    pub fn is_no_attach(&self) -> bool {
        *self == PLUG_CONTACT_A::NO_ATTACH
    }
    #[doc = "Checks if the value of the field is `ATTACH`"]
    #[inline(always)]
    pub fn is_attach(&self) -> bool {
        *self == PLUG_CONTACT_A::ATTACH
    }
}
#[doc = "Field `CHRG_DETECTED` reader - Battery Charging Primary Detection phase output"]
pub type CHRG_DETECTED_R = crate::BitReader<CHRG_DETECTED_A>;
#[doc = "Battery Charging Primary Detection phase output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHRG_DETECTED_A {
    #[doc = "0: Standard Downstream Port (SDP) has been detected"]
    SDP = 0,
    #[doc = "1: Charging Port has been detected"]
    CHRG_PORT = 1,
}
impl From<CHRG_DETECTED_A> for bool {
    #[inline(always)]
    fn from(variant: CHRG_DETECTED_A) -> Self {
        variant as u8 != 0
    }
}
impl CHRG_DETECTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHRG_DETECTED_A {
        match self.bits {
            false => CHRG_DETECTED_A::SDP,
            true => CHRG_DETECTED_A::CHRG_PORT,
        }
    }
    #[doc = "Checks if the value of the field is `SDP`"]
    #[inline(always)]
    pub fn is_sdp(&self) -> bool {
        *self == CHRG_DETECTED_A::SDP
    }
    #[doc = "Checks if the value of the field is `CHRG_PORT`"]
    #[inline(always)]
    pub fn is_chrg_port(&self) -> bool {
        *self == CHRG_DETECTED_A::CHRG_PORT
    }
}
#[doc = "Field `DM_STATE` reader - DM_STATE"]
pub type DM_STATE_R = crate::BitReader<DM_STATE_A>;
#[doc = "DM_STATE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DM_STATE_A {
    #[doc = "0: DM pin voltage is < 0.8V"]
    BELOW_P8 = 0,
    #[doc = "1: DM pin voltage is > 2.0V"]
    ABOVE_2 = 1,
}
impl From<DM_STATE_A> for bool {
    #[inline(always)]
    fn from(variant: DM_STATE_A) -> Self {
        variant as u8 != 0
    }
}
impl DM_STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DM_STATE_A {
        match self.bits {
            false => DM_STATE_A::BELOW_P8,
            true => DM_STATE_A::ABOVE_2,
        }
    }
    #[doc = "Checks if the value of the field is `BELOW_P8`"]
    #[inline(always)]
    pub fn is_below_p8(&self) -> bool {
        *self == DM_STATE_A::BELOW_P8
    }
    #[doc = "Checks if the value of the field is `ABOVE_2`"]
    #[inline(always)]
    pub fn is_above_2(&self) -> bool {
        *self == DM_STATE_A::ABOVE_2
    }
}
#[doc = "Field `DP_STATE` reader - DP_STATE"]
pub type DP_STATE_R = crate::BitReader<DP_STATE_A>;
#[doc = "DP_STATE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DP_STATE_A {
    #[doc = "0: DP pin voltage is < 0.8V"]
    BELOW_P8 = 0,
    #[doc = "1: DP pin voltage is > 2.0V"]
    ABOVE_2 = 1,
}
impl From<DP_STATE_A> for bool {
    #[inline(always)]
    fn from(variant: DP_STATE_A) -> Self {
        variant as u8 != 0
    }
}
impl DP_STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DP_STATE_A {
        match self.bits {
            false => DP_STATE_A::BELOW_P8,
            true => DP_STATE_A::ABOVE_2,
        }
    }
    #[doc = "Checks if the value of the field is `BELOW_P8`"]
    #[inline(always)]
    pub fn is_below_p8(&self) -> bool {
        *self == DP_STATE_A::BELOW_P8
    }
    #[doc = "Checks if the value of the field is `ABOVE_2`"]
    #[inline(always)]
    pub fn is_above_2(&self) -> bool {
        *self == DP_STATE_A::ABOVE_2
    }
}
#[doc = "Field `SECDET_DCP` reader - Battery Charging Secondary Detection phase output"]
pub type SECDET_DCP_R = crate::BitReader<SECDET_DCP_A>;
#[doc = "Battery Charging Secondary Detection phase output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECDET_DCP_A {
    #[doc = "0: Charging Downstream Port (CDP) has been detected"]
    CDP = 0,
    #[doc = "1: Downstream Charging Port (DCP) has been detected"]
    DCP = 1,
}
impl From<SECDET_DCP_A> for bool {
    #[inline(always)]
    fn from(variant: SECDET_DCP_A) -> Self {
        variant as u8 != 0
    }
}
impl SECDET_DCP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECDET_DCP_A {
        match self.bits {
            false => SECDET_DCP_A::CDP,
            true => SECDET_DCP_A::DCP,
        }
    }
    #[doc = "Checks if the value of the field is `CDP`"]
    #[inline(always)]
    pub fn is_cdp(&self) -> bool {
        *self == SECDET_DCP_A::CDP
    }
    #[doc = "Checks if the value of the field is `DCP`"]
    #[inline(always)]
    pub fn is_dcp(&self) -> bool {
        *self == SECDET_DCP_A::DCP
    }
}
impl R {
    #[doc = "Bit 0 - Battery Charging Data Contact Detection phase output"]
    #[inline(always)]
    pub fn plug_contact(&self) -> PLUG_CONTACT_R {
        PLUG_CONTACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Battery Charging Primary Detection phase output"]
    #[inline(always)]
    pub fn chrg_detected(&self) -> CHRG_DETECTED_R {
        CHRG_DETECTED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DM_STATE"]
    #[inline(always)]
    pub fn dm_state(&self) -> DM_STATE_R {
        DM_STATE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DP_STATE"]
    #[inline(always)]
    pub fn dp_state(&self) -> DP_STATE_R {
        DP_STATE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Battery Charging Secondary Detection phase output"]
    #[inline(always)]
    pub fn secdet_dcp(&self) -> SECDET_DCP_R {
        SECDET_DCP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Charge Detect Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_chrg_det_stat](index.html) module"]
pub struct USB1_CHRG_DET_STAT_SPEC;
impl crate::RegisterSpec for USB1_CHRG_DET_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb1_chrg_det_stat::R](R) reader structure"]
impl crate::Readable for USB1_CHRG_DET_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USB1_CHRG_DET_STAT to value 0"]
impl crate::Resettable for USB1_CHRG_DET_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
