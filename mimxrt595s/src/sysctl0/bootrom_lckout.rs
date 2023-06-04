#[doc = "Register `BOOTROM_LCKOUT` reader"]
pub struct R(crate::R<BOOTROM_LCKOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOTROM_LCKOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOTROM_LCKOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOTROM_LCKOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOOTROM_LCKOUT` writer"]
pub struct W(crate::W<BOOTROM_LCKOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOOTROM_LCKOUT_SPEC>;
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
impl From<crate::W<BOOTROM_LCKOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOOTROM_LCKOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READ_LCKOUT_SPACE` writer - Read Lockout"]
pub type READ_LCKOUT_SPACE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BOOTROM_LCKOUT_SPEC, u32, u32, 17, O>;
#[doc = "Field `WRITE_LOCK` reader - Self Write Disable"]
pub type WRITE_LOCK_R = crate::FieldReader<u8, WRITE_LOCK_A>;
#[doc = "Self Write Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WRITE_LOCK_A {
    #[doc = "0: Write disable"]
    WRITE_LOCK_00 = 0,
    #[doc = "1: Write disable"]
    WRITE_LOCK_01 = 1,
    #[doc = "2: Write enable"]
    WRITE_LOCK_11 = 2,
    #[doc = "3: Write disable"]
    WRITE_LOCK_10 = 3,
}
impl From<WRITE_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: WRITE_LOCK_A) -> Self {
        variant as _
    }
}
impl WRITE_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRITE_LOCK_A {
        match self.bits {
            0 => WRITE_LOCK_A::WRITE_LOCK_00,
            1 => WRITE_LOCK_A::WRITE_LOCK_01,
            2 => WRITE_LOCK_A::WRITE_LOCK_11,
            3 => WRITE_LOCK_A::WRITE_LOCK_10,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_LOCK_00`"]
    #[inline(always)]
    pub fn is_write_lock_00(&self) -> bool {
        *self == WRITE_LOCK_A::WRITE_LOCK_00
    }
    #[doc = "Checks if the value of the field is `WRITE_LOCK_01`"]
    #[inline(always)]
    pub fn is_write_lock_01(&self) -> bool {
        *self == WRITE_LOCK_A::WRITE_LOCK_01
    }
    #[doc = "Checks if the value of the field is `WRITE_LOCK_11`"]
    #[inline(always)]
    pub fn is_write_lock_11(&self) -> bool {
        *self == WRITE_LOCK_A::WRITE_LOCK_11
    }
    #[doc = "Checks if the value of the field is `WRITE_LOCK_10`"]
    #[inline(always)]
    pub fn is_write_lock_10(&self) -> bool {
        *self == WRITE_LOCK_A::WRITE_LOCK_10
    }
}
#[doc = "Field `WRITE_LOCK` writer - Self Write Disable"]
pub type WRITE_LOCK_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, BOOTROM_LCKOUT_SPEC, u8, WRITE_LOCK_A, 2, O>;
impl<'a, const O: u8> WRITE_LOCK_W<'a, O> {
    #[doc = "Write disable"]
    #[inline(always)]
    pub fn write_lock_00(self) -> &'a mut W {
        self.variant(WRITE_LOCK_A::WRITE_LOCK_00)
    }
    #[doc = "Write disable"]
    #[inline(always)]
    pub fn write_lock_01(self) -> &'a mut W {
        self.variant(WRITE_LOCK_A::WRITE_LOCK_01)
    }
    #[doc = "Write enable"]
    #[inline(always)]
    pub fn write_lock_11(self) -> &'a mut W {
        self.variant(WRITE_LOCK_A::WRITE_LOCK_11)
    }
    #[doc = "Write disable"]
    #[inline(always)]
    pub fn write_lock_10(self) -> &'a mut W {
        self.variant(WRITE_LOCK_A::WRITE_LOCK_10)
    }
}
impl R {
    #[doc = "Bits 30:31 - Self Write Disable"]
    #[inline(always)]
    pub fn write_lock(&self) -> WRITE_LOCK_R {
        WRITE_LOCK_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:16 - Read Lockout"]
    #[inline(always)]
    #[must_use]
    pub fn read_lckout_space(&mut self) -> READ_LCKOUT_SPACE_W<0> {
        READ_LCKOUT_SPACE_W::new(self)
    }
    #[doc = "Bits 30:31 - Self Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn write_lock(&mut self) -> WRITE_LOCK_W<30> {
        WRITE_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BOOT ROM lockout\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bootrom_lckout](index.html) module"]
pub struct BOOTROM_LCKOUT_SPEC;
impl crate::RegisterSpec for BOOTROM_LCKOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bootrom_lckout::R](R) reader structure"]
impl crate::Readable for BOOTROM_LCKOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bootrom_lckout::W](W) writer structure"]
impl crate::Writable for BOOTROM_LCKOUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xc000_0000;
}
#[doc = "`reset()` method sets BOOTROM_LCKOUT to value 0"]
impl crate::Resettable for BOOTROM_LCKOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
