#[doc = "Register `TCTRL[%s]` reader"]
pub struct R(crate::R<TCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCTRL[%s]` writer"]
pub struct W(crate::W<TCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCTRL_SPEC>;
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
impl From<crate::W<TCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HTEN` reader - Trigger Enable"]
pub type HTEN_R = crate::BitReader<HTEN_A>;
#[doc = "Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTEN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<HTEN_A> for bool {
    #[inline(always)]
    fn from(variant: HTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl HTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTEN_A {
        match self.bits {
            false => HTEN_A::DISABLED,
            true => HTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HTEN_A::ENABLED
    }
}
#[doc = "Field `HTEN` writer - Trigger Enable"]
pub type HTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCTRL_SPEC, HTEN_A, O>;
impl<'a, const O: u8> HTEN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HTEN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HTEN_A::ENABLED)
    }
}
#[doc = "Field `FIFO_SEL_A` reader - SAR Result Destination For Channel A"]
pub type FIFO_SEL_A_R = crate::BitReader<FIFO_SEL_A_A>;
#[doc = "SAR Result Destination For Channel A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFO_SEL_A_A {
    #[doc = "0: FIFO 0"]
    STORE_TO_FIFO0 = 0,
    #[doc = "1: FIFO 1"]
    STORE_TO_FIFO1 = 1,
}
impl From<FIFO_SEL_A_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_SEL_A_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_SEL_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_SEL_A_A {
        match self.bits {
            false => FIFO_SEL_A_A::STORE_TO_FIFO0,
            true => FIFO_SEL_A_A::STORE_TO_FIFO1,
        }
    }
    #[doc = "Checks if the value of the field is `STORE_TO_FIFO0`"]
    #[inline(always)]
    pub fn is_store_to_fifo0(&self) -> bool {
        *self == FIFO_SEL_A_A::STORE_TO_FIFO0
    }
    #[doc = "Checks if the value of the field is `STORE_TO_FIFO1`"]
    #[inline(always)]
    pub fn is_store_to_fifo1(&self) -> bool {
        *self == FIFO_SEL_A_A::STORE_TO_FIFO1
    }
}
#[doc = "Field `FIFO_SEL_A` writer - SAR Result Destination For Channel A"]
pub type FIFO_SEL_A_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCTRL_SPEC, FIFO_SEL_A_A, O>;
impl<'a, const O: u8> FIFO_SEL_A_W<'a, O> {
    #[doc = "FIFO 0"]
    #[inline(always)]
    pub fn store_to_fifo0(self) -> &'a mut W {
        self.variant(FIFO_SEL_A_A::STORE_TO_FIFO0)
    }
    #[doc = "FIFO 1"]
    #[inline(always)]
    pub fn store_to_fifo1(self) -> &'a mut W {
        self.variant(FIFO_SEL_A_A::STORE_TO_FIFO1)
    }
}
#[doc = "Field `TPRI` reader - Trigger Priority Setting"]
pub type TPRI_R = crate::FieldReader<u8, TPRI_A>;
#[doc = "Trigger Priority Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TPRI_A {
    #[doc = "0: Highest priority, Level 1"]
    HIGHEST_PRIORITY = 0,
    #[doc = "1: Set to corresponding priority level."]
    CORRESPONDING_LOWER_PRIORITY_1 = 1,
    #[doc = "2: Set to corresponding priority level."]
    CORRESPONDING_LOWER_PRIORITY_2 = 2,
    #[doc = "3: Set to corresponding priority level."]
    CORRESPONDING_LOWER_PRIORITY_3 = 3,
    #[doc = "4: Set to corresponding priority level."]
    CORRESPONDING_LOWER_PRIORITY_4 = 4,
    #[doc = "5: Set to corresponding priority level."]
    CORRESPONDING_LOWER_PRIORITY_5 = 5,
    #[doc = "6: Set to corresponding priority level."]
    CORRESPONDING_LOWER_PRIORITY_6 = 6,
    #[doc = "7: Set to corresponding priority level."]
    CORRESPONDING_LOWER_PRIORITY_7 = 7,
    #[doc = "8: Set to corresponding priority level."]
    CORRESPONDING_LOWER_PRIORITY_8 = 8,
    #[doc = "9: Set to corresponding priority level."]
    CORRESPONDING_LOWER_PRIORITY_9 = 9,
    #[doc = "15: Lowest priority, Level 16"]
    LOWEST_PRIORITY = 15,
}
impl From<TPRI_A> for u8 {
    #[inline(always)]
    fn from(variant: TPRI_A) -> Self {
        variant as _
    }
}
impl TPRI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TPRI_A> {
        match self.bits {
            0 => Some(TPRI_A::HIGHEST_PRIORITY),
            1 => Some(TPRI_A::CORRESPONDING_LOWER_PRIORITY_1),
            2 => Some(TPRI_A::CORRESPONDING_LOWER_PRIORITY_2),
            3 => Some(TPRI_A::CORRESPONDING_LOWER_PRIORITY_3),
            4 => Some(TPRI_A::CORRESPONDING_LOWER_PRIORITY_4),
            5 => Some(TPRI_A::CORRESPONDING_LOWER_PRIORITY_5),
            6 => Some(TPRI_A::CORRESPONDING_LOWER_PRIORITY_6),
            7 => Some(TPRI_A::CORRESPONDING_LOWER_PRIORITY_7),
            8 => Some(TPRI_A::CORRESPONDING_LOWER_PRIORITY_8),
            9 => Some(TPRI_A::CORRESPONDING_LOWER_PRIORITY_9),
            15 => Some(TPRI_A::LOWEST_PRIORITY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HIGHEST_PRIORITY`"]
    #[inline(always)]
    pub fn is_highest_priority(&self) -> bool {
        *self == TPRI_A::HIGHEST_PRIORITY
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_LOWER_PRIORITY_1`"]
    #[inline(always)]
    pub fn is_corresponding_lower_priority_1(&self) -> bool {
        *self == TPRI_A::CORRESPONDING_LOWER_PRIORITY_1
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_LOWER_PRIORITY_2`"]
    #[inline(always)]
    pub fn is_corresponding_lower_priority_2(&self) -> bool {
        *self == TPRI_A::CORRESPONDING_LOWER_PRIORITY_2
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_LOWER_PRIORITY_3`"]
    #[inline(always)]
    pub fn is_corresponding_lower_priority_3(&self) -> bool {
        *self == TPRI_A::CORRESPONDING_LOWER_PRIORITY_3
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_LOWER_PRIORITY_4`"]
    #[inline(always)]
    pub fn is_corresponding_lower_priority_4(&self) -> bool {
        *self == TPRI_A::CORRESPONDING_LOWER_PRIORITY_4
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_LOWER_PRIORITY_5`"]
    #[inline(always)]
    pub fn is_corresponding_lower_priority_5(&self) -> bool {
        *self == TPRI_A::CORRESPONDING_LOWER_PRIORITY_5
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_LOWER_PRIORITY_6`"]
    #[inline(always)]
    pub fn is_corresponding_lower_priority_6(&self) -> bool {
        *self == TPRI_A::CORRESPONDING_LOWER_PRIORITY_6
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_LOWER_PRIORITY_7`"]
    #[inline(always)]
    pub fn is_corresponding_lower_priority_7(&self) -> bool {
        *self == TPRI_A::CORRESPONDING_LOWER_PRIORITY_7
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_LOWER_PRIORITY_8`"]
    #[inline(always)]
    pub fn is_corresponding_lower_priority_8(&self) -> bool {
        *self == TPRI_A::CORRESPONDING_LOWER_PRIORITY_8
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING_LOWER_PRIORITY_9`"]
    #[inline(always)]
    pub fn is_corresponding_lower_priority_9(&self) -> bool {
        *self == TPRI_A::CORRESPONDING_LOWER_PRIORITY_9
    }
    #[doc = "Checks if the value of the field is `LOWEST_PRIORITY`"]
    #[inline(always)]
    pub fn is_lowest_priority(&self) -> bool {
        *self == TPRI_A::LOWEST_PRIORITY
    }
}
#[doc = "Field `TPRI` writer - Trigger Priority Setting"]
pub type TPRI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCTRL_SPEC, u8, TPRI_A, 4, O>;
impl<'a, const O: u8> TPRI_W<'a, O> {
    #[doc = "Highest priority, Level 1"]
    #[inline(always)]
    pub fn highest_priority(self) -> &'a mut W {
        self.variant(TPRI_A::HIGHEST_PRIORITY)
    }
    #[doc = "Set to corresponding priority level."]
    #[inline(always)]
    pub fn corresponding_lower_priority_1(self) -> &'a mut W {
        self.variant(TPRI_A::CORRESPONDING_LOWER_PRIORITY_1)
    }
    #[doc = "Set to corresponding priority level."]
    #[inline(always)]
    pub fn corresponding_lower_priority_2(self) -> &'a mut W {
        self.variant(TPRI_A::CORRESPONDING_LOWER_PRIORITY_2)
    }
    #[doc = "Set to corresponding priority level."]
    #[inline(always)]
    pub fn corresponding_lower_priority_3(self) -> &'a mut W {
        self.variant(TPRI_A::CORRESPONDING_LOWER_PRIORITY_3)
    }
    #[doc = "Set to corresponding priority level."]
    #[inline(always)]
    pub fn corresponding_lower_priority_4(self) -> &'a mut W {
        self.variant(TPRI_A::CORRESPONDING_LOWER_PRIORITY_4)
    }
    #[doc = "Set to corresponding priority level."]
    #[inline(always)]
    pub fn corresponding_lower_priority_5(self) -> &'a mut W {
        self.variant(TPRI_A::CORRESPONDING_LOWER_PRIORITY_5)
    }
    #[doc = "Set to corresponding priority level."]
    #[inline(always)]
    pub fn corresponding_lower_priority_6(self) -> &'a mut W {
        self.variant(TPRI_A::CORRESPONDING_LOWER_PRIORITY_6)
    }
    #[doc = "Set to corresponding priority level."]
    #[inline(always)]
    pub fn corresponding_lower_priority_7(self) -> &'a mut W {
        self.variant(TPRI_A::CORRESPONDING_LOWER_PRIORITY_7)
    }
    #[doc = "Set to corresponding priority level."]
    #[inline(always)]
    pub fn corresponding_lower_priority_8(self) -> &'a mut W {
        self.variant(TPRI_A::CORRESPONDING_LOWER_PRIORITY_8)
    }
    #[doc = "Set to corresponding priority level."]
    #[inline(always)]
    pub fn corresponding_lower_priority_9(self) -> &'a mut W {
        self.variant(TPRI_A::CORRESPONDING_LOWER_PRIORITY_9)
    }
    #[doc = "Lowest priority, Level 16"]
    #[inline(always)]
    pub fn lowest_priority(self) -> &'a mut W {
        self.variant(TPRI_A::LOWEST_PRIORITY)
    }
}
#[doc = "Field `RSYNC` reader - Trigger Resync"]
pub type RSYNC_R = crate::BitReader<RSYNC_A>;
#[doc = "Trigger Resync\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSYNC_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<RSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: RSYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl RSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSYNC_A {
        match self.bits {
            false => RSYNC_A::DISABLE,
            true => RSYNC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RSYNC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RSYNC_A::ENABLE
    }
}
#[doc = "Field `RSYNC` writer - Trigger Resync"]
pub type RSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCTRL_SPEC, RSYNC_A, O>;
impl<'a, const O: u8> RSYNC_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RSYNC_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RSYNC_A::ENABLE)
    }
}
#[doc = "Field `TDLY` reader - Trigger Delay Select"]
pub type TDLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TDLY` writer - Trigger Delay Select"]
pub type TDLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `TCMD` reader - Trigger Command Select"]
pub type TCMD_R = crate::FieldReader<u8, TCMD_A>;
#[doc = "Trigger Command Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCMD_A {
    #[doc = "0: Not a valid selection from the command buffer. Trigger event is ignored."]
    NOT_VALID = 0,
    #[doc = "1: CMD1"]
    EXECUTE_CMD1 = 1,
    #[doc = "2: Corresponding CMD is executed"]
    EXECUTE_CORRESPONDING_CMD_2 = 2,
    #[doc = "3: Corresponding CMD is executed"]
    EXECUTE_CORRESPONDING_CMD_3 = 3,
    #[doc = "4: Corresponding CMD is executed"]
    EXECUTE_CORRESPONDING_CMD_4 = 4,
    #[doc = "5: Corresponding CMD is executed"]
    EXECUTE_CORRESPONDING_CMD_5 = 5,
    #[doc = "6: Corresponding CMD is executed"]
    EXECUTE_CORRESPONDING_CMD_6 = 6,
    #[doc = "7: Corresponding CMD is executed"]
    EXECUTE_CORRESPONDING_CMD_7 = 7,
    #[doc = "8: Corresponding CMD is executed"]
    EXECUTE_CORRESPONDING_CMD_8 = 8,
    #[doc = "9: Corresponding CMD is executed"]
    EXECUTE_CORRESPONDING_CMD_9 = 9,
    #[doc = "15: CMD15"]
    EXECUTE_CMD15 = 15,
}
impl From<TCMD_A> for u8 {
    #[inline(always)]
    fn from(variant: TCMD_A) -> Self {
        variant as _
    }
}
impl TCMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TCMD_A> {
        match self.bits {
            0 => Some(TCMD_A::NOT_VALID),
            1 => Some(TCMD_A::EXECUTE_CMD1),
            2 => Some(TCMD_A::EXECUTE_CORRESPONDING_CMD_2),
            3 => Some(TCMD_A::EXECUTE_CORRESPONDING_CMD_3),
            4 => Some(TCMD_A::EXECUTE_CORRESPONDING_CMD_4),
            5 => Some(TCMD_A::EXECUTE_CORRESPONDING_CMD_5),
            6 => Some(TCMD_A::EXECUTE_CORRESPONDING_CMD_6),
            7 => Some(TCMD_A::EXECUTE_CORRESPONDING_CMD_7),
            8 => Some(TCMD_A::EXECUTE_CORRESPONDING_CMD_8),
            9 => Some(TCMD_A::EXECUTE_CORRESPONDING_CMD_9),
            15 => Some(TCMD_A::EXECUTE_CMD15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == TCMD_A::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `EXECUTE_CMD1`"]
    #[inline(always)]
    pub fn is_execute_cmd1(&self) -> bool {
        *self == TCMD_A::EXECUTE_CMD1
    }
    #[doc = "Checks if the value of the field is `EXECUTE_CORRESPONDING_CMD_2`"]
    #[inline(always)]
    pub fn is_execute_corresponding_cmd_2(&self) -> bool {
        *self == TCMD_A::EXECUTE_CORRESPONDING_CMD_2
    }
    #[doc = "Checks if the value of the field is `EXECUTE_CORRESPONDING_CMD_3`"]
    #[inline(always)]
    pub fn is_execute_corresponding_cmd_3(&self) -> bool {
        *self == TCMD_A::EXECUTE_CORRESPONDING_CMD_3
    }
    #[doc = "Checks if the value of the field is `EXECUTE_CORRESPONDING_CMD_4`"]
    #[inline(always)]
    pub fn is_execute_corresponding_cmd_4(&self) -> bool {
        *self == TCMD_A::EXECUTE_CORRESPONDING_CMD_4
    }
    #[doc = "Checks if the value of the field is `EXECUTE_CORRESPONDING_CMD_5`"]
    #[inline(always)]
    pub fn is_execute_corresponding_cmd_5(&self) -> bool {
        *self == TCMD_A::EXECUTE_CORRESPONDING_CMD_5
    }
    #[doc = "Checks if the value of the field is `EXECUTE_CORRESPONDING_CMD_6`"]
    #[inline(always)]
    pub fn is_execute_corresponding_cmd_6(&self) -> bool {
        *self == TCMD_A::EXECUTE_CORRESPONDING_CMD_6
    }
    #[doc = "Checks if the value of the field is `EXECUTE_CORRESPONDING_CMD_7`"]
    #[inline(always)]
    pub fn is_execute_corresponding_cmd_7(&self) -> bool {
        *self == TCMD_A::EXECUTE_CORRESPONDING_CMD_7
    }
    #[doc = "Checks if the value of the field is `EXECUTE_CORRESPONDING_CMD_8`"]
    #[inline(always)]
    pub fn is_execute_corresponding_cmd_8(&self) -> bool {
        *self == TCMD_A::EXECUTE_CORRESPONDING_CMD_8
    }
    #[doc = "Checks if the value of the field is `EXECUTE_CORRESPONDING_CMD_9`"]
    #[inline(always)]
    pub fn is_execute_corresponding_cmd_9(&self) -> bool {
        *self == TCMD_A::EXECUTE_CORRESPONDING_CMD_9
    }
    #[doc = "Checks if the value of the field is `EXECUTE_CMD15`"]
    #[inline(always)]
    pub fn is_execute_cmd15(&self) -> bool {
        *self == TCMD_A::EXECUTE_CMD15
    }
}
#[doc = "Field `TCMD` writer - Trigger Command Select"]
pub type TCMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCTRL_SPEC, u8, TCMD_A, 4, O>;
impl<'a, const O: u8> TCMD_W<'a, O> {
    #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(TCMD_A::NOT_VALID)
    }
    #[doc = "CMD1"]
    #[inline(always)]
    pub fn execute_cmd1(self) -> &'a mut W {
        self.variant(TCMD_A::EXECUTE_CMD1)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn execute_corresponding_cmd_2(self) -> &'a mut W {
        self.variant(TCMD_A::EXECUTE_CORRESPONDING_CMD_2)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn execute_corresponding_cmd_3(self) -> &'a mut W {
        self.variant(TCMD_A::EXECUTE_CORRESPONDING_CMD_3)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn execute_corresponding_cmd_4(self) -> &'a mut W {
        self.variant(TCMD_A::EXECUTE_CORRESPONDING_CMD_4)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn execute_corresponding_cmd_5(self) -> &'a mut W {
        self.variant(TCMD_A::EXECUTE_CORRESPONDING_CMD_5)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn execute_corresponding_cmd_6(self) -> &'a mut W {
        self.variant(TCMD_A::EXECUTE_CORRESPONDING_CMD_6)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn execute_corresponding_cmd_7(self) -> &'a mut W {
        self.variant(TCMD_A::EXECUTE_CORRESPONDING_CMD_7)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn execute_corresponding_cmd_8(self) -> &'a mut W {
        self.variant(TCMD_A::EXECUTE_CORRESPONDING_CMD_8)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn execute_corresponding_cmd_9(self) -> &'a mut W {
        self.variant(TCMD_A::EXECUTE_CORRESPONDING_CMD_9)
    }
    #[doc = "CMD15"]
    #[inline(always)]
    pub fn execute_cmd15(self) -> &'a mut W {
        self.variant(TCMD_A::EXECUTE_CMD15)
    }
}
impl R {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    pub fn hten(&self) -> HTEN_R {
        HTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SAR Result Destination For Channel A"]
    #[inline(always)]
    pub fn fifo_sel_a(&self) -> FIFO_SEL_A_R {
        FIFO_SEL_A_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Trigger Priority Setting"]
    #[inline(always)]
    pub fn tpri(&self) -> TPRI_R {
        TPRI_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Trigger Resync"]
    #[inline(always)]
    pub fn rsync(&self) -> RSYNC_R {
        RSYNC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Trigger Delay Select"]
    #[inline(always)]
    pub fn tdly(&self) -> TDLY_R {
        TDLY_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Trigger Command Select"]
    #[inline(always)]
    pub fn tcmd(&self) -> TCMD_R {
        TCMD_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hten(&mut self) -> HTEN_W<0> {
        HTEN_W::new(self)
    }
    #[doc = "Bit 1 - SAR Result Destination For Channel A"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_sel_a(&mut self) -> FIFO_SEL_A_W<1> {
        FIFO_SEL_A_W::new(self)
    }
    #[doc = "Bits 8:11 - Trigger Priority Setting"]
    #[inline(always)]
    #[must_use]
    pub fn tpri(&mut self) -> TPRI_W<8> {
        TPRI_W::new(self)
    }
    #[doc = "Bit 15 - Trigger Resync"]
    #[inline(always)]
    #[must_use]
    pub fn rsync(&mut self) -> RSYNC_W<15> {
        RSYNC_W::new(self)
    }
    #[doc = "Bits 16:19 - Trigger Delay Select"]
    #[inline(always)]
    #[must_use]
    pub fn tdly(&mut self) -> TDLY_W<16> {
        TDLY_W::new(self)
    }
    #[doc = "Bits 24:27 - Trigger Command Select"]
    #[inline(always)]
    #[must_use]
    pub fn tcmd(&mut self) -> TCMD_W<24> {
        TCMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tctrl](index.html) module"]
pub struct TCTRL_SPEC;
impl crate::RegisterSpec for TCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tctrl::R](R) reader structure"]
impl crate::Readable for TCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tctrl::W](W) writer structure"]
impl crate::Writable for TCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCTRL[%s]
to value 0"]
impl crate::Resettable for TCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
