#[doc = "Register `KEYRESET` writer"]
pub struct W(crate::W<KEYRESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYRESET_SPEC>;
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
impl From<crate::W<KEYRESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYRESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Key 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY0_AW {
    #[doc = "2: Reset KEY0 Hold register and SHIFT_STATUS\\[KEY0\\]."]
    RESET = 2,
}
impl From<KEY0_AW> for u8 {
    #[inline(always)]
    fn from(variant: KEY0_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY0` writer - Key 0"]
pub type KEY0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYRESET_SPEC, u8, KEY0_AW, 2, O>;
impl<'a, const O: u8> KEY0_W<'a, O> {
    #[doc = "Reset KEY0 Hold register and SHIFT_STATUS\\[KEY0\\]."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(KEY0_AW::RESET)
    }
}
#[doc = "Key 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY1_AW {
    #[doc = "2: Reset KEY1 Hold register and SHIFT_STATUS\\[KEY1\\]."]
    RESET = 2,
}
impl From<KEY1_AW> for u8 {
    #[inline(always)]
    fn from(variant: KEY1_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY1` writer - Key 1"]
pub type KEY1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYRESET_SPEC, u8, KEY1_AW, 2, O>;
impl<'a, const O: u8> KEY1_W<'a, O> {
    #[doc = "Reset KEY1 Hold register and SHIFT_STATUS\\[KEY1\\]."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(KEY1_AW::RESET)
    }
}
#[doc = "Key 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY2_AW {
    #[doc = "2: Reset KEY2 Hold register and SHIFT_STATUS\\[KEY2\\]."]
    RESET = 2,
}
impl From<KEY2_AW> for u8 {
    #[inline(always)]
    fn from(variant: KEY2_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY2` writer - Key 2"]
pub type KEY2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYRESET_SPEC, u8, KEY2_AW, 2, O>;
impl<'a, const O: u8> KEY2_W<'a, O> {
    #[doc = "Reset KEY2 Hold register and SHIFT_STATUS\\[KEY2\\]."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(KEY2_AW::RESET)
    }
}
#[doc = "Key 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY3_AW {
    #[doc = "2: Reset KEY3 Hold register and SHIFT_STATUS\\[KEY3\\]."]
    RESET = 2,
}
impl From<KEY3_AW> for u8 {
    #[inline(always)]
    fn from(variant: KEY3_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY3` writer - Key 3"]
pub type KEY3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYRESET_SPEC, u8, KEY3_AW, 2, O>;
impl<'a, const O: u8> KEY3_W<'a, O> {
    #[doc = "Reset KEY3 Hold register and SHIFT_STATUS\\[KEY3\\]."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(KEY3_AW::RESET)
    }
}
impl W {
    #[doc = "Bits 0:1 - Key 0"]
    #[inline(always)]
    #[must_use]
    pub fn key0(&mut self) -> KEY0_W<0> {
        KEY0_W::new(self)
    }
    #[doc = "Bits 2:3 - Key 1"]
    #[inline(always)]
    #[must_use]
    pub fn key1(&mut self) -> KEY1_W<2> {
        KEY1_W::new(self)
    }
    #[doc = "Bits 4:5 - Key 2"]
    #[inline(always)]
    #[must_use]
    pub fn key2(&mut self) -> KEY2_W<4> {
        KEY2_W::new(self)
    }
    #[doc = "Bits 6:7 - Key 3"]
    #[inline(always)]
    #[must_use]
    pub fn key3(&mut self) -> KEY3_W<6> {
        KEY3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key Reset\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyreset](index.html) module"]
pub struct KEYRESET_SPEC;
impl crate::RegisterSpec for KEYRESET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [keyreset::W](W) writer structure"]
impl crate::Writable for KEYRESET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYRESET to value 0"]
impl crate::Resettable for KEYRESET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
