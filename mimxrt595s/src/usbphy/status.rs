#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HOSTDISCONDETECT_STATUS` reader - Host disconnect status"]
pub type HOSTDISCONDETECT_STATUS_R = crate::BitReader<HOSTDISCONDETECT_STATUS_A>;
#[doc = "Host disconnect status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HOSTDISCONDETECT_STATUS_A {
    #[doc = "0: USB cable disconnect has not been detected at the local host"]
    DISABLE = 0,
    #[doc = "1: USB cable disconnect has been detected at the local host"]
    ENABLE = 1,
}
impl From<HOSTDISCONDETECT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: HOSTDISCONDETECT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl HOSTDISCONDETECT_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOSTDISCONDETECT_STATUS_A {
        match self.bits {
            false => HOSTDISCONDETECT_STATUS_A::DISABLE,
            true => HOSTDISCONDETECT_STATUS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HOSTDISCONDETECT_STATUS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HOSTDISCONDETECT_STATUS_A::ENABLE
    }
}
#[doc = "Field `DEVPLUGIN_STATUS` reader - Status indicator for non-standard resistive plugged-in detection."]
pub type DEVPLUGIN_STATUS_R = crate::BitReader<DEVPLUGIN_STATUS_A>;
#[doc = "Status indicator for non-standard resistive plugged-in detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEVPLUGIN_STATUS_A {
    #[doc = "0: No attachment to a USB host is detected"]
    DISABLE = 0,
    #[doc = "1: Cable attachment to a USB host is detected"]
    ENABLE = 1,
}
impl From<DEVPLUGIN_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: DEVPLUGIN_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl DEVPLUGIN_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEVPLUGIN_STATUS_A {
        match self.bits {
            false => DEVPLUGIN_STATUS_A::DISABLE,
            true => DEVPLUGIN_STATUS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DEVPLUGIN_STATUS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DEVPLUGIN_STATUS_A::ENABLE
    }
}
#[doc = "Field `RESUME_STATUS` reader - Resume status"]
pub type RESUME_STATUS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 3 - Host disconnect status"]
    #[inline(always)]
    pub fn hostdiscondetect_status(&self) -> HOSTDISCONDETECT_STATUS_R {
        HOSTDISCONDETECT_STATUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Status indicator for non-standard resistive plugged-in detection."]
    #[inline(always)]
    pub fn devplugin_status(&self) -> DEVPLUGIN_STATUS_R {
        DEVPLUGIN_STATUS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Resume status"]
    #[inline(always)]
    pub fn resume_status(&self) -> RESUME_STATUS_R {
        RESUME_STATUS_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
