#[doc = "Register `CMDH7` reader"]
pub struct R(crate::R<CMDH7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDH7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDH7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDH7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDH7` writer"]
pub struct W(crate::W<CMDH7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDH7_SPEC>;
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
impl From<crate::W<CMDH7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDH7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAIT_TRIG` reader - Wait for Trigger Assertion Before Execution"]
pub type WAIT_TRIG_R = crate::BitReader<WAIT_TRIG_A>;
#[doc = "Wait for Trigger Assertion Before Execution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAIT_TRIG_A {
    #[doc = "0: Command executes automatically."]
    DISABLED = 0,
    #[doc = "1: Active trigger must be asserted again before executing this command."]
    ENABLED = 1,
}
impl From<WAIT_TRIG_A> for bool {
    #[inline(always)]
    fn from(variant: WAIT_TRIG_A) -> Self {
        variant as u8 != 0
    }
}
impl WAIT_TRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAIT_TRIG_A {
        match self.bits {
            false => WAIT_TRIG_A::DISABLED,
            true => WAIT_TRIG_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAIT_TRIG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAIT_TRIG_A::ENABLED
    }
}
#[doc = "Field `WAIT_TRIG` writer - Wait for Trigger Assertion Before Execution"]
pub type WAIT_TRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDH7_SPEC, WAIT_TRIG_A, O>;
impl<'a, const O: u8> WAIT_TRIG_W<'a, O> {
    #[doc = "Command executes automatically."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAIT_TRIG_A::DISABLED)
    }
    #[doc = "Active trigger must be asserted again before executing this command."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAIT_TRIG_A::ENABLED)
    }
}
#[doc = "Field `LWI` reader - Loop with Increment"]
pub type LWI_R = crate::BitReader<LWI_A>;
#[doc = "Loop with Increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LWI_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<LWI_A> for bool {
    #[inline(always)]
    fn from(variant: LWI_A) -> Self {
        variant as u8 != 0
    }
}
impl LWI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LWI_A {
        match self.bits {
            false => LWI_A::DISABLED,
            true => LWI_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LWI_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LWI_A::ENABLED
    }
}
#[doc = "Field `LWI` writer - Loop with Increment"]
pub type LWI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDH7_SPEC, LWI_A, O>;
impl<'a, const O: u8> LWI_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LWI_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LWI_A::ENABLED)
    }
}
#[doc = "Field `STS` reader - Sample Time Select"]
pub type STS_R = crate::FieldReader<u8, STS_A>;
#[doc = "Sample Time Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STS_A {
    #[doc = "0: Minimum sample time of 3.5 ADCK cycles."]
    SAMPLE_3P5 = 0,
    #[doc = "1: 5.5 ADCK cycles"]
    SAMPLE_5P5 = 1,
    #[doc = "2: 7.5 ADCK cycles"]
    SAMPLE_7P5 = 2,
    #[doc = "3: 11.5 ADCK cycles"]
    SAMPLE_11P5 = 3,
    #[doc = "4: 19.5 ADCK cycles"]
    SAMPLE_19P5 = 4,
    #[doc = "5: 35.5 ADCK cycles"]
    SAMPLE_35P5 = 5,
    #[doc = "6: 67.5 ADCK cycles"]
    SAMPLE_67P5 = 6,
    #[doc = "7: 131.5 ADCK cycles"]
    SAMPLE_131P5 = 7,
}
impl From<STS_A> for u8 {
    #[inline(always)]
    fn from(variant: STS_A) -> Self {
        variant as _
    }
}
impl STS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STS_A {
        match self.bits {
            0 => STS_A::SAMPLE_3P5,
            1 => STS_A::SAMPLE_5P5,
            2 => STS_A::SAMPLE_7P5,
            3 => STS_A::SAMPLE_11P5,
            4 => STS_A::SAMPLE_19P5,
            5 => STS_A::SAMPLE_35P5,
            6 => STS_A::SAMPLE_67P5,
            7 => STS_A::SAMPLE_131P5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SAMPLE_3P5`"]
    #[inline(always)]
    pub fn is_sample_3p5(&self) -> bool {
        *self == STS_A::SAMPLE_3P5
    }
    #[doc = "Checks if the value of the field is `SAMPLE_5P5`"]
    #[inline(always)]
    pub fn is_sample_5p5(&self) -> bool {
        *self == STS_A::SAMPLE_5P5
    }
    #[doc = "Checks if the value of the field is `SAMPLE_7P5`"]
    #[inline(always)]
    pub fn is_sample_7p5(&self) -> bool {
        *self == STS_A::SAMPLE_7P5
    }
    #[doc = "Checks if the value of the field is `SAMPLE_11P5`"]
    #[inline(always)]
    pub fn is_sample_11p5(&self) -> bool {
        *self == STS_A::SAMPLE_11P5
    }
    #[doc = "Checks if the value of the field is `SAMPLE_19P5`"]
    #[inline(always)]
    pub fn is_sample_19p5(&self) -> bool {
        *self == STS_A::SAMPLE_19P5
    }
    #[doc = "Checks if the value of the field is `SAMPLE_35P5`"]
    #[inline(always)]
    pub fn is_sample_35p5(&self) -> bool {
        *self == STS_A::SAMPLE_35P5
    }
    #[doc = "Checks if the value of the field is `SAMPLE_67P5`"]
    #[inline(always)]
    pub fn is_sample_67p5(&self) -> bool {
        *self == STS_A::SAMPLE_67P5
    }
    #[doc = "Checks if the value of the field is `SAMPLE_131P5`"]
    #[inline(always)]
    pub fn is_sample_131p5(&self) -> bool {
        *self == STS_A::SAMPLE_131P5
    }
}
#[doc = "Field `STS` writer - Sample Time Select"]
pub type STS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CMDH7_SPEC, u8, STS_A, 3, O>;
impl<'a, const O: u8> STS_W<'a, O> {
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    #[inline(always)]
    pub fn sample_3p5(self) -> &'a mut W {
        self.variant(STS_A::SAMPLE_3P5)
    }
    #[doc = "5.5 ADCK cycles"]
    #[inline(always)]
    pub fn sample_5p5(self) -> &'a mut W {
        self.variant(STS_A::SAMPLE_5P5)
    }
    #[doc = "7.5 ADCK cycles"]
    #[inline(always)]
    pub fn sample_7p5(self) -> &'a mut W {
        self.variant(STS_A::SAMPLE_7P5)
    }
    #[doc = "11.5 ADCK cycles"]
    #[inline(always)]
    pub fn sample_11p5(self) -> &'a mut W {
        self.variant(STS_A::SAMPLE_11P5)
    }
    #[doc = "19.5 ADCK cycles"]
    #[inline(always)]
    pub fn sample_19p5(self) -> &'a mut W {
        self.variant(STS_A::SAMPLE_19P5)
    }
    #[doc = "35.5 ADCK cycles"]
    #[inline(always)]
    pub fn sample_35p5(self) -> &'a mut W {
        self.variant(STS_A::SAMPLE_35P5)
    }
    #[doc = "67.5 ADCK cycles"]
    #[inline(always)]
    pub fn sample_67p5(self) -> &'a mut W {
        self.variant(STS_A::SAMPLE_67P5)
    }
    #[doc = "131.5 ADCK cycles"]
    #[inline(always)]
    pub fn sample_131p5(self) -> &'a mut W {
        self.variant(STS_A::SAMPLE_131P5)
    }
}
#[doc = "Field `AVGS` reader - Hardware Average Select"]
pub type AVGS_R = crate::FieldReader<u8, AVGS_A>;
#[doc = "Hardware Average Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AVGS_A {
    #[doc = "0: Single conversion"]
    NO_AVERAGE = 0,
    #[doc = "1: 2"]
    AVERAGE_2 = 1,
    #[doc = "2: 4"]
    AVERAGE_4 = 2,
    #[doc = "3: 8"]
    AVERAGE_8 = 3,
    #[doc = "4: 16"]
    AVERAGE_16 = 4,
    #[doc = "5: 32"]
    AVERAGE_32 = 5,
    #[doc = "6: 64"]
    AVERAGE_64 = 6,
    #[doc = "7: 128"]
    AVERAGE_128 = 7,
}
impl From<AVGS_A> for u8 {
    #[inline(always)]
    fn from(variant: AVGS_A) -> Self {
        variant as _
    }
}
impl AVGS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVGS_A {
        match self.bits {
            0 => AVGS_A::NO_AVERAGE,
            1 => AVGS_A::AVERAGE_2,
            2 => AVGS_A::AVERAGE_4,
            3 => AVGS_A::AVERAGE_8,
            4 => AVGS_A::AVERAGE_16,
            5 => AVGS_A::AVERAGE_32,
            6 => AVGS_A::AVERAGE_64,
            7 => AVGS_A::AVERAGE_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_AVERAGE`"]
    #[inline(always)]
    pub fn is_no_average(&self) -> bool {
        *self == AVGS_A::NO_AVERAGE
    }
    #[doc = "Checks if the value of the field is `AVERAGE_2`"]
    #[inline(always)]
    pub fn is_average_2(&self) -> bool {
        *self == AVGS_A::AVERAGE_2
    }
    #[doc = "Checks if the value of the field is `AVERAGE_4`"]
    #[inline(always)]
    pub fn is_average_4(&self) -> bool {
        *self == AVGS_A::AVERAGE_4
    }
    #[doc = "Checks if the value of the field is `AVERAGE_8`"]
    #[inline(always)]
    pub fn is_average_8(&self) -> bool {
        *self == AVGS_A::AVERAGE_8
    }
    #[doc = "Checks if the value of the field is `AVERAGE_16`"]
    #[inline(always)]
    pub fn is_average_16(&self) -> bool {
        *self == AVGS_A::AVERAGE_16
    }
    #[doc = "Checks if the value of the field is `AVERAGE_32`"]
    #[inline(always)]
    pub fn is_average_32(&self) -> bool {
        *self == AVGS_A::AVERAGE_32
    }
    #[doc = "Checks if the value of the field is `AVERAGE_64`"]
    #[inline(always)]
    pub fn is_average_64(&self) -> bool {
        *self == AVGS_A::AVERAGE_64
    }
    #[doc = "Checks if the value of the field is `AVERAGE_128`"]
    #[inline(always)]
    pub fn is_average_128(&self) -> bool {
        *self == AVGS_A::AVERAGE_128
    }
}
#[doc = "Field `AVGS` writer - Hardware Average Select"]
pub type AVGS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CMDH7_SPEC, u8, AVGS_A, 3, O>;
impl<'a, const O: u8> AVGS_W<'a, O> {
    #[doc = "Single conversion"]
    #[inline(always)]
    pub fn no_average(self) -> &'a mut W {
        self.variant(AVGS_A::NO_AVERAGE)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn average_2(self) -> &'a mut W {
        self.variant(AVGS_A::AVERAGE_2)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn average_4(self) -> &'a mut W {
        self.variant(AVGS_A::AVERAGE_4)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn average_8(self) -> &'a mut W {
        self.variant(AVGS_A::AVERAGE_8)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn average_16(self) -> &'a mut W {
        self.variant(AVGS_A::AVERAGE_16)
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn average_32(self) -> &'a mut W {
        self.variant(AVGS_A::AVERAGE_32)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn average_64(self) -> &'a mut W {
        self.variant(AVGS_A::AVERAGE_64)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn average_128(self) -> &'a mut W {
        self.variant(AVGS_A::AVERAGE_128)
    }
}
#[doc = "Field `LOOP` reader - Loop Count Select"]
pub type LOOP_R = crate::FieldReader<u8, LOOP_A>;
#[doc = "Loop Count Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOOP_A {
    #[doc = "0: Looping not enabled. Command executes one time."]
    CMD_EXEC_1X = 0,
    #[doc = "1: Loop one time. Command executes two times."]
    CMD_EXEC_2X = 1,
    #[doc = "2: Loop two times. Command executes three times."]
    CMD_EXEC_3X = 2,
    #[doc = "3: Loop corresponding number of times. Command executes LOOP + 1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_3 = 3,
    #[doc = "4: Loop corresponding number of times. Command executes LOOP + 1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_4 = 4,
    #[doc = "5: Loop corresponding number of times. Command executes LOOP + 1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_5 = 5,
    #[doc = "6: Loop corresponding number of times. Command executes LOOP + 1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_6 = 6,
    #[doc = "7: Loop corresponding number of times. Command executes LOOP + 1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_7 = 7,
    #[doc = "8: Loop corresponding number of times. Command executes LOOP + 1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_8 = 8,
    #[doc = "9: Loop corresponding number of times. Command executes LOOP + 1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_9 = 9,
    #[doc = "15: Loop 15 times. Command executes 16 times."]
    CMD_EXEC_15X = 15,
}
impl From<LOOP_A> for u8 {
    #[inline(always)]
    fn from(variant: LOOP_A) -> Self {
        variant as _
    }
}
impl LOOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOOP_A> {
        match self.bits {
            0 => Some(LOOP_A::CMD_EXEC_1X),
            1 => Some(LOOP_A::CMD_EXEC_2X),
            2 => Some(LOOP_A::CMD_EXEC_3X),
            3 => Some(LOOP_A::CMD_EXECUTES_CORRESPONDING_TIMES_3),
            4 => Some(LOOP_A::CMD_EXECUTES_CORRESPONDING_TIMES_4),
            5 => Some(LOOP_A::CMD_EXECUTES_CORRESPONDING_TIMES_5),
            6 => Some(LOOP_A::CMD_EXECUTES_CORRESPONDING_TIMES_6),
            7 => Some(LOOP_A::CMD_EXECUTES_CORRESPONDING_TIMES_7),
            8 => Some(LOOP_A::CMD_EXECUTES_CORRESPONDING_TIMES_8),
            9 => Some(LOOP_A::CMD_EXECUTES_CORRESPONDING_TIMES_9),
            15 => Some(LOOP_A::CMD_EXEC_15X),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CMD_EXEC_1X`"]
    #[inline(always)]
    pub fn is_cmd_exec_1x(&self) -> bool {
        *self == LOOP_A::CMD_EXEC_1X
    }
    #[doc = "Checks if the value of the field is `CMD_EXEC_2X`"]
    #[inline(always)]
    pub fn is_cmd_exec_2x(&self) -> bool {
        *self == LOOP_A::CMD_EXEC_2X
    }
    #[doc = "Checks if the value of the field is `CMD_EXEC_3X`"]
    #[inline(always)]
    pub fn is_cmd_exec_3x(&self) -> bool {
        *self == LOOP_A::CMD_EXEC_3X
    }
    #[doc = "Checks if the value of the field is `CMD_EXECUTES_CORRESPONDING_TIMES_3`"]
    #[inline(always)]
    pub fn is_cmd_executes_corresponding_times_3(&self) -> bool {
        *self == LOOP_A::CMD_EXECUTES_CORRESPONDING_TIMES_3
    }
    #[doc = "Checks if the value of the field is `CMD_EXECUTES_CORRESPONDING_TIMES_4`"]
    #[inline(always)]
    pub fn is_cmd_executes_corresponding_times_4(&self) -> bool {
        *self == LOOP_A::CMD_EXECUTES_CORRESPONDING_TIMES_4
    }
    #[doc = "Checks if the value of the field is `CMD_EXECUTES_CORRESPONDING_TIMES_5`"]
    #[inline(always)]
    pub fn is_cmd_executes_corresponding_times_5(&self) -> bool {
        *self == LOOP_A::CMD_EXECUTES_CORRESPONDING_TIMES_5
    }
    #[doc = "Checks if the value of the field is `CMD_EXECUTES_CORRESPONDING_TIMES_6`"]
    #[inline(always)]
    pub fn is_cmd_executes_corresponding_times_6(&self) -> bool {
        *self == LOOP_A::CMD_EXECUTES_CORRESPONDING_TIMES_6
    }
    #[doc = "Checks if the value of the field is `CMD_EXECUTES_CORRESPONDING_TIMES_7`"]
    #[inline(always)]
    pub fn is_cmd_executes_corresponding_times_7(&self) -> bool {
        *self == LOOP_A::CMD_EXECUTES_CORRESPONDING_TIMES_7
    }
    #[doc = "Checks if the value of the field is `CMD_EXECUTES_CORRESPONDING_TIMES_8`"]
    #[inline(always)]
    pub fn is_cmd_executes_corresponding_times_8(&self) -> bool {
        *self == LOOP_A::CMD_EXECUTES_CORRESPONDING_TIMES_8
    }
    #[doc = "Checks if the value of the field is `CMD_EXECUTES_CORRESPONDING_TIMES_9`"]
    #[inline(always)]
    pub fn is_cmd_executes_corresponding_times_9(&self) -> bool {
        *self == LOOP_A::CMD_EXECUTES_CORRESPONDING_TIMES_9
    }
    #[doc = "Checks if the value of the field is `CMD_EXEC_15X`"]
    #[inline(always)]
    pub fn is_cmd_exec_15x(&self) -> bool {
        *self == LOOP_A::CMD_EXEC_15X
    }
}
#[doc = "Field `LOOP` writer - Loop Count Select"]
pub type LOOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDH7_SPEC, u8, LOOP_A, 4, O>;
impl<'a, const O: u8> LOOP_W<'a, O> {
    #[doc = "Looping not enabled. Command executes one time."]
    #[inline(always)]
    pub fn cmd_exec_1x(self) -> &'a mut W {
        self.variant(LOOP_A::CMD_EXEC_1X)
    }
    #[doc = "Loop one time. Command executes two times."]
    #[inline(always)]
    pub fn cmd_exec_2x(self) -> &'a mut W {
        self.variant(LOOP_A::CMD_EXEC_2X)
    }
    #[doc = "Loop two times. Command executes three times."]
    #[inline(always)]
    pub fn cmd_exec_3x(self) -> &'a mut W {
        self.variant(LOOP_A::CMD_EXEC_3X)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
    #[inline(always)]
    pub fn cmd_executes_corresponding_times_3(self) -> &'a mut W {
        self.variant(LOOP_A::CMD_EXECUTES_CORRESPONDING_TIMES_3)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
    #[inline(always)]
    pub fn cmd_executes_corresponding_times_4(self) -> &'a mut W {
        self.variant(LOOP_A::CMD_EXECUTES_CORRESPONDING_TIMES_4)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
    #[inline(always)]
    pub fn cmd_executes_corresponding_times_5(self) -> &'a mut W {
        self.variant(LOOP_A::CMD_EXECUTES_CORRESPONDING_TIMES_5)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
    #[inline(always)]
    pub fn cmd_executes_corresponding_times_6(self) -> &'a mut W {
        self.variant(LOOP_A::CMD_EXECUTES_CORRESPONDING_TIMES_6)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
    #[inline(always)]
    pub fn cmd_executes_corresponding_times_7(self) -> &'a mut W {
        self.variant(LOOP_A::CMD_EXECUTES_CORRESPONDING_TIMES_7)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
    #[inline(always)]
    pub fn cmd_executes_corresponding_times_8(self) -> &'a mut W {
        self.variant(LOOP_A::CMD_EXECUTES_CORRESPONDING_TIMES_8)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
    #[inline(always)]
    pub fn cmd_executes_corresponding_times_9(self) -> &'a mut W {
        self.variant(LOOP_A::CMD_EXECUTES_CORRESPONDING_TIMES_9)
    }
    #[doc = "Loop 15 times. Command executes 16 times."]
    #[inline(always)]
    pub fn cmd_exec_15x(self) -> &'a mut W {
        self.variant(LOOP_A::CMD_EXEC_15X)
    }
}
#[doc = "Field `NEXT` reader - Next Command Select"]
pub type NEXT_R = crate::FieldReader<u8, NEXT_A>;
#[doc = "Next Command Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NEXT_A {
    #[doc = "0: No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NO_NEXT_CMD_TERMINATE_ON_FINISH = 0,
    #[doc = "1: CMD1"]
    DO_CMD1_NEXT = 1,
    #[doc = "2: Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_2 = 2,
    #[doc = "3: Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_3 = 3,
    #[doc = "4: Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_4 = 4,
    #[doc = "5: Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_5 = 5,
    #[doc = "6: Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_6 = 6,
    #[doc = "7: Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_7 = 7,
    #[doc = "8: Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_8 = 8,
    #[doc = "9: Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_9 = 9,
    #[doc = "15: CMD15"]
    DO_CMD15_NEXT = 15,
}
impl From<NEXT_A> for u8 {
    #[inline(always)]
    fn from(variant: NEXT_A) -> Self {
        variant as _
    }
}
impl NEXT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NEXT_A> {
        match self.bits {
            0 => Some(NEXT_A::NO_NEXT_CMD_TERMINATE_ON_FINISH),
            1 => Some(NEXT_A::DO_CMD1_NEXT),
            2 => Some(NEXT_A::DO_CORRESPONDING_CMD_NEXT_2),
            3 => Some(NEXT_A::DO_CORRESPONDING_CMD_NEXT_3),
            4 => Some(NEXT_A::DO_CORRESPONDING_CMD_NEXT_4),
            5 => Some(NEXT_A::DO_CORRESPONDING_CMD_NEXT_5),
            6 => Some(NEXT_A::DO_CORRESPONDING_CMD_NEXT_6),
            7 => Some(NEXT_A::DO_CORRESPONDING_CMD_NEXT_7),
            8 => Some(NEXT_A::DO_CORRESPONDING_CMD_NEXT_8),
            9 => Some(NEXT_A::DO_CORRESPONDING_CMD_NEXT_9),
            15 => Some(NEXT_A::DO_CMD15_NEXT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_NEXT_CMD_TERMINATE_ON_FINISH`"]
    #[inline(always)]
    pub fn is_no_next_cmd_terminate_on_finish(&self) -> bool {
        *self == NEXT_A::NO_NEXT_CMD_TERMINATE_ON_FINISH
    }
    #[doc = "Checks if the value of the field is `DO_CMD1_NEXT`"]
    #[inline(always)]
    pub fn is_do_cmd1_next(&self) -> bool {
        *self == NEXT_A::DO_CMD1_NEXT
    }
    #[doc = "Checks if the value of the field is `DO_CORRESPONDING_CMD_NEXT_2`"]
    #[inline(always)]
    pub fn is_do_corresponding_cmd_next_2(&self) -> bool {
        *self == NEXT_A::DO_CORRESPONDING_CMD_NEXT_2
    }
    #[doc = "Checks if the value of the field is `DO_CORRESPONDING_CMD_NEXT_3`"]
    #[inline(always)]
    pub fn is_do_corresponding_cmd_next_3(&self) -> bool {
        *self == NEXT_A::DO_CORRESPONDING_CMD_NEXT_3
    }
    #[doc = "Checks if the value of the field is `DO_CORRESPONDING_CMD_NEXT_4`"]
    #[inline(always)]
    pub fn is_do_corresponding_cmd_next_4(&self) -> bool {
        *self == NEXT_A::DO_CORRESPONDING_CMD_NEXT_4
    }
    #[doc = "Checks if the value of the field is `DO_CORRESPONDING_CMD_NEXT_5`"]
    #[inline(always)]
    pub fn is_do_corresponding_cmd_next_5(&self) -> bool {
        *self == NEXT_A::DO_CORRESPONDING_CMD_NEXT_5
    }
    #[doc = "Checks if the value of the field is `DO_CORRESPONDING_CMD_NEXT_6`"]
    #[inline(always)]
    pub fn is_do_corresponding_cmd_next_6(&self) -> bool {
        *self == NEXT_A::DO_CORRESPONDING_CMD_NEXT_6
    }
    #[doc = "Checks if the value of the field is `DO_CORRESPONDING_CMD_NEXT_7`"]
    #[inline(always)]
    pub fn is_do_corresponding_cmd_next_7(&self) -> bool {
        *self == NEXT_A::DO_CORRESPONDING_CMD_NEXT_7
    }
    #[doc = "Checks if the value of the field is `DO_CORRESPONDING_CMD_NEXT_8`"]
    #[inline(always)]
    pub fn is_do_corresponding_cmd_next_8(&self) -> bool {
        *self == NEXT_A::DO_CORRESPONDING_CMD_NEXT_8
    }
    #[doc = "Checks if the value of the field is `DO_CORRESPONDING_CMD_NEXT_9`"]
    #[inline(always)]
    pub fn is_do_corresponding_cmd_next_9(&self) -> bool {
        *self == NEXT_A::DO_CORRESPONDING_CMD_NEXT_9
    }
    #[doc = "Checks if the value of the field is `DO_CMD15_NEXT`"]
    #[inline(always)]
    pub fn is_do_cmd15_next(&self) -> bool {
        *self == NEXT_A::DO_CMD15_NEXT
    }
}
#[doc = "Field `NEXT` writer - Next Command Select"]
pub type NEXT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDH7_SPEC, u8, NEXT_A, 4, O>;
impl<'a, const O: u8> NEXT_W<'a, O> {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    #[inline(always)]
    pub fn no_next_cmd_terminate_on_finish(self) -> &'a mut W {
        self.variant(NEXT_A::NO_NEXT_CMD_TERMINATE_ON_FINISH)
    }
    #[doc = "CMD1"]
    #[inline(always)]
    pub fn do_cmd1_next(self) -> &'a mut W {
        self.variant(NEXT_A::DO_CMD1_NEXT)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn do_corresponding_cmd_next_2(self) -> &'a mut W {
        self.variant(NEXT_A::DO_CORRESPONDING_CMD_NEXT_2)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn do_corresponding_cmd_next_3(self) -> &'a mut W {
        self.variant(NEXT_A::DO_CORRESPONDING_CMD_NEXT_3)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn do_corresponding_cmd_next_4(self) -> &'a mut W {
        self.variant(NEXT_A::DO_CORRESPONDING_CMD_NEXT_4)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn do_corresponding_cmd_next_5(self) -> &'a mut W {
        self.variant(NEXT_A::DO_CORRESPONDING_CMD_NEXT_5)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn do_corresponding_cmd_next_6(self) -> &'a mut W {
        self.variant(NEXT_A::DO_CORRESPONDING_CMD_NEXT_6)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn do_corresponding_cmd_next_7(self) -> &'a mut W {
        self.variant(NEXT_A::DO_CORRESPONDING_CMD_NEXT_7)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn do_corresponding_cmd_next_8(self) -> &'a mut W {
        self.variant(NEXT_A::DO_CORRESPONDING_CMD_NEXT_8)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn do_corresponding_cmd_next_9(self) -> &'a mut W {
        self.variant(NEXT_A::DO_CORRESPONDING_CMD_NEXT_9)
    }
    #[doc = "CMD15"]
    #[inline(always)]
    pub fn do_cmd15_next(self) -> &'a mut W {
        self.variant(NEXT_A::DO_CMD15_NEXT)
    }
}
impl R {
    #[doc = "Bit 2 - Wait for Trigger Assertion Before Execution"]
    #[inline(always)]
    pub fn wait_trig(&self) -> WAIT_TRIG_R {
        WAIT_TRIG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Loop with Increment"]
    #[inline(always)]
    pub fn lwi(&self) -> LWI_R {
        LWI_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Sample Time Select"]
    #[inline(always)]
    pub fn sts(&self) -> STS_R {
        STS_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Hardware Average Select"]
    #[inline(always)]
    pub fn avgs(&self) -> AVGS_R {
        AVGS_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Loop Count Select"]
    #[inline(always)]
    pub fn loop_(&self) -> LOOP_R {
        LOOP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Next Command Select"]
    #[inline(always)]
    pub fn next(&self) -> NEXT_R {
        NEXT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Wait for Trigger Assertion Before Execution"]
    #[inline(always)]
    #[must_use]
    pub fn wait_trig(&mut self) -> WAIT_TRIG_W<2> {
        WAIT_TRIG_W::new(self)
    }
    #[doc = "Bit 7 - Loop with Increment"]
    #[inline(always)]
    #[must_use]
    pub fn lwi(&mut self) -> LWI_W<7> {
        LWI_W::new(self)
    }
    #[doc = "Bits 8:10 - Sample Time Select"]
    #[inline(always)]
    #[must_use]
    pub fn sts(&mut self) -> STS_W<8> {
        STS_W::new(self)
    }
    #[doc = "Bits 12:14 - Hardware Average Select"]
    #[inline(always)]
    #[must_use]
    pub fn avgs(&mut self) -> AVGS_W<12> {
        AVGS_W::new(self)
    }
    #[doc = "Bits 16:19 - Loop Count Select"]
    #[inline(always)]
    #[must_use]
    pub fn loop_(&mut self) -> LOOP_W<16> {
        LOOP_W::new(self)
    }
    #[doc = "Bits 24:27 - Next Command Select"]
    #[inline(always)]
    #[must_use]
    pub fn next(&mut self) -> NEXT_W<24> {
        NEXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command High Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdh7](index.html) module"]
pub struct CMDH7_SPEC;
impl crate::RegisterSpec for CMDH7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdh7::R](R) reader structure"]
impl crate::Readable for CMDH7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdh7::W](W) writer structure"]
impl crate::Writable for CMDH7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDH7 to value 0"]
impl crate::Resettable for CMDH7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
