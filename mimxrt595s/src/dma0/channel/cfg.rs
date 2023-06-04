#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERIPHREQEN` reader - Peripheral request Enable."]
pub type PERIPHREQEN_R = crate::BitReader<PERIPHREQEN_A>;
#[doc = "Peripheral request Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERIPHREQEN_A {
    #[doc = "0: Peripheral DMA requests disabled."]
    DISABLED = 0,
    #[doc = "1: Peripheral DMA requests enabled."]
    ENABLED = 1,
}
impl From<PERIPHREQEN_A> for bool {
    #[inline(always)]
    fn from(variant: PERIPHREQEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PERIPHREQEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERIPHREQEN_A {
        match self.bits {
            false => PERIPHREQEN_A::DISABLED,
            true => PERIPHREQEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PERIPHREQEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PERIPHREQEN_A::ENABLED
    }
}
#[doc = "Field `PERIPHREQEN` writer - Peripheral request Enable."]
pub type PERIPHREQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, PERIPHREQEN_A, O>;
impl<'a, const O: u8> PERIPHREQEN_W<'a, O> {
    #[doc = "Peripheral DMA requests disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PERIPHREQEN_A::DISABLED)
    }
    #[doc = "Peripheral DMA requests enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PERIPHREQEN_A::ENABLED)
    }
}
#[doc = "Field `HWTRIGEN` reader - Hardware Triggering Enable for channel."]
pub type HWTRIGEN_R = crate::BitReader<HWTRIGEN_A>;
#[doc = "Hardware Triggering Enable for channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HWTRIGEN_A {
    #[doc = "0: Hardware triggering not used for channel."]
    DISABLED = 0,
    #[doc = "1: Hardware triggering used for channel."]
    ENABLED = 1,
}
impl From<HWTRIGEN_A> for bool {
    #[inline(always)]
    fn from(variant: HWTRIGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl HWTRIGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWTRIGEN_A {
        match self.bits {
            false => HWTRIGEN_A::DISABLED,
            true => HWTRIGEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HWTRIGEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HWTRIGEN_A::ENABLED
    }
}
#[doc = "Field `HWTRIGEN` writer - Hardware Triggering Enable for channel."]
pub type HWTRIGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, HWTRIGEN_A, O>;
impl<'a, const O: u8> HWTRIGEN_W<'a, O> {
    #[doc = "Hardware triggering not used for channel."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HWTRIGEN_A::DISABLED)
    }
    #[doc = "Hardware triggering used for channel."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HWTRIGEN_A::ENABLED)
    }
}
#[doc = "Field `TRIGPOL` reader - Trigger Polarity."]
pub type TRIGPOL_R = crate::BitReader<TRIGPOL_A>;
#[doc = "Trigger Polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIGPOL_A {
    #[doc = "0: Active low - falling edge. Hardware trigger is active low or falling edge triggered, based on TRIGTYPE."]
    ACTIVE_LOW_FALLING = 0,
    #[doc = "1: Active high - rising edge. Hardware trigger is active high or rising edge triggered, based on TRIGTYPE."]
    ACTIVE_HIGH_RISING = 1,
}
impl From<TRIGPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIGPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGPOL_A {
        match self.bits {
            false => TRIGPOL_A::ACTIVE_LOW_FALLING,
            true => TRIGPOL_A::ACTIVE_HIGH_RISING,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW_FALLING`"]
    #[inline(always)]
    pub fn is_active_low_falling(&self) -> bool {
        *self == TRIGPOL_A::ACTIVE_LOW_FALLING
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH_RISING`"]
    #[inline(always)]
    pub fn is_active_high_rising(&self) -> bool {
        *self == TRIGPOL_A::ACTIVE_HIGH_RISING
    }
}
#[doc = "Field `TRIGPOL` writer - Trigger Polarity."]
pub type TRIGPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, TRIGPOL_A, O>;
impl<'a, const O: u8> TRIGPOL_W<'a, O> {
    #[doc = "Active low - falling edge. Hardware trigger is active low or falling edge triggered, based on TRIGTYPE."]
    #[inline(always)]
    pub fn active_low_falling(self) -> &'a mut W {
        self.variant(TRIGPOL_A::ACTIVE_LOW_FALLING)
    }
    #[doc = "Active high - rising edge. Hardware trigger is active high or rising edge triggered, based on TRIGTYPE."]
    #[inline(always)]
    pub fn active_high_rising(self) -> &'a mut W {
        self.variant(TRIGPOL_A::ACTIVE_HIGH_RISING)
    }
}
#[doc = "Field `TRIGTYPE` reader - Trigger Type."]
pub type TRIGTYPE_R = crate::BitReader<TRIGTYPE_A>;
#[doc = "Trigger Type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIGTYPE_A {
    #[doc = "0: Edge. Hardware trigger is edge triggered. Transfers will be initiated and completed, as specified for a single trigger."]
    EDGE = 0,
    #[doc = "1: Level."]
    LEVEL = 1,
}
impl From<TRIGTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGTYPE_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIGTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGTYPE_A {
        match self.bits {
            false => TRIGTYPE_A::EDGE,
            true => TRIGTYPE_A::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == TRIGTYPE_A::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == TRIGTYPE_A::LEVEL
    }
}
#[doc = "Field `TRIGTYPE` writer - Trigger Type."]
pub type TRIGTYPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, TRIGTYPE_A, O>;
impl<'a, const O: u8> TRIGTYPE_W<'a, O> {
    #[doc = "Edge. Hardware trigger is edge triggered. Transfers will be initiated and completed, as specified for a single trigger."]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(TRIGTYPE_A::EDGE)
    }
    #[doc = "Level."]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(TRIGTYPE_A::LEVEL)
    }
}
#[doc = "Field `TRIGBURST` reader - Trigger Burst."]
pub type TRIGBURST_R = crate::BitReader<TRIGBURST_A>;
#[doc = "Trigger Burst.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIGBURST_A {
    #[doc = "0: Single transfer."]
    SINGLE = 0,
    #[doc = "1: Burst transfer."]
    BURST = 1,
}
impl From<TRIGBURST_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGBURST_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIGBURST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGBURST_A {
        match self.bits {
            false => TRIGBURST_A::SINGLE,
            true => TRIGBURST_A::BURST,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == TRIGBURST_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `BURST`"]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == TRIGBURST_A::BURST
    }
}
#[doc = "Field `TRIGBURST` writer - Trigger Burst."]
pub type TRIGBURST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, TRIGBURST_A, O>;
impl<'a, const O: u8> TRIGBURST_W<'a, O> {
    #[doc = "Single transfer."]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(TRIGBURST_A::SINGLE)
    }
    #[doc = "Burst transfer."]
    #[inline(always)]
    pub fn burst(self) -> &'a mut W {
        self.variant(TRIGBURST_A::BURST)
    }
}
#[doc = "Field `BURSTPOWER` reader - Burst Power."]
pub type BURSTPOWER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BURSTPOWER` writer - Burst Power."]
pub type BURSTPOWER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `SRCBURSTWRAP` reader - Source Burst Wrap."]
pub type SRCBURSTWRAP_R = crate::BitReader<SRCBURSTWRAP_A>;
#[doc = "Source Burst Wrap.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRCBURSTWRAP_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<SRCBURSTWRAP_A> for bool {
    #[inline(always)]
    fn from(variant: SRCBURSTWRAP_A) -> Self {
        variant as u8 != 0
    }
}
impl SRCBURSTWRAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRCBURSTWRAP_A {
        match self.bits {
            false => SRCBURSTWRAP_A::DISABLED,
            true => SRCBURSTWRAP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRCBURSTWRAP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRCBURSTWRAP_A::ENABLED
    }
}
#[doc = "Field `SRCBURSTWRAP` writer - Source Burst Wrap."]
pub type SRCBURSTWRAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, SRCBURSTWRAP_A, O>;
impl<'a, const O: u8> SRCBURSTWRAP_W<'a, O> {
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SRCBURSTWRAP_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SRCBURSTWRAP_A::ENABLED)
    }
}
#[doc = "Field `DSTBURSTWRAP` reader - Destination Burst Wrap."]
pub type DSTBURSTWRAP_R = crate::BitReader<DSTBURSTWRAP_A>;
#[doc = "Destination Burst Wrap.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSTBURSTWRAP_A {
    #[doc = "0: Disabled. Destination burst wrapping is not enabled for this DMA channel."]
    DISABLED = 0,
    #[doc = "1: Enabled. Destination burst wrapping is enabled for this DMA channel."]
    ENABLED = 1,
}
impl From<DSTBURSTWRAP_A> for bool {
    #[inline(always)]
    fn from(variant: DSTBURSTWRAP_A) -> Self {
        variant as u8 != 0
    }
}
impl DSTBURSTWRAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSTBURSTWRAP_A {
        match self.bits {
            false => DSTBURSTWRAP_A::DISABLED,
            true => DSTBURSTWRAP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DSTBURSTWRAP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DSTBURSTWRAP_A::ENABLED
    }
}
#[doc = "Field `DSTBURSTWRAP` writer - Destination Burst Wrap."]
pub type DSTBURSTWRAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, DSTBURSTWRAP_A, O>;
impl<'a, const O: u8> DSTBURSTWRAP_W<'a, O> {
    #[doc = "Disabled. Destination burst wrapping is not enabled for this DMA channel."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DSTBURSTWRAP_A::DISABLED)
    }
    #[doc = "Enabled. Destination burst wrapping is enabled for this DMA channel."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DSTBURSTWRAP_A::ENABLED)
    }
}
#[doc = "Field `CHPRIORITY` reader - Priority of channel when multiple DMA requests are pending."]
pub type CHPRIORITY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHPRIORITY` writer - Priority of channel when multiple DMA requests are pending."]
pub type CHPRIORITY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Peripheral request Enable."]
    #[inline(always)]
    pub fn periphreqen(&self) -> PERIPHREQEN_R {
        PERIPHREQEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hardware Triggering Enable for channel."]
    #[inline(always)]
    pub fn hwtrigen(&self) -> HWTRIGEN_R {
        HWTRIGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Trigger Polarity."]
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Trigger Type."]
    #[inline(always)]
    pub fn trigtype(&self) -> TRIGTYPE_R {
        TRIGTYPE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger Burst."]
    #[inline(always)]
    pub fn trigburst(&self) -> TRIGBURST_R {
        TRIGBURST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Burst Power."]
    #[inline(always)]
    pub fn burstpower(&self) -> BURSTPOWER_R {
        BURSTPOWER_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Source Burst Wrap."]
    #[inline(always)]
    pub fn srcburstwrap(&self) -> SRCBURSTWRAP_R {
        SRCBURSTWRAP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Destination Burst Wrap."]
    #[inline(always)]
    pub fn dstburstwrap(&self) -> DSTBURSTWRAP_R {
        DSTBURSTWRAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Priority of channel when multiple DMA requests are pending."]
    #[inline(always)]
    pub fn chpriority(&self) -> CHPRIORITY_R {
        CHPRIORITY_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral request Enable."]
    #[inline(always)]
    #[must_use]
    pub fn periphreqen(&mut self) -> PERIPHREQEN_W<0> {
        PERIPHREQEN_W::new(self)
    }
    #[doc = "Bit 1 - Hardware Triggering Enable for channel."]
    #[inline(always)]
    #[must_use]
    pub fn hwtrigen(&mut self) -> HWTRIGEN_W<1> {
        HWTRIGEN_W::new(self)
    }
    #[doc = "Bit 4 - Trigger Polarity."]
    #[inline(always)]
    #[must_use]
    pub fn trigpol(&mut self) -> TRIGPOL_W<4> {
        TRIGPOL_W::new(self)
    }
    #[doc = "Bit 5 - Trigger Type."]
    #[inline(always)]
    #[must_use]
    pub fn trigtype(&mut self) -> TRIGTYPE_W<5> {
        TRIGTYPE_W::new(self)
    }
    #[doc = "Bit 6 - Trigger Burst."]
    #[inline(always)]
    #[must_use]
    pub fn trigburst(&mut self) -> TRIGBURST_W<6> {
        TRIGBURST_W::new(self)
    }
    #[doc = "Bits 8:11 - Burst Power."]
    #[inline(always)]
    #[must_use]
    pub fn burstpower(&mut self) -> BURSTPOWER_W<8> {
        BURSTPOWER_W::new(self)
    }
    #[doc = "Bit 14 - Source Burst Wrap."]
    #[inline(always)]
    #[must_use]
    pub fn srcburstwrap(&mut self) -> SRCBURSTWRAP_W<14> {
        SRCBURSTWRAP_W::new(self)
    }
    #[doc = "Bit 15 - Destination Burst Wrap."]
    #[inline(always)]
    #[must_use]
    pub fn dstburstwrap(&mut self) -> DSTBURSTWRAP_W<15> {
        DSTBURSTWRAP_W::new(self)
    }
    #[doc = "Bits 16:18 - Priority of channel when multiple DMA requests are pending."]
    #[inline(always)]
    #[must_use]
    pub fn chpriority(&mut self) -> CHPRIORITY_W<16> {
        CHPRIORITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register for DMA channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
