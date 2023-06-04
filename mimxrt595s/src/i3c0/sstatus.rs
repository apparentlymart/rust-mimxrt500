#[doc = "Register `SSTATUS` reader"]
pub struct R(crate::R<SSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSTATUS` writer"]
pub struct W(crate::W<SSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSTATUS_SPEC>;
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
impl From<crate::W<SSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STNOTSTOP` reader - Status not stop"]
pub type STNOTSTOP_R = crate::BitReader<bool>;
#[doc = "Field `STMSG` reader - Status message"]
pub type STMSG_R = crate::BitReader<bool>;
#[doc = "Field `STCCCH` reader - Status Common Command Code Handler"]
pub type STCCCH_R = crate::BitReader<bool>;
#[doc = "Field `STREQRD` reader - Status required"]
pub type STREQRD_R = crate::BitReader<bool>;
#[doc = "Field `STREQWR` reader - Status request write"]
pub type STREQWR_R = crate::BitReader<bool>;
#[doc = "Field `STDAA` reader - Status Dynamic Address Assignment"]
pub type STDAA_R = crate::BitReader<bool>;
#[doc = "Field `STHDR` reader - Status High Data Rate"]
pub type STHDR_R = crate::BitReader<bool>;
#[doc = "Field `START` reader - Start"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Start"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSTATUS_SPEC, bool, O>;
#[doc = "Field `MATCHED` reader - Matched"]
pub type MATCHED_R = crate::BitReader<bool>;
#[doc = "Field `MATCHED` writer - Matched"]
pub type MATCHED_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSTATUS_SPEC, bool, O>;
#[doc = "Field `STOP` reader - Stop"]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - Stop"]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSTATUS_SPEC, bool, O>;
#[doc = "Field `RX_PEND` reader - Received message pending"]
pub type RX_PEND_R = crate::BitReader<bool>;
#[doc = "Field `TXNOTFULL` reader - Transmit buffer is not full"]
pub type TXNOTFULL_R = crate::BitReader<bool>;
#[doc = "Field `DACHG` reader - DACHG"]
pub type DACHG_R = crate::BitReader<bool>;
#[doc = "Field `DACHG` writer - DACHG"]
pub type DACHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSTATUS_SPEC, bool, O>;
#[doc = "Field `CCC` reader - Common Command Code"]
pub type CCC_R = crate::BitReader<bool>;
#[doc = "Field `CCC` writer - Common Command Code"]
pub type CCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSTATUS_SPEC, bool, O>;
#[doc = "Field `ERRWARN` reader - Error warning"]
pub type ERRWARN_R = crate::BitReader<bool>;
#[doc = "Field `HDRMATCH` reader - High Data Rate command match"]
pub type HDRMATCH_R = crate::BitReader<bool>;
#[doc = "Field `HDRMATCH` writer - High Data Rate command match"]
pub type HDRMATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSTATUS_SPEC, bool, O>;
#[doc = "Field `CHANDLED` reader - Common-Command-Code handled"]
pub type CHANDLED_R = crate::BitReader<bool>;
#[doc = "Field `CHANDLED` writer - Common-Command-Code handled"]
pub type CHANDLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSTATUS_SPEC, bool, O>;
#[doc = "Field `EVENT` reader - Event"]
pub type EVENT_R = crate::BitReader<bool>;
#[doc = "Field `EVENT` writer - Event"]
pub type EVENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSTATUS_SPEC, bool, O>;
#[doc = "Field `EVDET` reader - Event details"]
pub type EVDET_R = crate::FieldReader<u8, EVDET_A>;
#[doc = "Event details\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EVDET_A {
    #[doc = "0: NONE"]
    NONE = 0,
    #[doc = "1: NO_REQUEST"]
    NO_REQUEST = 1,
    #[doc = "2: NACKED"]
    NACKED = 2,
    #[doc = "3: ACKED"]
    ACKED = 3,
}
impl From<EVDET_A> for u8 {
    #[inline(always)]
    fn from(variant: EVDET_A) -> Self {
        variant as _
    }
}
impl EVDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVDET_A {
        match self.bits {
            0 => EVDET_A::NONE,
            1 => EVDET_A::NO_REQUEST,
            2 => EVDET_A::NACKED,
            3 => EVDET_A::ACKED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == EVDET_A::NONE
    }
    #[doc = "Checks if the value of the field is `NO_REQUEST`"]
    #[inline(always)]
    pub fn is_no_request(&self) -> bool {
        *self == EVDET_A::NO_REQUEST
    }
    #[doc = "Checks if the value of the field is `NACKED`"]
    #[inline(always)]
    pub fn is_nacked(&self) -> bool {
        *self == EVDET_A::NACKED
    }
    #[doc = "Checks if the value of the field is `ACKED`"]
    #[inline(always)]
    pub fn is_acked(&self) -> bool {
        *self == EVDET_A::ACKED
    }
}
#[doc = "Field `IBIDIS` reader - In-Band Interrupts are disabled"]
pub type IBIDIS_R = crate::BitReader<bool>;
#[doc = "Field `MRDIS` reader - Master requests are disabled"]
pub type MRDIS_R = crate::BitReader<bool>;
#[doc = "Field `HJDIS` reader - Hot-Join is disabled"]
pub type HJDIS_R = crate::BitReader<bool>;
#[doc = "Field `ACTSTATE` reader - Activity state from Common Command Codes (CCC)"]
pub type ACTSTATE_R = crate::FieldReader<u8, ACTSTATE_A>;
#[doc = "Activity state from Common Command Codes (CCC)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACTSTATE_A {
    #[doc = "0: NO_LATENCY"]
    NO_LATENCY = 0,
    #[doc = "1: LATENCY_1MS"]
    LATENCY_1MS = 1,
    #[doc = "2: LATENCY_100MS"]
    LATENCY_100MS = 2,
    #[doc = "3: LATENCY_10S"]
    LATENCY_10S = 3,
}
impl From<ACTSTATE_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTSTATE_A) -> Self {
        variant as _
    }
}
impl ACTSTATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTSTATE_A {
        match self.bits {
            0 => ACTSTATE_A::NO_LATENCY,
            1 => ACTSTATE_A::LATENCY_1MS,
            2 => ACTSTATE_A::LATENCY_100MS,
            3 => ACTSTATE_A::LATENCY_10S,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_LATENCY`"]
    #[inline(always)]
    pub fn is_no_latency(&self) -> bool {
        *self == ACTSTATE_A::NO_LATENCY
    }
    #[doc = "Checks if the value of the field is `LATENCY_1MS`"]
    #[inline(always)]
    pub fn is_latency_1ms(&self) -> bool {
        *self == ACTSTATE_A::LATENCY_1MS
    }
    #[doc = "Checks if the value of the field is `LATENCY_100MS`"]
    #[inline(always)]
    pub fn is_latency_100ms(&self) -> bool {
        *self == ACTSTATE_A::LATENCY_100MS
    }
    #[doc = "Checks if the value of the field is `LATENCY_10S`"]
    #[inline(always)]
    pub fn is_latency_10s(&self) -> bool {
        *self == ACTSTATE_A::LATENCY_10S
    }
}
#[doc = "Field `TIMECTRL` reader - Time control"]
pub type TIMECTRL_R = crate::FieldReader<u8, TIMECTRL_A>;
#[doc = "Time control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMECTRL_A {
    #[doc = "0: NO_TIME_CONTROL"]
    NO_TIME_CONTROL = 0,
    #[doc = "2: ASYNC_MODE"]
    ASYNC_MODE = 2,
}
impl From<TIMECTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMECTRL_A) -> Self {
        variant as _
    }
}
impl TIMECTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIMECTRL_A> {
        match self.bits {
            0 => Some(TIMECTRL_A::NO_TIME_CONTROL),
            2 => Some(TIMECTRL_A::ASYNC_MODE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TIME_CONTROL`"]
    #[inline(always)]
    pub fn is_no_time_control(&self) -> bool {
        *self == TIMECTRL_A::NO_TIME_CONTROL
    }
    #[doc = "Checks if the value of the field is `ASYNC_MODE`"]
    #[inline(always)]
    pub fn is_async_mode(&self) -> bool {
        *self == TIMECTRL_A::ASYNC_MODE
    }
}
impl R {
    #[doc = "Bit 0 - Status not stop"]
    #[inline(always)]
    pub fn stnotstop(&self) -> STNOTSTOP_R {
        STNOTSTOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status message"]
    #[inline(always)]
    pub fn stmsg(&self) -> STMSG_R {
        STMSG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status Common Command Code Handler"]
    #[inline(always)]
    pub fn stccch(&self) -> STCCCH_R {
        STCCCH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status required"]
    #[inline(always)]
    pub fn streqrd(&self) -> STREQRD_R {
        STREQRD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Status request write"]
    #[inline(always)]
    pub fn streqwr(&self) -> STREQWR_R {
        STREQWR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status Dynamic Address Assignment"]
    #[inline(always)]
    pub fn stdaa(&self) -> STDAA_R {
        STDAA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Status High Data Rate"]
    #[inline(always)]
    pub fn sthdr(&self) -> STHDR_R {
        STHDR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Start"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Matched"]
    #[inline(always)]
    pub fn matched(&self) -> MATCHED_R {
        MATCHED_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Stop"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Received message pending"]
    #[inline(always)]
    pub fn rx_pend(&self) -> RX_PEND_R {
        RX_PEND_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit buffer is not full"]
    #[inline(always)]
    pub fn txnotfull(&self) -> TXNOTFULL_R {
        TXNOTFULL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DACHG"]
    #[inline(always)]
    pub fn dachg(&self) -> DACHG_R {
        DACHG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Common Command Code"]
    #[inline(always)]
    pub fn ccc(&self) -> CCC_R {
        CCC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Error warning"]
    #[inline(always)]
    pub fn errwarn(&self) -> ERRWARN_R {
        ERRWARN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - High Data Rate command match"]
    #[inline(always)]
    pub fn hdrmatch(&self) -> HDRMATCH_R {
        HDRMATCH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Common-Command-Code handled"]
    #[inline(always)]
    pub fn chandled(&self) -> CHANDLED_R {
        CHANDLED_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Event"]
    #[inline(always)]
    pub fn event(&self) -> EVENT_R {
        EVENT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Event details"]
    #[inline(always)]
    pub fn evdet(&self) -> EVDET_R {
        EVDET_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 24 - In-Band Interrupts are disabled"]
    #[inline(always)]
    pub fn ibidis(&self) -> IBIDIS_R {
        IBIDIS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Master requests are disabled"]
    #[inline(always)]
    pub fn mrdis(&self) -> MRDIS_R {
        MRDIS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Hot-Join is disabled"]
    #[inline(always)]
    pub fn hjdis(&self) -> HJDIS_R {
        HJDIS_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Activity state from Common Command Codes (CCC)"]
    #[inline(always)]
    pub fn actstate(&self) -> ACTSTATE_R {
        ACTSTATE_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Time control"]
    #[inline(always)]
    pub fn timectrl(&self) -> TIMECTRL_R {
        TIMECTRL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Start"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<8> {
        START_W::new(self)
    }
    #[doc = "Bit 9 - Matched"]
    #[inline(always)]
    #[must_use]
    pub fn matched(&mut self) -> MATCHED_W<9> {
        MATCHED_W::new(self)
    }
    #[doc = "Bit 10 - Stop"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<10> {
        STOP_W::new(self)
    }
    #[doc = "Bit 13 - DACHG"]
    #[inline(always)]
    #[must_use]
    pub fn dachg(&mut self) -> DACHG_W<13> {
        DACHG_W::new(self)
    }
    #[doc = "Bit 14 - Common Command Code"]
    #[inline(always)]
    #[must_use]
    pub fn ccc(&mut self) -> CCC_W<14> {
        CCC_W::new(self)
    }
    #[doc = "Bit 16 - High Data Rate command match"]
    #[inline(always)]
    #[must_use]
    pub fn hdrmatch(&mut self) -> HDRMATCH_W<16> {
        HDRMATCH_W::new(self)
    }
    #[doc = "Bit 17 - Common-Command-Code handled"]
    #[inline(always)]
    #[must_use]
    pub fn chandled(&mut self) -> CHANDLED_W<17> {
        CHANDLED_W::new(self)
    }
    #[doc = "Bit 18 - Event"]
    #[inline(always)]
    #[must_use]
    pub fn event(&mut self) -> EVENT_W<18> {
        EVENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sstatus](index.html) module"]
pub struct SSTATUS_SPEC;
impl crate::RegisterSpec for SSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sstatus::R](R) reader structure"]
impl crate::Readable for SSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sstatus::W](W) writer structure"]
impl crate::Writable for SSTATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSTATUS to value 0x1000"]
impl crate::Resettable for SSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
