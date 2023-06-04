#[doc = "Register `PWRCTRL` reader"]
pub struct R(crate::R<PWRCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRCTRL` writer"]
pub struct W(crate::W<PWRCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRCTRL_SPEC>;
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
impl From<crate::W<PWRCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAM_ON` reader - RAM Power On"]
pub type RAM_ON_R = crate::BitReader<RAM_ON_A>;
#[doc = "RAM Power On\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM_ON_A {
    #[doc = "0: Power Off"]
    POWER_OFF = 0,
    #[doc = "1: Power On"]
    POWER_ON = 1,
}
impl From<RAM_ON_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_ON_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM_ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_ON_A {
        match self.bits {
            false => RAM_ON_A::POWER_OFF,
            true => RAM_ON_A::POWER_ON,
        }
    }
    #[doc = "Checks if the value of the field is `POWER_OFF`"]
    #[inline(always)]
    pub fn is_power_off(&self) -> bool {
        *self == RAM_ON_A::POWER_OFF
    }
    #[doc = "Checks if the value of the field is `POWER_ON`"]
    #[inline(always)]
    pub fn is_power_on(&self) -> bool {
        *self == RAM_ON_A::POWER_ON
    }
}
#[doc = "Field `RAM_ON` writer - RAM Power On"]
pub type RAM_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRCTRL_SPEC, RAM_ON_A, O>;
impl<'a, const O: u8> RAM_ON_W<'a, O> {
    #[doc = "Power Off"]
    #[inline(always)]
    pub fn power_off(self) -> &'a mut W {
        self.variant(RAM_ON_A::POWER_OFF)
    }
    #[doc = "Power On"]
    #[inline(always)]
    pub fn power_on(self) -> &'a mut W {
        self.variant(RAM_ON_A::POWER_ON)
    }
}
#[doc = "Field `CK_DIS` reader - PUF RAM Clock Disable"]
pub type CK_DIS_R = crate::BitReader<CK_DIS_A>;
#[doc = "PUF RAM Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CK_DIS_A {
    #[doc = "0: PUF RAM clock is disabled."]
    DISABLED = 0,
    #[doc = "1: PUF RAM clock is enabled."]
    ENABLED = 1,
}
impl From<CK_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: CK_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl CK_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CK_DIS_A {
        match self.bits {
            false => CK_DIS_A::DISABLED,
            true => CK_DIS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CK_DIS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CK_DIS_A::ENABLED
    }
}
#[doc = "Field `CK_DIS` writer - PUF RAM Clock Disable"]
pub type CK_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRCTRL_SPEC, CK_DIS_A, O>;
impl<'a, const O: u8> CK_DIS_W<'a, O> {
    #[doc = "PUF RAM clock is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CK_DIS_A::DISABLED)
    }
    #[doc = "PUF RAM clock is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CK_DIS_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - RAM Power On"]
    #[inline(always)]
    pub fn ram_on(&self) -> RAM_ON_R {
        RAM_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PUF RAM Clock Disable"]
    #[inline(always)]
    pub fn ck_dis(&self) -> CK_DIS_R {
        CK_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM Power On"]
    #[inline(always)]
    #[must_use]
    pub fn ram_on(&mut self) -> RAM_ON_W<0> {
        RAM_ON_W::new(self)
    }
    #[doc = "Bit 2 - PUF RAM Clock Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ck_dis(&mut self) -> CK_DIS_W<2> {
        CK_DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUF Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrctrl](index.html) module"]
pub struct PWRCTRL_SPEC;
impl crate::RegisterSpec for PWRCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrctrl::R](R) reader structure"]
impl crate::Readable for PWRCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrctrl::W](W) writer structure"]
impl crate::Writable for PWRCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRCTRL to value 0x01"]
impl crate::Resettable for PWRCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
