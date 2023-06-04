#[doc = "Register `RESFIFO[%s]` reader"]
pub struct R(crate::R<RESFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `D` reader - Data Result"]
pub type D_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TSRC` reader - Trigger Source"]
pub type TSRC_R = crate::FieldReader<u8, TSRC_A>;
#[doc = "Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSRC_A {
    #[doc = "0: Trigger source 0"]
    TRIGGER_0 = 0,
    #[doc = "1: Trigger source 1"]
    TRIGGER_1 = 1,
    #[doc = "2: Corresponding trigger source initiated this conversion."]
    CORRESPONDING_TRIGGER_2 = 2,
    #[doc = "3: Corresponding trigger source initiated this conversion."]
    CORRESPONDING_TRIGGER_3 = 3,
    #[doc = "4: Corresponding trigger source initiated this conversion."]
    CORRESPONDING_TRIGGER_4 = 4,
    #[doc = "5: Corresponding trigger source initiated this conversion."]
    CORRESPONDING_TRIGGER_5 = 5,
    #[doc = "6: Corresponding trigger source initiated this conversion."]
    CORRESPONDING_TRIGGER_6 = 6,
    #[doc = "7: Corresponding trigger source initiated this conversion."]
    CORRESPONDING_TRIGGER_7 = 7,
    #[doc = "8: Corresponding trigger source initiated this conversion."]
    CORRESPONDING_TRIGGER_8 = 8,
    #[doc = "9: Corresponding trigger source initiated this conversion."]
    CORRESPONDING_TRIGGER_9 = 9,
    #[doc = "15: Trigger source 15"]
    TRIGGER_15 = 15,
}
impl From<TSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: TSRC_A) -> Self {
        variant as _
    }
}
impl TSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSRC_A> {
        match self.bits {
            0 => Some(TSRC_A::TRIGGER_0),
            1 => Some(TSRC_A::TRIGGER_1),
            2 => Some(TSRC_A::CORRESPONDING_TRIGGER_2),
            3 => Some(TSRC_A::CORRESPONDING_TRIGGER_3),
            4 => Some(TSRC_A::CORRESPONDING_TRIGGER_4),
            5 => Some(TSRC_A::CORRESPONDING_TRIGGER_5),
            6 => Some(TSRC_A::CORRESPONDING_TRIGGER_6),
            7 => Some(TSRC_A::CORRESPONDING_TRIGGER_7),
            8 => Some(TSRC_A::CORRESPONDING_TRIGGER_8),
            9 => Some(TSRC_A::CORRESPONDING_TRIGGER_9),
            15 => Some(TSRC_A::TRIGGER_15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRIGGER_0`"]
    #[inline(always)]
    pub fn is_trigger_0(&self) -> bool {
        *self == TSRC_A::TRIGGER_0
    }
    #[doc = "Checks if the value of the field is `TRIGGER_1`"]
    #[inline(always)]
    pub fn is_trigger_1(&self) -> bool {
        *self == TSRC_A::TRIGGER_1
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_TRIGGER_2`"]
    #[inline(always)]
    pub fn is_corresponding_trigger_2(&self) -> bool {
        *self == TSRC_A::CORRESPONDING_TRIGGER_2
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_TRIGGER_3`"]
    #[inline(always)]
    pub fn is_corresponding_trigger_3(&self) -> bool {
        *self == TSRC_A::CORRESPONDING_TRIGGER_3
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_TRIGGER_4`"]
    #[inline(always)]
    pub fn is_corresponding_trigger_4(&self) -> bool {
        *self == TSRC_A::CORRESPONDING_TRIGGER_4
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_TRIGGER_5`"]
    #[inline(always)]
    pub fn is_corresponding_trigger_5(&self) -> bool {
        *self == TSRC_A::CORRESPONDING_TRIGGER_5
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_TRIGGER_6`"]
    #[inline(always)]
    pub fn is_corresponding_trigger_6(&self) -> bool {
        *self == TSRC_A::CORRESPONDING_TRIGGER_6
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_TRIGGER_7`"]
    #[inline(always)]
    pub fn is_corresponding_trigger_7(&self) -> bool {
        *self == TSRC_A::CORRESPONDING_TRIGGER_7
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_TRIGGER_8`"]
    #[inline(always)]
    pub fn is_corresponding_trigger_8(&self) -> bool {
        *self == TSRC_A::CORRESPONDING_TRIGGER_8
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_TRIGGER_9`"]
    #[inline(always)]
    pub fn is_corresponding_trigger_9(&self) -> bool {
        *self == TSRC_A::CORRESPONDING_TRIGGER_9
    }
    #[doc = "Checks if the value of the field is `TRIGGER_15`"]
    #[inline(always)]
    pub fn is_trigger_15(&self) -> bool {
        *self == TSRC_A::TRIGGER_15
    }
}
#[doc = "Field `LOOPCNT` reader - Loop Count Value"]
pub type LOOPCNT_R = crate::FieldReader<u8, LOOPCNT_A>;
#[doc = "Loop Count Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOOPCNT_A {
    #[doc = "0: Result is from initial conversion in command."]
    RESULT_1 = 0,
    #[doc = "1: Result is from second conversion in command."]
    RESULT_2 = 1,
    #[doc = "2: Result is from (LOOPCNT + 1) conversion in command."]
    CORRESPONDING_RESULT_2 = 2,
    #[doc = "3: Result is from (LOOPCNT + 1) conversion in command."]
    CORRESPONDING_RESULT_3 = 3,
    #[doc = "4: Result is from (LOOPCNT + 1) conversion in command."]
    CORRESPONDING_RESULT_4 = 4,
    #[doc = "5: Result is from (LOOPCNT + 1) conversion in command."]
    CORRESPONDING_RESULT_5 = 5,
    #[doc = "6: Result is from (LOOPCNT + 1) conversion in command."]
    CORRESPONDING_RESULT_6 = 6,
    #[doc = "7: Result is from (LOOPCNT + 1) conversion in command."]
    CORRESPONDING_RESULT_7 = 7,
    #[doc = "8: Result is from (LOOPCNT + 1) conversion in command."]
    CORRESPONDING_RESULT_8 = 8,
    #[doc = "9: Result is from (LOOPCNT + 1) conversion in command."]
    CORRESPONDING_RESULT_9 = 9,
    #[doc = "15: Result is from 16th conversion in command."]
    RESULT_16 = 15,
}
impl From<LOOPCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: LOOPCNT_A) -> Self {
        variant as _
    }
}
impl LOOPCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOOPCNT_A> {
        match self.bits {
            0 => Some(LOOPCNT_A::RESULT_1),
            1 => Some(LOOPCNT_A::RESULT_2),
            2 => Some(LOOPCNT_A::CORRESPONDING_RESULT_2),
            3 => Some(LOOPCNT_A::CORRESPONDING_RESULT_3),
            4 => Some(LOOPCNT_A::CORRESPONDING_RESULT_4),
            5 => Some(LOOPCNT_A::CORRESPONDING_RESULT_5),
            6 => Some(LOOPCNT_A::CORRESPONDING_RESULT_6),
            7 => Some(LOOPCNT_A::CORRESPONDING_RESULT_7),
            8 => Some(LOOPCNT_A::CORRESPONDING_RESULT_8),
            9 => Some(LOOPCNT_A::CORRESPONDING_RESULT_9),
            15 => Some(LOOPCNT_A::RESULT_16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESULT_1`"]
    #[inline(always)]
    pub fn is_result_1(&self) -> bool {
        *self == LOOPCNT_A::RESULT_1
    }
    #[doc = "Checks if the value of the field is `RESULT_2`"]
    #[inline(always)]
    pub fn is_result_2(&self) -> bool {
        *self == LOOPCNT_A::RESULT_2
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_RESULT_2`"]
    #[inline(always)]
    pub fn is_corresponding_result_2(&self) -> bool {
        *self == LOOPCNT_A::CORRESPONDING_RESULT_2
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_RESULT_3`"]
    #[inline(always)]
    pub fn is_corresponding_result_3(&self) -> bool {
        *self == LOOPCNT_A::CORRESPONDING_RESULT_3
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_RESULT_4`"]
    #[inline(always)]
    pub fn is_corresponding_result_4(&self) -> bool {
        *self == LOOPCNT_A::CORRESPONDING_RESULT_4
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_RESULT_5`"]
    #[inline(always)]
    pub fn is_corresponding_result_5(&self) -> bool {
        *self == LOOPCNT_A::CORRESPONDING_RESULT_5
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_RESULT_6`"]
    #[inline(always)]
    pub fn is_corresponding_result_6(&self) -> bool {
        *self == LOOPCNT_A::CORRESPONDING_RESULT_6
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_RESULT_7`"]
    #[inline(always)]
    pub fn is_corresponding_result_7(&self) -> bool {
        *self == LOOPCNT_A::CORRESPONDING_RESULT_7
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_RESULT_8`"]
    #[inline(always)]
    pub fn is_corresponding_result_8(&self) -> bool {
        *self == LOOPCNT_A::CORRESPONDING_RESULT_8
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_RESULT_9`"]
    #[inline(always)]
    pub fn is_corresponding_result_9(&self) -> bool {
        *self == LOOPCNT_A::CORRESPONDING_RESULT_9
    }
    #[doc = "Checks if the value of the field is `RESULT_16`"]
    #[inline(always)]
    pub fn is_result_16(&self) -> bool {
        *self == LOOPCNT_A::RESULT_16
    }
}
#[doc = "Field `CMDSRC` reader - Command Buffer Source"]
pub type CMDSRC_R = crate::FieldReader<u8, CMDSRC_A>;
#[doc = "Command Buffer Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDSRC_A {
    #[doc = "0: Not a valid value CMDSRC value for a data word in RESFIFO. 0h is only found in the initial FIFO state, prior to the storage of an ADC conversion result into a RESFIFO buffer."]
    NOT_VALID = 0,
    #[doc = "1: CMD1"]
    CMD1 = 1,
    #[doc = "2: Corresponding command buffer used as control settings for this conversion."]
    CORRESPONDING_CMD_2 = 2,
    #[doc = "3: Corresponding command buffer used as control settings for this conversion."]
    CORRESPONDING_CMD_3 = 3,
    #[doc = "4: Corresponding command buffer used as control settings for this conversion."]
    CORRESPONDING_CMD_4 = 4,
    #[doc = "5: Corresponding command buffer used as control settings for this conversion."]
    CORRESPONDING_CMD_5 = 5,
    #[doc = "6: Corresponding command buffer used as control settings for this conversion."]
    CORRESPONDING_CMD_6 = 6,
    #[doc = "7: Corresponding command buffer used as control settings for this conversion."]
    CORRESPONDING_CMD_7 = 7,
    #[doc = "8: Corresponding command buffer used as control settings for this conversion."]
    CORRESPONDING_CMD_8 = 8,
    #[doc = "9: Corresponding command buffer used as control settings for this conversion."]
    CORRESPONDING_CMD_9 = 9,
    #[doc = "15: CMD15"]
    CMD15 = 15,
}
impl From<CMDSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDSRC_A) -> Self {
        variant as _
    }
}
impl CMDSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMDSRC_A> {
        match self.bits {
            0 => Some(CMDSRC_A::NOT_VALID),
            1 => Some(CMDSRC_A::CMD1),
            2 => Some(CMDSRC_A::CORRESPONDING_CMD_2),
            3 => Some(CMDSRC_A::CORRESPONDING_CMD_3),
            4 => Some(CMDSRC_A::CORRESPONDING_CMD_4),
            5 => Some(CMDSRC_A::CORRESPONDING_CMD_5),
            6 => Some(CMDSRC_A::CORRESPONDING_CMD_6),
            7 => Some(CMDSRC_A::CORRESPONDING_CMD_7),
            8 => Some(CMDSRC_A::CORRESPONDING_CMD_8),
            9 => Some(CMDSRC_A::CORRESPONDING_CMD_9),
            15 => Some(CMDSRC_A::CMD15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == CMDSRC_A::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `CMD1`"]
    #[inline(always)]
    pub fn is_cmd1(&self) -> bool {
        *self == CMDSRC_A::CMD1
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_CMD_2`"]
    #[inline(always)]
    pub fn is_corresponding_cmd_2(&self) -> bool {
        *self == CMDSRC_A::CORRESPONDING_CMD_2
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_CMD_3`"]
    #[inline(always)]
    pub fn is_corresponding_cmd_3(&self) -> bool {
        *self == CMDSRC_A::CORRESPONDING_CMD_3
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_CMD_4`"]
    #[inline(always)]
    pub fn is_corresponding_cmd_4(&self) -> bool {
        *self == CMDSRC_A::CORRESPONDING_CMD_4
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_CMD_5`"]
    #[inline(always)]
    pub fn is_corresponding_cmd_5(&self) -> bool {
        *self == CMDSRC_A::CORRESPONDING_CMD_5
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_CMD_6`"]
    #[inline(always)]
    pub fn is_corresponding_cmd_6(&self) -> bool {
        *self == CMDSRC_A::CORRESPONDING_CMD_6
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_CMD_7`"]
    #[inline(always)]
    pub fn is_corresponding_cmd_7(&self) -> bool {
        *self == CMDSRC_A::CORRESPONDING_CMD_7
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_CMD_8`"]
    #[inline(always)]
    pub fn is_corresponding_cmd_8(&self) -> bool {
        *self == CMDSRC_A::CORRESPONDING_CMD_8
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_CMD_9`"]
    #[inline(always)]
    pub fn is_corresponding_cmd_9(&self) -> bool {
        *self == CMDSRC_A::CORRESPONDING_CMD_9
    }
    #[doc = "Checks if the value of the field is `CMD15`"]
    #[inline(always)]
    pub fn is_cmd15(&self) -> bool {
        *self == CMDSRC_A::CMD15
    }
}
#[doc = "Field `VALID` reader - FIFO Entry is Valid"]
pub type VALID_R = crate::BitReader<VALID_A>;
#[doc = "FIFO Entry is Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VALID_A {
    #[doc = "0: FIFO is empty. Discard any read from RESFIFO."]
    NOT_VALID = 0,
    #[doc = "1: FIFO contains data. FIFO record read from RESFIFO is valid."]
    VALID = 1,
}
impl From<VALID_A> for bool {
    #[inline(always)]
    fn from(variant: VALID_A) -> Self {
        variant as u8 != 0
    }
}
impl VALID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VALID_A {
        match self.bits {
            false => VALID_A::NOT_VALID,
            true => VALID_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == VALID_A::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VALID_A::VALID
    }
}
impl R {
    #[doc = "Bits 0:15 - Data Result"]
    #[inline(always)]
    pub fn d(&self) -> D_R {
        D_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Trigger Source"]
    #[inline(always)]
    pub fn tsrc(&self) -> TSRC_R {
        TSRC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Loop Count Value"]
    #[inline(always)]
    pub fn loopcnt(&self) -> LOOPCNT_R {
        LOOPCNT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Command Buffer Source"]
    #[inline(always)]
    pub fn cmdsrc(&self) -> CMDSRC_R {
        CMDSRC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - FIFO Entry is Valid"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Data Result FIFO Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resfifo](index.html) module"]
pub struct RESFIFO_SPEC;
impl crate::RegisterSpec for RESFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resfifo::R](R) reader structure"]
impl crate::Readable for RESFIFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESFIFO[%s]
to value 0"]
impl crate::Resettable for RESFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
