#[doc = "Register `MSTATUS` reader"]
pub struct R(crate::R<MSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSTATUS` writer"]
pub struct W(crate::W<MSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSTATUS_SPEC>;
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
impl From<crate::W<MSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATE` reader - State of the master"]
pub type STATE_R = crate::FieldReader<u8, STATE_A>;
#[doc = "State of the master\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STATE_A {
    #[doc = "0: IDLE"]
    IDLE = 0,
    #[doc = "1: SLVREQ"]
    SLVREQ = 1,
    #[doc = "2: MSGSDR"]
    MSGSDR = 2,
    #[doc = "3: NORMACT"]
    NORMACT = 3,
    #[doc = "4: MSGDDR"]
    DDR = 4,
    #[doc = "5: DAA"]
    DAA = 5,
    #[doc = "6: IBIACK"]
    IBIACK = 6,
    #[doc = "7: IBIRCV"]
    IBIRCV = 7,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as _
    }
}
impl STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATE_A {
        match self.bits {
            0 => STATE_A::IDLE,
            1 => STATE_A::SLVREQ,
            2 => STATE_A::MSGSDR,
            3 => STATE_A::NORMACT,
            4 => STATE_A::DDR,
            5 => STATE_A::DAA,
            6 => STATE_A::IBIACK,
            7 => STATE_A::IBIRCV,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == STATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `SLVREQ`"]
    #[inline(always)]
    pub fn is_slvreq(&self) -> bool {
        *self == STATE_A::SLVREQ
    }
    #[doc = "Checks if the value of the field is `MSGSDR`"]
    #[inline(always)]
    pub fn is_msgsdr(&self) -> bool {
        *self == STATE_A::MSGSDR
    }
    #[doc = "Checks if the value of the field is `NORMACT`"]
    #[inline(always)]
    pub fn is_normact(&self) -> bool {
        *self == STATE_A::NORMACT
    }
    #[doc = "Checks if the value of the field is `DDR`"]
    #[inline(always)]
    pub fn is_ddr(&self) -> bool {
        *self == STATE_A::DDR
    }
    #[doc = "Checks if the value of the field is `DAA`"]
    #[inline(always)]
    pub fn is_daa(&self) -> bool {
        *self == STATE_A::DAA
    }
    #[doc = "Checks if the value of the field is `IBIACK`"]
    #[inline(always)]
    pub fn is_ibiack(&self) -> bool {
        *self == STATE_A::IBIACK
    }
    #[doc = "Checks if the value of the field is `IBIRCV`"]
    #[inline(always)]
    pub fn is_ibircv(&self) -> bool {
        *self == STATE_A::IBIRCV
    }
}
#[doc = "Field `BETWEEN` reader - Between"]
pub type BETWEEN_R = crate::BitReader<BETWEEN_A>;
#[doc = "Between\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BETWEEN_A {
    #[doc = "0: Inactive"]
    INACTIVE = 0,
    #[doc = "1: Active"]
    ACTIVE = 1,
}
impl From<BETWEEN_A> for bool {
    #[inline(always)]
    fn from(variant: BETWEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BETWEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BETWEEN_A {
        match self.bits {
            false => BETWEEN_A::INACTIVE,
            true => BETWEEN_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == BETWEEN_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BETWEEN_A::ACTIVE
    }
}
#[doc = "Field `NACKED` reader - Not acknowledged"]
pub type NACKED_R = crate::BitReader<bool>;
#[doc = "Field `IBITYPE` reader - In-Band Interrupt (IBI) type"]
pub type IBITYPE_R = crate::FieldReader<u8, IBITYPE_A>;
#[doc = "In-Band Interrupt (IBI) type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IBITYPE_A {
    #[doc = "0: NONE"]
    NONE = 0,
    #[doc = "1: IBI"]
    IBI = 1,
    #[doc = "2: MR"]
    MR = 2,
    #[doc = "3: HJ"]
    HJ = 3,
}
impl From<IBITYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: IBITYPE_A) -> Self {
        variant as _
    }
}
impl IBITYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBITYPE_A {
        match self.bits {
            0 => IBITYPE_A::NONE,
            1 => IBITYPE_A::IBI,
            2 => IBITYPE_A::MR,
            3 => IBITYPE_A::HJ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == IBITYPE_A::NONE
    }
    #[doc = "Checks if the value of the field is `IBI`"]
    #[inline(always)]
    pub fn is_ibi(&self) -> bool {
        *self == IBITYPE_A::IBI
    }
    #[doc = "Checks if the value of the field is `MR`"]
    #[inline(always)]
    pub fn is_mr(&self) -> bool {
        *self == IBITYPE_A::MR
    }
    #[doc = "Checks if the value of the field is `HJ`"]
    #[inline(always)]
    pub fn is_hj(&self) -> bool {
        *self == IBITYPE_A::HJ
    }
}
#[doc = "Field `SLVSTART` reader - Slave start"]
pub type SLVSTART_R = crate::BitReader<bool>;
#[doc = "Field `SLVSTART` writer - Slave start"]
pub type SLVSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTATUS_SPEC, bool, O>;
#[doc = "Field `MCTRLDONE` reader - Master control done"]
pub type MCTRLDONE_R = crate::BitReader<bool>;
#[doc = "Field `MCTRLDONE` writer - Master control done"]
pub type MCTRLDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTATUS_SPEC, bool, O>;
#[doc = "Field `COMPLETE` reader - COMPLETE"]
pub type COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `COMPLETE` writer - COMPLETE"]
pub type COMPLETE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTATUS_SPEC, bool, O>;
#[doc = "Field `RXPEND` reader - RXPEND"]
pub type RXPEND_R = crate::BitReader<bool>;
#[doc = "Field `TXNOTFULL` reader - TX buffer/FIFO not yet full"]
pub type TXNOTFULL_R = crate::BitReader<bool>;
#[doc = "Field `IBIWON` reader - In-Band Interrupt (IBI) won"]
pub type IBIWON_R = crate::BitReader<bool>;
#[doc = "Field `IBIWON` writer - In-Band Interrupt (IBI) won"]
pub type IBIWON_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTATUS_SPEC, bool, O>;
#[doc = "Field `ERRWARN` reader - Error or warning"]
pub type ERRWARN_R = crate::BitReader<bool>;
#[doc = "Field `NOWMASTER` reader - Now master (now this module is a master)"]
pub type NOWMASTER_R = crate::BitReader<bool>;
#[doc = "Field `NOWMASTER` writer - Now master (now this module is a master)"]
pub type NOWMASTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTATUS_SPEC, bool, O>;
#[doc = "Field `IBIADDR` reader - IBI address"]
pub type IBIADDR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - State of the master"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Between"]
    #[inline(always)]
    pub fn between(&self) -> BETWEEN_R {
        BETWEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Not acknowledged"]
    #[inline(always)]
    pub fn nacked(&self) -> NACKED_R {
        NACKED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - In-Band Interrupt (IBI) type"]
    #[inline(always)]
    pub fn ibitype(&self) -> IBITYPE_R {
        IBITYPE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Slave start"]
    #[inline(always)]
    pub fn slvstart(&self) -> SLVSTART_R {
        SLVSTART_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Master control done"]
    #[inline(always)]
    pub fn mctrldone(&self) -> MCTRLDONE_R {
        MCTRLDONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - COMPLETE"]
    #[inline(always)]
    pub fn complete(&self) -> COMPLETE_R {
        COMPLETE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RXPEND"]
    #[inline(always)]
    pub fn rxpend(&self) -> RXPEND_R {
        RXPEND_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TX buffer/FIFO not yet full"]
    #[inline(always)]
    pub fn txnotfull(&self) -> TXNOTFULL_R {
        TXNOTFULL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - In-Band Interrupt (IBI) won"]
    #[inline(always)]
    pub fn ibiwon(&self) -> IBIWON_R {
        IBIWON_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Error or warning"]
    #[inline(always)]
    pub fn errwarn(&self) -> ERRWARN_R {
        ERRWARN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - Now master (now this module is a master)"]
    #[inline(always)]
    pub fn nowmaster(&self) -> NOWMASTER_R {
        NOWMASTER_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:30 - IBI address"]
    #[inline(always)]
    pub fn ibiaddr(&self) -> IBIADDR_R {
        IBIADDR_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Slave start"]
    #[inline(always)]
    #[must_use]
    pub fn slvstart(&mut self) -> SLVSTART_W<8> {
        SLVSTART_W::new(self)
    }
    #[doc = "Bit 9 - Master control done"]
    #[inline(always)]
    #[must_use]
    pub fn mctrldone(&mut self) -> MCTRLDONE_W<9> {
        MCTRLDONE_W::new(self)
    }
    #[doc = "Bit 10 - COMPLETE"]
    #[inline(always)]
    #[must_use]
    pub fn complete(&mut self) -> COMPLETE_W<10> {
        COMPLETE_W::new(self)
    }
    #[doc = "Bit 13 - In-Band Interrupt (IBI) won"]
    #[inline(always)]
    #[must_use]
    pub fn ibiwon(&mut self) -> IBIWON_W<13> {
        IBIWON_W::new(self)
    }
    #[doc = "Bit 19 - Now master (now this module is a master)"]
    #[inline(always)]
    #[must_use]
    pub fn nowmaster(&mut self) -> NOWMASTER_W<19> {
        NOWMASTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mstatus](index.html) module"]
pub struct MSTATUS_SPEC;
impl crate::RegisterSpec for MSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mstatus::R](R) reader structure"]
impl crate::Readable for MSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mstatus::W](W) writer structure"]
impl crate::Writable for MSTATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSTATUS to value 0x1000"]
impl crate::Resettable for MSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
