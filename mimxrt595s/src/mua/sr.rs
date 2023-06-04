#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Fn` reader - Fn"]
pub type FN_R = crate::FieldReader<u8, FN_A>;
#[doc = "Fn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FN_A {
    #[doc = "0: Fn bit in the MUB CR register is written 0 (default)."]
    ZERO = 0,
    #[doc = "1: Fn bit in the MUB CR register is written 1."]
    ONE = 1,
}
impl From<FN_A> for u8 {
    #[inline(always)]
    fn from(variant: FN_A) -> Self {
        variant as _
    }
}
impl FN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FN_A> {
        match self.bits {
            0 => Some(FN_A::ZERO),
            1 => Some(FN_A::ONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == FN_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == FN_A::ONE
    }
}
#[doc = "Field `EP` reader - EP"]
pub type EP_R = crate::BitReader<EP_A>;
#[doc = "EP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EP_A {
    #[doc = "0: The MUA side event is not pending (default)."]
    NOT_PENDING = 0,
    #[doc = "1: The MUA side event is pending."]
    PENDING = 1,
}
impl From<EP_A> for bool {
    #[inline(always)]
    fn from(variant: EP_A) -> Self {
        variant as u8 != 0
    }
}
impl EP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP_A {
        match self.bits {
            false => EP_A::NOT_PENDING,
            true => EP_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == EP_A::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == EP_A::PENDING
    }
}
#[doc = "Field `PM` reader - PM"]
pub type PM_R = crate::FieldReader<u8, PM_A>;
#[doc = "PM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PM_A {
    #[doc = "0: The MUB processor is in Run Mode."]
    RUN = 0,
    #[doc = "1: The MUB processor is in WAIT Mode."]
    WAIT = 1,
    #[doc = "2: The MUB processor is in STOP/VLPS Mode."]
    STOP = 2,
    #[doc = "3: The MUB processor is in LLS/VLLS Mode."]
    DSM = 3,
}
impl From<PM_A> for u8 {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as _
    }
}
impl PM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PM_A {
        match self.bits {
            0 => PM_A::RUN,
            1 => PM_A::WAIT,
            2 => PM_A::STOP,
            3 => PM_A::DSM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == PM_A::RUN
    }
    #[doc = "Checks if the value of the field is `WAIT`"]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == PM_A::WAIT
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == PM_A::STOP
    }
    #[doc = "Checks if the value of the field is `DSM`"]
    #[inline(always)]
    pub fn is_dsm(&self) -> bool {
        *self == PM_A::DSM
    }
}
#[doc = "Field `RS` reader - RS"]
pub type RS_R = crate::BitReader<RS_A>;
#[doc = "RS\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RS_A {
    #[doc = "0: The MUB side of the MU is not in reset."]
    NOT_RESET = 0,
    #[doc = "1: The MUB side of the MU is in reset."]
    RESET = 1,
}
impl From<RS_A> for bool {
    #[inline(always)]
    fn from(variant: RS_A) -> Self {
        variant as u8 != 0
    }
}
impl RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RS_A {
        match self.bits {
            false => RS_A::NOT_RESET,
            true => RS_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_RESET`"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == RS_A::NOT_RESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RS_A::RESET
    }
}
#[doc = "Field `FUP` reader - FUP"]
pub type FUP_R = crate::BitReader<FUP_A>;
#[doc = "FUP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FUP_A {
    #[doc = "0: No flags updated, initiated by the MUA, in progress (default)"]
    NO_FUP = 0,
    #[doc = "1: MUA initiated flags update, processing"]
    FUP = 1,
}
impl From<FUP_A> for bool {
    #[inline(always)]
    fn from(variant: FUP_A) -> Self {
        variant as u8 != 0
    }
}
impl FUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FUP_A {
        match self.bits {
            false => FUP_A::NO_FUP,
            true => FUP_A::FUP,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FUP`"]
    #[inline(always)]
    pub fn is_no_fup(&self) -> bool {
        *self == FUP_A::NO_FUP
    }
    #[doc = "Checks if the value of the field is `FUP`"]
    #[inline(always)]
    pub fn is_fup(&self) -> bool {
        *self == FUP_A::FUP
    }
}
#[doc = "Field `RDIP` reader - RDIP"]
pub type RDIP_R = crate::BitReader<RDIP_A>;
#[doc = "RDIP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDIP_A {
    #[doc = "0: Processor B-side did not exit reset"]
    NO_EXIT_RST = 0,
    #[doc = "1: Processor B-side exited from reset"]
    EXIT_RST = 1,
}
impl From<RDIP_A> for bool {
    #[inline(always)]
    fn from(variant: RDIP_A) -> Self {
        variant as u8 != 0
    }
}
impl RDIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDIP_A {
        match self.bits {
            false => RDIP_A::NO_EXIT_RST,
            true => RDIP_A::EXIT_RST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EXIT_RST`"]
    #[inline(always)]
    pub fn is_no_exit_rst(&self) -> bool {
        *self == RDIP_A::NO_EXIT_RST
    }
    #[doc = "Checks if the value of the field is `EXIT_RST`"]
    #[inline(always)]
    pub fn is_exit_rst(&self) -> bool {
        *self == RDIP_A::EXIT_RST
    }
}
#[doc = "Field `RDIP` writer - RDIP"]
pub type RDIP_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SR_SPEC, RDIP_A, O>;
impl<'a, const O: u8> RDIP_W<'a, O> {
    #[doc = "Processor B-side did not exit reset"]
    #[inline(always)]
    pub fn no_exit_rst(self) -> &'a mut W {
        self.variant(RDIP_A::NO_EXIT_RST)
    }
    #[doc = "Processor B-side exited from reset"]
    #[inline(always)]
    pub fn exit_rst(self) -> &'a mut W {
        self.variant(RDIP_A::EXIT_RST)
    }
}
#[doc = "Field `RAIP` reader - RAIP"]
pub type RAIP_R = crate::BitReader<RAIP_A>;
#[doc = "RAIP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAIP_A {
    #[doc = "0: Processor B-side did not enter reset"]
    NO_ENTER_RST = 0,
    #[doc = "1: Processor B-side entered reset"]
    ENTER_RST = 1,
}
impl From<RAIP_A> for bool {
    #[inline(always)]
    fn from(variant: RAIP_A) -> Self {
        variant as u8 != 0
    }
}
impl RAIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAIP_A {
        match self.bits {
            false => RAIP_A::NO_ENTER_RST,
            true => RAIP_A::ENTER_RST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ENTER_RST`"]
    #[inline(always)]
    pub fn is_no_enter_rst(&self) -> bool {
        *self == RAIP_A::NO_ENTER_RST
    }
    #[doc = "Checks if the value of the field is `ENTER_RST`"]
    #[inline(always)]
    pub fn is_enter_rst(&self) -> bool {
        *self == RAIP_A::ENTER_RST
    }
}
#[doc = "Field `RAIP` writer - RAIP"]
pub type RAIP_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SR_SPEC, RAIP_A, O>;
impl<'a, const O: u8> RAIP_W<'a, O> {
    #[doc = "Processor B-side did not enter reset"]
    #[inline(always)]
    pub fn no_enter_rst(self) -> &'a mut W {
        self.variant(RAIP_A::NO_ENTER_RST)
    }
    #[doc = "Processor B-side entered reset"]
    #[inline(always)]
    pub fn enter_rst(self) -> &'a mut W {
        self.variant(RAIP_A::ENTER_RST)
    }
}
#[doc = "Field `TEn` reader - TEn"]
pub type TEN_R = crate::FieldReader<u8, TEN_A>;
#[doc = "TEn\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TEN_A {
    #[doc = "0: MUA TRn register is not empty."]
    NOT_EMPTY = 0,
    #[doc = "1: MUA TRn register is empty (default)."]
    EMPTY = 1,
}
impl From<TEN_A> for u8 {
    #[inline(always)]
    fn from(variant: TEN_A) -> Self {
        variant as _
    }
}
impl TEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TEN_A> {
        match self.bits {
            0 => Some(TEN_A::NOT_EMPTY),
            1 => Some(TEN_A::EMPTY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TEN_A::NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TEN_A::EMPTY
    }
}
#[doc = "Field `RFn` reader - RFn"]
pub type RFN_R = crate::FieldReader<u8, RFN_A>;
#[doc = "RFn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RFN_A {
    #[doc = "0: MUA RRn register is not full (default)."]
    NOT_FULL = 0,
    #[doc = "1: MUA RRn register has received data from MUB TRn register and is ready to be read by the MUA."]
    FULL = 1,
}
impl From<RFN_A> for u8 {
    #[inline(always)]
    fn from(variant: RFN_A) -> Self {
        variant as _
    }
}
impl RFN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RFN_A> {
        match self.bits {
            0 => Some(RFN_A::NOT_FULL),
            1 => Some(RFN_A::FULL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_FULL`"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RFN_A::NOT_FULL
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RFN_A::FULL
    }
}
#[doc = "Field `GIPn` reader - GIPn"]
pub type GIPN_R = crate::FieldReader<u8, GIPN_A>;
#[doc = "GIPn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GIPN_A {
    #[doc = "0: MUA general purpose interrupt n is not pending. (default)"]
    NO_GIP = 0,
    #[doc = "1: MUA general purpose interrupt n is pending."]
    GIP = 1,
}
impl From<GIPN_A> for u8 {
    #[inline(always)]
    fn from(variant: GIPN_A) -> Self {
        variant as _
    }
}
impl GIPN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GIPN_A> {
        match self.bits {
            0 => Some(GIPN_A::NO_GIP),
            1 => Some(GIPN_A::GIP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_GIP`"]
    #[inline(always)]
    pub fn is_no_gip(&self) -> bool {
        *self == GIPN_A::NO_GIP
    }
    #[doc = "Checks if the value of the field is `GIP`"]
    #[inline(always)]
    pub fn is_gip(&self) -> bool {
        *self == GIPN_A::GIP
    }
}
#[doc = "Field `GIPn` writer - GIPn"]
pub type GIPN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SR_SPEC, u8, GIPN_A, 4, O>;
impl<'a, const O: u8> GIPN_W<'a, O> {
    #[doc = "MUA general purpose interrupt n is not pending. (default)"]
    #[inline(always)]
    pub fn no_gip(self) -> &'a mut W {
        self.variant(GIPN_A::NO_GIP)
    }
    #[doc = "MUA general purpose interrupt n is pending."]
    #[inline(always)]
    pub fn gip(self) -> &'a mut W {
        self.variant(GIPN_A::GIP)
    }
}
impl R {
    #[doc = "Bits 0:2 - Fn"]
    #[inline(always)]
    pub fn fn_(&self) -> FN_R {
        FN_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - EP"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - PM"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - RS"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FUP"]
    #[inline(always)]
    pub fn fup(&self) -> FUP_R {
        FUP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RDIP"]
    #[inline(always)]
    pub fn rdip(&self) -> RDIP_R {
        RDIP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RAIP"]
    #[inline(always)]
    pub fn raip(&self) -> RAIP_R {
        RAIP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 20:23 - TEn"]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - RFn"]
    #[inline(always)]
    pub fn rfn(&self) -> RFN_R {
        RFN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - GIPn"]
    #[inline(always)]
    pub fn gipn(&self) -> GIPN_R {
        GIPN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 9 - RDIP"]
    #[inline(always)]
    #[must_use]
    pub fn rdip(&mut self) -> RDIP_W<9> {
        RDIP_W::new(self)
    }
    #[doc = "Bit 10 - RAIP"]
    #[inline(always)]
    #[must_use]
    pub fn raip(&mut self) -> RAIP_W<10> {
        RAIP_W::new(self)
    }
    #[doc = "Bits 28:31 - GIPn"]
    #[inline(always)]
    #[must_use]
    pub fn gipn(&mut self) -> GIPN_W<28> {
        GIPN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xf000_0600;
}
#[doc = "`reset()` method sets SR to value 0x00f0_0080"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x00f0_0080;
}
