#[doc = "Register `PanelTiming0` reader"]
pub struct R(crate::R<PANEL_TIMING0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PANEL_TIMING0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PANEL_TIMING0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PANEL_TIMING0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PanelTiming0` writer"]
pub struct W(crate::W<PANEL_TIMING0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PANEL_TIMING0_SPEC>;
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
impl From<crate::W<PANEL_TIMING0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PANEL_TIMING0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POWER_ENABLE` reader - Number of VSYNCsto wait after power has been enabled."]
pub type POWER_ENABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POWER_ENABLE` writer - Number of VSYNCsto wait after power has been enabled."]
pub type POWER_ENABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PANEL_TIMING0_SPEC, u8, u8, 4, O>;
#[doc = "Field `BACKLIGHT_ENABLE` reader - Number of VSYNCs to wait after backlight has been enabled."]
pub type BACKLIGHT_ENABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKLIGHT_ENABLE` writer - Number of VSYNCs to wait after backlight has been enabled."]
pub type BACKLIGHT_ENABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PANEL_TIMING0_SPEC, u8, u8, 4, O>;
#[doc = "Field `CLOCK_ENABLE` reader - Number of VSYNCs to wait after clock has been enabled."]
pub type CLOCK_ENABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLOCK_ENABLE` writer - Number of VSYNCs to wait after clock has been enabled."]
pub type CLOCK_ENABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PANEL_TIMING0_SPEC, u8, u8, 4, O>;
#[doc = "Field `DATA_ENABLE` reader - Number of VSYNCs to wait after data has been enabled."]
pub type DATA_ENABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_ENABLE` writer - Number of VSYNCs to wait after data has been enabled."]
pub type DATA_ENABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PANEL_TIMING0_SPEC, u8, u8, 4, O>;
#[doc = "Field `DATA_DISABLE` reader - Number of VSYNCs to wait after data has been disabled."]
pub type DATA_DISABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_DISABLE` writer - Number of VSYNCs to wait after data has been disabled."]
pub type DATA_DISABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PANEL_TIMING0_SPEC, u8, u8, 4, O>;
#[doc = "Field `CLOCK_DISABLE` reader - Number of VSYNCs to wait after clock has been disabled."]
pub type CLOCK_DISABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLOCK_DISABLE` writer - Number of VSYNCs to wait after clock has been disabled."]
pub type CLOCK_DISABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PANEL_TIMING0_SPEC, u8, u8, 4, O>;
#[doc = "Field `BACKLIGHT_DISABLE` reader - Number of VSYNCs to wait after backlight has been disabled."]
pub type BACKLIGHT_DISABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BACKLIGHT_DISABLE` writer - Number of VSYNCs to wait after backlight has been disabled."]
pub type BACKLIGHT_DISABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PANEL_TIMING0_SPEC, u8, u8, 4, O>;
#[doc = "Field `POWER_DISABLE` reader - Number of VSYNCs to wait after power has been disabled."]
pub type POWER_DISABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POWER_DISABLE` writer - Number of VSYNCs to wait after power has been disabled."]
pub type POWER_DISABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PANEL_TIMING0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Number of VSYNCsto wait after power has been enabled."]
    #[inline(always)]
    pub fn power_enable(&self) -> POWER_ENABLE_R {
        POWER_ENABLE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Number of VSYNCs to wait after backlight has been enabled."]
    #[inline(always)]
    pub fn backlight_enable(&self) -> BACKLIGHT_ENABLE_R {
        BACKLIGHT_ENABLE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Number of VSYNCs to wait after clock has been enabled."]
    #[inline(always)]
    pub fn clock_enable(&self) -> CLOCK_ENABLE_R {
        CLOCK_ENABLE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Number of VSYNCs to wait after data has been enabled."]
    #[inline(always)]
    pub fn data_enable(&self) -> DATA_ENABLE_R {
        DATA_ENABLE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Number of VSYNCs to wait after data has been disabled."]
    #[inline(always)]
    pub fn data_disable(&self) -> DATA_DISABLE_R {
        DATA_DISABLE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Number of VSYNCs to wait after clock has been disabled."]
    #[inline(always)]
    pub fn clock_disable(&self) -> CLOCK_DISABLE_R {
        CLOCK_DISABLE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Number of VSYNCs to wait after backlight has been disabled."]
    #[inline(always)]
    pub fn backlight_disable(&self) -> BACKLIGHT_DISABLE_R {
        BACKLIGHT_DISABLE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Number of VSYNCs to wait after power has been disabled."]
    #[inline(always)]
    pub fn power_disable(&self) -> POWER_DISABLE_R {
        POWER_DISABLE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of VSYNCsto wait after power has been enabled."]
    #[inline(always)]
    #[must_use]
    pub fn power_enable(&mut self) -> POWER_ENABLE_W<0> {
        POWER_ENABLE_W::new(self)
    }
    #[doc = "Bits 4:7 - Number of VSYNCs to wait after backlight has been enabled."]
    #[inline(always)]
    #[must_use]
    pub fn backlight_enable(&mut self) -> BACKLIGHT_ENABLE_W<4> {
        BACKLIGHT_ENABLE_W::new(self)
    }
    #[doc = "Bits 8:11 - Number of VSYNCs to wait after clock has been enabled."]
    #[inline(always)]
    #[must_use]
    pub fn clock_enable(&mut self) -> CLOCK_ENABLE_W<8> {
        CLOCK_ENABLE_W::new(self)
    }
    #[doc = "Bits 12:15 - Number of VSYNCs to wait after data has been enabled."]
    #[inline(always)]
    #[must_use]
    pub fn data_enable(&mut self) -> DATA_ENABLE_W<12> {
        DATA_ENABLE_W::new(self)
    }
    #[doc = "Bits 16:19 - Number of VSYNCs to wait after data has been disabled."]
    #[inline(always)]
    #[must_use]
    pub fn data_disable(&mut self) -> DATA_DISABLE_W<16> {
        DATA_DISABLE_W::new(self)
    }
    #[doc = "Bits 20:23 - Number of VSYNCs to wait after clock has been disabled."]
    #[inline(always)]
    #[must_use]
    pub fn clock_disable(&mut self) -> CLOCK_DISABLE_W<20> {
        CLOCK_DISABLE_W::new(self)
    }
    #[doc = "Bits 24:27 - Number of VSYNCs to wait after backlight has been disabled."]
    #[inline(always)]
    #[must_use]
    pub fn backlight_disable(&mut self) -> BACKLIGHT_DISABLE_W<24> {
        BACKLIGHT_DISABLE_W::new(self)
    }
    #[doc = "Bits 28:31 - Number of VSYNCs to wait after power has been disabled."]
    #[inline(always)]
    #[must_use]
    pub fn power_disable(&mut self) -> POWER_DISABLE_W<28> {
        POWER_DISABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing for Hardware Panel Sequencing\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [panel_timing0](index.html) module"]
pub struct PANEL_TIMING0_SPEC;
impl crate::RegisterSpec for PANEL_TIMING0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [panel_timing0::R](R) reader structure"]
impl crate::Readable for PANEL_TIMING0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [panel_timing0::W](W) writer structure"]
impl crate::Writable for PANEL_TIMING0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PanelTiming0 to value 0"]
impl crate::Resettable for PANEL_TIMING0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
