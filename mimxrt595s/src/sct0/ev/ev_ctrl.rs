#[doc = "Register `EV_CTRL` reader"]
pub struct R(crate::R<EV_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EV_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EV_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EV_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EV_CTRL` writer"]
pub struct W(crate::W<EV_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EV_CTRL_SPEC>;
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
impl From<crate::W<EV_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EV_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MATCHSEL` reader - Match Select"]
pub type MATCHSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MATCHSEL` writer - Match Select"]
pub type MATCHSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EV_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `HEVENT` reader - High Event"]
pub type HEVENT_R = crate::BitReader<HEVENT_A>;
#[doc = "High Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HEVENT_A {
    #[doc = "0: Low Counter"]
    L_COUNTER = 0,
    #[doc = "1: High Counter"]
    H_COUNTER = 1,
}
impl From<HEVENT_A> for bool {
    #[inline(always)]
    fn from(variant: HEVENT_A) -> Self {
        variant as u8 != 0
    }
}
impl HEVENT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HEVENT_A {
        match self.bits {
            false => HEVENT_A::L_COUNTER,
            true => HEVENT_A::H_COUNTER,
        }
    }
    #[doc = "Checks if the value of the field is `L_COUNTER`"]
    #[inline(always)]
    pub fn is_l_counter(&self) -> bool {
        *self == HEVENT_A::L_COUNTER
    }
    #[doc = "Checks if the value of the field is `H_COUNTER`"]
    #[inline(always)]
    pub fn is_h_counter(&self) -> bool {
        *self == HEVENT_A::H_COUNTER
    }
}
#[doc = "Field `HEVENT` writer - High Event"]
pub type HEVENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EV_CTRL_SPEC, HEVENT_A, O>;
impl<'a, const O: u8> HEVENT_W<'a, O> {
    #[doc = "Low Counter"]
    #[inline(always)]
    pub fn l_counter(self) -> &'a mut W {
        self.variant(HEVENT_A::L_COUNTER)
    }
    #[doc = "High Counter"]
    #[inline(always)]
    pub fn h_counter(self) -> &'a mut W {
        self.variant(HEVENT_A::H_COUNTER)
    }
}
#[doc = "Field `OUTSEL` reader - Input/Output Select"]
pub type OUTSEL_R = crate::BitReader<OUTSEL_A>;
#[doc = "Input/Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTSEL_A {
    #[doc = "0: Selects the inputs selected by IOSEL."]
    INPUT = 0,
    #[doc = "1: Selects the outputs selected by IOSEL."]
    OUTPUT = 1,
}
impl From<OUTSEL_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl OUTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSEL_A {
        match self.bits {
            false => OUTSEL_A::INPUT,
            true => OUTSEL_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == OUTSEL_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == OUTSEL_A::OUTPUT
    }
}
#[doc = "Field `OUTSEL` writer - Input/Output Select"]
pub type OUTSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EV_CTRL_SPEC, OUTSEL_A, O>;
impl<'a, const O: u8> OUTSEL_W<'a, O> {
    #[doc = "Selects the inputs selected by IOSEL."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(OUTSEL_A::INPUT)
    }
    #[doc = "Selects the outputs selected by IOSEL."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(OUTSEL_A::OUTPUT)
    }
}
#[doc = "Field `IOSEL` reader - Input/Output Signal Select"]
pub type IOSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IOSEL` writer - Input/Output Signal Select"]
pub type IOSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EV_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `IOCOND` reader - Input/Output Condition"]
pub type IOCOND_R = crate::FieldReader<u8, IOCOND_A>;
#[doc = "Input/Output Condition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IOCOND_A {
    #[doc = "0: Low"]
    LOW = 0,
    #[doc = "1: Rise"]
    RISE = 1,
    #[doc = "2: Fall"]
    FALL = 2,
    #[doc = "3: High"]
    HIGH = 3,
}
impl From<IOCOND_A> for u8 {
    #[inline(always)]
    fn from(variant: IOCOND_A) -> Self {
        variant as _
    }
}
impl IOCOND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOCOND_A {
        match self.bits {
            0 => IOCOND_A::LOW,
            1 => IOCOND_A::RISE,
            2 => IOCOND_A::FALL,
            3 => IOCOND_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IOCOND_A::LOW
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == IOCOND_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == IOCOND_A::FALL
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IOCOND_A::HIGH
    }
}
#[doc = "Field `IOCOND` writer - Input/Output Condition"]
pub type IOCOND_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EV_CTRL_SPEC, u8, IOCOND_A, 2, O>;
impl<'a, const O: u8> IOCOND_W<'a, O> {
    #[doc = "Low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(IOCOND_A::LOW)
    }
    #[doc = "Rise"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(IOCOND_A::RISE)
    }
    #[doc = "Fall"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(IOCOND_A::FALL)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(IOCOND_A::HIGH)
    }
}
#[doc = "Field `COMBMODE` reader - Combination Mode"]
pub type COMBMODE_R = crate::FieldReader<u8, COMBMODE_A>;
#[doc = "Combination Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMBMODE_A {
    #[doc = "0: OR. The event occurs when either the specified match or I/O condition occurs."]
    OR = 0,
    #[doc = "1: MATCH. Uses the specified match only."]
    MATCH = 1,
    #[doc = "2: IO. Uses the specified I/O condition only."]
    IO = 2,
    #[doc = "3: AND. The event occurs when the specified match and I/O condition occur simultaneously."]
    AND = 3,
}
impl From<COMBMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: COMBMODE_A) -> Self {
        variant as _
    }
}
impl COMBMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMBMODE_A {
        match self.bits {
            0 => COMBMODE_A::OR,
            1 => COMBMODE_A::MATCH,
            2 => COMBMODE_A::IO,
            3 => COMBMODE_A::AND,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OR`"]
    #[inline(always)]
    pub fn is_or(&self) -> bool {
        *self == COMBMODE_A::OR
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == COMBMODE_A::MATCH
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == COMBMODE_A::IO
    }
    #[doc = "Checks if the value of the field is `AND`"]
    #[inline(always)]
    pub fn is_and(&self) -> bool {
        *self == COMBMODE_A::AND
    }
}
#[doc = "Field `COMBMODE` writer - Combination Mode"]
pub type COMBMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EV_CTRL_SPEC, u8, COMBMODE_A, 2, O>;
impl<'a, const O: u8> COMBMODE_W<'a, O> {
    #[doc = "OR. The event occurs when either the specified match or I/O condition occurs."]
    #[inline(always)]
    pub fn or(self) -> &'a mut W {
        self.variant(COMBMODE_A::OR)
    }
    #[doc = "MATCH. Uses the specified match only."]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(COMBMODE_A::MATCH)
    }
    #[doc = "IO. Uses the specified I/O condition only."]
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(COMBMODE_A::IO)
    }
    #[doc = "AND. The event occurs when the specified match and I/O condition occur simultaneously."]
    #[inline(always)]
    pub fn and(self) -> &'a mut W {
        self.variant(COMBMODE_A::AND)
    }
}
#[doc = "Field `STATELD` reader - State Load"]
pub type STATELD_R = crate::BitReader<STATELD_A>;
#[doc = "State Load\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STATELD_A {
    #[doc = "0: Add. STATEV value is added into STATE (the carry-out is ignored)."]
    ADD = 0,
    #[doc = "1: Load. STATEV value is loaded into STATE."]
    LOAD = 1,
}
impl From<STATELD_A> for bool {
    #[inline(always)]
    fn from(variant: STATELD_A) -> Self {
        variant as u8 != 0
    }
}
impl STATELD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATELD_A {
        match self.bits {
            false => STATELD_A::ADD,
            true => STATELD_A::LOAD,
        }
    }
    #[doc = "Checks if the value of the field is `ADD`"]
    #[inline(always)]
    pub fn is_add(&self) -> bool {
        *self == STATELD_A::ADD
    }
    #[doc = "Checks if the value of the field is `LOAD`"]
    #[inline(always)]
    pub fn is_load(&self) -> bool {
        *self == STATELD_A::LOAD
    }
}
#[doc = "Field `STATELD` writer - State Load"]
pub type STATELD_W<'a, const O: u8> = crate::BitWriter<'a, u32, EV_CTRL_SPEC, STATELD_A, O>;
impl<'a, const O: u8> STATELD_W<'a, O> {
    #[doc = "Add. STATEV value is added into STATE (the carry-out is ignored)."]
    #[inline(always)]
    pub fn add(self) -> &'a mut W {
        self.variant(STATELD_A::ADD)
    }
    #[doc = "Load. STATEV value is loaded into STATE."]
    #[inline(always)]
    pub fn load(self) -> &'a mut W {
        self.variant(STATELD_A::LOAD)
    }
}
#[doc = "Field `STATEV` reader - State Value"]
pub type STATEV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STATEV` writer - State Value"]
pub type STATEV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EV_CTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `MATCHMEM` reader - Match Mem"]
pub type MATCHMEM_R = crate::BitReader<bool>;
#[doc = "Field `MATCHMEM` writer - Match Mem"]
pub type MATCHMEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, EV_CTRL_SPEC, bool, O>;
#[doc = "Field `DIRECTION` reader - Direction"]
pub type DIRECTION_R = crate::FieldReader<u8, DIRECTION_A>;
#[doc = "Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIRECTION_A {
    #[doc = "0: Direction independent. This event is triggered regardless of the count direction."]
    DIRECTION_INDEPENDENT = 0,
    #[doc = "1: Counting up. This event is triggered only during up-counting when BIDIR = 1."]
    COUNTING_UP = 1,
    #[doc = "2: Counting down. This event is triggered only during down-counting when BIDIR = 1."]
    COUNTING_DOWN = 2,
}
impl From<DIRECTION_A> for u8 {
    #[inline(always)]
    fn from(variant: DIRECTION_A) -> Self {
        variant as _
    }
}
impl DIRECTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIRECTION_A> {
        match self.bits {
            0 => Some(DIRECTION_A::DIRECTION_INDEPENDENT),
            1 => Some(DIRECTION_A::COUNTING_UP),
            2 => Some(DIRECTION_A::COUNTING_DOWN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIRECTION_INDEPENDENT`"]
    #[inline(always)]
    pub fn is_direction_independent(&self) -> bool {
        *self == DIRECTION_A::DIRECTION_INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `COUNTING_UP`"]
    #[inline(always)]
    pub fn is_counting_up(&self) -> bool {
        *self == DIRECTION_A::COUNTING_UP
    }
    #[doc = "Checks if the value of the field is `COUNTING_DOWN`"]
    #[inline(always)]
    pub fn is_counting_down(&self) -> bool {
        *self == DIRECTION_A::COUNTING_DOWN
    }
}
#[doc = "Field `DIRECTION` writer - Direction"]
pub type DIRECTION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EV_CTRL_SPEC, u8, DIRECTION_A, 2, O>;
impl<'a, const O: u8> DIRECTION_W<'a, O> {
    #[doc = "Direction independent. This event is triggered regardless of the count direction."]
    #[inline(always)]
    pub fn direction_independent(self) -> &'a mut W {
        self.variant(DIRECTION_A::DIRECTION_INDEPENDENT)
    }
    #[doc = "Counting up. This event is triggered only during up-counting when BIDIR = 1."]
    #[inline(always)]
    pub fn counting_up(self) -> &'a mut W {
        self.variant(DIRECTION_A::COUNTING_UP)
    }
    #[doc = "Counting down. This event is triggered only during down-counting when BIDIR = 1."]
    #[inline(always)]
    pub fn counting_down(self) -> &'a mut W {
        self.variant(DIRECTION_A::COUNTING_DOWN)
    }
}
impl R {
    #[doc = "Bits 0:3 - Match Select"]
    #[inline(always)]
    pub fn matchsel(&self) -> MATCHSEL_R {
        MATCHSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - High Event"]
    #[inline(always)]
    pub fn hevent(&self) -> HEVENT_R {
        HEVENT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Input/Output Select"]
    #[inline(always)]
    pub fn outsel(&self) -> OUTSEL_R {
        OUTSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:9 - Input/Output Signal Select"]
    #[inline(always)]
    pub fn iosel(&self) -> IOSEL_R {
        IOSEL_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - Input/Output Condition"]
    #[inline(always)]
    pub fn iocond(&self) -> IOCOND_R {
        IOCOND_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Combination Mode"]
    #[inline(always)]
    pub fn combmode(&self) -> COMBMODE_R {
        COMBMODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - State Load"]
    #[inline(always)]
    pub fn stateld(&self) -> STATELD_R {
        STATELD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:19 - State Value"]
    #[inline(always)]
    pub fn statev(&self) -> STATEV_R {
        STATEV_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bit 20 - Match Mem"]
    #[inline(always)]
    pub fn matchmem(&self) -> MATCHMEM_R {
        MATCHMEM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Direction"]
    #[inline(always)]
    pub fn direction(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 21) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Match Select"]
    #[inline(always)]
    #[must_use]
    pub fn matchsel(&mut self) -> MATCHSEL_W<0> {
        MATCHSEL_W::new(self)
    }
    #[doc = "Bit 4 - High Event"]
    #[inline(always)]
    #[must_use]
    pub fn hevent(&mut self) -> HEVENT_W<4> {
        HEVENT_W::new(self)
    }
    #[doc = "Bit 5 - Input/Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn outsel(&mut self) -> OUTSEL_W<5> {
        OUTSEL_W::new(self)
    }
    #[doc = "Bits 6:9 - Input/Output Signal Select"]
    #[inline(always)]
    #[must_use]
    pub fn iosel(&mut self) -> IOSEL_W<6> {
        IOSEL_W::new(self)
    }
    #[doc = "Bits 10:11 - Input/Output Condition"]
    #[inline(always)]
    #[must_use]
    pub fn iocond(&mut self) -> IOCOND_W<10> {
        IOCOND_W::new(self)
    }
    #[doc = "Bits 12:13 - Combination Mode"]
    #[inline(always)]
    #[must_use]
    pub fn combmode(&mut self) -> COMBMODE_W<12> {
        COMBMODE_W::new(self)
    }
    #[doc = "Bit 14 - State Load"]
    #[inline(always)]
    #[must_use]
    pub fn stateld(&mut self) -> STATELD_W<14> {
        STATELD_W::new(self)
    }
    #[doc = "Bits 15:19 - State Value"]
    #[inline(always)]
    #[must_use]
    pub fn statev(&mut self) -> STATEV_W<15> {
        STATEV_W::new(self)
    }
    #[doc = "Bit 20 - Match Mem"]
    #[inline(always)]
    #[must_use]
    pub fn matchmem(&mut self) -> MATCHMEM_W<20> {
        MATCHMEM_W::new(self)
    }
    #[doc = "Bits 21:22 - Direction"]
    #[inline(always)]
    #[must_use]
    pub fn direction(&mut self) -> DIRECTION_W<21> {
        DIRECTION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event n Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ev_ctrl](index.html) module"]
pub struct EV_CTRL_SPEC;
impl crate::RegisterSpec for EV_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ev_ctrl::R](R) reader structure"]
impl crate::Readable for EV_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ev_ctrl::W](W) writer structure"]
impl crate::Writable for EV_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EV_CTRL to value 0"]
impl crate::Resettable for EV_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
