#[doc = "Register `KEYLOCK` reader"]
pub struct R(crate::R<KEYLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYLOCK` writer"]
pub struct W(crate::W<KEYLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYLOCK_SPEC>;
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
impl From<crate::W<KEYLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY0` reader - Key 0"]
pub type KEY0_R = crate::FieldReader<u8, KEY0_A>;
#[doc = "Key 0\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY0_A {
    #[doc = "0: Write access to KEY0MASK, KEYENABLE\\[KEY0\\]
and KEYRESET\\[KEY0\\]
is NOT allowed."]
    KEY0LOCK_0 = 0,
    #[doc = "1: Write access to KEY0MASK, KEYENABLE\\[KEY0\\]
and KEYRESET\\[KEY0\\]
is NOT allowed."]
    KEY0LOCK_1 = 1,
    #[doc = "2: Write access to KEY0MASK, KEYENABLE\\[KEY0\\]
and KEYRESET\\[KEY0\\]
is allowed."]
    KEY0UNLOCK = 2,
    #[doc = "3: Write access to KEY0MASK, KEYENABLE\\[KEY0\\]
and KEYRESET\\[KEY0\\]
is NOT allowed."]
    KEY0LOCK_3 = 3,
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
            0 => KEY0_A::KEY0LOCK_0,
            1 => KEY0_A::KEY0LOCK_1,
            2 => KEY0_A::KEY0UNLOCK,
            3 => KEY0_A::KEY0LOCK_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `KEY0LOCK_0`"]
    #[inline(always)]
    pub fn is_key0lock_0(&self) -> bool {
        *self == KEY0_A::KEY0LOCK_0
    }
    #[doc = "Checks if the value of the field is `KEY0LOCK_1`"]
    #[inline(always)]
    pub fn is_key0lock_1(&self) -> bool {
        *self == KEY0_A::KEY0LOCK_1
    }
    #[doc = "Checks if the value of the field is `KEY0UNLOCK`"]
    #[inline(always)]
    pub fn is_key0unlock(&self) -> bool {
        *self == KEY0_A::KEY0UNLOCK
    }
    #[doc = "Checks if the value of the field is `KEY0LOCK_3`"]
    #[inline(always)]
    pub fn is_key0lock_3(&self) -> bool {
        *self == KEY0_A::KEY0LOCK_3
    }
}
#[doc = "Field `KEY0` writer - Key 0"]
pub type KEY0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, KEYLOCK_SPEC, u8, KEY0_A, 2, O>;
impl<'a, const O: u8> KEY0_W<'a, O> {
    #[doc = "Write access to KEY0MASK, KEYENABLE\\[KEY0\\]
and KEYRESET\\[KEY0\\]
is NOT allowed."]
    #[inline(always)]
    pub fn key0lock_0(self) -> &'a mut W {
        self.variant(KEY0_A::KEY0LOCK_0)
    }
    #[doc = "Write access to KEY0MASK, KEYENABLE\\[KEY0\\]
and KEYRESET\\[KEY0\\]
is NOT allowed."]
    #[inline(always)]
    pub fn key0lock_1(self) -> &'a mut W {
        self.variant(KEY0_A::KEY0LOCK_1)
    }
    #[doc = "Write access to KEY0MASK, KEYENABLE\\[KEY0\\]
and KEYRESET\\[KEY0\\]
is allowed."]
    #[inline(always)]
    pub fn key0unlock(self) -> &'a mut W {
        self.variant(KEY0_A::KEY0UNLOCK)
    }
    #[doc = "Write access to KEY0MASK, KEYENABLE\\[KEY0\\]
and KEYRESET\\[KEY0\\]
is NOT allowed."]
    #[inline(always)]
    pub fn key0lock_3(self) -> &'a mut W {
        self.variant(KEY0_A::KEY0LOCK_3)
    }
}
#[doc = "Field `KEY1` reader - Key 1"]
pub type KEY1_R = crate::FieldReader<u8, KEY1_A>;
#[doc = "Key 1\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY1_A {
    #[doc = "0: Write access to KEY1MASK, KEYENABLE\\[KEY1\\]
and KEYRESET\\[KEY1\\]
is NOT allowed."]
    KEY1LOCK_0 = 0,
    #[doc = "1: Write access to KEY1MASK, KEYENABLE\\[KEY1\\]
and KEYRESET\\[KEY1\\]
is NOT allowed."]
    KEY1LOCK_1 = 1,
    #[doc = "2: Write access to KEY1MASK, KEYENABLE\\[KEY1\\]
and KEYRESET\\[KEY1\\]
is allowed."]
    KEY1UNLOCK = 2,
    #[doc = "3: Write access to KEY1MASK, KEYENABLE\\[KEY1\\]
and KEYRESET\\[KEY1\\]
is NOT allowed."]
    KEY1LOCK_3 = 3,
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
            0 => KEY1_A::KEY1LOCK_0,
            1 => KEY1_A::KEY1LOCK_1,
            2 => KEY1_A::KEY1UNLOCK,
            3 => KEY1_A::KEY1LOCK_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `KEY1LOCK_0`"]
    #[inline(always)]
    pub fn is_key1lock_0(&self) -> bool {
        *self == KEY1_A::KEY1LOCK_0
    }
    #[doc = "Checks if the value of the field is `KEY1LOCK_1`"]
    #[inline(always)]
    pub fn is_key1lock_1(&self) -> bool {
        *self == KEY1_A::KEY1LOCK_1
    }
    #[doc = "Checks if the value of the field is `KEY1UNLOCK`"]
    #[inline(always)]
    pub fn is_key1unlock(&self) -> bool {
        *self == KEY1_A::KEY1UNLOCK
    }
    #[doc = "Checks if the value of the field is `KEY1LOCK_3`"]
    #[inline(always)]
    pub fn is_key1lock_3(&self) -> bool {
        *self == KEY1_A::KEY1LOCK_3
    }
}
#[doc = "Field `KEY1` writer - Key 1"]
pub type KEY1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, KEYLOCK_SPEC, u8, KEY1_A, 2, O>;
impl<'a, const O: u8> KEY1_W<'a, O> {
    #[doc = "Write access to KEY1MASK, KEYENABLE\\[KEY1\\]
and KEYRESET\\[KEY1\\]
is NOT allowed."]
    #[inline(always)]
    pub fn key1lock_0(self) -> &'a mut W {
        self.variant(KEY1_A::KEY1LOCK_0)
    }
    #[doc = "Write access to KEY1MASK, KEYENABLE\\[KEY1\\]
and KEYRESET\\[KEY1\\]
is NOT allowed."]
    #[inline(always)]
    pub fn key1lock_1(self) -> &'a mut W {
        self.variant(KEY1_A::KEY1LOCK_1)
    }
    #[doc = "Write access to KEY1MASK, KEYENABLE\\[KEY1\\]
and KEYRESET\\[KEY1\\]
is allowed."]
    #[inline(always)]
    pub fn key1unlock(self) -> &'a mut W {
        self.variant(KEY1_A::KEY1UNLOCK)
    }
    #[doc = "Write access to KEY1MASK, KEYENABLE\\[KEY1\\]
and KEYRESET\\[KEY1\\]
is NOT allowed."]
    #[inline(always)]
    pub fn key1lock_3(self) -> &'a mut W {
        self.variant(KEY1_A::KEY1LOCK_3)
    }
}
#[doc = "Field `KEY2` reader - Key 2"]
pub type KEY2_R = crate::FieldReader<u8, KEY2_A>;
#[doc = "Key 2\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY2_A {
    #[doc = "0: Write access to KEY2MASK, KEYENABLE\\[KEY2\\]
and KEYRESET\\[KEY2\\]
is NOT allowed."]
    KEY2LOCK_0 = 0,
    #[doc = "1: Write access to KEY2MASK, KEYENABLE\\[KEY2\\]
and KEYRESET\\[KEY2\\]
is NOT allowed."]
    KEY2LOCK_1 = 1,
    #[doc = "2: Write access to KEY2MASK, KEYENABLE\\[KEY2\\]
and KEYRESET\\[KEY2\\]
is allowed."]
    KEY2UNLOCK = 2,
    #[doc = "3: Write access to KEY2MASK, KEYENABLE\\[KEY2\\]
and KEYRESET\\[KEY2\\]
is NOT allowed."]
    KEY2LOCK_3 = 3,
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
            0 => KEY2_A::KEY2LOCK_0,
            1 => KEY2_A::KEY2LOCK_1,
            2 => KEY2_A::KEY2UNLOCK,
            3 => KEY2_A::KEY2LOCK_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `KEY2LOCK_0`"]
    #[inline(always)]
    pub fn is_key2lock_0(&self) -> bool {
        *self == KEY2_A::KEY2LOCK_0
    }
    #[doc = "Checks if the value of the field is `KEY2LOCK_1`"]
    #[inline(always)]
    pub fn is_key2lock_1(&self) -> bool {
        *self == KEY2_A::KEY2LOCK_1
    }
    #[doc = "Checks if the value of the field is `KEY2UNLOCK`"]
    #[inline(always)]
    pub fn is_key2unlock(&self) -> bool {
        *self == KEY2_A::KEY2UNLOCK
    }
    #[doc = "Checks if the value of the field is `KEY2LOCK_3`"]
    #[inline(always)]
    pub fn is_key2lock_3(&self) -> bool {
        *self == KEY2_A::KEY2LOCK_3
    }
}
#[doc = "Field `KEY2` writer - Key 2"]
pub type KEY2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, KEYLOCK_SPEC, u8, KEY2_A, 2, O>;
impl<'a, const O: u8> KEY2_W<'a, O> {
    #[doc = "Write access to KEY2MASK, KEYENABLE\\[KEY2\\]
and KEYRESET\\[KEY2\\]
is NOT allowed."]
    #[inline(always)]
    pub fn key2lock_0(self) -> &'a mut W {
        self.variant(KEY2_A::KEY2LOCK_0)
    }
    #[doc = "Write access to KEY2MASK, KEYENABLE\\[KEY2\\]
and KEYRESET\\[KEY2\\]
is NOT allowed."]
    #[inline(always)]
    pub fn key2lock_1(self) -> &'a mut W {
        self.variant(KEY2_A::KEY2LOCK_1)
    }
    #[doc = "Write access to KEY2MASK, KEYENABLE\\[KEY2\\]
and KEYRESET\\[KEY2\\]
is allowed."]
    #[inline(always)]
    pub fn key2unlock(self) -> &'a mut W {
        self.variant(KEY2_A::KEY2UNLOCK)
    }
    #[doc = "Write access to KEY2MASK, KEYENABLE\\[KEY2\\]
and KEYRESET\\[KEY2\\]
is NOT allowed."]
    #[inline(always)]
    pub fn key2lock_3(self) -> &'a mut W {
        self.variant(KEY2_A::KEY2LOCK_3)
    }
}
#[doc = "Field `KEY3` reader - Key 3"]
pub type KEY3_R = crate::FieldReader<u8, KEY3_A>;
#[doc = "Key 3\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY3_A {
    #[doc = "0: Write access to KEY3MASK, KEYENABLE\\[KEY3\\]
and KEYRESET\\[KEY3\\]
is NOT allowed."]
    KEY3LOCK_0 = 0,
    #[doc = "1: Write access to KEY3MASK, KEYENABLE\\[KEY3\\]
and KEYRESET\\[KEY3\\]
is NOT allowed."]
    KEY3LOCK_1 = 1,
    #[doc = "2: Write access to KEY3MASK, KEYENABLE\\[KEY3\\]
and KEYRESET\\[KEY3\\]
is allowed."]
    KEY3UNLOCK = 2,
    #[doc = "3: Write access to KEY3MASK, KEYENABLE\\[KEY3\\]
and KEYRESET\\[KEY3\\]
is NOT allowed."]
    KEY3LOCK_3 = 3,
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
            0 => KEY3_A::KEY3LOCK_0,
            1 => KEY3_A::KEY3LOCK_1,
            2 => KEY3_A::KEY3UNLOCK,
            3 => KEY3_A::KEY3LOCK_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `KEY3LOCK_0`"]
    #[inline(always)]
    pub fn is_key3lock_0(&self) -> bool {
        *self == KEY3_A::KEY3LOCK_0
    }
    #[doc = "Checks if the value of the field is `KEY3LOCK_1`"]
    #[inline(always)]
    pub fn is_key3lock_1(&self) -> bool {
        *self == KEY3_A::KEY3LOCK_1
    }
    #[doc = "Checks if the value of the field is `KEY3UNLOCK`"]
    #[inline(always)]
    pub fn is_key3unlock(&self) -> bool {
        *self == KEY3_A::KEY3UNLOCK
    }
    #[doc = "Checks if the value of the field is `KEY3LOCK_3`"]
    #[inline(always)]
    pub fn is_key3lock_3(&self) -> bool {
        *self == KEY3_A::KEY3LOCK_3
    }
}
#[doc = "Field `KEY3` writer - Key 3"]
pub type KEY3_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, KEYLOCK_SPEC, u8, KEY3_A, 2, O>;
impl<'a, const O: u8> KEY3_W<'a, O> {
    #[doc = "Write access to KEY3MASK, KEYENABLE\\[KEY3\\]
and KEYRESET\\[KEY3\\]
is NOT allowed."]
    #[inline(always)]
    pub fn key3lock_0(self) -> &'a mut W {
        self.variant(KEY3_A::KEY3LOCK_0)
    }
    #[doc = "Write access to KEY3MASK, KEYENABLE\\[KEY3\\]
and KEYRESET\\[KEY3\\]
is NOT allowed."]
    #[inline(always)]
    pub fn key3lock_1(self) -> &'a mut W {
        self.variant(KEY3_A::KEY3LOCK_1)
    }
    #[doc = "Write access to KEY3MASK, KEYENABLE\\[KEY3\\]
and KEYRESET\\[KEY3\\]
is allowed."]
    #[inline(always)]
    pub fn key3unlock(self) -> &'a mut W {
        self.variant(KEY3_A::KEY3UNLOCK)
    }
    #[doc = "Write access to KEY3MASK, KEYENABLE\\[KEY3\\]
and KEYRESET\\[KEY3\\]
is NOT allowed."]
    #[inline(always)]
    pub fn key3lock_3(self) -> &'a mut W {
        self.variant(KEY3_A::KEY3LOCK_3)
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
#[doc = "Key Lock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keylock](index.html) module"]
pub struct KEYLOCK_SPEC;
impl crate::RegisterSpec for KEYLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keylock::R](R) reader structure"]
impl crate::Readable for KEYLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keylock::W](W) writer structure"]
impl crate::Writable for KEYLOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYLOCK to value 0xaa"]
impl crate::Resettable for KEYLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0xaa;
}
