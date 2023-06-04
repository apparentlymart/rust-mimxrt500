#[doc = "Register `USB1_VBUS_DET_STAT` reader"]
pub struct R(crate::R<USB1_VBUS_DET_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB1_VBUS_DET_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB1_VBUS_DET_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB1_VBUS_DET_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SESSEND` reader - Session End indicator"]
pub type SESSEND_R = crate::BitReader<SESSEND_A>;
#[doc = "Session End indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SESSEND_A {
    #[doc = "0: The VBUS voltage is above the Session Valid threshold"]
    ABOVE = 0,
    #[doc = "1: The VBUS voltage is below the Session Valid threshold"]
    BELOW = 1,
}
impl From<SESSEND_A> for bool {
    #[inline(always)]
    fn from(variant: SESSEND_A) -> Self {
        variant as u8 != 0
    }
}
impl SESSEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SESSEND_A {
        match self.bits {
            false => SESSEND_A::ABOVE,
            true => SESSEND_A::BELOW,
        }
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == SESSEND_A::ABOVE
    }
    #[doc = "Checks if the value of the field is `BELOW`"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == SESSEND_A::BELOW
    }
}
#[doc = "Field `BVALID` reader - B-Device Session Valid status"]
pub type BVALID_R = crate::BitReader<BVALID_A>;
#[doc = "B-Device Session Valid status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BVALID_A {
    #[doc = "0: The VBUS voltage is below the Session Valid threshold"]
    BELOW = 0,
    #[doc = "1: The VBUS voltage is above the Session Valid threshold"]
    ABOVE = 1,
}
impl From<BVALID_A> for bool {
    #[inline(always)]
    fn from(variant: BVALID_A) -> Self {
        variant as u8 != 0
    }
}
impl BVALID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BVALID_A {
        match self.bits {
            false => BVALID_A::BELOW,
            true => BVALID_A::ABOVE,
        }
    }
    #[doc = "Checks if the value of the field is `BELOW`"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == BVALID_A::BELOW
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == BVALID_A::ABOVE
    }
}
#[doc = "Field `AVALID` reader - A-Device Session Valid status"]
pub type AVALID_R = crate::BitReader<AVALID_A>;
#[doc = "A-Device Session Valid status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVALID_A {
    #[doc = "0: The VBUS voltage is below the Session Valid threshold"]
    BELOW = 0,
    #[doc = "1: The VBUS voltage is above the Session Valid threshold"]
    ABOVE = 1,
}
impl From<AVALID_A> for bool {
    #[inline(always)]
    fn from(variant: AVALID_A) -> Self {
        variant as u8 != 0
    }
}
impl AVALID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVALID_A {
        match self.bits {
            false => AVALID_A::BELOW,
            true => AVALID_A::ABOVE,
        }
    }
    #[doc = "Checks if the value of the field is `BELOW`"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == AVALID_A::BELOW
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == AVALID_A::ABOVE
    }
}
#[doc = "Field `VBUS_VALID` reader - VBUS voltage status"]
pub type VBUS_VALID_R = crate::BitReader<VBUS_VALID_A>;
#[doc = "VBUS voltage status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBUS_VALID_A {
    #[doc = "0: VBUS is below the comparator threshold"]
    BELOW = 0,
    #[doc = "1: VBUS is above the comparator threshold"]
    ABOVE = 1,
}
impl From<VBUS_VALID_A> for bool {
    #[inline(always)]
    fn from(variant: VBUS_VALID_A) -> Self {
        variant as u8 != 0
    }
}
impl VBUS_VALID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUS_VALID_A {
        match self.bits {
            false => VBUS_VALID_A::BELOW,
            true => VBUS_VALID_A::ABOVE,
        }
    }
    #[doc = "Checks if the value of the field is `BELOW`"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == VBUS_VALID_A::BELOW
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == VBUS_VALID_A::ABOVE
    }
}
#[doc = "Field `VBUS_VALID_3V` reader - VBUS_VALID_3V detector status"]
pub type VBUS_VALID_3V_R = crate::BitReader<VBUS_VALID_3V_A>;
#[doc = "VBUS_VALID_3V detector status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBUS_VALID_3V_A {
    #[doc = "0: VBUS voltage is below VBUS_VALID_3V threshold"]
    BELOW = 0,
    #[doc = "1: VBUS voltage is above VBUS_VALID_3V threshold"]
    ABOVE = 1,
}
impl From<VBUS_VALID_3V_A> for bool {
    #[inline(always)]
    fn from(variant: VBUS_VALID_3V_A) -> Self {
        variant as u8 != 0
    }
}
impl VBUS_VALID_3V_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUS_VALID_3V_A {
        match self.bits {
            false => VBUS_VALID_3V_A::BELOW,
            true => VBUS_VALID_3V_A::ABOVE,
        }
    }
    #[doc = "Checks if the value of the field is `BELOW`"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == VBUS_VALID_3V_A::BELOW
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == VBUS_VALID_3V_A::ABOVE
    }
}
impl R {
    #[doc = "Bit 0 - Session End indicator"]
    #[inline(always)]
    pub fn sessend(&self) -> SESSEND_R {
        SESSEND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - B-Device Session Valid status"]
    #[inline(always)]
    pub fn bvalid(&self) -> BVALID_R {
        BVALID_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A-Device Session Valid status"]
    #[inline(always)]
    pub fn avalid(&self) -> AVALID_R {
        AVALID_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VBUS voltage status"]
    #[inline(always)]
    pub fn vbus_valid(&self) -> VBUS_VALID_R {
        VBUS_VALID_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VBUS_VALID_3V detector status"]
    #[inline(always)]
    pub fn vbus_valid_3v(&self) -> VBUS_VALID_3V_R {
        VBUS_VALID_3V_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "VBUS Detect Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_vbus_det_stat](index.html) module"]
pub struct USB1_VBUS_DET_STAT_SPEC;
impl crate::RegisterSpec for USB1_VBUS_DET_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb1_vbus_det_stat::R](R) reader structure"]
impl crate::Readable for USB1_VBUS_DET_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USB1_VBUS_DET_STAT to value 0"]
impl crate::Resettable for USB1_VBUS_DET_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
