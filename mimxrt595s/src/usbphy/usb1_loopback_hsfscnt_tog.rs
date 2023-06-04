#[doc = "Register `USB1_LOOPBACK_HSFSCNT_TOG` reader"]
pub struct R(crate::R<USB1_LOOPBACK_HSFSCNT_TOG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB1_LOOPBACK_HSFSCNT_TOG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB1_LOOPBACK_HSFSCNT_TOG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB1_LOOPBACK_HSFSCNT_TOG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB1_LOOPBACK_HSFSCNT_TOG` writer"]
pub struct W(crate::W<USB1_LOOPBACK_HSFSCNT_TOG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB1_LOOPBACK_HSFSCNT_TOG_SPEC>;
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
impl From<crate::W<USB1_LOOPBACK_HSFSCNT_TOG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB1_LOOPBACK_HSFSCNT_TOG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSTI_HS_NUMBER` reader - USB loopback test HS CNT."]
pub type TSTI_HS_NUMBER_R = crate::FieldReader<u16, TSTI_HS_NUMBER_A>;
#[doc = "USB loopback test HS CNT.\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TSTI_HS_NUMBER_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding bit"]
    ENABLE = 1,
}
impl From<TSTI_HS_NUMBER_A> for u16 {
    #[inline(always)]
    fn from(variant: TSTI_HS_NUMBER_A) -> Self {
        variant as _
    }
}
impl TSTI_HS_NUMBER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSTI_HS_NUMBER_A> {
        match self.bits {
            0 => Some(TSTI_HS_NUMBER_A::DISABLE),
            1 => Some(TSTI_HS_NUMBER_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TSTI_HS_NUMBER_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TSTI_HS_NUMBER_A::ENABLE
    }
}
#[doc = "Field `TSTI_HS_NUMBER` writer - USB loopback test HS CNT."]
pub type TSTI_HS_NUMBER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB1_LOOPBACK_HSFSCNT_TOG_SPEC, u16, TSTI_HS_NUMBER_A, 16, O>;
impl<'a, const O: u8> TSTI_HS_NUMBER_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TSTI_HS_NUMBER_A::DISABLE)
    }
    #[doc = "Toggles the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TSTI_HS_NUMBER_A::ENABLE)
    }
}
#[doc = "Field `TSTI_FS_NUMBER` reader - USB loopback test FS CNT."]
pub type TSTI_FS_NUMBER_R = crate::FieldReader<u16, TSTI_FS_NUMBER_A>;
#[doc = "USB loopback test FS CNT.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TSTI_FS_NUMBER_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding bit"]
    ENABLE = 1,
}
impl From<TSTI_FS_NUMBER_A> for u16 {
    #[inline(always)]
    fn from(variant: TSTI_FS_NUMBER_A) -> Self {
        variant as _
    }
}
impl TSTI_FS_NUMBER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSTI_FS_NUMBER_A> {
        match self.bits {
            0 => Some(TSTI_FS_NUMBER_A::DISABLE),
            1 => Some(TSTI_FS_NUMBER_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TSTI_FS_NUMBER_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TSTI_FS_NUMBER_A::ENABLE
    }
}
#[doc = "Field `TSTI_FS_NUMBER` writer - USB loopback test FS CNT."]
pub type TSTI_FS_NUMBER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB1_LOOPBACK_HSFSCNT_TOG_SPEC, u16, TSTI_FS_NUMBER_A, 16, O>;
impl<'a, const O: u8> TSTI_FS_NUMBER_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TSTI_FS_NUMBER_A::DISABLE)
    }
    #[doc = "Toggles the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TSTI_FS_NUMBER_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:15 - USB loopback test HS CNT."]
    #[inline(always)]
    pub fn tsti_hs_number(&self) -> TSTI_HS_NUMBER_R {
        TSTI_HS_NUMBER_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - USB loopback test FS CNT."]
    #[inline(always)]
    pub fn tsti_fs_number(&self) -> TSTI_FS_NUMBER_R {
        TSTI_FS_NUMBER_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - USB loopback test HS CNT."]
    #[inline(always)]
    #[must_use]
    pub fn tsti_hs_number(&mut self) -> TSTI_HS_NUMBER_W<0> {
        TSTI_HS_NUMBER_W::new(self)
    }
    #[doc = "Bits 16:31 - USB loopback test FS CNT."]
    #[inline(always)]
    #[must_use]
    pub fn tsti_fs_number(&mut self) -> TSTI_FS_NUMBER_W<16> {
        TSTI_FS_NUMBER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PHY Loopback Packet Number Select Toggle\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_loopback_hsfscnt_tog](index.html) module"]
pub struct USB1_LOOPBACK_HSFSCNT_TOG_SPEC;
impl crate::RegisterSpec for USB1_LOOPBACK_HSFSCNT_TOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb1_loopback_hsfscnt_tog::R](R) reader structure"]
impl crate::Readable for USB1_LOOPBACK_HSFSCNT_TOG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb1_loopback_hsfscnt_tog::W](W) writer structure"]
impl crate::Writable for USB1_LOOPBACK_HSFSCNT_TOG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB1_LOOPBACK_HSFSCNT_TOG to value 0x0004_0010"]
impl crate::Resettable for USB1_LOOPBACK_HSFSCNT_TOG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0004_0010;
}
