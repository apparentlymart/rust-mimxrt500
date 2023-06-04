#[doc = "Register `MOD` reader"]
pub struct R(crate::R<MOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOD` writer"]
pub struct W(crate::W<MOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOD_SPEC>;
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
impl From<crate::W<MOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDEN` reader - Watchdog Enable"]
pub type WDEN_R = crate::BitReader<WDEN_A>;
#[doc = "Watchdog Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDEN_A {
    #[doc = "0: Stop. The Watchdog timer is stopped."]
    STOP = 0,
    #[doc = "1: Run. The Watchdog timer is running."]
    RUN = 1,
}
impl From<WDEN_A> for bool {
    #[inline(always)]
    fn from(variant: WDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDEN_A {
        match self.bits {
            false => WDEN_A::STOP,
            true => WDEN_A::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == WDEN_A::STOP
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == WDEN_A::RUN
    }
}
#[doc = "Field `WDEN` writer - Watchdog Enable"]
pub type WDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MOD_SPEC, WDEN_A, O>;
impl<'a, const O: u8> WDEN_W<'a, O> {
    #[doc = "Stop. The Watchdog timer is stopped."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(WDEN_A::STOP)
    }
    #[doc = "Run. The Watchdog timer is running."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(WDEN_A::RUN)
    }
}
#[doc = "Field `WDRESET` reader - Watchdog Reset Enable"]
pub type WDRESET_R = crate::BitReader<WDRESET_A>;
#[doc = "Watchdog Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDRESET_A {
    #[doc = "0: Interrupt. A Watchdog timeout will not cause a chip reset."]
    INTERRUPT = 0,
    #[doc = "1: Reset. A Watchdog timeout will cause a chip reset."]
    RESET = 1,
}
impl From<WDRESET_A> for bool {
    #[inline(always)]
    fn from(variant: WDRESET_A) -> Self {
        variant as u8 != 0
    }
}
impl WDRESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDRESET_A {
        match self.bits {
            false => WDRESET_A::INTERRUPT,
            true => WDRESET_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == WDRESET_A::INTERRUPT
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WDRESET_A::RESET
    }
}
#[doc = "Field `WDRESET` writer - Watchdog Reset Enable"]
pub type WDRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, MOD_SPEC, WDRESET_A, O>;
impl<'a, const O: u8> WDRESET_W<'a, O> {
    #[doc = "Interrupt. A Watchdog timeout will not cause a chip reset."]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(WDRESET_A::INTERRUPT)
    }
    #[doc = "Reset. A Watchdog timeout will cause a chip reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(WDRESET_A::RESET)
    }
}
#[doc = "Field `WDTOF` reader - Watchdog Timeout Flag"]
pub type WDTOF_R = crate::BitReader<WDTOF_A>;
#[doc = "Watchdog Timeout Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTOF_A {
    #[doc = "0: Clear."]
    CLEAR = 0,
    #[doc = "1: Reset. Causes a chip reset if WDRESET = 1."]
    RESET = 1,
}
impl From<WDTOF_A> for bool {
    #[inline(always)]
    fn from(variant: WDTOF_A) -> Self {
        variant as u8 != 0
    }
}
impl WDTOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTOF_A {
        match self.bits {
            false => WDTOF_A::CLEAR,
            true => WDTOF_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WDTOF_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WDTOF_A::RESET
    }
}
#[doc = "Field `WDTOF` writer - Watchdog Timeout Flag"]
pub type WDTOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MOD_SPEC, WDTOF_A, O>;
impl<'a, const O: u8> WDTOF_W<'a, O> {
    #[doc = "Clear."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WDTOF_A::CLEAR)
    }
    #[doc = "Reset. Causes a chip reset if WDRESET = 1."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(WDTOF_A::RESET)
    }
}
#[doc = "Field `WDINT` reader - Warning Interrupt Flag"]
pub type WDINT_R = crate::BitReader<WDINT_A>;
#[doc = "Warning Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDINT_A {
    #[doc = "0: No flag."]
    NO_FLAG = 0,
    #[doc = "1: Flag. The Watchdog interrupt flag is set when the Watchdog counter is no longer greater than the value specified by WARNINT."]
    FLAG = 1,
}
impl From<WDINT_A> for bool {
    #[inline(always)]
    fn from(variant: WDINT_A) -> Self {
        variant as u8 != 0
    }
}
impl WDINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDINT_A {
        match self.bits {
            false => WDINT_A::NO_FLAG,
            true => WDINT_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == WDINT_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == WDINT_A::FLAG
    }
}
#[doc = "Field `WDINT` writer - Warning Interrupt Flag"]
pub type WDINT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, MOD_SPEC, WDINT_A, O>;
impl<'a, const O: u8> WDINT_W<'a, O> {
    #[doc = "No flag."]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(WDINT_A::NO_FLAG)
    }
    #[doc = "Flag. The Watchdog interrupt flag is set when the Watchdog counter is no longer greater than the value specified by WARNINT."]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(WDINT_A::FLAG)
    }
}
#[doc = "Field `WDPROTECT` reader - Watchdog Update Mode"]
pub type WDPROTECT_R = crate::BitReader<WDPROTECT_A>;
#[doc = "Watchdog Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDPROTECT_A {
    #[doc = "0: Flexible"]
    FLEXIBLE = 0,
    #[doc = "1: Threshold"]
    THRESHOLD = 1,
}
impl From<WDPROTECT_A> for bool {
    #[inline(always)]
    fn from(variant: WDPROTECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WDPROTECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDPROTECT_A {
        match self.bits {
            false => WDPROTECT_A::FLEXIBLE,
            true => WDPROTECT_A::THRESHOLD,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIBLE`"]
    #[inline(always)]
    pub fn is_flexible(&self) -> bool {
        *self == WDPROTECT_A::FLEXIBLE
    }
    #[doc = "Checks if the value of the field is `THRESHOLD`"]
    #[inline(always)]
    pub fn is_threshold(&self) -> bool {
        *self == WDPROTECT_A::THRESHOLD
    }
}
#[doc = "Field `WDPROTECT` writer - Watchdog Update Mode"]
pub type WDPROTECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MOD_SPEC, WDPROTECT_A, O>;
impl<'a, const O: u8> WDPROTECT_W<'a, O> {
    #[doc = "Flexible"]
    #[inline(always)]
    pub fn flexible(self) -> &'a mut W {
        self.variant(WDPROTECT_A::FLEXIBLE)
    }
    #[doc = "Threshold"]
    #[inline(always)]
    pub fn threshold(self) -> &'a mut W {
        self.variant(WDPROTECT_A::THRESHOLD)
    }
}
#[doc = "Field `LOCK` reader - Lock"]
pub type LOCK_R = crate::BitReader<LOCK_A>;
#[doc = "Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_A {
    #[doc = "0: No Lock"]
    NO_LOCK = 0,
    #[doc = "1: Lock"]
    LOCK = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::NO_LOCK,
            true => LOCK_A::LOCK,
        }
    }
    #[doc = "Checks if the value of the field is `NO_LOCK`"]
    #[inline(always)]
    pub fn is_no_lock(&self) -> bool {
        *self == LOCK_A::NO_LOCK
    }
    #[doc = "Checks if the value of the field is `LOCK`"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == LOCK_A::LOCK
    }
}
#[doc = "Field `LOCK` writer - Lock"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, MOD_SPEC, LOCK_A, O>;
impl<'a, const O: u8> LOCK_W<'a, O> {
    #[doc = "No Lock"]
    #[inline(always)]
    pub fn no_lock(self) -> &'a mut W {
        self.variant(LOCK_A::NO_LOCK)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LOCK_A::LOCK)
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog Enable"]
    #[inline(always)]
    pub fn wden(&self) -> WDEN_R {
        WDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog Reset Enable"]
    #[inline(always)]
    pub fn wdreset(&self) -> WDRESET_R {
        WDRESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Watchdog Timeout Flag"]
    #[inline(always)]
    pub fn wdtof(&self) -> WDTOF_R {
        WDTOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Warning Interrupt Flag"]
    #[inline(always)]
    pub fn wdint(&self) -> WDINT_R {
        WDINT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Watchdog Update Mode"]
    #[inline(always)]
    pub fn wdprotect(&self) -> WDPROTECT_R {
        WDPROTECT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wden(&mut self) -> WDEN_W<0> {
        WDEN_W::new(self)
    }
    #[doc = "Bit 1 - Watchdog Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdreset(&mut self) -> WDRESET_W<1> {
        WDRESET_W::new(self)
    }
    #[doc = "Bit 2 - Watchdog Timeout Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wdtof(&mut self) -> WDTOF_W<2> {
        WDTOF_W::new(self)
    }
    #[doc = "Bit 3 - Warning Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wdint(&mut self) -> WDINT_W<3> {
        WDINT_W::new(self)
    }
    #[doc = "Bit 4 - Watchdog Update Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wdprotect(&mut self) -> WDPROTECT_W<4> {
        WDPROTECT_W::new(self)
    }
    #[doc = "Bit 5 - Lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<5> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mod_](index.html) module"]
pub struct MOD_SPEC;
impl crate::RegisterSpec for MOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mod_::R](R) reader structure"]
impl crate::Readable for MOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mod_::W](W) writer structure"]
impl crate::Writable for MOD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x08;
}
#[doc = "`reset()` method sets MOD to value 0"]
impl crate::Resettable for MOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
