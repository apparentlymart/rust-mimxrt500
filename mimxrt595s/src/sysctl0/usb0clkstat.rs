#[doc = "Register `USB0CLKSTAT` reader"]
pub struct R(crate::R<USB0CLKSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB0CLKSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB0CLKSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB0CLKSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DEV_NEED_CLKST` reader - USB0 Device USB0_NEEDCLK signal status"]
pub type DEV_NEED_CLKST_R = crate::BitReader<DEV_NEED_CLKST_A>;
#[doc = "USB0 Device USB0_NEEDCLK signal status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEV_NEED_CLKST_A {
    #[doc = "0: Low"]
    _0 = 0,
    #[doc = "1: High"]
    _1 = 1,
}
impl From<DEV_NEED_CLKST_A> for bool {
    #[inline(always)]
    fn from(variant: DEV_NEED_CLKST_A) -> Self {
        variant as u8 != 0
    }
}
impl DEV_NEED_CLKST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEV_NEED_CLKST_A {
        match self.bits {
            false => DEV_NEED_CLKST_A::_0,
            true => DEV_NEED_CLKST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DEV_NEED_CLKST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DEV_NEED_CLKST_A::_1
    }
}
#[doc = "Field `HOST_NEED_CLKST` reader - USB0 Device Host USB0_NEEDCLK signal status"]
pub type HOST_NEED_CLKST_R = crate::BitReader<HOST_NEED_CLKST_A>;
#[doc = "USB0 Device Host USB0_NEEDCLK signal status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HOST_NEED_CLKST_A {
    #[doc = "0: Low"]
    HOST_NEED_CLKST_0 = 0,
    #[doc = "1: High"]
    HOST_NEED_CLKST_1 = 1,
}
impl From<HOST_NEED_CLKST_A> for bool {
    #[inline(always)]
    fn from(variant: HOST_NEED_CLKST_A) -> Self {
        variant as u8 != 0
    }
}
impl HOST_NEED_CLKST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOST_NEED_CLKST_A {
        match self.bits {
            false => HOST_NEED_CLKST_A::HOST_NEED_CLKST_0,
            true => HOST_NEED_CLKST_A::HOST_NEED_CLKST_1,
        }
    }
    #[doc = "Checks if the value of the field is `HOST_NEED_CLKST_0`"]
    #[inline(always)]
    pub fn is_host_need_clkst_0(&self) -> bool {
        *self == HOST_NEED_CLKST_A::HOST_NEED_CLKST_0
    }
    #[doc = "Checks if the value of the field is `HOST_NEED_CLKST_1`"]
    #[inline(always)]
    pub fn is_host_need_clkst_1(&self) -> bool {
        *self == HOST_NEED_CLKST_A::HOST_NEED_CLKST_1
    }
}
impl R {
    #[doc = "Bit 0 - USB0 Device USB0_NEEDCLK signal status"]
    #[inline(always)]
    pub fn dev_need_clkst(&self) -> DEV_NEED_CLKST_R {
        DEV_NEED_CLKST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB0 Device Host USB0_NEEDCLK signal status"]
    #[inline(always)]
    pub fn host_need_clkst(&self) -> HOST_NEED_CLKST_R {
        HOST_NEED_CLKST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "USB Clock Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb0clkstat](index.html) module"]
pub struct USB0CLKSTAT_SPEC;
impl crate::RegisterSpec for USB0CLKSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb0clkstat::R](R) reader structure"]
impl crate::Readable for USB0CLKSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USB0CLKSTAT to value 0x03"]
impl crate::Resettable for USB0CLKSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
