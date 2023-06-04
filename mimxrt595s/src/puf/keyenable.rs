#[doc = "Register `KEYENABLE` reader"]
pub struct R(crate::R<KEYENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYENABLE` writer"]
pub struct W(crate::W<KEYENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYENABLE_SPEC>;
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
impl From<crate::W<KEYENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY0` reader - Key 0"]
pub type KEY0_R = crate::FieldReader<u8, KEY0_A>;
#[doc = "Key 0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY0_A {
    #[doc = "0: Disabled"]
    DISABLED_0 = 0,
    #[doc = "1: Disabled"]
    DISABLED_1 = 1,
    #[doc = "2: Enabled"]
    ENABLED = 2,
    #[doc = "3: Disabled"]
    DISABLED_3 = 3,
}
impl From<KEY0_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY0_A) -> Self {
        variant as _
    }
}
impl KEY0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEY0_A {
        match self.bits {
            0 => KEY0_A::DISABLED_0,
            1 => KEY0_A::DISABLED_1,
            2 => KEY0_A::ENABLED,
            3 => KEY0_A::DISABLED_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_0`"]
    #[inline(always)]
    pub fn is_disabled_0(&self) -> bool {
        *self == KEY0_A::DISABLED_0
    }
    #[doc = "Checks if the value of the field is `DISABLED_1`"]
    #[inline(always)]
    pub fn is_disabled_1(&self) -> bool {
        *self == KEY0_A::DISABLED_1
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == KEY0_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED_3`"]
    #[inline(always)]
    pub fn is_disabled_3(&self) -> bool {
        *self == KEY0_A::DISABLED_3
    }
}
#[doc = "Field `KEY0` writer - Key 0"]
pub type KEY0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, KEYENABLE_SPEC, u8, KEY0_A, 2, O>;
impl<'a, const O: u8> KEY0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled_0(self) -> &'a mut W {
        self.variant(KEY0_A::DISABLED_0)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled_1(self) -> &'a mut W {
        self.variant(KEY0_A::DISABLED_1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(KEY0_A::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled_3(self) -> &'a mut W {
        self.variant(KEY0_A::DISABLED_3)
    }
}
#[doc = "Field `KEY1` reader - Key 1"]
pub type KEY1_R = crate::FieldReader<u8, KEY1_A>;
#[doc = "Key 1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY1_A {
    #[doc = "0: Data coming from the PUF Index 0 interface are NOT shifted in the KEY1 register."]
    DISABLED_0 = 0,
    #[doc = "1: Data coming from the PUF Index 0 interface are NOT shifted in the KEY1 register."]
    DISABLED_1 = 1,
    #[doc = "2: Data coming from the PUF Index 0 interface are shifted in the KEY1 register."]
    ENABLED = 2,
    #[doc = "3: Data coming from the PUF Index 0 interface are NOT shifted in the KEY1 register."]
    DISABLED_3 = 3,
}
impl From<KEY1_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY1_A) -> Self {
        variant as _
    }
}
impl KEY1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEY1_A {
        match self.bits {
            0 => KEY1_A::DISABLED_0,
            1 => KEY1_A::DISABLED_1,
            2 => KEY1_A::ENABLED,
            3 => KEY1_A::DISABLED_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_0`"]
    #[inline(always)]
    pub fn is_disabled_0(&self) -> bool {
        *self == KEY1_A::DISABLED_0
    }
    #[doc = "Checks if the value of the field is `DISABLED_1`"]
    #[inline(always)]
    pub fn is_disabled_1(&self) -> bool {
        *self == KEY1_A::DISABLED_1
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == KEY1_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED_3`"]
    #[inline(always)]
    pub fn is_disabled_3(&self) -> bool {
        *self == KEY1_A::DISABLED_3
    }
}
#[doc = "Field `KEY1` writer - Key 1"]
pub type KEY1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, KEYENABLE_SPEC, u8, KEY1_A, 2, O>;
impl<'a, const O: u8> KEY1_W<'a, O> {
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY1 register."]
    #[inline(always)]
    pub fn disabled_0(self) -> &'a mut W {
        self.variant(KEY1_A::DISABLED_0)
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY1 register."]
    #[inline(always)]
    pub fn disabled_1(self) -> &'a mut W {
        self.variant(KEY1_A::DISABLED_1)
    }
    #[doc = "Data coming from the PUF Index 0 interface are shifted in the KEY1 register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(KEY1_A::ENABLED)
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY1 register."]
    #[inline(always)]
    pub fn disabled_3(self) -> &'a mut W {
        self.variant(KEY1_A::DISABLED_3)
    }
}
#[doc = "Field `KEY2` reader - Key 2"]
pub type KEY2_R = crate::FieldReader<u8, KEY2_A>;
#[doc = "Key 2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY2_A {
    #[doc = "0: Data coming from the PUF Index 0 interface are NOT shifted in the KEY2 register."]
    DISABLED_0 = 0,
    #[doc = "1: Data coming from the PUF Index 0 interface are NOT shifted in the KEY2 register."]
    DISABLED_1 = 1,
    #[doc = "2: Data coming from the PUF Index 0 interface are shifted in the KEY2 register."]
    ENABLED = 2,
    #[doc = "3: Data coming from the PUF Index 0 interface are NOT shifted in the KEY2 register."]
    DISABLED_3 = 3,
}
impl From<KEY2_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY2_A) -> Self {
        variant as _
    }
}
impl KEY2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEY2_A {
        match self.bits {
            0 => KEY2_A::DISABLED_0,
            1 => KEY2_A::DISABLED_1,
            2 => KEY2_A::ENABLED,
            3 => KEY2_A::DISABLED_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_0`"]
    #[inline(always)]
    pub fn is_disabled_0(&self) -> bool {
        *self == KEY2_A::DISABLED_0
    }
    #[doc = "Checks if the value of the field is `DISABLED_1`"]
    #[inline(always)]
    pub fn is_disabled_1(&self) -> bool {
        *self == KEY2_A::DISABLED_1
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == KEY2_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED_3`"]
    #[inline(always)]
    pub fn is_disabled_3(&self) -> bool {
        *self == KEY2_A::DISABLED_3
    }
}
#[doc = "Field `KEY2` writer - Key 2"]
pub type KEY2_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, KEYENABLE_SPEC, u8, KEY2_A, 2, O>;
impl<'a, const O: u8> KEY2_W<'a, O> {
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY2 register."]
    #[inline(always)]
    pub fn disabled_0(self) -> &'a mut W {
        self.variant(KEY2_A::DISABLED_0)
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY2 register."]
    #[inline(always)]
    pub fn disabled_1(self) -> &'a mut W {
        self.variant(KEY2_A::DISABLED_1)
    }
    #[doc = "Data coming from the PUF Index 0 interface are shifted in the KEY2 register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(KEY2_A::ENABLED)
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY2 register."]
    #[inline(always)]
    pub fn disabled_3(self) -> &'a mut W {
        self.variant(KEY2_A::DISABLED_3)
    }
}
#[doc = "Field `KEY3` reader - Key 3"]
pub type KEY3_R = crate::FieldReader<u8, KEY3_A>;
#[doc = "Key 3\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY3_A {
    #[doc = "0: Data coming from the PUF Index 0 interface are NOT shifted in the KEY3 register."]
    DISABLED_0 = 0,
    #[doc = "1: Data coming from the PUF Index 0 interface are NOT shifted in the KEY3 register."]
    DISABLED_1 = 1,
    #[doc = "2: Data coming from the PUF Index 0 interface are shifted in the KEY3 register."]
    ENABLED = 2,
    #[doc = "3: Data coming from the PUF Index 0 interface are NOT shifted in the KEY3 register."]
    DISABLED_3 = 3,
}
impl From<KEY3_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY3_A) -> Self {
        variant as _
    }
}
impl KEY3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEY3_A {
        match self.bits {
            0 => KEY3_A::DISABLED_0,
            1 => KEY3_A::DISABLED_1,
            2 => KEY3_A::ENABLED,
            3 => KEY3_A::DISABLED_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_0`"]
    #[inline(always)]
    pub fn is_disabled_0(&self) -> bool {
        *self == KEY3_A::DISABLED_0
    }
    #[doc = "Checks if the value of the field is `DISABLED_1`"]
    #[inline(always)]
    pub fn is_disabled_1(&self) -> bool {
        *self == KEY3_A::DISABLED_1
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == KEY3_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED_3`"]
    #[inline(always)]
    pub fn is_disabled_3(&self) -> bool {
        *self == KEY3_A::DISABLED_3
    }
}
#[doc = "Field `KEY3` writer - Key 3"]
pub type KEY3_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, KEYENABLE_SPEC, u8, KEY3_A, 2, O>;
impl<'a, const O: u8> KEY3_W<'a, O> {
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY3 register."]
    #[inline(always)]
    pub fn disabled_0(self) -> &'a mut W {
        self.variant(KEY3_A::DISABLED_0)
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY3 register."]
    #[inline(always)]
    pub fn disabled_1(self) -> &'a mut W {
        self.variant(KEY3_A::DISABLED_1)
    }
    #[doc = "Data coming from the PUF Index 0 interface are shifted in the KEY3 register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(KEY3_A::ENABLED)
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY3 register."]
    #[inline(always)]
    pub fn disabled_3(self) -> &'a mut W {
        self.variant(KEY3_A::DISABLED_3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Key 0"]
    #[inline(always)]
    pub fn key0(&self) -> KEY0_R {
        KEY0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Key 1"]
    #[inline(always)]
    pub fn key1(&self) -> KEY1_R {
        KEY1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Key 2"]
    #[inline(always)]
    pub fn key2(&self) -> KEY2_R {
        KEY2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Key 3"]
    #[inline(always)]
    pub fn key3(&self) -> KEY3_R {
        KEY3_R::new(((self.bits >> 6) & 3) as u8)
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
#[doc = "Key Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyenable](index.html) module"]
pub struct KEYENABLE_SPEC;
impl crate::RegisterSpec for KEYENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyenable::R](R) reader structure"]
impl crate::Readable for KEYENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keyenable::W](W) writer structure"]
impl crate::Writable for KEYENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYENABLE to value 0x55"]
impl crate::Resettable for KEYENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0x55;
}
