#[doc = "Register `C3` reader"]
pub struct R(crate::R<C3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C3` writer"]
pub struct W(crate::W<C3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C3_SPEC>;
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
impl From<crate::W<C3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACPH2TC` reader - Analog Comparator Phase2 Timing Control."]
pub type ACPH2TC_R = crate::FieldReader<u8, ACPH2TC_A>;
#[doc = "Analog Comparator Phase2 Timing Control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACPH2TC_A {
    #[doc = "0: Phase2 active time in one sampling period equals to T"]
    ACPH2TC_0 = 0,
    #[doc = "1: Phase2 active time in one sampling period equals to 2*T"]
    ACPH2TC_1 = 1,
    #[doc = "2: Phase2 active time in one sampling period equals to 4*T"]
    ACPH2TC_2 = 2,
    #[doc = "3: Phase2 active time in one sampling period equals to 8*T"]
    ACPH2TC_3 = 3,
    #[doc = "4: Phase2 active time in one sampling period equals to 16*T"]
    ACPH2TC_4 = 4,
    #[doc = "5: Phase2 active time in one sampling period equals to 32*T"]
    ACPH2TC_5 = 5,
    #[doc = "6: Phase2 active time in one sampling period equals to 64*T"]
    ACPH2TC_6 = 6,
    #[doc = "7: Phase2 active time in one sampling period equals to 16*T"]
    ACPH2TC_7 = 7,
}
impl From<ACPH2TC_A> for u8 {
    #[inline(always)]
    fn from(variant: ACPH2TC_A) -> Self {
        variant as _
    }
}
impl ACPH2TC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACPH2TC_A {
        match self.bits {
            0 => ACPH2TC_A::ACPH2TC_0,
            1 => ACPH2TC_A::ACPH2TC_1,
            2 => ACPH2TC_A::ACPH2TC_2,
            3 => ACPH2TC_A::ACPH2TC_3,
            4 => ACPH2TC_A::ACPH2TC_4,
            5 => ACPH2TC_A::ACPH2TC_5,
            6 => ACPH2TC_A::ACPH2TC_6,
            7 => ACPH2TC_A::ACPH2TC_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ACPH2TC_0`"]
    #[inline(always)]
    pub fn is_acph2tc_0(&self) -> bool {
        *self == ACPH2TC_A::ACPH2TC_0
    }
    #[doc = "Checks if the value of the field is `ACPH2TC_1`"]
    #[inline(always)]
    pub fn is_acph2tc_1(&self) -> bool {
        *self == ACPH2TC_A::ACPH2TC_1
    }
    #[doc = "Checks if the value of the field is `ACPH2TC_2`"]
    #[inline(always)]
    pub fn is_acph2tc_2(&self) -> bool {
        *self == ACPH2TC_A::ACPH2TC_2
    }
    #[doc = "Checks if the value of the field is `ACPH2TC_3`"]
    #[inline(always)]
    pub fn is_acph2tc_3(&self) -> bool {
        *self == ACPH2TC_A::ACPH2TC_3
    }
    #[doc = "Checks if the value of the field is `ACPH2TC_4`"]
    #[inline(always)]
    pub fn is_acph2tc_4(&self) -> bool {
        *self == ACPH2TC_A::ACPH2TC_4
    }
    #[doc = "Checks if the value of the field is `ACPH2TC_5`"]
    #[inline(always)]
    pub fn is_acph2tc_5(&self) -> bool {
        *self == ACPH2TC_A::ACPH2TC_5
    }
    #[doc = "Checks if the value of the field is `ACPH2TC_6`"]
    #[inline(always)]
    pub fn is_acph2tc_6(&self) -> bool {
        *self == ACPH2TC_A::ACPH2TC_6
    }
    #[doc = "Checks if the value of the field is `ACPH2TC_7`"]
    #[inline(always)]
    pub fn is_acph2tc_7(&self) -> bool {
        *self == ACPH2TC_A::ACPH2TC_7
    }
}
#[doc = "Field `ACPH2TC` writer - Analog Comparator Phase2 Timing Control."]
pub type ACPH2TC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, C3_SPEC, u8, ACPH2TC_A, 3, O>;
impl<'a, const O: u8> ACPH2TC_W<'a, O> {
    #[doc = "Phase2 active time in one sampling period equals to T"]
    #[inline(always)]
    pub fn acph2tc_0(self) -> &'a mut W {
        self.variant(ACPH2TC_A::ACPH2TC_0)
    }
    #[doc = "Phase2 active time in one sampling period equals to 2*T"]
    #[inline(always)]
    pub fn acph2tc_1(self) -> &'a mut W {
        self.variant(ACPH2TC_A::ACPH2TC_1)
    }
    #[doc = "Phase2 active time in one sampling period equals to 4*T"]
    #[inline(always)]
    pub fn acph2tc_2(self) -> &'a mut W {
        self.variant(ACPH2TC_A::ACPH2TC_2)
    }
    #[doc = "Phase2 active time in one sampling period equals to 8*T"]
    #[inline(always)]
    pub fn acph2tc_3(self) -> &'a mut W {
        self.variant(ACPH2TC_A::ACPH2TC_3)
    }
    #[doc = "Phase2 active time in one sampling period equals to 16*T"]
    #[inline(always)]
    pub fn acph2tc_4(self) -> &'a mut W {
        self.variant(ACPH2TC_A::ACPH2TC_4)
    }
    #[doc = "Phase2 active time in one sampling period equals to 32*T"]
    #[inline(always)]
    pub fn acph2tc_5(self) -> &'a mut W {
        self.variant(ACPH2TC_A::ACPH2TC_5)
    }
    #[doc = "Phase2 active time in one sampling period equals to 64*T"]
    #[inline(always)]
    pub fn acph2tc_6(self) -> &'a mut W {
        self.variant(ACPH2TC_A::ACPH2TC_6)
    }
    #[doc = "Phase2 active time in one sampling period equals to 16*T"]
    #[inline(always)]
    pub fn acph2tc_7(self) -> &'a mut W {
        self.variant(ACPH2TC_A::ACPH2TC_7)
    }
}
#[doc = "Field `ACPH1TC` reader - Analog Comparator Phase1 Timing Control."]
pub type ACPH1TC_R = crate::FieldReader<u8, ACPH1TC_A>;
#[doc = "Analog Comparator Phase1 Timing Control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACPH1TC_A {
    #[doc = "0: Phase1 active time in one sampling period equals to T"]
    ACPH1TC_0 = 0,
    #[doc = "1: Phase1 active time in one sampling period equals to 2*T"]
    ACPH1TC_1 = 1,
    #[doc = "2: Phase1 active time in one sampling period equals to 4*T"]
    ACPH1TC_2 = 2,
    #[doc = "3: Phase1 active time in one sampling period equals to 8*T"]
    ACPH1TC_3 = 3,
    #[doc = "4: Phase1 active time in one sampling period equals to T"]
    ACPH1TC_4 = 4,
    #[doc = "5: Phase1 active time in one sampling period equals to T"]
    ACPH1TC_5 = 5,
    #[doc = "6: Phase1 active time in one sampling period equals to T"]
    ACPH1TC_6 = 6,
    #[doc = "7: Phase1 active time in one sampling period equals to 0"]
    ACPH1TC_7 = 7,
}
impl From<ACPH1TC_A> for u8 {
    #[inline(always)]
    fn from(variant: ACPH1TC_A) -> Self {
        variant as _
    }
}
impl ACPH1TC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACPH1TC_A {
        match self.bits {
            0 => ACPH1TC_A::ACPH1TC_0,
            1 => ACPH1TC_A::ACPH1TC_1,
            2 => ACPH1TC_A::ACPH1TC_2,
            3 => ACPH1TC_A::ACPH1TC_3,
            4 => ACPH1TC_A::ACPH1TC_4,
            5 => ACPH1TC_A::ACPH1TC_5,
            6 => ACPH1TC_A::ACPH1TC_6,
            7 => ACPH1TC_A::ACPH1TC_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ACPH1TC_0`"]
    #[inline(always)]
    pub fn is_acph1tc_0(&self) -> bool {
        *self == ACPH1TC_A::ACPH1TC_0
    }
    #[doc = "Checks if the value of the field is `ACPH1TC_1`"]
    #[inline(always)]
    pub fn is_acph1tc_1(&self) -> bool {
        *self == ACPH1TC_A::ACPH1TC_1
    }
    #[doc = "Checks if the value of the field is `ACPH1TC_2`"]
    #[inline(always)]
    pub fn is_acph1tc_2(&self) -> bool {
        *self == ACPH1TC_A::ACPH1TC_2
    }
    #[doc = "Checks if the value of the field is `ACPH1TC_3`"]
    #[inline(always)]
    pub fn is_acph1tc_3(&self) -> bool {
        *self == ACPH1TC_A::ACPH1TC_3
    }
    #[doc = "Checks if the value of the field is `ACPH1TC_4`"]
    #[inline(always)]
    pub fn is_acph1tc_4(&self) -> bool {
        *self == ACPH1TC_A::ACPH1TC_4
    }
    #[doc = "Checks if the value of the field is `ACPH1TC_5`"]
    #[inline(always)]
    pub fn is_acph1tc_5(&self) -> bool {
        *self == ACPH1TC_A::ACPH1TC_5
    }
    #[doc = "Checks if the value of the field is `ACPH1TC_6`"]
    #[inline(always)]
    pub fn is_acph1tc_6(&self) -> bool {
        *self == ACPH1TC_A::ACPH1TC_6
    }
    #[doc = "Checks if the value of the field is `ACPH1TC_7`"]
    #[inline(always)]
    pub fn is_acph1tc_7(&self) -> bool {
        *self == ACPH1TC_A::ACPH1TC_7
    }
}
#[doc = "Field `ACPH1TC` writer - Analog Comparator Phase1 Timing Control."]
pub type ACPH1TC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, C3_SPEC, u8, ACPH1TC_A, 3, O>;
impl<'a, const O: u8> ACPH1TC_W<'a, O> {
    #[doc = "Phase1 active time in one sampling period equals to T"]
    #[inline(always)]
    pub fn acph1tc_0(self) -> &'a mut W {
        self.variant(ACPH1TC_A::ACPH1TC_0)
    }
    #[doc = "Phase1 active time in one sampling period equals to 2*T"]
    #[inline(always)]
    pub fn acph1tc_1(self) -> &'a mut W {
        self.variant(ACPH1TC_A::ACPH1TC_1)
    }
    #[doc = "Phase1 active time in one sampling period equals to 4*T"]
    #[inline(always)]
    pub fn acph1tc_2(self) -> &'a mut W {
        self.variant(ACPH1TC_A::ACPH1TC_2)
    }
    #[doc = "Phase1 active time in one sampling period equals to 8*T"]
    #[inline(always)]
    pub fn acph1tc_3(self) -> &'a mut W {
        self.variant(ACPH1TC_A::ACPH1TC_3)
    }
    #[doc = "Phase1 active time in one sampling period equals to T"]
    #[inline(always)]
    pub fn acph1tc_4(self) -> &'a mut W {
        self.variant(ACPH1TC_A::ACPH1TC_4)
    }
    #[doc = "Phase1 active time in one sampling period equals to T"]
    #[inline(always)]
    pub fn acph1tc_5(self) -> &'a mut W {
        self.variant(ACPH1TC_A::ACPH1TC_5)
    }
    #[doc = "Phase1 active time in one sampling period equals to T"]
    #[inline(always)]
    pub fn acph1tc_6(self) -> &'a mut W {
        self.variant(ACPH1TC_A::ACPH1TC_6)
    }
    #[doc = "Phase1 active time in one sampling period equals to 0"]
    #[inline(always)]
    pub fn acph1tc_7(self) -> &'a mut W {
        self.variant(ACPH1TC_A::ACPH1TC_7)
    }
}
#[doc = "Field `ACSAT` reader - Analog Comparator Sampling Time control."]
pub type ACSAT_R = crate::FieldReader<u8, ACSAT_A>;
#[doc = "Analog Comparator Sampling Time control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACSAT_A {
    #[doc = "0: The sampling time equals to T"]
    ACSAT_0 = 0,
    #[doc = "1: The sampling time equasl to 2*T"]
    ACSAT_1 = 1,
    #[doc = "2: The sampling time equasl to 4*T"]
    ACSAT_2 = 2,
    #[doc = "3: The sampling time equasl to 8*T"]
    ACSAT_3 = 3,
    #[doc = "4: The sampling time equasl to 16*T"]
    ACSAT_4 = 4,
    #[doc = "5: The sampling time equasl to 32*T"]
    ACSAT_5 = 5,
    #[doc = "6: The sampling time equasl to 64*T"]
    ACSAT_6 = 6,
    #[doc = "7: The sampling time equasl to 256*T"]
    ACSAT_7 = 7,
}
impl From<ACSAT_A> for u8 {
    #[inline(always)]
    fn from(variant: ACSAT_A) -> Self {
        variant as _
    }
}
impl ACSAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACSAT_A {
        match self.bits {
            0 => ACSAT_A::ACSAT_0,
            1 => ACSAT_A::ACSAT_1,
            2 => ACSAT_A::ACSAT_2,
            3 => ACSAT_A::ACSAT_3,
            4 => ACSAT_A::ACSAT_4,
            5 => ACSAT_A::ACSAT_5,
            6 => ACSAT_A::ACSAT_6,
            7 => ACSAT_A::ACSAT_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ACSAT_0`"]
    #[inline(always)]
    pub fn is_acsat_0(&self) -> bool {
        *self == ACSAT_A::ACSAT_0
    }
    #[doc = "Checks if the value of the field is `ACSAT_1`"]
    #[inline(always)]
    pub fn is_acsat_1(&self) -> bool {
        *self == ACSAT_A::ACSAT_1
    }
    #[doc = "Checks if the value of the field is `ACSAT_2`"]
    #[inline(always)]
    pub fn is_acsat_2(&self) -> bool {
        *self == ACSAT_A::ACSAT_2
    }
    #[doc = "Checks if the value of the field is `ACSAT_3`"]
    #[inline(always)]
    pub fn is_acsat_3(&self) -> bool {
        *self == ACSAT_A::ACSAT_3
    }
    #[doc = "Checks if the value of the field is `ACSAT_4`"]
    #[inline(always)]
    pub fn is_acsat_4(&self) -> bool {
        *self == ACSAT_A::ACSAT_4
    }
    #[doc = "Checks if the value of the field is `ACSAT_5`"]
    #[inline(always)]
    pub fn is_acsat_5(&self) -> bool {
        *self == ACSAT_A::ACSAT_5
    }
    #[doc = "Checks if the value of the field is `ACSAT_6`"]
    #[inline(always)]
    pub fn is_acsat_6(&self) -> bool {
        *self == ACSAT_A::ACSAT_6
    }
    #[doc = "Checks if the value of the field is `ACSAT_7`"]
    #[inline(always)]
    pub fn is_acsat_7(&self) -> bool {
        *self == ACSAT_A::ACSAT_7
    }
}
#[doc = "Field `ACSAT` writer - Analog Comparator Sampling Time control."]
pub type ACSAT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, C3_SPEC, u8, ACSAT_A, 3, O>;
impl<'a, const O: u8> ACSAT_W<'a, O> {
    #[doc = "The sampling time equals to T"]
    #[inline(always)]
    pub fn acsat_0(self) -> &'a mut W {
        self.variant(ACSAT_A::ACSAT_0)
    }
    #[doc = "The sampling time equasl to 2*T"]
    #[inline(always)]
    pub fn acsat_1(self) -> &'a mut W {
        self.variant(ACSAT_A::ACSAT_1)
    }
    #[doc = "The sampling time equasl to 4*T"]
    #[inline(always)]
    pub fn acsat_2(self) -> &'a mut W {
        self.variant(ACSAT_A::ACSAT_2)
    }
    #[doc = "The sampling time equasl to 8*T"]
    #[inline(always)]
    pub fn acsat_3(self) -> &'a mut W {
        self.variant(ACSAT_A::ACSAT_3)
    }
    #[doc = "The sampling time equasl to 16*T"]
    #[inline(always)]
    pub fn acsat_4(self) -> &'a mut W {
        self.variant(ACSAT_A::ACSAT_4)
    }
    #[doc = "The sampling time equasl to 32*T"]
    #[inline(always)]
    pub fn acsat_5(self) -> &'a mut W {
        self.variant(ACSAT_A::ACSAT_5)
    }
    #[doc = "The sampling time equasl to 64*T"]
    #[inline(always)]
    pub fn acsat_6(self) -> &'a mut W {
        self.variant(ACSAT_A::ACSAT_6)
    }
    #[doc = "The sampling time equasl to 256*T"]
    #[inline(always)]
    pub fn acsat_7(self) -> &'a mut W {
        self.variant(ACSAT_A::ACSAT_7)
    }
}
#[doc = "Field `DMCS` reader - Discrete Mode Clock Selection"]
pub type DMCS_R = crate::BitReader<DMCS_A>;
#[doc = "Discrete Mode Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMCS_A {
    #[doc = "0: Slow clock is selected for the timing generation."]
    DMCS_0 = 0,
    #[doc = "1: Fast clock is selected for the timing generation."]
    DMCS_1 = 1,
}
impl From<DMCS_A> for bool {
    #[inline(always)]
    fn from(variant: DMCS_A) -> Self {
        variant as u8 != 0
    }
}
impl DMCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMCS_A {
        match self.bits {
            false => DMCS_A::DMCS_0,
            true => DMCS_A::DMCS_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMCS_0`"]
    #[inline(always)]
    pub fn is_dmcs_0(&self) -> bool {
        *self == DMCS_A::DMCS_0
    }
    #[doc = "Checks if the value of the field is `DMCS_1`"]
    #[inline(always)]
    pub fn is_dmcs_1(&self) -> bool {
        *self == DMCS_A::DMCS_1
    }
}
#[doc = "Field `DMCS` writer - Discrete Mode Clock Selection"]
pub type DMCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, C3_SPEC, DMCS_A, O>;
impl<'a, const O: u8> DMCS_W<'a, O> {
    #[doc = "Slow clock is selected for the timing generation."]
    #[inline(always)]
    pub fn dmcs_0(self) -> &'a mut W {
        self.variant(DMCS_A::DMCS_0)
    }
    #[doc = "Fast clock is selected for the timing generation."]
    #[inline(always)]
    pub fn dmcs_1(self) -> &'a mut W {
        self.variant(DMCS_A::DMCS_1)
    }
}
#[doc = "Field `RDIVE` reader - Resistor Divider Enable"]
pub type RDIVE_R = crate::BitReader<RDIVE_A>;
#[doc = "Resistor Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDIVE_A {
    #[doc = "0: The resistor is not enabled even when either NCHEN or PCHEN is set to1 but the actual input is in the range of 0 - 1.8v."]
    RDIVE_0 = 0,
    #[doc = "1: The resistor is enabled because the inputs are above 1.8v."]
    RDIVE_1 = 1,
}
impl From<RDIVE_A> for bool {
    #[inline(always)]
    fn from(variant: RDIVE_A) -> Self {
        variant as u8 != 0
    }
}
impl RDIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDIVE_A {
        match self.bits {
            false => RDIVE_A::RDIVE_0,
            true => RDIVE_A::RDIVE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RDIVE_0`"]
    #[inline(always)]
    pub fn is_rdive_0(&self) -> bool {
        *self == RDIVE_A::RDIVE_0
    }
    #[doc = "Checks if the value of the field is `RDIVE_1`"]
    #[inline(always)]
    pub fn is_rdive_1(&self) -> bool {
        *self == RDIVE_A::RDIVE_1
    }
}
#[doc = "Field `RDIVE` writer - Resistor Divider Enable"]
pub type RDIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C3_SPEC, RDIVE_A, O>;
impl<'a, const O: u8> RDIVE_W<'a, O> {
    #[doc = "The resistor is not enabled even when either NCHEN or PCHEN is set to1 but the actual input is in the range of 0 - 1.8v."]
    #[inline(always)]
    pub fn rdive_0(self) -> &'a mut W {
        self.variant(RDIVE_A::RDIVE_0)
    }
    #[doc = "The resistor is enabled because the inputs are above 1.8v."]
    #[inline(always)]
    pub fn rdive_1(self) -> &'a mut W {
        self.variant(RDIVE_A::RDIVE_1)
    }
}
#[doc = "Field `NCHCTEN` reader - Negative Channel Continuous Mode Enable."]
pub type NCHCTEN_R = crate::BitReader<NCHCTEN_A>;
#[doc = "Negative Channel Continuous Mode Enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCHCTEN_A {
    #[doc = "0: Negative channel is in Discrete Mode and special timing needs to be configured."]
    NCHCTEN_0 = 0,
    #[doc = "1: Negative channel is in Continuous Mode and no special timing is requried."]
    NCHCTEN_1 = 1,
}
impl From<NCHCTEN_A> for bool {
    #[inline(always)]
    fn from(variant: NCHCTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl NCHCTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCHCTEN_A {
        match self.bits {
            false => NCHCTEN_A::NCHCTEN_0,
            true => NCHCTEN_A::NCHCTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `NCHCTEN_0`"]
    #[inline(always)]
    pub fn is_nchcten_0(&self) -> bool {
        *self == NCHCTEN_A::NCHCTEN_0
    }
    #[doc = "Checks if the value of the field is `NCHCTEN_1`"]
    #[inline(always)]
    pub fn is_nchcten_1(&self) -> bool {
        *self == NCHCTEN_A::NCHCTEN_1
    }
}
#[doc = "Field `NCHCTEN` writer - Negative Channel Continuous Mode Enable."]
pub type NCHCTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C3_SPEC, NCHCTEN_A, O>;
impl<'a, const O: u8> NCHCTEN_W<'a, O> {
    #[doc = "Negative channel is in Discrete Mode and special timing needs to be configured."]
    #[inline(always)]
    pub fn nchcten_0(self) -> &'a mut W {
        self.variant(NCHCTEN_A::NCHCTEN_0)
    }
    #[doc = "Negative channel is in Continuous Mode and no special timing is requried."]
    #[inline(always)]
    pub fn nchcten_1(self) -> &'a mut W {
        self.variant(NCHCTEN_A::NCHCTEN_1)
    }
}
#[doc = "Field `PCHCTEN` reader - Positive Channel Continuous Mode Enable."]
pub type PCHCTEN_R = crate::BitReader<PCHCTEN_A>;
#[doc = "Positive Channel Continuous Mode Enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCHCTEN_A {
    #[doc = "0: Positive channel is in Discrete Mode and special timing needs to be configured."]
    PCHCTEN_0 = 0,
    #[doc = "1: Positive channel is in Continuous Mode and no special timing is requried."]
    PCHCTEN_1 = 1,
}
impl From<PCHCTEN_A> for bool {
    #[inline(always)]
    fn from(variant: PCHCTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PCHCTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCHCTEN_A {
        match self.bits {
            false => PCHCTEN_A::PCHCTEN_0,
            true => PCHCTEN_A::PCHCTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `PCHCTEN_0`"]
    #[inline(always)]
    pub fn is_pchcten_0(&self) -> bool {
        *self == PCHCTEN_A::PCHCTEN_0
    }
    #[doc = "Checks if the value of the field is `PCHCTEN_1`"]
    #[inline(always)]
    pub fn is_pchcten_1(&self) -> bool {
        *self == PCHCTEN_A::PCHCTEN_1
    }
}
#[doc = "Field `PCHCTEN` writer - Positive Channel Continuous Mode Enable."]
pub type PCHCTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C3_SPEC, PCHCTEN_A, O>;
impl<'a, const O: u8> PCHCTEN_W<'a, O> {
    #[doc = "Positive channel is in Discrete Mode and special timing needs to be configured."]
    #[inline(always)]
    pub fn pchcten_0(self) -> &'a mut W {
        self.variant(PCHCTEN_A::PCHCTEN_0)
    }
    #[doc = "Positive channel is in Continuous Mode and no special timing is requried."]
    #[inline(always)]
    pub fn pchcten_1(self) -> &'a mut W {
        self.variant(PCHCTEN_A::PCHCTEN_1)
    }
}
impl R {
    #[doc = "Bits 4:6 - Analog Comparator Phase2 Timing Control."]
    #[inline(always)]
    pub fn acph2tc(&self) -> ACPH2TC_R {
        ACPH2TC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Analog Comparator Phase1 Timing Control."]
    #[inline(always)]
    pub fn acph1tc(&self) -> ACPH1TC_R {
        ACPH1TC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Analog Comparator Sampling Time control."]
    #[inline(always)]
    pub fn acsat(&self) -> ACSAT_R {
        ACSAT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - Discrete Mode Clock Selection"]
    #[inline(always)]
    pub fn dmcs(&self) -> DMCS_R {
        DMCS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Resistor Divider Enable"]
    #[inline(always)]
    pub fn rdive(&self) -> RDIVE_R {
        RDIVE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Negative Channel Continuous Mode Enable."]
    #[inline(always)]
    pub fn nchcten(&self) -> NCHCTEN_R {
        NCHCTEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Positive Channel Continuous Mode Enable."]
    #[inline(always)]
    pub fn pchcten(&self) -> PCHCTEN_R {
        PCHCTEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - Analog Comparator Phase2 Timing Control."]
    #[inline(always)]
    #[must_use]
    pub fn acph2tc(&mut self) -> ACPH2TC_W<4> {
        ACPH2TC_W::new(self)
    }
    #[doc = "Bits 8:10 - Analog Comparator Phase1 Timing Control."]
    #[inline(always)]
    #[must_use]
    pub fn acph1tc(&mut self) -> ACPH1TC_W<8> {
        ACPH1TC_W::new(self)
    }
    #[doc = "Bits 12:14 - Analog Comparator Sampling Time control."]
    #[inline(always)]
    #[must_use]
    pub fn acsat(&mut self) -> ACSAT_W<12> {
        ACSAT_W::new(self)
    }
    #[doc = "Bit 16 - Discrete Mode Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn dmcs(&mut self) -> DMCS_W<16> {
        DMCS_W::new(self)
    }
    #[doc = "Bit 20 - Resistor Divider Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdive(&mut self) -> RDIVE_W<20> {
        RDIVE_W::new(self)
    }
    #[doc = "Bit 24 - Negative Channel Continuous Mode Enable."]
    #[inline(always)]
    #[must_use]
    pub fn nchcten(&mut self) -> NCHCTEN_W<24> {
        NCHCTEN_W::new(self)
    }
    #[doc = "Bit 28 - Positive Channel Continuous Mode Enable."]
    #[inline(always)]
    #[must_use]
    pub fn pchcten(&mut self) -> PCHCTEN_W<28> {
        PCHCTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMP Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c3](index.html) module"]
pub struct C3_SPEC;
impl crate::RegisterSpec for C3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c3::R](R) reader structure"]
impl crate::Readable for C3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c3::W](W) writer structure"]
impl crate::Writable for C3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C3 to value 0x1100_0000"]
impl crate::Resettable for C3_SPEC {
    const RESET_VALUE: Self::Ux = 0x1100_0000;
}
