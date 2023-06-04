#[doc = "Register `CONTROL` reader"]
pub struct R(crate::R<CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONTROL` writer"]
pub struct W(crate::W<CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROL_SPEC>;
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
impl From<crate::W<CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IACK` reader - Interrupt Acknowledge"]
pub type IACK_R = crate::BitReader<IACK_A>;
#[doc = "Interrupt Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IACK_A {
    #[doc = "0: Do not clear the interrupt."]
    INT_NOCLEAR = 0,
    #[doc = "1: Clear the IF bit (interrupt flag)."]
    INT_CLEAR = 1,
}
impl From<IACK_A> for bool {
    #[inline(always)]
    fn from(variant: IACK_A) -> Self {
        variant as u8 != 0
    }
}
impl IACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IACK_A {
        match self.bits {
            false => IACK_A::INT_NOCLEAR,
            true => IACK_A::INT_CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `INT_NOCLEAR`"]
    #[inline(always)]
    pub fn is_int_noclear(&self) -> bool {
        *self == IACK_A::INT_NOCLEAR
    }
    #[doc = "Checks if the value of the field is `INT_CLEAR`"]
    #[inline(always)]
    pub fn is_int_clear(&self) -> bool {
        *self == IACK_A::INT_CLEAR
    }
}
#[doc = "Field `IACK` writer - Interrupt Acknowledge"]
pub type IACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, IACK_A, O>;
impl<'a, const O: u8> IACK_W<'a, O> {
    #[doc = "Do not clear the interrupt."]
    #[inline(always)]
    pub fn int_noclear(self) -> &'a mut W {
        self.variant(IACK_A::INT_NOCLEAR)
    }
    #[doc = "Clear the IF bit (interrupt flag)."]
    #[inline(always)]
    pub fn int_clear(self) -> &'a mut W {
        self.variant(IACK_A::INT_CLEAR)
    }
}
#[doc = "Field `IF` reader - Interrupt Flag"]
pub type IF_R = crate::BitReader<IF_A>;
#[doc = "Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IF_A {
    #[doc = "0: No interrupt is pending."]
    INT_PEND = 0,
    #[doc = "1: An interrupt is pending."]
    INT_NOPEND = 1,
}
impl From<IF_A> for bool {
    #[inline(always)]
    fn from(variant: IF_A) -> Self {
        variant as u8 != 0
    }
}
impl IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IF_A {
        match self.bits {
            false => IF_A::INT_PEND,
            true => IF_A::INT_NOPEND,
        }
    }
    #[doc = "Checks if the value of the field is `INT_PEND`"]
    #[inline(always)]
    pub fn is_int_pend(&self) -> bool {
        *self == IF_A::INT_PEND
    }
    #[doc = "Checks if the value of the field is `INT_NOPEND`"]
    #[inline(always)]
    pub fn is_int_nopend(&self) -> bool {
        *self == IF_A::INT_NOPEND
    }
}
#[doc = "Field `IE` reader - Interrupt Enable"]
pub type IE_R = crate::BitReader<IE_A>;
#[doc = "Interrupt Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IE_A {
    #[doc = "0: Disable interrupts to the system."]
    DIS_INT = 0,
    #[doc = "1: Enable interrupts to the system."]
    EN_INT = 1,
}
impl From<IE_A> for bool {
    #[inline(always)]
    fn from(variant: IE_A) -> Self {
        variant as u8 != 0
    }
}
impl IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE_A {
        match self.bits {
            false => IE_A::DIS_INT,
            true => IE_A::EN_INT,
        }
    }
    #[doc = "Checks if the value of the field is `DIS_INT`"]
    #[inline(always)]
    pub fn is_dis_int(&self) -> bool {
        *self == IE_A::DIS_INT
    }
    #[doc = "Checks if the value of the field is `EN_INT`"]
    #[inline(always)]
    pub fn is_en_int(&self) -> bool {
        *self == IE_A::EN_INT
    }
}
#[doc = "Field `IE` writer - Interrupt Enable"]
pub type IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, IE_A, O>;
impl<'a, const O: u8> IE_W<'a, O> {
    #[doc = "Disable interrupts to the system."]
    #[inline(always)]
    pub fn dis_int(self) -> &'a mut W {
        self.variant(IE_A::DIS_INT)
    }
    #[doc = "Enable interrupts to the system."]
    #[inline(always)]
    pub fn en_int(self) -> &'a mut W {
        self.variant(IE_A::EN_INT)
    }
}
#[doc = "Field `BC12` reader - BC12"]
pub type BC12_R = crate::BitReader<BC12_A>;
#[doc = "BC12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BC12_A {
    #[doc = "0: Compatible with BC1.1 (default)"]
    BC11 = 0,
    #[doc = "1: Compatible with BC1.2"]
    BC12 = 1,
}
impl From<BC12_A> for bool {
    #[inline(always)]
    fn from(variant: BC12_A) -> Self {
        variant as u8 != 0
    }
}
impl BC12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BC12_A {
        match self.bits {
            false => BC12_A::BC11,
            true => BC12_A::BC12,
        }
    }
    #[doc = "Checks if the value of the field is `BC11`"]
    #[inline(always)]
    pub fn is_bc11(&self) -> bool {
        *self == BC12_A::BC11
    }
    #[doc = "Checks if the value of the field is `BC12`"]
    #[inline(always)]
    pub fn is_bc12(&self) -> bool {
        *self == BC12_A::BC12
    }
}
#[doc = "Field `BC12` writer - BC12"]
pub type BC12_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, BC12_A, O>;
impl<'a, const O: u8> BC12_W<'a, O> {
    #[doc = "Compatible with BC1.1 (default)"]
    #[inline(always)]
    pub fn bc11(self) -> &'a mut W {
        self.variant(BC12_A::BC11)
    }
    #[doc = "Compatible with BC1.2"]
    #[inline(always)]
    pub fn bc12(self) -> &'a mut W {
        self.variant(BC12_A::BC12)
    }
}
#[doc = "Field `START` reader - Start Change Detection Sequence"]
pub type START_R = crate::BitReader<START_A>;
#[doc = "Start Change Detection Sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_A {
    #[doc = "0: Do not start the sequence. Writes of this value have no effect."]
    NO_START = 0,
    #[doc = "1: Initiate the charger detection sequence. If the sequence is already running, writes of this value have no effect."]
    START = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_A {
        match self.bits {
            false => START_A::NO_START,
            true => START_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NO_START`"]
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == START_A::NO_START
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START_A::START
    }
}
#[doc = "Field `START` writer - Start Change Detection Sequence"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, START_A, O>;
impl<'a, const O: u8> START_W<'a, O> {
    #[doc = "Do not start the sequence. Writes of this value have no effect."]
    #[inline(always)]
    pub fn no_start(self) -> &'a mut W {
        self.variant(START_A::NO_START)
    }
    #[doc = "Initiate the charger detection sequence. If the sequence is already running, writes of this value have no effect."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(START_A::START)
    }
}
#[doc = "Field `SR` reader - Software Reset"]
pub type SR_R = crate::BitReader<SR_A>;
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR_A {
    #[doc = "0: Do not perform a software reset."]
    NO_RESET = 0,
    #[doc = "1: Perform a software reset."]
    SW_RESET = 1,
}
impl From<SR_A> for bool {
    #[inline(always)]
    fn from(variant: SR_A) -> Self {
        variant as u8 != 0
    }
}
impl SR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR_A {
        match self.bits {
            false => SR_A::NO_RESET,
            true => SR_A::SW_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NO_RESET`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == SR_A::NO_RESET
    }
    #[doc = "Checks if the value of the field is `SW_RESET`"]
    #[inline(always)]
    pub fn is_sw_reset(&self) -> bool {
        *self == SR_A::SW_RESET
    }
}
#[doc = "Field `SR` writer - Software Reset"]
pub type SR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, SR_A, O>;
impl<'a, const O: u8> SR_W<'a, O> {
    #[doc = "Do not perform a software reset."]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(SR_A::NO_RESET)
    }
    #[doc = "Perform a software reset."]
    #[inline(always)]
    pub fn sw_reset(self) -> &'a mut W {
        self.variant(SR_A::SW_RESET)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Acknowledge"]
    #[inline(always)]
    pub fn iack(&self) -> IACK_R {
        IACK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Flag"]
    #[inline(always)]
    pub fn if_(&self) -> IF_R {
        IF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - BC12"]
    #[inline(always)]
    pub fn bc12(&self) -> BC12_R {
        BC12_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - Start Change Detection Sequence"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Software Reset"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn iack(&mut self) -> IACK_W<0> {
        IACK_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<16> {
        IE_W::new(self)
    }
    #[doc = "Bit 17 - BC12"]
    #[inline(always)]
    #[must_use]
    pub fn bc12(&mut self) -> BC12_W<17> {
        BC12_W::new(self)
    }
    #[doc = "Bit 24 - Start Change Detection Sequence"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<24> {
        START_W::new(self)
    }
    #[doc = "Bit 25 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SR_W<25> {
        SR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](index.html) module"]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [control::R](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [control::W](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0x0001_0000"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
