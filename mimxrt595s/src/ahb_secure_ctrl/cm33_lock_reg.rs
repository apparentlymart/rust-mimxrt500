#[doc = "Register `CM33_LOCK_REG` reader"]
pub struct R(crate::R<CM33_LOCK_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM33_LOCK_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM33_LOCK_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM33_LOCK_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM33_LOCK_REG` writer"]
pub struct W(crate::W<CM33_LOCK_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM33_LOCK_REG_SPEC>;
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
impl From<crate::W<CM33_LOCK_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM33_LOCK_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK_NS_VTOR` reader - Lock Non-Secure VTOR"]
pub type LOCK_NS_VTOR_R = crate::FieldReader<u8, LOCK_NS_VTOR_A>;
#[doc = "Lock Non-Secure VTOR\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK_NS_VTOR_A {
    #[doc = "1: Locks Non-Secure VTOR"]
    NONSEC_VTOR_LOCKED = 1,
    #[doc = "2: Non-Secure VTOR can be used"]
    NONSEC_VTOR_NOT_LOCKED = 2,
}
impl From<LOCK_NS_VTOR_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_NS_VTOR_A) -> Self {
        variant as _
    }
}
impl LOCK_NS_VTOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_NS_VTOR_A> {
        match self.bits {
            1 => Some(LOCK_NS_VTOR_A::NONSEC_VTOR_LOCKED),
            2 => Some(LOCK_NS_VTOR_A::NONSEC_VTOR_NOT_LOCKED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONSEC_VTOR_LOCKED`"]
    #[inline(always)]
    pub fn is_nonsec_vtor_locked(&self) -> bool {
        *self == LOCK_NS_VTOR_A::NONSEC_VTOR_LOCKED
    }
    #[doc = "Checks if the value of the field is `NONSEC_VTOR_NOT_LOCKED`"]
    #[inline(always)]
    pub fn is_nonsec_vtor_not_locked(&self) -> bool {
        *self == LOCK_NS_VTOR_A::NONSEC_VTOR_NOT_LOCKED
    }
}
#[doc = "Field `LOCK_NS_VTOR` writer - Lock Non-Secure VTOR"]
pub type LOCK_NS_VTOR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CM33_LOCK_REG_SPEC, u8, LOCK_NS_VTOR_A, 2, O>;
impl<'a, const O: u8> LOCK_NS_VTOR_W<'a, O> {
    #[doc = "Locks Non-Secure VTOR"]
    #[inline(always)]
    pub fn nonsec_vtor_locked(self) -> &'a mut W {
        self.variant(LOCK_NS_VTOR_A::NONSEC_VTOR_LOCKED)
    }
    #[doc = "Non-Secure VTOR can be used"]
    #[inline(always)]
    pub fn nonsec_vtor_not_locked(self) -> &'a mut W {
        self.variant(LOCK_NS_VTOR_A::NONSEC_VTOR_NOT_LOCKED)
    }
}
#[doc = "Field `LOCK_NS_MPU` reader - Lock Non-Secure MPU"]
pub type LOCK_NS_MPU_R = crate::FieldReader<u8, LOCK_NS_MPU_A>;
#[doc = "Lock Non-Secure MPU\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK_NS_MPU_A {
    #[doc = "1: Locks Non-Secure MPU"]
    NONSEC_MPU_LOCKED = 1,
    #[doc = "2: Non-Secure MPU can be used"]
    NONSEC_MPU_NOT_LOCKED = 2,
}
impl From<LOCK_NS_MPU_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_NS_MPU_A) -> Self {
        variant as _
    }
}
impl LOCK_NS_MPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_NS_MPU_A> {
        match self.bits {
            1 => Some(LOCK_NS_MPU_A::NONSEC_MPU_LOCKED),
            2 => Some(LOCK_NS_MPU_A::NONSEC_MPU_NOT_LOCKED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONSEC_MPU_LOCKED`"]
    #[inline(always)]
    pub fn is_nonsec_mpu_locked(&self) -> bool {
        *self == LOCK_NS_MPU_A::NONSEC_MPU_LOCKED
    }
    #[doc = "Checks if the value of the field is `NONSEC_MPU_NOT_LOCKED`"]
    #[inline(always)]
    pub fn is_nonsec_mpu_not_locked(&self) -> bool {
        *self == LOCK_NS_MPU_A::NONSEC_MPU_NOT_LOCKED
    }
}
#[doc = "Field `LOCK_NS_MPU` writer - Lock Non-Secure MPU"]
pub type LOCK_NS_MPU_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CM33_LOCK_REG_SPEC, u8, LOCK_NS_MPU_A, 2, O>;
impl<'a, const O: u8> LOCK_NS_MPU_W<'a, O> {
    #[doc = "Locks Non-Secure MPU"]
    #[inline(always)]
    pub fn nonsec_mpu_locked(self) -> &'a mut W {
        self.variant(LOCK_NS_MPU_A::NONSEC_MPU_LOCKED)
    }
    #[doc = "Non-Secure MPU can be used"]
    #[inline(always)]
    pub fn nonsec_mpu_not_locked(self) -> &'a mut W {
        self.variant(LOCK_NS_MPU_A::NONSEC_MPU_NOT_LOCKED)
    }
}
#[doc = "Field `LOCK_S_VTOR` reader - Lock Secure VTOR"]
pub type LOCK_S_VTOR_R = crate::FieldReader<u8, LOCK_S_VTOR_A>;
#[doc = "Lock Secure VTOR\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK_S_VTOR_A {
    #[doc = "1: Locks Secure VTOR"]
    SEC_VTOR_LOCKED = 1,
    #[doc = "2: Secure VTOR can be used"]
    SEC_VTOR_NOT_LOCKED = 2,
}
impl From<LOCK_S_VTOR_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_S_VTOR_A) -> Self {
        variant as _
    }
}
impl LOCK_S_VTOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_S_VTOR_A> {
        match self.bits {
            1 => Some(LOCK_S_VTOR_A::SEC_VTOR_LOCKED),
            2 => Some(LOCK_S_VTOR_A::SEC_VTOR_NOT_LOCKED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SEC_VTOR_LOCKED`"]
    #[inline(always)]
    pub fn is_sec_vtor_locked(&self) -> bool {
        *self == LOCK_S_VTOR_A::SEC_VTOR_LOCKED
    }
    #[doc = "Checks if the value of the field is `SEC_VTOR_NOT_LOCKED`"]
    #[inline(always)]
    pub fn is_sec_vtor_not_locked(&self) -> bool {
        *self == LOCK_S_VTOR_A::SEC_VTOR_NOT_LOCKED
    }
}
#[doc = "Field `LOCK_S_VTOR` writer - Lock Secure VTOR"]
pub type LOCK_S_VTOR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CM33_LOCK_REG_SPEC, u8, LOCK_S_VTOR_A, 2, O>;
impl<'a, const O: u8> LOCK_S_VTOR_W<'a, O> {
    #[doc = "Locks Secure VTOR"]
    #[inline(always)]
    pub fn sec_vtor_locked(self) -> &'a mut W {
        self.variant(LOCK_S_VTOR_A::SEC_VTOR_LOCKED)
    }
    #[doc = "Secure VTOR can be used"]
    #[inline(always)]
    pub fn sec_vtor_not_locked(self) -> &'a mut W {
        self.variant(LOCK_S_VTOR_A::SEC_VTOR_NOT_LOCKED)
    }
}
#[doc = "Field `LOCK_S_MPU` reader - Lock Secure MPU"]
pub type LOCK_S_MPU_R = crate::FieldReader<u8, LOCK_S_MPU_A>;
#[doc = "Lock Secure MPU\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK_S_MPU_A {
    #[doc = "1: Locks Secure MPU"]
    S_MPU_LOCKED = 1,
    #[doc = "2: Secure MPU can be used"]
    S_MPU_NOT_LOCKED = 2,
}
impl From<LOCK_S_MPU_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_S_MPU_A) -> Self {
        variant as _
    }
}
impl LOCK_S_MPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_S_MPU_A> {
        match self.bits {
            1 => Some(LOCK_S_MPU_A::S_MPU_LOCKED),
            2 => Some(LOCK_S_MPU_A::S_MPU_NOT_LOCKED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `S_MPU_LOCKED`"]
    #[inline(always)]
    pub fn is_s_mpu_locked(&self) -> bool {
        *self == LOCK_S_MPU_A::S_MPU_LOCKED
    }
    #[doc = "Checks if the value of the field is `S_MPU_NOT_LOCKED`"]
    #[inline(always)]
    pub fn is_s_mpu_not_locked(&self) -> bool {
        *self == LOCK_S_MPU_A::S_MPU_NOT_LOCKED
    }
}
#[doc = "Field `LOCK_S_MPU` writer - Lock Secure MPU"]
pub type LOCK_S_MPU_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CM33_LOCK_REG_SPEC, u8, LOCK_S_MPU_A, 2, O>;
impl<'a, const O: u8> LOCK_S_MPU_W<'a, O> {
    #[doc = "Locks Secure MPU"]
    #[inline(always)]
    pub fn s_mpu_locked(self) -> &'a mut W {
        self.variant(LOCK_S_MPU_A::S_MPU_LOCKED)
    }
    #[doc = "Secure MPU can be used"]
    #[inline(always)]
    pub fn s_mpu_not_locked(self) -> &'a mut W {
        self.variant(LOCK_S_MPU_A::S_MPU_NOT_LOCKED)
    }
}
#[doc = "Field `LOCK_SAU` reader - Lock SAU"]
pub type LOCK_SAU_R = crate::FieldReader<u8, LOCK_SAU_A>;
#[doc = "Lock SAU\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK_SAU_A {
    #[doc = "1: SAU is locked"]
    SAU_LOCKED = 1,
    #[doc = "2: SAU can be used"]
    SAU_NOT_LOCKED = 2,
}
impl From<LOCK_SAU_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_SAU_A) -> Self {
        variant as _
    }
}
impl LOCK_SAU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_SAU_A> {
        match self.bits {
            1 => Some(LOCK_SAU_A::SAU_LOCKED),
            2 => Some(LOCK_SAU_A::SAU_NOT_LOCKED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SAU_LOCKED`"]
    #[inline(always)]
    pub fn is_sau_locked(&self) -> bool {
        *self == LOCK_SAU_A::SAU_LOCKED
    }
    #[doc = "Checks if the value of the field is `SAU_NOT_LOCKED`"]
    #[inline(always)]
    pub fn is_sau_not_locked(&self) -> bool {
        *self == LOCK_SAU_A::SAU_NOT_LOCKED
    }
}
#[doc = "Field `LOCK_SAU` writer - Lock SAU"]
pub type LOCK_SAU_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CM33_LOCK_REG_SPEC, u8, LOCK_SAU_A, 2, O>;
impl<'a, const O: u8> LOCK_SAU_W<'a, O> {
    #[doc = "SAU is locked"]
    #[inline(always)]
    pub fn sau_locked(self) -> &'a mut W {
        self.variant(LOCK_SAU_A::SAU_LOCKED)
    }
    #[doc = "SAU can be used"]
    #[inline(always)]
    pub fn sau_not_locked(self) -> &'a mut W {
        self.variant(LOCK_SAU_A::SAU_NOT_LOCKED)
    }
}
#[doc = "Field `CM33_LOCK_REG_LOCK` reader - Lock CM33 Lock Register"]
pub type CM33_LOCK_REG_LOCK_R = crate::FieldReader<u8, CM33_LOCK_REG_LOCK_A>;
#[doc = "Lock CM33 Lock Register\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CM33_LOCK_REG_LOCK_A {
    #[doc = "1: Does not allow writing to this register"]
    WRITE_NOT_ALLOWED = 1,
    #[doc = "2: Allows writing to this register"]
    WRITE_ALLOWED = 2,
}
impl From<CM33_LOCK_REG_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: CM33_LOCK_REG_LOCK_A) -> Self {
        variant as _
    }
}
impl CM33_LOCK_REG_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CM33_LOCK_REG_LOCK_A> {
        match self.bits {
            1 => Some(CM33_LOCK_REG_LOCK_A::WRITE_NOT_ALLOWED),
            2 => Some(CM33_LOCK_REG_LOCK_A::WRITE_ALLOWED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_NOT_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_not_allowed(&self) -> bool {
        *self == CM33_LOCK_REG_LOCK_A::WRITE_NOT_ALLOWED
    }
    #[doc = "Checks if the value of the field is `WRITE_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_allowed(&self) -> bool {
        *self == CM33_LOCK_REG_LOCK_A::WRITE_ALLOWED
    }
}
#[doc = "Field `CM33_LOCK_REG_LOCK` writer - Lock CM33 Lock Register"]
pub type CM33_LOCK_REG_LOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CM33_LOCK_REG_SPEC, u8, CM33_LOCK_REG_LOCK_A, 2, O>;
impl<'a, const O: u8> CM33_LOCK_REG_LOCK_W<'a, O> {
    #[doc = "Does not allow writing to this register"]
    #[inline(always)]
    pub fn write_not_allowed(self) -> &'a mut W {
        self.variant(CM33_LOCK_REG_LOCK_A::WRITE_NOT_ALLOWED)
    }
    #[doc = "Allows writing to this register"]
    #[inline(always)]
    pub fn write_allowed(self) -> &'a mut W {
        self.variant(CM33_LOCK_REG_LOCK_A::WRITE_ALLOWED)
    }
}
impl R {
    #[doc = "Bits 0:1 - Lock Non-Secure VTOR"]
    #[inline(always)]
    pub fn lock_ns_vtor(&self) -> LOCK_NS_VTOR_R {
        LOCK_NS_VTOR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Lock Non-Secure MPU"]
    #[inline(always)]
    pub fn lock_ns_mpu(&self) -> LOCK_NS_MPU_R {
        LOCK_NS_MPU_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Lock Secure VTOR"]
    #[inline(always)]
    pub fn lock_s_vtor(&self) -> LOCK_S_VTOR_R {
        LOCK_S_VTOR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Lock Secure MPU"]
    #[inline(always)]
    pub fn lock_s_mpu(&self) -> LOCK_S_MPU_R {
        LOCK_S_MPU_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Lock SAU"]
    #[inline(always)]
    pub fn lock_sau(&self) -> LOCK_SAU_R {
        LOCK_SAU_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Lock CM33 Lock Register"]
    #[inline(always)]
    pub fn cm33_lock_reg_lock(&self) -> CM33_LOCK_REG_LOCK_R {
        CM33_LOCK_REG_LOCK_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Lock Non-Secure VTOR"]
    #[inline(always)]
    #[must_use]
    pub fn lock_ns_vtor(&mut self) -> LOCK_NS_VTOR_W<0> {
        LOCK_NS_VTOR_W::new(self)
    }
    #[doc = "Bits 2:3 - Lock Non-Secure MPU"]
    #[inline(always)]
    #[must_use]
    pub fn lock_ns_mpu(&mut self) -> LOCK_NS_MPU_W<2> {
        LOCK_NS_MPU_W::new(self)
    }
    #[doc = "Bits 4:5 - Lock Secure VTOR"]
    #[inline(always)]
    #[must_use]
    pub fn lock_s_vtor(&mut self) -> LOCK_S_VTOR_W<4> {
        LOCK_S_VTOR_W::new(self)
    }
    #[doc = "Bits 6:7 - Lock Secure MPU"]
    #[inline(always)]
    #[must_use]
    pub fn lock_s_mpu(&mut self) -> LOCK_S_MPU_W<6> {
        LOCK_S_MPU_W::new(self)
    }
    #[doc = "Bits 8:9 - Lock SAU"]
    #[inline(always)]
    #[must_use]
    pub fn lock_sau(&mut self) -> LOCK_SAU_W<8> {
        LOCK_SAU_W::new(self)
    }
    #[doc = "Bits 30:31 - Lock CM33 Lock Register"]
    #[inline(always)]
    #[must_use]
    pub fn cm33_lock_reg_lock(&mut self) -> CM33_LOCK_REG_LOCK_W<30> {
        CM33_LOCK_REG_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Miscellaneous CPU0 Control Signals Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm33_lock_reg](index.html) module"]
pub struct CM33_LOCK_REG_SPEC;
impl crate::RegisterSpec for CM33_LOCK_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm33_lock_reg::R](R) reader structure"]
impl crate::Readable for CM33_LOCK_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm33_lock_reg::W](W) writer structure"]
impl crate::Writable for CM33_LOCK_REG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM33_LOCK_REG to value 0x8000_02aa"]
impl crate::Resettable for CM33_LOCK_REG_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_02aa;
}
