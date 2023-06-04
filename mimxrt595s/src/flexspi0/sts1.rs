#[doc = "Register `STS1` reader"]
pub struct R(crate::R<STS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AHBCMDERRID` reader - Indicates the sequence index when an AHB command error is detected. This field will be cleared when INTR\\[AHBCMDERR\\]
is write-1-clear(w1c)."]
pub type AHBCMDERRID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AHBCMDERRCODE` reader - Indicates the Error Code when AHB command Error detected. This field will be cleared when INTR\\[AHBCMDERR\\]
is write-1-clear(w1c)."]
pub type AHBCMDERRCODE_R = crate::FieldReader<u8, AHBCMDERRCODE_A>;
#[doc = "Indicates the Error Code when AHB command Error detected. This field will be cleared when INTR\\[AHBCMDERR\\]
is write-1-clear(w1c).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AHBCMDERRCODE_A {
    #[doc = "0: No error."]
    VAL0 = 0,
    #[doc = "2: AHB Write command with JMP_ON_CS instruction used in the sequence."]
    VAL2 = 2,
    #[doc = "3: There is unknown instruction opcode in the sequence."]
    VAL3 = 3,
    #[doc = "4: Instruction DUMMY_SDR/DUMMY_RWDS_SDR used in DDR sequence."]
    VAL4 = 4,
    #[doc = "5: Instruction DUMMY_DDR/DUMMY_RWDS_DDR used in SDR sequence."]
    VAL5 = 5,
    #[doc = "14: Sequence execution timeout."]
    VAL6 = 14,
}
impl From<AHBCMDERRCODE_A> for u8 {
    #[inline(always)]
    fn from(variant: AHBCMDERRCODE_A) -> Self {
        variant as _
    }
}
impl AHBCMDERRCODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AHBCMDERRCODE_A> {
        match self.bits {
            0 => Some(AHBCMDERRCODE_A::VAL0),
            2 => Some(AHBCMDERRCODE_A::VAL2),
            3 => Some(AHBCMDERRCODE_A::VAL3),
            4 => Some(AHBCMDERRCODE_A::VAL4),
            5 => Some(AHBCMDERRCODE_A::VAL5),
            14 => Some(AHBCMDERRCODE_A::VAL6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == AHBCMDERRCODE_A::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL2`"]
    #[inline(always)]
    pub fn is_val2(&self) -> bool {
        *self == AHBCMDERRCODE_A::VAL2
    }
    #[doc = "Checks if the value of the field is `VAL3`"]
    #[inline(always)]
    pub fn is_val3(&self) -> bool {
        *self == AHBCMDERRCODE_A::VAL3
    }
    #[doc = "Checks if the value of the field is `VAL4`"]
    #[inline(always)]
    pub fn is_val4(&self) -> bool {
        *self == AHBCMDERRCODE_A::VAL4
    }
    #[doc = "Checks if the value of the field is `VAL5`"]
    #[inline(always)]
    pub fn is_val5(&self) -> bool {
        *self == AHBCMDERRCODE_A::VAL5
    }
    #[doc = "Checks if the value of the field is `VAL6`"]
    #[inline(always)]
    pub fn is_val6(&self) -> bool {
        *self == AHBCMDERRCODE_A::VAL6
    }
}
#[doc = "Field `IPCMDERRID` reader - Indicates the sequence Index when IP command error detected."]
pub type IPCMDERRID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPCMDERRCODE` reader - Indicates the Error Code when IP command Error detected. This field will be cleared when INTR\\[IPCMDERR\\]
is write-1-clear(w1c)."]
pub type IPCMDERRCODE_R = crate::FieldReader<u8, IPCMDERRCODE_A>;
#[doc = "Indicates the Error Code when IP command Error detected. This field will be cleared when INTR\\[IPCMDERR\\]
is write-1-clear(w1c).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IPCMDERRCODE_A {
    #[doc = "0: No error."]
    VAL0 = 0,
    #[doc = "2: IP command with JMP_ON_CS instruction used in the sequence."]
    VAL2 = 2,
    #[doc = "3: There is unknown instruction opcode in the sequence."]
    VAL3 = 3,
    #[doc = "4: Instruction DUMMY_SDR/DUMMY_RWDS_SDR used in DDR sequence."]
    VAL4 = 4,
    #[doc = "5: Instruction DUMMY_DDR/DUMMY_RWDS_DDR used in SDR sequence."]
    VAL5 = 5,
    #[doc = "6: Flash access start address exceed the whole flash address range (A1/A2/B1/B2)."]
    VAL6 = 6,
    #[doc = "14: Sequence execution timeout."]
    VAL7 = 14,
    #[doc = "15: Flash boundary crossed."]
    VAL8 = 15,
}
impl From<IPCMDERRCODE_A> for u8 {
    #[inline(always)]
    fn from(variant: IPCMDERRCODE_A) -> Self {
        variant as _
    }
}
impl IPCMDERRCODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IPCMDERRCODE_A> {
        match self.bits {
            0 => Some(IPCMDERRCODE_A::VAL0),
            2 => Some(IPCMDERRCODE_A::VAL2),
            3 => Some(IPCMDERRCODE_A::VAL3),
            4 => Some(IPCMDERRCODE_A::VAL4),
            5 => Some(IPCMDERRCODE_A::VAL5),
            6 => Some(IPCMDERRCODE_A::VAL6),
            14 => Some(IPCMDERRCODE_A::VAL7),
            15 => Some(IPCMDERRCODE_A::VAL8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == IPCMDERRCODE_A::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL2`"]
    #[inline(always)]
    pub fn is_val2(&self) -> bool {
        *self == IPCMDERRCODE_A::VAL2
    }
    #[doc = "Checks if the value of the field is `VAL3`"]
    #[inline(always)]
    pub fn is_val3(&self) -> bool {
        *self == IPCMDERRCODE_A::VAL3
    }
    #[doc = "Checks if the value of the field is `VAL4`"]
    #[inline(always)]
    pub fn is_val4(&self) -> bool {
        *self == IPCMDERRCODE_A::VAL4
    }
    #[doc = "Checks if the value of the field is `VAL5`"]
    #[inline(always)]
    pub fn is_val5(&self) -> bool {
        *self == IPCMDERRCODE_A::VAL5
    }
    #[doc = "Checks if the value of the field is `VAL6`"]
    #[inline(always)]
    pub fn is_val6(&self) -> bool {
        *self == IPCMDERRCODE_A::VAL6
    }
    #[doc = "Checks if the value of the field is `VAL7`"]
    #[inline(always)]
    pub fn is_val7(&self) -> bool {
        *self == IPCMDERRCODE_A::VAL7
    }
    #[doc = "Checks if the value of the field is `VAL8`"]
    #[inline(always)]
    pub fn is_val8(&self) -> bool {
        *self == IPCMDERRCODE_A::VAL8
    }
}
impl R {
    #[doc = "Bits 0:3 - Indicates the sequence index when an AHB command error is detected. This field will be cleared when INTR\\[AHBCMDERR\\]
is write-1-clear(w1c)."]
    #[inline(always)]
    pub fn ahbcmderrid(&self) -> AHBCMDERRID_R {
        AHBCMDERRID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Indicates the Error Code when AHB command Error detected. This field will be cleared when INTR\\[AHBCMDERR\\]
is write-1-clear(w1c)."]
    #[inline(always)]
    pub fn ahbcmderrcode(&self) -> AHBCMDERRCODE_R {
        AHBCMDERRCODE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Indicates the sequence Index when IP command error detected."]
    #[inline(always)]
    pub fn ipcmderrid(&self) -> IPCMDERRID_R {
        IPCMDERRID_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Indicates the Error Code when IP command Error detected. This field will be cleared when INTR\\[IPCMDERR\\]
is write-1-clear(w1c)."]
    #[inline(always)]
    pub fn ipcmderrcode(&self) -> IPCMDERRCODE_R {
        IPCMDERRCODE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[doc = "Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts1](index.html) module"]
pub struct STS1_SPEC;
impl crate::RegisterSpec for STS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts1::R](R) reader structure"]
impl crate::Readable for STS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STS1 to value 0"]
impl crate::Resettable for STS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
