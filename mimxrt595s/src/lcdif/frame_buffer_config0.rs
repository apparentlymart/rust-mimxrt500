#[doc = "Register `FrameBufferConfig0` reader"]
pub struct R(crate::R<FRAME_BUFFER_CONFIG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAME_BUFFER_CONFIG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAME_BUFFER_CONFIG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAME_BUFFER_CONFIG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FrameBufferConfig0` writer"]
pub struct W(crate::W<FRAME_BUFFER_CONFIG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAME_BUFFER_CONFIG0_SPEC>;
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
impl From<crate::W<FRAME_BUFFER_CONFIG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAME_BUFFER_CONFIG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORMAT` reader - The format of the frame buffer."]
pub type FORMAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FORMAT` writer - The format of the frame buffer."]
pub type FORMAT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRAME_BUFFER_CONFIG0_SPEC, u8, u8, 3, O>;
#[doc = "Field `MODE` reader - Mode of the frame buffer."]
pub type MODE_R = crate::BitReader<MODE_A>;
#[doc = "Mode of the frame buffer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_A {
    #[doc = "0: LINEAR"]
    DISABLE = 0,
    #[doc = "1: TILE4x4 INPUT"]
    ENABLE = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::DISABLE,
            true => MODE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MODE_A::ENABLE
    }
}
#[doc = "Field `MODE` writer - Mode of the frame buffer."]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRAME_BUFFER_CONFIG0_SPEC, MODE_A, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "LINEAR"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MODE_A::DISABLE)
    }
    #[doc = "TILE4x4 INPUT"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MODE_A::ENABLE)
    }
}
#[doc = "Field `OUTPUT` reader - Output"]
pub type OUTPUT_R = crate::BitReader<OUTPUT_A>;
#[doc = "Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTPUT_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<OUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: OUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl OUTPUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTPUT_A {
        match self.bits {
            false => OUTPUT_A::DISABLE,
            true => OUTPUT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OUTPUT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OUTPUT_A::ENABLE
    }
}
#[doc = "Field `OUTPUT` writer - Output"]
pub type OUTPUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FRAME_BUFFER_CONFIG0_SPEC, OUTPUT_A, O>;
impl<'a, const O: u8> OUTPUT_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OUTPUT_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OUTPUT_A::ENABLE)
    }
}
#[doc = "Field `SWITCHPANEL` reader - Switch Panel"]
pub type SWITCHPANEL_R = crate::BitReader<SWITCHPANEL_A>;
#[doc = "Switch Panel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWITCHPANEL_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<SWITCHPANEL_A> for bool {
    #[inline(always)]
    fn from(variant: SWITCHPANEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SWITCHPANEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWITCHPANEL_A {
        match self.bits {
            false => SWITCHPANEL_A::DISABLE,
            true => SWITCHPANEL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SWITCHPANEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SWITCHPANEL_A::ENABLE
    }
}
#[doc = "Field `SWITCHPANEL` writer - Switch Panel"]
pub type SWITCHPANEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FRAME_BUFFER_CONFIG0_SPEC, SWITCHPANEL_A, O>;
impl<'a, const O: u8> SWITCHPANEL_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SWITCHPANEL_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SWITCHPANEL_A::ENABLE)
    }
}
#[doc = "Field `GAMMA` reader - Gamma"]
pub type GAMMA_R = crate::BitReader<GAMMA_A>;
#[doc = "Gamma\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GAMMA_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<GAMMA_A> for bool {
    #[inline(always)]
    fn from(variant: GAMMA_A) -> Self {
        variant as u8 != 0
    }
}
impl GAMMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAMMA_A {
        match self.bits {
            false => GAMMA_A::DISABLE,
            true => GAMMA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GAMMA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GAMMA_A::ENABLE
    }
}
#[doc = "Field `GAMMA` writer - Gamma"]
pub type GAMMA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FRAME_BUFFER_CONFIG0_SPEC, GAMMA_A, O>;
impl<'a, const O: u8> GAMMA_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GAMMA_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GAMMA_A::ENABLE)
    }
}
#[doc = "Field `VALID` reader - Valid"]
pub type VALID_R = crate::BitReader<VALID_A>;
#[doc = "Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VALID_A {
    #[doc = "0: Working"]
    ENABLE = 0,
    #[doc = "1: Pending"]
    DISABLE = 1,
}
impl From<VALID_A> for bool {
    #[inline(always)]
    fn from(variant: VALID_A) -> Self {
        variant as u8 != 0
    }
}
impl VALID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VALID_A {
        match self.bits {
            false => VALID_A::ENABLE,
            true => VALID_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VALID_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VALID_A::DISABLE
    }
}
#[doc = "Field `VALID` writer - Valid"]
pub type VALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FRAME_BUFFER_CONFIG0_SPEC, VALID_A, O>;
impl<'a, const O: u8> VALID_W<'a, O> {
    #[doc = "Working"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VALID_A::ENABLE)
    }
    #[doc = "Pending"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VALID_A::DISABLE)
    }
}
#[doc = "Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_AW {
    #[doc = "0: For DBI, this field should be = 0."]
    DISABLE = 0,
    #[doc = "1: Enable DPI Timing, start a DPI transfer."]
    ENABLE = 1,
}
impl From<RESET_AW> for bool {
    #[inline(always)]
    fn from(variant: RESET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET` writer - Reset"]
pub type RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FRAME_BUFFER_CONFIG0_SPEC, RESET_AW, O>;
impl<'a, const O: u8> RESET_W<'a, O> {
    #[doc = "For DBI, this field should be = 0."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RESET_AW::DISABLE)
    }
    #[doc = "Enable DPI Timing, start a DPI transfer."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RESET_AW::ENABLE)
    }
}
#[doc = "Field `UNDERFLOW` reader - Underflow"]
pub type UNDERFLOW_R = crate::BitReader<UNDERFLOW_A>;
#[doc = "Underflow\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNDERFLOW_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<UNDERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: UNDERFLOW_A) -> Self {
        variant as u8 != 0
    }
}
impl UNDERFLOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNDERFLOW_A {
        match self.bits {
            false => UNDERFLOW_A::DISABLE,
            true => UNDERFLOW_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == UNDERFLOW_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == UNDERFLOW_A::ENABLE
    }
}
#[doc = "Field `FLIP_IN_PROGRESS` reader - Flip in Progress"]
pub type FLIP_IN_PROGRESS_R = crate::BitReader<FLIP_IN_PROGRESS_A>;
#[doc = "Flip in Progress\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLIP_IN_PROGRESS_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<FLIP_IN_PROGRESS_A> for bool {
    #[inline(always)]
    fn from(variant: FLIP_IN_PROGRESS_A) -> Self {
        variant as u8 != 0
    }
}
impl FLIP_IN_PROGRESS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLIP_IN_PROGRESS_A {
        match self.bits {
            false => FLIP_IN_PROGRESS_A::DISABLE,
            true => FLIP_IN_PROGRESS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLIP_IN_PROGRESS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLIP_IN_PROGRESS_A::ENABLE
    }
}
#[doc = "Disable Back Pressure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BACK_PRESSURE_DISABLE_AW {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<BACK_PRESSURE_DISABLE_AW> for bool {
    #[inline(always)]
    fn from(variant: BACK_PRESSURE_DISABLE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BACK_PRESSURE_DISABLE` writer - Disable Back Pressure"]
pub type BACK_PRESSURE_DISABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FRAME_BUFFER_CONFIG0_SPEC, BACK_PRESSURE_DISABLE_AW, O>;
impl<'a, const O: u8> BACK_PRESSURE_DISABLE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BACK_PRESSURE_DISABLE_AW::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BACK_PRESSURE_DISABLE_AW::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:2 - The format of the frame buffer."]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Mode of the frame buffer."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Output"]
    #[inline(always)]
    pub fn output(&self) -> OUTPUT_R {
        OUTPUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Switch Panel"]
    #[inline(always)]
    pub fn switchpanel(&self) -> SWITCHPANEL_R {
        SWITCHPANEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Gamma"]
    #[inline(always)]
    pub fn gamma(&self) -> GAMMA_R {
        GAMMA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Valid"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Underflow"]
    #[inline(always)]
    pub fn underflow(&self) -> UNDERFLOW_R {
        UNDERFLOW_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Flip in Progress"]
    #[inline(always)]
    pub fn flip_in_progress(&self) -> FLIP_IN_PROGRESS_R {
        FLIP_IN_PROGRESS_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - The format of the frame buffer."]
    #[inline(always)]
    #[must_use]
    pub fn format(&mut self) -> FORMAT_W<0> {
        FORMAT_W::new(self)
    }
    #[doc = "Bit 4 - Mode of the frame buffer."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<4> {
        MODE_W::new(self)
    }
    #[doc = "Bit 8 - Output"]
    #[inline(always)]
    #[must_use]
    pub fn output(&mut self) -> OUTPUT_W<8> {
        OUTPUT_W::new(self)
    }
    #[doc = "Bit 9 - Switch Panel"]
    #[inline(always)]
    #[must_use]
    pub fn switchpanel(&mut self) -> SWITCHPANEL_W<9> {
        SWITCHPANEL_W::new(self)
    }
    #[doc = "Bit 12 - Gamma"]
    #[inline(always)]
    #[must_use]
    pub fn gamma(&mut self) -> GAMMA_W<12> {
        GAMMA_W::new(self)
    }
    #[doc = "Bit 16 - Valid"]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> VALID_W<16> {
        VALID_W::new(self)
    }
    #[doc = "Bit 20 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<20> {
        RESET_W::new(self)
    }
    #[doc = "Bit 29 - Disable Back Pressure"]
    #[inline(always)]
    #[must_use]
    pub fn back_pressure_disable(&mut self) -> BACK_PRESSURE_DISABLE_W<29> {
        BACK_PRESSURE_DISABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame Buffer Configuration 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frame_buffer_config0](index.html) module"]
pub struct FRAME_BUFFER_CONFIG0_SPEC;
impl crate::RegisterSpec for FRAME_BUFFER_CONFIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frame_buffer_config0::R](R) reader structure"]
impl crate::Readable for FRAME_BUFFER_CONFIG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frame_buffer_config0::W](W) writer structure"]
impl crate::Writable for FRAME_BUFFER_CONFIG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FrameBufferConfig0 to value 0x0100_0000"]
impl crate::Resettable for FRAME_BUFFER_CONFIG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0000;
}
