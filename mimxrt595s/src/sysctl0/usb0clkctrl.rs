#[doc = "Register `USB0CLKCTRL` reader"]
pub struct R(crate::R<USB0CLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB0CLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB0CLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB0CLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB0CLKCTRL` writer"]
pub struct W(crate::W<USB0CLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB0CLKCTRL_SPEC>;
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
impl From<crate::W<USB0CLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB0CLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AP_FS_DEV_CLK` reader - USB0 Device need clock signal control"]
pub type AP_FS_DEV_CLK_R = crate::BitReader<AP_FS_DEV_CLK_A>;
#[doc = "USB0 Device need clock signal control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AP_FS_DEV_CLK_A {
    #[doc = "0: Under hardware control"]
    AP_FS_DEV_CLK_0 = 0,
    #[doc = "1: Forced high"]
    AP_FS_DEV_CLK_1 = 1,
}
impl From<AP_FS_DEV_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: AP_FS_DEV_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl AP_FS_DEV_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AP_FS_DEV_CLK_A {
        match self.bits {
            false => AP_FS_DEV_CLK_A::AP_FS_DEV_CLK_0,
            true => AP_FS_DEV_CLK_A::AP_FS_DEV_CLK_1,
        }
    }
    #[doc = "Checks if the value of the field is `AP_FS_DEV_CLK_0`"]
    #[inline(always)]
    pub fn is_ap_fs_dev_clk_0(&self) -> bool {
        *self == AP_FS_DEV_CLK_A::AP_FS_DEV_CLK_0
    }
    #[doc = "Checks if the value of the field is `AP_FS_DEV_CLK_1`"]
    #[inline(always)]
    pub fn is_ap_fs_dev_clk_1(&self) -> bool {
        *self == AP_FS_DEV_CLK_A::AP_FS_DEV_CLK_1
    }
}
#[doc = "Field `AP_FS_DEV_CLK` writer - USB0 Device need clock signal control"]
pub type AP_FS_DEV_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB0CLKCTRL_SPEC, AP_FS_DEV_CLK_A, O>;
impl<'a, const O: u8> AP_FS_DEV_CLK_W<'a, O> {
    #[doc = "Under hardware control"]
    #[inline(always)]
    pub fn ap_fs_dev_clk_0(self) -> &'a mut W {
        self.variant(AP_FS_DEV_CLK_A::AP_FS_DEV_CLK_0)
    }
    #[doc = "Forced high"]
    #[inline(always)]
    pub fn ap_fs_dev_clk_1(self) -> &'a mut W {
        self.variant(AP_FS_DEV_CLK_A::AP_FS_DEV_CLK_1)
    }
}
#[doc = "Field `POL_FS_DEV_CLK` reader - USB0 Device need clock polarity for triggering the USB0 wake-up interrupt"]
pub type POL_FS_DEV_CLK_R = crate::BitReader<POL_FS_DEV_CLK_A>;
#[doc = "USB0 Device need clock polarity for triggering the USB0 wake-up interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_FS_DEV_CLK_A {
    #[doc = "0: Falling edge of device need_clock triggers wake-up"]
    POL_FS_DEV_CLK_0 = 0,
    #[doc = "1: Rising edge of device need_clock triggers wake-up"]
    POL_FS_DEV_CLK_1 = 1,
}
impl From<POL_FS_DEV_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: POL_FS_DEV_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_FS_DEV_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_FS_DEV_CLK_A {
        match self.bits {
            false => POL_FS_DEV_CLK_A::POL_FS_DEV_CLK_0,
            true => POL_FS_DEV_CLK_A::POL_FS_DEV_CLK_1,
        }
    }
    #[doc = "Checks if the value of the field is `POL_FS_DEV_CLK_0`"]
    #[inline(always)]
    pub fn is_pol_fs_dev_clk_0(&self) -> bool {
        *self == POL_FS_DEV_CLK_A::POL_FS_DEV_CLK_0
    }
    #[doc = "Checks if the value of the field is `POL_FS_DEV_CLK_1`"]
    #[inline(always)]
    pub fn is_pol_fs_dev_clk_1(&self) -> bool {
        *self == POL_FS_DEV_CLK_A::POL_FS_DEV_CLK_1
    }
}
#[doc = "Field `POL_FS_DEV_CLK` writer - USB0 Device need clock polarity for triggering the USB0 wake-up interrupt"]
pub type POL_FS_DEV_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB0CLKCTRL_SPEC, POL_FS_DEV_CLK_A, O>;
impl<'a, const O: u8> POL_FS_DEV_CLK_W<'a, O> {
    #[doc = "Falling edge of device need_clock triggers wake-up"]
    #[inline(always)]
    pub fn pol_fs_dev_clk_0(self) -> &'a mut W {
        self.variant(POL_FS_DEV_CLK_A::POL_FS_DEV_CLK_0)
    }
    #[doc = "Rising edge of device need_clock triggers wake-up"]
    #[inline(always)]
    pub fn pol_fs_dev_clk_1(self) -> &'a mut W {
        self.variant(POL_FS_DEV_CLK_A::POL_FS_DEV_CLK_1)
    }
}
#[doc = "Field `AP_FS_HOST_CLK` reader - USB0 Host need clock signal control"]
pub type AP_FS_HOST_CLK_R = crate::BitReader<AP_FS_HOST_CLK_A>;
#[doc = "USB0 Host need clock signal control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AP_FS_HOST_CLK_A {
    #[doc = "0: Under hardware control"]
    AP_FS_HOST_CLK_0 = 0,
    #[doc = "1: Forced high"]
    AP_FS_HOST_CLK_1 = 1,
}
impl From<AP_FS_HOST_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: AP_FS_HOST_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl AP_FS_HOST_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AP_FS_HOST_CLK_A {
        match self.bits {
            false => AP_FS_HOST_CLK_A::AP_FS_HOST_CLK_0,
            true => AP_FS_HOST_CLK_A::AP_FS_HOST_CLK_1,
        }
    }
    #[doc = "Checks if the value of the field is `AP_FS_HOST_CLK_0`"]
    #[inline(always)]
    pub fn is_ap_fs_host_clk_0(&self) -> bool {
        *self == AP_FS_HOST_CLK_A::AP_FS_HOST_CLK_0
    }
    #[doc = "Checks if the value of the field is `AP_FS_HOST_CLK_1`"]
    #[inline(always)]
    pub fn is_ap_fs_host_clk_1(&self) -> bool {
        *self == AP_FS_HOST_CLK_A::AP_FS_HOST_CLK_1
    }
}
#[doc = "Field `AP_FS_HOST_CLK` writer - USB0 Host need clock signal control"]
pub type AP_FS_HOST_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB0CLKCTRL_SPEC, AP_FS_HOST_CLK_A, O>;
impl<'a, const O: u8> AP_FS_HOST_CLK_W<'a, O> {
    #[doc = "Under hardware control"]
    #[inline(always)]
    pub fn ap_fs_host_clk_0(self) -> &'a mut W {
        self.variant(AP_FS_HOST_CLK_A::AP_FS_HOST_CLK_0)
    }
    #[doc = "Forced high"]
    #[inline(always)]
    pub fn ap_fs_host_clk_1(self) -> &'a mut W {
        self.variant(AP_FS_HOST_CLK_A::AP_FS_HOST_CLK_1)
    }
}
#[doc = "Field `POL_FS_HOST_CLK` reader - USB0 HOST need clock polarity for triggering the USB0 wake-up interrupt"]
pub type POL_FS_HOST_CLK_R = crate::BitReader<POL_FS_HOST_CLK_A>;
#[doc = "USB0 HOST need clock polarity for triggering the USB0 wake-up interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_FS_HOST_CLK_A {
    #[doc = "0: Falling edge of host need_clock triggers wake-up"]
    POL_FS_HOST_CLK_0 = 0,
    #[doc = "1: Rising edge of host need_clock triggers wake-up"]
    POL_FS_HOST_CLK_1 = 1,
}
impl From<POL_FS_HOST_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: POL_FS_HOST_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_FS_HOST_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_FS_HOST_CLK_A {
        match self.bits {
            false => POL_FS_HOST_CLK_A::POL_FS_HOST_CLK_0,
            true => POL_FS_HOST_CLK_A::POL_FS_HOST_CLK_1,
        }
    }
    #[doc = "Checks if the value of the field is `POL_FS_HOST_CLK_0`"]
    #[inline(always)]
    pub fn is_pol_fs_host_clk_0(&self) -> bool {
        *self == POL_FS_HOST_CLK_A::POL_FS_HOST_CLK_0
    }
    #[doc = "Checks if the value of the field is `POL_FS_HOST_CLK_1`"]
    #[inline(always)]
    pub fn is_pol_fs_host_clk_1(&self) -> bool {
        *self == POL_FS_HOST_CLK_A::POL_FS_HOST_CLK_1
    }
}
#[doc = "Field `POL_FS_HOST_CLK` writer - USB0 HOST need clock polarity for triggering the USB0 wake-up interrupt"]
pub type POL_FS_HOST_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB0CLKCTRL_SPEC, POL_FS_HOST_CLK_A, O>;
impl<'a, const O: u8> POL_FS_HOST_CLK_W<'a, O> {
    #[doc = "Falling edge of host need_clock triggers wake-up"]
    #[inline(always)]
    pub fn pol_fs_host_clk_0(self) -> &'a mut W {
        self.variant(POL_FS_HOST_CLK_A::POL_FS_HOST_CLK_0)
    }
    #[doc = "Rising edge of host need_clock triggers wake-up"]
    #[inline(always)]
    pub fn pol_fs_host_clk_1(self) -> &'a mut W {
        self.variant(POL_FS_HOST_CLK_A::POL_FS_HOST_CLK_1)
    }
}
#[doc = "Field `HS_DEV_WAKEUP_N` reader - External user wake-up signal for device mode"]
pub type HS_DEV_WAKEUP_N_R = crate::BitReader<HS_DEV_WAKEUP_N_A>;
#[doc = "External user wake-up signal for device mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HS_DEV_WAKEUP_N_A {
    #[doc = "0: Forces USB0 PHY to wake-up"]
    HS_DEV_WAKEUP_N_0 = 0,
    #[doc = "1: Normal USB0 PHY behavior"]
    HS_DEV_WAKEUP_N_1 = 1,
}
impl From<HS_DEV_WAKEUP_N_A> for bool {
    #[inline(always)]
    fn from(variant: HS_DEV_WAKEUP_N_A) -> Self {
        variant as u8 != 0
    }
}
impl HS_DEV_WAKEUP_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_DEV_WAKEUP_N_A {
        match self.bits {
            false => HS_DEV_WAKEUP_N_A::HS_DEV_WAKEUP_N_0,
            true => HS_DEV_WAKEUP_N_A::HS_DEV_WAKEUP_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `HS_DEV_WAKEUP_N_0`"]
    #[inline(always)]
    pub fn is_hs_dev_wakeup_n_0(&self) -> bool {
        *self == HS_DEV_WAKEUP_N_A::HS_DEV_WAKEUP_N_0
    }
    #[doc = "Checks if the value of the field is `HS_DEV_WAKEUP_N_1`"]
    #[inline(always)]
    pub fn is_hs_dev_wakeup_n_1(&self) -> bool {
        *self == HS_DEV_WAKEUP_N_A::HS_DEV_WAKEUP_N_1
    }
}
#[doc = "Field `HS_DEV_WAKEUP_N` writer - External user wake-up signal for device mode"]
pub type HS_DEV_WAKEUP_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB0CLKCTRL_SPEC, HS_DEV_WAKEUP_N_A, O>;
impl<'a, const O: u8> HS_DEV_WAKEUP_N_W<'a, O> {
    #[doc = "Forces USB0 PHY to wake-up"]
    #[inline(always)]
    pub fn hs_dev_wakeup_n_0(self) -> &'a mut W {
        self.variant(HS_DEV_WAKEUP_N_A::HS_DEV_WAKEUP_N_0)
    }
    #[doc = "Normal USB0 PHY behavior"]
    #[inline(always)]
    pub fn hs_dev_wakeup_n_1(self) -> &'a mut W {
        self.variant(HS_DEV_WAKEUP_N_A::HS_DEV_WAKEUP_N_1)
    }
}
impl R {
    #[doc = "Bit 0 - USB0 Device need clock signal control"]
    #[inline(always)]
    pub fn ap_fs_dev_clk(&self) -> AP_FS_DEV_CLK_R {
        AP_FS_DEV_CLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB0 Device need clock polarity for triggering the USB0 wake-up interrupt"]
    #[inline(always)]
    pub fn pol_fs_dev_clk(&self) -> POL_FS_DEV_CLK_R {
        POL_FS_DEV_CLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB0 Host need clock signal control"]
    #[inline(always)]
    pub fn ap_fs_host_clk(&self) -> AP_FS_HOST_CLK_R {
        AP_FS_HOST_CLK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB0 HOST need clock polarity for triggering the USB0 wake-up interrupt"]
    #[inline(always)]
    pub fn pol_fs_host_clk(&self) -> POL_FS_HOST_CLK_R {
        POL_FS_HOST_CLK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External user wake-up signal for device mode"]
    #[inline(always)]
    pub fn hs_dev_wakeup_n(&self) -> HS_DEV_WAKEUP_N_R {
        HS_DEV_WAKEUP_N_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB0 Device need clock signal control"]
    #[inline(always)]
    #[must_use]
    pub fn ap_fs_dev_clk(&mut self) -> AP_FS_DEV_CLK_W<0> {
        AP_FS_DEV_CLK_W::new(self)
    }
    #[doc = "Bit 1 - USB0 Device need clock polarity for triggering the USB0 wake-up interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pol_fs_dev_clk(&mut self) -> POL_FS_DEV_CLK_W<1> {
        POL_FS_DEV_CLK_W::new(self)
    }
    #[doc = "Bit 2 - USB0 Host need clock signal control"]
    #[inline(always)]
    #[must_use]
    pub fn ap_fs_host_clk(&mut self) -> AP_FS_HOST_CLK_W<2> {
        AP_FS_HOST_CLK_W::new(self)
    }
    #[doc = "Bit 3 - USB0 HOST need clock polarity for triggering the USB0 wake-up interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pol_fs_host_clk(&mut self) -> POL_FS_HOST_CLK_W<3> {
        POL_FS_HOST_CLK_W::new(self)
    }
    #[doc = "Bit 4 - External user wake-up signal for device mode"]
    #[inline(always)]
    #[must_use]
    pub fn hs_dev_wakeup_n(&mut self) -> HS_DEV_WAKEUP_N_W<4> {
        HS_DEV_WAKEUP_N_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb0clkctrl](index.html) module"]
pub struct USB0CLKCTRL_SPEC;
impl crate::RegisterSpec for USB0CLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb0clkctrl::R](R) reader structure"]
impl crate::Readable for USB0CLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb0clkctrl::W](W) writer structure"]
impl crate::Writable for USB0CLKCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB0CLKCTRL to value 0x10"]
impl crate::Resettable for USB0CLKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
