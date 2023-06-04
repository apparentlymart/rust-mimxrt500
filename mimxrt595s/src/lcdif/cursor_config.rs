#[doc = "Register `CursorConfig` reader"]
pub struct R(crate::R<CURSOR_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CURSOR_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CURSOR_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CURSOR_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CursorConfig` writer"]
pub struct W(crate::W<CURSOR_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CURSOR_CONFIG_SPEC>;
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
impl From<crate::W<CURSOR_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CURSOR_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORMAT` reader - Format of the cursor."]
pub type FORMAT_R = crate::FieldReader<u8, FORMAT_A>;
#[doc = "Format of the cursor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FORMAT_A {
    #[doc = "0: DISABLED"]
    DISABLE = 0,
    #[doc = "1: MASKED"]
    ENABLE = 1,
    #[doc = "2: A8R8G8B8"]
    ENABLE1 = 2,
}
impl From<FORMAT_A> for u8 {
    #[inline(always)]
    fn from(variant: FORMAT_A) -> Self {
        variant as _
    }
}
impl FORMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FORMAT_A> {
        match self.bits {
            0 => Some(FORMAT_A::DISABLE),
            1 => Some(FORMAT_A::ENABLE),
            2 => Some(FORMAT_A::ENABLE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FORMAT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FORMAT_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE1`"]
    #[inline(always)]
    pub fn is_enable1(&self) -> bool {
        *self == FORMAT_A::ENABLE1
    }
}
#[doc = "Field `FORMAT` writer - Format of the cursor."]
pub type FORMAT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CURSOR_CONFIG_SPEC, u8, FORMAT_A, 2, O>;
impl<'a, const O: u8> FORMAT_W<'a, O> {
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FORMAT_A::DISABLE)
    }
    #[doc = "MASKED"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FORMAT_A::ENABLE)
    }
    #[doc = "A8R8G8B8"]
    #[inline(always)]
    pub fn enable1(self) -> &'a mut W {
        self.variant(FORMAT_A::ENABLE1)
    }
}
#[doc = "Field `DISPLAY` reader - Display Controller owning the cursor."]
pub type DISPLAY_R = crate::BitReader<DISPLAY_A>;
#[doc = "Display Controller owning the cursor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISPLAY_A {
    #[doc = "0: DISPLAY0"]
    DIS0 = 0,
    #[doc = "1: DISPLAY1"]
    DIS1 = 1,
}
impl From<DISPLAY_A> for bool {
    #[inline(always)]
    fn from(variant: DISPLAY_A) -> Self {
        variant as u8 != 0
    }
}
impl DISPLAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISPLAY_A {
        match self.bits {
            false => DISPLAY_A::DIS0,
            true => DISPLAY_A::DIS1,
        }
    }
    #[doc = "Checks if the value of the field is `DIS0`"]
    #[inline(always)]
    pub fn is_dis0(&self) -> bool {
        *self == DISPLAY_A::DIS0
    }
    #[doc = "Checks if the value of the field is `DIS1`"]
    #[inline(always)]
    pub fn is_dis1(&self) -> bool {
        *self == DISPLAY_A::DIS1
    }
}
#[doc = "Field `DISPLAY` writer - Display Controller owning the cursor."]
pub type DISPLAY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CURSOR_CONFIG_SPEC, DISPLAY_A, O>;
impl<'a, const O: u8> DISPLAY_W<'a, O> {
    #[doc = "DISPLAY0"]
    #[inline(always)]
    pub fn dis0(self) -> &'a mut W {
        self.variant(DISPLAY_A::DIS0)
    }
    #[doc = "DISPLAY1"]
    #[inline(always)]
    pub fn dis1(self) -> &'a mut W {
        self.variant(DISPLAY_A::DIS1)
    }
}
#[doc = "Field `HOT_SPOT_Y` reader - Vertical offset to cursor hotspot."]
pub type HOT_SPOT_Y_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOT_SPOT_Y` writer - Vertical offset to cursor hotspot."]
pub type HOT_SPOT_Y_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CURSOR_CONFIG_SPEC, u8, u8, 5, O>;
#[doc = "Field `HOT_SPOT_X` reader - Horizontal offset to cursor hotspot."]
pub type HOT_SPOT_X_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOT_SPOT_X` writer - Horizontal offset to cursor hotspot."]
pub type HOT_SPOT_X_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CURSOR_CONFIG_SPEC, u8, u8, 5, O>;
#[doc = "Field `FLIP_IN_PROGRESS` reader - When the cursor address gets written to, this bit gets set to one."]
pub type FLIP_IN_PROGRESS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - Format of the cursor."]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Display Controller owning the cursor."]
    #[inline(always)]
    pub fn display(&self) -> DISPLAY_R {
        DISPLAY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Vertical offset to cursor hotspot."]
    #[inline(always)]
    pub fn hot_spot_y(&self) -> HOT_SPOT_Y_R {
        HOT_SPOT_Y_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Horizontal offset to cursor hotspot."]
    #[inline(always)]
    pub fn hot_spot_x(&self) -> HOT_SPOT_X_R {
        HOT_SPOT_X_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - When the cursor address gets written to, this bit gets set to one."]
    #[inline(always)]
    pub fn flip_in_progress(&self) -> FLIP_IN_PROGRESS_R {
        FLIP_IN_PROGRESS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Format of the cursor."]
    #[inline(always)]
    #[must_use]
    pub fn format(&mut self) -> FORMAT_W<0> {
        FORMAT_W::new(self)
    }
    #[doc = "Bit 4 - Display Controller owning the cursor."]
    #[inline(always)]
    #[must_use]
    pub fn display(&mut self) -> DISPLAY_W<4> {
        DISPLAY_W::new(self)
    }
    #[doc = "Bits 8:12 - Vertical offset to cursor hotspot."]
    #[inline(always)]
    #[must_use]
    pub fn hot_spot_y(&mut self) -> HOT_SPOT_Y_W<8> {
        HOT_SPOT_Y_W::new(self)
    }
    #[doc = "Bits 16:20 - Horizontal offset to cursor hotspot."]
    #[inline(always)]
    #[must_use]
    pub fn hot_spot_x(&mut self) -> HOT_SPOT_X_W<16> {
        HOT_SPOT_X_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration for the Cursor\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cursor_config](index.html) module"]
pub struct CURSOR_CONFIG_SPEC;
impl crate::RegisterSpec for CURSOR_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cursor_config::R](R) reader structure"]
impl crate::Readable for CURSOR_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cursor_config::W](W) writer structure"]
impl crate::Writable for CURSOR_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CursorConfig to value 0"]
impl crate::Resettable for CURSOR_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
