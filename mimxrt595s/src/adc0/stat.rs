#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDY0` reader - Result FIFO 0 Ready Flag"]
pub type RDY0_R = crate::BitReader<RDY0_A>;
#[doc = "Result FIFO 0 Ready Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDY0_A {
    #[doc = "0: Not above watermark"]
    BELOW_THRESHOLD = 0,
    #[doc = "1: Above watermark"]
    ABOVE_THRESHOLD = 1,
}
impl From<RDY0_A> for bool {
    #[inline(always)]
    fn from(variant: RDY0_A) -> Self {
        variant as u8 != 0
    }
}
impl RDY0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDY0_A {
        match self.bits {
            false => RDY0_A::BELOW_THRESHOLD,
            true => RDY0_A::ABOVE_THRESHOLD,
        }
    }
    #[doc = "Checks if the value of the field is `BELOW_THRESHOLD`"]
    #[inline(always)]
    pub fn is_below_threshold(&self) -> bool {
        *self == RDY0_A::BELOW_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `ABOVE_THRESHOLD`"]
    #[inline(always)]
    pub fn is_above_threshold(&self) -> bool {
        *self == RDY0_A::ABOVE_THRESHOLD
    }
}
#[doc = "Field `FOF0` reader - Result FIFO 0 Overflow Flag"]
pub type FOF0_R = crate::BitReader<FOF0_A>;
#[doc = "Result FIFO 0 Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOF0_A {
    #[doc = "0: No result FIFO 0 overflow has occurred since the last time that the flag was cleared."]
    NO_OVERFLOW = 0,
    #[doc = "1: At least one result FIFO 0 overflow has occurred since the last time that the flag was cleared."]
    OVERFLOW_DETECTED = 1,
}
impl From<FOF0_A> for bool {
    #[inline(always)]
    fn from(variant: FOF0_A) -> Self {
        variant as u8 != 0
    }
}
impl FOF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOF0_A {
        match self.bits {
            false => FOF0_A::NO_OVERFLOW,
            true => FOF0_A::OVERFLOW_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OVERFLOW`"]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == FOF0_A::NO_OVERFLOW
    }
    #[doc = "Checks if the value of the field is `OVERFLOW_DETECTED`"]
    #[inline(always)]
    pub fn is_overflow_detected(&self) -> bool {
        *self == FOF0_A::OVERFLOW_DETECTED
    }
}
#[doc = "Field `FOF0` writer - Result FIFO 0 Overflow Flag"]
pub type FOF0_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STAT_SPEC, FOF0_A, O>;
impl<'a, const O: u8> FOF0_W<'a, O> {
    #[doc = "No result FIFO 0 overflow has occurred since the last time that the flag was cleared."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut W {
        self.variant(FOF0_A::NO_OVERFLOW)
    }
    #[doc = "At least one result FIFO 0 overflow has occurred since the last time that the flag was cleared."]
    #[inline(always)]
    pub fn overflow_detected(self) -> &'a mut W {
        self.variant(FOF0_A::OVERFLOW_DETECTED)
    }
}
#[doc = "Field `RDY1` reader - Result FIFO1 Ready Flag"]
pub type RDY1_R = crate::BitReader<RDY1_A>;
#[doc = "Result FIFO1 Ready Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDY1_A {
    #[doc = "0: Not above watermark"]
    BELOW_THRESHOLD = 0,
    #[doc = "1: Above watermark"]
    ABOVE_THRESHOLD = 1,
}
impl From<RDY1_A> for bool {
    #[inline(always)]
    fn from(variant: RDY1_A) -> Self {
        variant as u8 != 0
    }
}
impl RDY1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDY1_A {
        match self.bits {
            false => RDY1_A::BELOW_THRESHOLD,
            true => RDY1_A::ABOVE_THRESHOLD,
        }
    }
    #[doc = "Checks if the value of the field is `BELOW_THRESHOLD`"]
    #[inline(always)]
    pub fn is_below_threshold(&self) -> bool {
        *self == RDY1_A::BELOW_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `ABOVE_THRESHOLD`"]
    #[inline(always)]
    pub fn is_above_threshold(&self) -> bool {
        *self == RDY1_A::ABOVE_THRESHOLD
    }
}
#[doc = "Field `FOF1` reader - Result FIFO1 Overflow Flag"]
pub type FOF1_R = crate::BitReader<FOF1_A>;
#[doc = "Result FIFO1 Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOF1_A {
    #[doc = "0: No result FIFO1 overflow has occurred since the last time that the flag was cleared."]
    NO_OVERFLOW = 0,
    #[doc = "1: At least one result FIFO1 overflow has occurred since the last time that the flag was cleared."]
    OVERFLOW_DETECTED = 1,
}
impl From<FOF1_A> for bool {
    #[inline(always)]
    fn from(variant: FOF1_A) -> Self {
        variant as u8 != 0
    }
}
impl FOF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOF1_A {
        match self.bits {
            false => FOF1_A::NO_OVERFLOW,
            true => FOF1_A::OVERFLOW_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OVERFLOW`"]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == FOF1_A::NO_OVERFLOW
    }
    #[doc = "Checks if the value of the field is `OVERFLOW_DETECTED`"]
    #[inline(always)]
    pub fn is_overflow_detected(&self) -> bool {
        *self == FOF1_A::OVERFLOW_DETECTED
    }
}
#[doc = "Field `FOF1` writer - Result FIFO1 Overflow Flag"]
pub type FOF1_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STAT_SPEC, FOF1_A, O>;
impl<'a, const O: u8> FOF1_W<'a, O> {
    #[doc = "No result FIFO1 overflow has occurred since the last time that the flag was cleared."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut W {
        self.variant(FOF1_A::NO_OVERFLOW)
    }
    #[doc = "At least one result FIFO1 overflow has occurred since the last time that the flag was cleared."]
    #[inline(always)]
    pub fn overflow_detected(self) -> &'a mut W {
        self.variant(FOF1_A::OVERFLOW_DETECTED)
    }
}
#[doc = "Field `TEXC_INT` reader - Interrupt Flag For High-Priority Trigger Exception"]
pub type TEXC_INT_R = crate::BitReader<TEXC_INT_A>;
#[doc = "Interrupt Flag For High-Priority Trigger Exception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEXC_INT_A {
    #[doc = "0: No trigger exceptions have occurred."]
    NO_EXCEPTION = 0,
    #[doc = "1: A trigger exception has occurred and is pending acknowledgment."]
    EXCEPTION_DETECTED = 1,
}
impl From<TEXC_INT_A> for bool {
    #[inline(always)]
    fn from(variant: TEXC_INT_A) -> Self {
        variant as u8 != 0
    }
}
impl TEXC_INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEXC_INT_A {
        match self.bits {
            false => TEXC_INT_A::NO_EXCEPTION,
            true => TEXC_INT_A::EXCEPTION_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EXCEPTION`"]
    #[inline(always)]
    pub fn is_no_exception(&self) -> bool {
        *self == TEXC_INT_A::NO_EXCEPTION
    }
    #[doc = "Checks if the value of the field is `EXCEPTION_DETECTED`"]
    #[inline(always)]
    pub fn is_exception_detected(&self) -> bool {
        *self == TEXC_INT_A::EXCEPTION_DETECTED
    }
}
#[doc = "Field `TEXC_INT` writer - Interrupt Flag For High-Priority Trigger Exception"]
pub type TEXC_INT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STAT_SPEC, TEXC_INT_A, O>;
impl<'a, const O: u8> TEXC_INT_W<'a, O> {
    #[doc = "No trigger exceptions have occurred."]
    #[inline(always)]
    pub fn no_exception(self) -> &'a mut W {
        self.variant(TEXC_INT_A::NO_EXCEPTION)
    }
    #[doc = "A trigger exception has occurred and is pending acknowledgment."]
    #[inline(always)]
    pub fn exception_detected(self) -> &'a mut W {
        self.variant(TEXC_INT_A::EXCEPTION_DETECTED)
    }
}
#[doc = "Field `TCOMP_INT` reader - Interrupt Flag For Trigger Completion"]
pub type TCOMP_INT_R = crate::BitReader<TCOMP_INT_A>;
#[doc = "Interrupt Flag For Trigger Completion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCOMP_INT_A {
    #[doc = "0: Either IE\\[TCOMP_IE\\]
= 0, or no trigger sequences have run to completion."]
    FLAG_CLEAR = 0,
    #[doc = "1: Trigger sequence has been completed and all data is stored in the associated FIFO."]
    COMPLETION_DETECTED = 1,
}
impl From<TCOMP_INT_A> for bool {
    #[inline(always)]
    fn from(variant: TCOMP_INT_A) -> Self {
        variant as u8 != 0
    }
}
impl TCOMP_INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCOMP_INT_A {
        match self.bits {
            false => TCOMP_INT_A::FLAG_CLEAR,
            true => TCOMP_INT_A::COMPLETION_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `FLAG_CLEAR`"]
    #[inline(always)]
    pub fn is_flag_clear(&self) -> bool {
        *self == TCOMP_INT_A::FLAG_CLEAR
    }
    #[doc = "Checks if the value of the field is `COMPLETION_DETECTED`"]
    #[inline(always)]
    pub fn is_completion_detected(&self) -> bool {
        *self == TCOMP_INT_A::COMPLETION_DETECTED
    }
}
#[doc = "Field `TCOMP_INT` writer - Interrupt Flag For Trigger Completion"]
pub type TCOMP_INT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STAT_SPEC, TCOMP_INT_A, O>;
impl<'a, const O: u8> TCOMP_INT_W<'a, O> {
    #[doc = "Either IE\\[TCOMP_IE\\]
= 0, or no trigger sequences have run to completion."]
    #[inline(always)]
    pub fn flag_clear(self) -> &'a mut W {
        self.variant(TCOMP_INT_A::FLAG_CLEAR)
    }
    #[doc = "Trigger sequence has been completed and all data is stored in the associated FIFO."]
    #[inline(always)]
    pub fn completion_detected(self) -> &'a mut W {
        self.variant(TCOMP_INT_A::COMPLETION_DETECTED)
    }
}
#[doc = "Field `ADC_ACTIVE` reader - ADC Active"]
pub type ADC_ACTIVE_R = crate::BitReader<ADC_ACTIVE_A>;
#[doc = "ADC Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_ACTIVE_A {
    #[doc = "0: ADC is idle. There are no pending triggers to service and no active commands are being processed."]
    NOT_ACTIVE = 0,
    #[doc = "1: ADC is processing a conversion, running through the power-up delay, or servicing a trigger."]
    BUSY = 1,
}
impl From<ADC_ACTIVE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_ACTIVE_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_ACTIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_ACTIVE_A {
        match self.bits {
            false => ADC_ACTIVE_A::NOT_ACTIVE,
            true => ADC_ACTIVE_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ADC_ACTIVE_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == ADC_ACTIVE_A::BUSY
    }
}
#[doc = "Field `TRGACT` reader - Trigger Active"]
pub type TRGACT_R = crate::FieldReader<u8, TRGACT_A>;
#[doc = "Trigger Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRGACT_A {
    #[doc = "0: Command (sequence) associated with Trigger 0 currently being executed."]
    TRIG_0 = 0,
    #[doc = "1: Command (sequence) associated with Trigger 1 currently being executed."]
    TRIG_1 = 1,
    #[doc = "2: Command (sequence) associated with Trigger 2 currently being executed."]
    TRIG_2 = 2,
    #[doc = "3: Command (sequence) from the associated Trigger number currently being executed."]
    TRIG_X_3 = 3,
    #[doc = "4: Command (sequence) from the associated Trigger number currently being executed."]
    TRIG_X_4 = 4,
    #[doc = "5: Command (sequence) from the associated Trigger number currently being executed."]
    TRIG_X_5 = 5,
    #[doc = "6: Command (sequence) from the associated Trigger number currently being executed."]
    TRIG_X_6 = 6,
    #[doc = "7: Command (sequence) from the associated Trigger number currently being executed."]
    TRIG_X_7 = 7,
    #[doc = "8: Command (sequence) from the associated Trigger number currently being executed."]
    TRIG_X_8 = 8,
    #[doc = "9: Command (sequence) from the associated Trigger number currently being executed."]
    TRIG_X_9 = 9,
}
impl From<TRGACT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGACT_A) -> Self {
        variant as _
    }
}
impl TRGACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRGACT_A> {
        match self.bits {
            0 => Some(TRGACT_A::TRIG_0),
            1 => Some(TRGACT_A::TRIG_1),
            2 => Some(TRGACT_A::TRIG_2),
            3 => Some(TRGACT_A::TRIG_X_3),
            4 => Some(TRGACT_A::TRIG_X_4),
            5 => Some(TRGACT_A::TRIG_X_5),
            6 => Some(TRGACT_A::TRIG_X_6),
            7 => Some(TRGACT_A::TRIG_X_7),
            8 => Some(TRGACT_A::TRIG_X_8),
            9 => Some(TRGACT_A::TRIG_X_9),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG_0`"]
    #[inline(always)]
    pub fn is_trig_0(&self) -> bool {
        *self == TRGACT_A::TRIG_0
    }
    #[doc = "Checks if the value of the field is `TRIG_1`"]
    #[inline(always)]
    pub fn is_trig_1(&self) -> bool {
        *self == TRGACT_A::TRIG_1
    }
    #[doc = "Checks if the value of the field is `TRIG_2`"]
    #[inline(always)]
    pub fn is_trig_2(&self) -> bool {
        *self == TRGACT_A::TRIG_2
    }
    #[doc = "Checks if the value of the field is `TRIG_X_3`"]
    #[inline(always)]
    pub fn is_trig_x_3(&self) -> bool {
        *self == TRGACT_A::TRIG_X_3
    }
    #[doc = "Checks if the value of the field is `TRIG_X_4`"]
    #[inline(always)]
    pub fn is_trig_x_4(&self) -> bool {
        *self == TRGACT_A::TRIG_X_4
    }
    #[doc = "Checks if the value of the field is `TRIG_X_5`"]
    #[inline(always)]
    pub fn is_trig_x_5(&self) -> bool {
        *self == TRGACT_A::TRIG_X_5
    }
    #[doc = "Checks if the value of the field is `TRIG_X_6`"]
    #[inline(always)]
    pub fn is_trig_x_6(&self) -> bool {
        *self == TRGACT_A::TRIG_X_6
    }
    #[doc = "Checks if the value of the field is `TRIG_X_7`"]
    #[inline(always)]
    pub fn is_trig_x_7(&self) -> bool {
        *self == TRGACT_A::TRIG_X_7
    }
    #[doc = "Checks if the value of the field is `TRIG_X_8`"]
    #[inline(always)]
    pub fn is_trig_x_8(&self) -> bool {
        *self == TRGACT_A::TRIG_X_8
    }
    #[doc = "Checks if the value of the field is `TRIG_X_9`"]
    #[inline(always)]
    pub fn is_trig_x_9(&self) -> bool {
        *self == TRGACT_A::TRIG_X_9
    }
}
#[doc = "Field `CMDACT` reader - Command Active"]
pub type CMDACT_R = crate::FieldReader<u8, CMDACT_A>;
#[doc = "Command Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDACT_A {
    #[doc = "0: No command currently in progress."]
    NO_COMMAND_ACTIVE = 0,
    #[doc = "1: Command 1 currently being executed."]
    COMMAND_1 = 1,
    #[doc = "2: Command 2 currently being executed."]
    COMMAND_2 = 2,
    #[doc = "3: Associated command number currently being executed."]
    COMMAND_X_3 = 3,
    #[doc = "4: Associated command number currently being executed."]
    COMMAND_X_4 = 4,
    #[doc = "5: Associated command number currently being executed."]
    COMMAND_X_5 = 5,
    #[doc = "6: Associated command number currently being executed."]
    COMMAND_X_6 = 6,
    #[doc = "7: Associated command number currently being executed."]
    COMMAND_X_7 = 7,
    #[doc = "8: Associated command number currently being executed."]
    COMMAND_X_8 = 8,
    #[doc = "9: Associated command number currently being executed."]
    COMMAND_X_9 = 9,
}
impl From<CMDACT_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDACT_A) -> Self {
        variant as _
    }
}
impl CMDACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMDACT_A> {
        match self.bits {
            0 => Some(CMDACT_A::NO_COMMAND_ACTIVE),
            1 => Some(CMDACT_A::COMMAND_1),
            2 => Some(CMDACT_A::COMMAND_2),
            3 => Some(CMDACT_A::COMMAND_X_3),
            4 => Some(CMDACT_A::COMMAND_X_4),
            5 => Some(CMDACT_A::COMMAND_X_5),
            6 => Some(CMDACT_A::COMMAND_X_6),
            7 => Some(CMDACT_A::COMMAND_X_7),
            8 => Some(CMDACT_A::COMMAND_X_8),
            9 => Some(CMDACT_A::COMMAND_X_9),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_COMMAND_ACTIVE`"]
    #[inline(always)]
    pub fn is_no_command_active(&self) -> bool {
        *self == CMDACT_A::NO_COMMAND_ACTIVE
    }
    #[doc = "Checks if the value of the field is `COMMAND_1`"]
    #[inline(always)]
    pub fn is_command_1(&self) -> bool {
        *self == CMDACT_A::COMMAND_1
    }
    #[doc = "Checks if the value of the field is `COMMAND_2`"]
    #[inline(always)]
    pub fn is_command_2(&self) -> bool {
        *self == CMDACT_A::COMMAND_2
    }
    #[doc = "Checks if the value of the field is `COMMAND_X_3`"]
    #[inline(always)]
    pub fn is_command_x_3(&self) -> bool {
        *self == CMDACT_A::COMMAND_X_3
    }
    #[doc = "Checks if the value of the field is `COMMAND_X_4`"]
    #[inline(always)]
    pub fn is_command_x_4(&self) -> bool {
        *self == CMDACT_A::COMMAND_X_4
    }
    #[doc = "Checks if the value of the field is `COMMAND_X_5`"]
    #[inline(always)]
    pub fn is_command_x_5(&self) -> bool {
        *self == CMDACT_A::COMMAND_X_5
    }
    #[doc = "Checks if the value of the field is `COMMAND_X_6`"]
    #[inline(always)]
    pub fn is_command_x_6(&self) -> bool {
        *self == CMDACT_A::COMMAND_X_6
    }
    #[doc = "Checks if the value of the field is `COMMAND_X_7`"]
    #[inline(always)]
    pub fn is_command_x_7(&self) -> bool {
        *self == CMDACT_A::COMMAND_X_7
    }
    #[doc = "Checks if the value of the field is `COMMAND_X_8`"]
    #[inline(always)]
    pub fn is_command_x_8(&self) -> bool {
        *self == CMDACT_A::COMMAND_X_8
    }
    #[doc = "Checks if the value of the field is `COMMAND_X_9`"]
    #[inline(always)]
    pub fn is_command_x_9(&self) -> bool {
        *self == CMDACT_A::COMMAND_X_9
    }
}
impl R {
    #[doc = "Bit 0 - Result FIFO 0 Ready Flag"]
    #[inline(always)]
    pub fn rdy0(&self) -> RDY0_R {
        RDY0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Result FIFO 0 Overflow Flag"]
    #[inline(always)]
    pub fn fof0(&self) -> FOF0_R {
        FOF0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Result FIFO1 Ready Flag"]
    #[inline(always)]
    pub fn rdy1(&self) -> RDY1_R {
        RDY1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Result FIFO1 Overflow Flag"]
    #[inline(always)]
    pub fn fof1(&self) -> FOF1_R {
        FOF1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Flag For High-Priority Trigger Exception"]
    #[inline(always)]
    pub fn texc_int(&self) -> TEXC_INT_R {
        TEXC_INT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Flag For Trigger Completion"]
    #[inline(always)]
    pub fn tcomp_int(&self) -> TCOMP_INT_R {
        TCOMP_INT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - ADC Active"]
    #[inline(always)]
    pub fn adc_active(&self) -> ADC_ACTIVE_R {
        ADC_ACTIVE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Trigger Active"]
    #[inline(always)]
    pub fn trgact(&self) -> TRGACT_R {
        TRGACT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Command Active"]
    #[inline(always)]
    pub fn cmdact(&self) -> CMDACT_R {
        CMDACT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Result FIFO 0 Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fof0(&mut self) -> FOF0_W<1> {
        FOF0_W::new(self)
    }
    #[doc = "Bit 3 - Result FIFO1 Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fof1(&mut self) -> FOF1_W<3> {
        FOF1_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt Flag For High-Priority Trigger Exception"]
    #[inline(always)]
    #[must_use]
    pub fn texc_int(&mut self) -> TEXC_INT_W<8> {
        TEXC_INT_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt Flag For Trigger Completion"]
    #[inline(always)]
    #[must_use]
    pub fn tcomp_int(&mut self) -> TCOMP_INT_W<9> {
        TCOMP_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x030a;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
