#[doc = "Register `C1` reader"]
pub struct R(crate::R<C1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1` writer"]
pub struct W(crate::W<C1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_SPEC>;
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
impl From<crate::W<C1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VOSEL` reader - DAC Output Voltage Select"]
pub type VOSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VOSEL` writer - DAC Output Voltage Select"]
pub type VOSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C1_SPEC, u8, u8, 8, O>;
#[doc = "Field `DMODE` reader - DAC Mode Selection"]
pub type DMODE_R = crate::BitReader<DMODE_A>;
#[doc = "DAC Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMODE_A {
    #[doc = "0: DAC is selected to work in low speed and low power mode."]
    DMODE_0 = 0,
    #[doc = "1: DAC is selected to work in high speed high power mode."]
    DMODE_1 = 1,
}
impl From<DMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl DMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMODE_A {
        match self.bits {
            false => DMODE_A::DMODE_0,
            true => DMODE_A::DMODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMODE_0`"]
    #[inline(always)]
    pub fn is_dmode_0(&self) -> bool {
        *self == DMODE_A::DMODE_0
    }
    #[doc = "Checks if the value of the field is `DMODE_1`"]
    #[inline(always)]
    pub fn is_dmode_1(&self) -> bool {
        *self == DMODE_A::DMODE_1
    }
}
#[doc = "Field `DMODE` writer - DAC Mode Selection"]
pub type DMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1_SPEC, DMODE_A, O>;
impl<'a, const O: u8> DMODE_W<'a, O> {
    #[doc = "DAC is selected to work in low speed and low power mode."]
    #[inline(always)]
    pub fn dmode_0(self) -> &'a mut W {
        self.variant(DMODE_A::DMODE_0)
    }
    #[doc = "DAC is selected to work in high speed high power mode."]
    #[inline(always)]
    pub fn dmode_1(self) -> &'a mut W {
        self.variant(DMODE_A::DMODE_1)
    }
}
#[doc = "Field `VRSEL` reader - Supply Voltage Reference Source Select"]
pub type VRSEL_R = crate::BitReader<VRSEL_A>;
#[doc = "Supply Voltage Reference Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VRSEL_A {
    #[doc = "0: Vin1 is selected as resistor ladder network supply reference Vin. Vin1 is from internal PMC."]
    VRSEL_0 = 0,
    #[doc = "1: Vin2 is selected as resistor ladder network supply reference Vin. Vin2 is from PAD."]
    VRSEL_1 = 1,
}
impl From<VRSEL_A> for bool {
    #[inline(always)]
    fn from(variant: VRSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl VRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VRSEL_A {
        match self.bits {
            false => VRSEL_A::VRSEL_0,
            true => VRSEL_A::VRSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `VRSEL_0`"]
    #[inline(always)]
    pub fn is_vrsel_0(&self) -> bool {
        *self == VRSEL_A::VRSEL_0
    }
    #[doc = "Checks if the value of the field is `VRSEL_1`"]
    #[inline(always)]
    pub fn is_vrsel_1(&self) -> bool {
        *self == VRSEL_A::VRSEL_1
    }
}
#[doc = "Field `VRSEL` writer - Supply Voltage Reference Source Select"]
pub type VRSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1_SPEC, VRSEL_A, O>;
impl<'a, const O: u8> VRSEL_W<'a, O> {
    #[doc = "Vin1 is selected as resistor ladder network supply reference Vin. Vin1 is from internal PMC."]
    #[inline(always)]
    pub fn vrsel_0(self) -> &'a mut W {
        self.variant(VRSEL_A::VRSEL_0)
    }
    #[doc = "Vin2 is selected as resistor ladder network supply reference Vin. Vin2 is from PAD."]
    #[inline(always)]
    pub fn vrsel_1(self) -> &'a mut W {
        self.variant(VRSEL_A::VRSEL_1)
    }
}
#[doc = "Field `DACEN` reader - DAC Enable"]
pub type DACEN_R = crate::BitReader<DACEN_A>;
#[doc = "DAC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACEN_A {
    #[doc = "0: DAC is disabled."]
    DACEN_0 = 0,
    #[doc = "1: DAC is enabled."]
    DACEN_1 = 1,
}
impl From<DACEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DACEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACEN_A {
        match self.bits {
            false => DACEN_A::DACEN_0,
            true => DACEN_A::DACEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DACEN_0`"]
    #[inline(always)]
    pub fn is_dacen_0(&self) -> bool {
        *self == DACEN_A::DACEN_0
    }
    #[doc = "Checks if the value of the field is `DACEN_1`"]
    #[inline(always)]
    pub fn is_dacen_1(&self) -> bool {
        *self == DACEN_A::DACEN_1
    }
}
#[doc = "Field `DACEN` writer - DAC Enable"]
pub type DACEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1_SPEC, DACEN_A, O>;
impl<'a, const O: u8> DACEN_W<'a, O> {
    #[doc = "DAC is disabled."]
    #[inline(always)]
    pub fn dacen_0(self) -> &'a mut W {
        self.variant(DACEN_A::DACEN_0)
    }
    #[doc = "DAC is enabled."]
    #[inline(always)]
    pub fn dacen_1(self) -> &'a mut W {
        self.variant(DACEN_A::DACEN_1)
    }
}
#[doc = "Field `PSEL_SEC` reader - Secondary Plus channel select."]
pub type PSEL_SEC_R = crate::FieldReader<u8, PSEL_SEC_A>;
#[doc = "Secondary Plus channel select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSEL_SEC_A {
    #[doc = "0: Input 0 for Plus Channel"]
    INPUT_0 = 0,
    #[doc = "1: Input 1 for Plus Channel"]
    INPUT_1 = 1,
    #[doc = "2: Input 2 for Plus Channel"]
    INPUT_2 = 2,
    #[doc = "3: Input 3 for Plus Channel"]
    INPUT_3 = 3,
    #[doc = "4: Input 4 for Plus Channel"]
    INPUT_4 = 4,
    #[doc = "5: Input 5 for Plus Channel"]
    INPUT_5 = 5,
    #[doc = "6: Input 6 for Plus Channel"]
    INPUT_6 = 6,
    #[doc = "7: Internal 8b DAC output for Plus Channel"]
    INTERNAL = 7,
}
impl From<PSEL_SEC_A> for u8 {
    #[inline(always)]
    fn from(variant: PSEL_SEC_A) -> Self {
        variant as _
    }
}
impl PSEL_SEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSEL_SEC_A {
        match self.bits {
            0 => PSEL_SEC_A::INPUT_0,
            1 => PSEL_SEC_A::INPUT_1,
            2 => PSEL_SEC_A::INPUT_2,
            3 => PSEL_SEC_A::INPUT_3,
            4 => PSEL_SEC_A::INPUT_4,
            5 => PSEL_SEC_A::INPUT_5,
            6 => PSEL_SEC_A::INPUT_6,
            7 => PSEL_SEC_A::INTERNAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT_0`"]
    #[inline(always)]
    pub fn is_input_0(&self) -> bool {
        *self == PSEL_SEC_A::INPUT_0
    }
    #[doc = "Checks if the value of the field is `INPUT_1`"]
    #[inline(always)]
    pub fn is_input_1(&self) -> bool {
        *self == PSEL_SEC_A::INPUT_1
    }
    #[doc = "Checks if the value of the field is `INPUT_2`"]
    #[inline(always)]
    pub fn is_input_2(&self) -> bool {
        *self == PSEL_SEC_A::INPUT_2
    }
    #[doc = "Checks if the value of the field is `INPUT_3`"]
    #[inline(always)]
    pub fn is_input_3(&self) -> bool {
        *self == PSEL_SEC_A::INPUT_3
    }
    #[doc = "Checks if the value of the field is `INPUT_4`"]
    #[inline(always)]
    pub fn is_input_4(&self) -> bool {
        *self == PSEL_SEC_A::INPUT_4
    }
    #[doc = "Checks if the value of the field is `INPUT_5`"]
    #[inline(always)]
    pub fn is_input_5(&self) -> bool {
        *self == PSEL_SEC_A::INPUT_5
    }
    #[doc = "Checks if the value of the field is `INPUT_6`"]
    #[inline(always)]
    pub fn is_input_6(&self) -> bool {
        *self == PSEL_SEC_A::INPUT_6
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == PSEL_SEC_A::INTERNAL
    }
}
#[doc = "Field `PSEL_SEC` writer - Secondary Plus channel select."]
pub type PSEL_SEC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, C1_SPEC, u8, PSEL_SEC_A, 3, O>;
impl<'a, const O: u8> PSEL_SEC_W<'a, O> {
    #[doc = "Input 0 for Plus Channel"]
    #[inline(always)]
    pub fn input_0(self) -> &'a mut W {
        self.variant(PSEL_SEC_A::INPUT_0)
    }
    #[doc = "Input 1 for Plus Channel"]
    #[inline(always)]
    pub fn input_1(self) -> &'a mut W {
        self.variant(PSEL_SEC_A::INPUT_1)
    }
    #[doc = "Input 2 for Plus Channel"]
    #[inline(always)]
    pub fn input_2(self) -> &'a mut W {
        self.variant(PSEL_SEC_A::INPUT_2)
    }
    #[doc = "Input 3 for Plus Channel"]
    #[inline(always)]
    pub fn input_3(self) -> &'a mut W {
        self.variant(PSEL_SEC_A::INPUT_3)
    }
    #[doc = "Input 4 for Plus Channel"]
    #[inline(always)]
    pub fn input_4(self) -> &'a mut W {
        self.variant(PSEL_SEC_A::INPUT_4)
    }
    #[doc = "Input 5 for Plus Channel"]
    #[inline(always)]
    pub fn input_5(self) -> &'a mut W {
        self.variant(PSEL_SEC_A::INPUT_5)
    }
    #[doc = "Input 6 for Plus Channel"]
    #[inline(always)]
    pub fn input_6(self) -> &'a mut W {
        self.variant(PSEL_SEC_A::INPUT_6)
    }
    #[doc = "Internal 8b DAC output for Plus Channel"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(PSEL_SEC_A::INTERNAL)
    }
}
#[doc = "Field `CHN0` reader - Channel 0 input enable"]
pub type CHN0_R = crate::BitReader<bool>;
#[doc = "Field `CHN0` writer - Channel 0 input enable"]
pub type CHN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1_SPEC, bool, O>;
#[doc = "Field `CHN1` reader - Channel 1 input enable"]
pub type CHN1_R = crate::BitReader<bool>;
#[doc = "Field `CHN1` writer - Channel 1 input enable"]
pub type CHN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1_SPEC, bool, O>;
#[doc = "Field `CHN2` reader - Channel 2 input enable"]
pub type CHN2_R = crate::BitReader<bool>;
#[doc = "Field `CHN2` writer - Channel 2 input enable"]
pub type CHN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1_SPEC, bool, O>;
#[doc = "Field `CHN3` reader - Channel 3 input enable"]
pub type CHN3_R = crate::BitReader<bool>;
#[doc = "Field `CHN3` writer - Channel 3 input enable"]
pub type CHN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1_SPEC, bool, O>;
#[doc = "Field `CHN4` reader - Channel 4 input enable"]
pub type CHN4_R = crate::BitReader<bool>;
#[doc = "Field `CHN4` writer - Channel 4 input enable"]
pub type CHN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1_SPEC, bool, O>;
#[doc = "Field `CHN5` reader - Channel 5 input enable"]
pub type CHN5_R = crate::BitReader<bool>;
#[doc = "Field `CHN5` writer - Channel 5 input enable"]
pub type CHN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1_SPEC, bool, O>;
#[doc = "Field `MSEL` reader - Minus Input MUX Control"]
pub type MSEL_R = crate::FieldReader<u8, MSEL_A>;
#[doc = "Minus Input MUX Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSEL_A {
    #[doc = "0: Internal Negative Input 0 for Minus Channel -- Internal Minus Input"]
    MSEL_0 = 0,
    #[doc = "1: External Input 1 for Minus Channel -- Reference Input 0"]
    MSEL_1 = 1,
    #[doc = "2: External Input 2 for Minus Channel -- Reference Input 1"]
    MSEL_2 = 2,
    #[doc = "3: External Input 3 for Minus Channel -- Reference Input 2"]
    MSEL_3 = 3,
    #[doc = "4: External Input 4 for Minus Channel -- Reference Input 3"]
    MSEL_4 = 4,
    #[doc = "5: External Input 5 for Minus Channel -- Reference Input 4"]
    MSEL_5 = 5,
    #[doc = "6: External Input 6 for Minus Channel -- Reference Input 5"]
    MSEL_6 = 6,
    #[doc = "7: Internal 8b DAC output"]
    MSEL_7 = 7,
}
impl From<MSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MSEL_A) -> Self {
        variant as _
    }
}
impl MSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSEL_A {
        match self.bits {
            0 => MSEL_A::MSEL_0,
            1 => MSEL_A::MSEL_1,
            2 => MSEL_A::MSEL_2,
            3 => MSEL_A::MSEL_3,
            4 => MSEL_A::MSEL_4,
            5 => MSEL_A::MSEL_5,
            6 => MSEL_A::MSEL_6,
            7 => MSEL_A::MSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MSEL_0`"]
    #[inline(always)]
    pub fn is_msel_0(&self) -> bool {
        *self == MSEL_A::MSEL_0
    }
    #[doc = "Checks if the value of the field is `MSEL_1`"]
    #[inline(always)]
    pub fn is_msel_1(&self) -> bool {
        *self == MSEL_A::MSEL_1
    }
    #[doc = "Checks if the value of the field is `MSEL_2`"]
    #[inline(always)]
    pub fn is_msel_2(&self) -> bool {
        *self == MSEL_A::MSEL_2
    }
    #[doc = "Checks if the value of the field is `MSEL_3`"]
    #[inline(always)]
    pub fn is_msel_3(&self) -> bool {
        *self == MSEL_A::MSEL_3
    }
    #[doc = "Checks if the value of the field is `MSEL_4`"]
    #[inline(always)]
    pub fn is_msel_4(&self) -> bool {
        *self == MSEL_A::MSEL_4
    }
    #[doc = "Checks if the value of the field is `MSEL_5`"]
    #[inline(always)]
    pub fn is_msel_5(&self) -> bool {
        *self == MSEL_A::MSEL_5
    }
    #[doc = "Checks if the value of the field is `MSEL_6`"]
    #[inline(always)]
    pub fn is_msel_6(&self) -> bool {
        *self == MSEL_A::MSEL_6
    }
    #[doc = "Checks if the value of the field is `MSEL_7`"]
    #[inline(always)]
    pub fn is_msel_7(&self) -> bool {
        *self == MSEL_A::MSEL_7
    }
}
#[doc = "Field `MSEL` writer - Minus Input MUX Control"]
pub type MSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, C1_SPEC, u8, MSEL_A, 3, O>;
impl<'a, const O: u8> MSEL_W<'a, O> {
    #[doc = "Internal Negative Input 0 for Minus Channel -- Internal Minus Input"]
    #[inline(always)]
    pub fn msel_0(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_0)
    }
    #[doc = "External Input 1 for Minus Channel -- Reference Input 0"]
    #[inline(always)]
    pub fn msel_1(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_1)
    }
    #[doc = "External Input 2 for Minus Channel -- Reference Input 1"]
    #[inline(always)]
    pub fn msel_2(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_2)
    }
    #[doc = "External Input 3 for Minus Channel -- Reference Input 2"]
    #[inline(always)]
    pub fn msel_3(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_3)
    }
    #[doc = "External Input 4 for Minus Channel -- Reference Input 3"]
    #[inline(always)]
    pub fn msel_4(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_4)
    }
    #[doc = "External Input 5 for Minus Channel -- Reference Input 4"]
    #[inline(always)]
    pub fn msel_5(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_5)
    }
    #[doc = "External Input 6 for Minus Channel -- Reference Input 5"]
    #[inline(always)]
    pub fn msel_6(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_6)
    }
    #[doc = "Internal 8b DAC output"]
    #[inline(always)]
    pub fn msel_7(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_7)
    }
}
#[doc = "Field `PSEL` reader - Plus Input MUX Control"]
pub type PSEL_R = crate::FieldReader<u8, PSEL_A>;
#[doc = "Plus Input MUX Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSEL_A {
    #[doc = "0: Internal Posivite Input 0 for Plus Channel -- Internal Minus Input"]
    PSEL_0 = 0,
    #[doc = "1: External Input 1 for Plus Channel -- Reference Input 0"]
    PSEL_1 = 1,
    #[doc = "2: External Input 2 for Plus Channel -- Reference Input 1"]
    PSEL_2 = 2,
    #[doc = "3: External Input 3 for Plus Channel -- Reference Input 2"]
    PSEL_3 = 3,
    #[doc = "4: External Input 4 for Plus Channel -- Reference Input 3"]
    PSEL_4 = 4,
    #[doc = "5: External Input 4 for Plus Channel -- Reference Input 4"]
    PSEL_5 = 5,
    #[doc = "6: External Input 4 for Plus Channel -- Reference Input 5"]
    PSEL_6 = 6,
    #[doc = "7: Internal 8b DAC output"]
    PSEL_7 = 7,
}
impl From<PSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PSEL_A) -> Self {
        variant as _
    }
}
impl PSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSEL_A {
        match self.bits {
            0 => PSEL_A::PSEL_0,
            1 => PSEL_A::PSEL_1,
            2 => PSEL_A::PSEL_2,
            3 => PSEL_A::PSEL_3,
            4 => PSEL_A::PSEL_4,
            5 => PSEL_A::PSEL_5,
            6 => PSEL_A::PSEL_6,
            7 => PSEL_A::PSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PSEL_0`"]
    #[inline(always)]
    pub fn is_psel_0(&self) -> bool {
        *self == PSEL_A::PSEL_0
    }
    #[doc = "Checks if the value of the field is `PSEL_1`"]
    #[inline(always)]
    pub fn is_psel_1(&self) -> bool {
        *self == PSEL_A::PSEL_1
    }
    #[doc = "Checks if the value of the field is `PSEL_2`"]
    #[inline(always)]
    pub fn is_psel_2(&self) -> bool {
        *self == PSEL_A::PSEL_2
    }
    #[doc = "Checks if the value of the field is `PSEL_3`"]
    #[inline(always)]
    pub fn is_psel_3(&self) -> bool {
        *self == PSEL_A::PSEL_3
    }
    #[doc = "Checks if the value of the field is `PSEL_4`"]
    #[inline(always)]
    pub fn is_psel_4(&self) -> bool {
        *self == PSEL_A::PSEL_4
    }
    #[doc = "Checks if the value of the field is `PSEL_5`"]
    #[inline(always)]
    pub fn is_psel_5(&self) -> bool {
        *self == PSEL_A::PSEL_5
    }
    #[doc = "Checks if the value of the field is `PSEL_6`"]
    #[inline(always)]
    pub fn is_psel_6(&self) -> bool {
        *self == PSEL_A::PSEL_6
    }
    #[doc = "Checks if the value of the field is `PSEL_7`"]
    #[inline(always)]
    pub fn is_psel_7(&self) -> bool {
        *self == PSEL_A::PSEL_7
    }
}
#[doc = "Field `PSEL` writer - Plus Input MUX Control"]
pub type PSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, C1_SPEC, u8, PSEL_A, 3, O>;
impl<'a, const O: u8> PSEL_W<'a, O> {
    #[doc = "Internal Posivite Input 0 for Plus Channel -- Internal Minus Input"]
    #[inline(always)]
    pub fn psel_0(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_0)
    }
    #[doc = "External Input 1 for Plus Channel -- Reference Input 0"]
    #[inline(always)]
    pub fn psel_1(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_1)
    }
    #[doc = "External Input 2 for Plus Channel -- Reference Input 1"]
    #[inline(always)]
    pub fn psel_2(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_2)
    }
    #[doc = "External Input 3 for Plus Channel -- Reference Input 2"]
    #[inline(always)]
    pub fn psel_3(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_3)
    }
    #[doc = "External Input 4 for Plus Channel -- Reference Input 3"]
    #[inline(always)]
    pub fn psel_4(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_4)
    }
    #[doc = "External Input 4 for Plus Channel -- Reference Input 4"]
    #[inline(always)]
    pub fn psel_5(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_5)
    }
    #[doc = "External Input 4 for Plus Channel -- Reference Input 5"]
    #[inline(always)]
    pub fn psel_6(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_6)
    }
    #[doc = "Internal 8b DAC output"]
    #[inline(always)]
    pub fn psel_7(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_7)
    }
}
impl R {
    #[doc = "Bits 0:7 - DAC Output Voltage Select"]
    #[inline(always)]
    pub fn vosel(&self) -> VOSEL_R {
        VOSEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - DAC Mode Selection"]
    #[inline(always)]
    pub fn dmode(&self) -> DMODE_R {
        DMODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Supply Voltage Reference Source Select"]
    #[inline(always)]
    pub fn vrsel(&self) -> VRSEL_R {
        VRSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DAC Enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Secondary Plus channel select."]
    #[inline(always)]
    pub fn psel_sec(&self) -> PSEL_SEC_R {
        PSEL_SEC_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - Channel 0 input enable"]
    #[inline(always)]
    pub fn chn0(&self) -> CHN0_R {
        CHN0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 1 input enable"]
    #[inline(always)]
    pub fn chn1(&self) -> CHN1_R {
        CHN1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 2 input enable"]
    #[inline(always)]
    pub fn chn2(&self) -> CHN2_R {
        CHN2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 3 input enable"]
    #[inline(always)]
    pub fn chn3(&self) -> CHN3_R {
        CHN3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel 4 input enable"]
    #[inline(always)]
    pub fn chn4(&self) -> CHN4_R {
        CHN4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel 5 input enable"]
    #[inline(always)]
    pub fn chn5(&self) -> CHN5_R {
        CHN5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Minus Input MUX Control"]
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Plus Input MUX Control"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC Output Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn vosel(&mut self) -> VOSEL_W<0> {
        VOSEL_W::new(self)
    }
    #[doc = "Bit 8 - DAC Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn dmode(&mut self) -> DMODE_W<8> {
        DMODE_W::new(self)
    }
    #[doc = "Bit 9 - Supply Voltage Reference Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn vrsel(&mut self) -> VRSEL_W<9> {
        VRSEL_W::new(self)
    }
    #[doc = "Bit 10 - DAC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dacen(&mut self) -> DACEN_W<10> {
        DACEN_W::new(self)
    }
    #[doc = "Bits 12:14 - Secondary Plus channel select."]
    #[inline(always)]
    #[must_use]
    pub fn psel_sec(&mut self) -> PSEL_SEC_W<12> {
        PSEL_SEC_W::new(self)
    }
    #[doc = "Bit 16 - Channel 0 input enable"]
    #[inline(always)]
    #[must_use]
    pub fn chn0(&mut self) -> CHN0_W<16> {
        CHN0_W::new(self)
    }
    #[doc = "Bit 17 - Channel 1 input enable"]
    #[inline(always)]
    #[must_use]
    pub fn chn1(&mut self) -> CHN1_W<17> {
        CHN1_W::new(self)
    }
    #[doc = "Bit 18 - Channel 2 input enable"]
    #[inline(always)]
    #[must_use]
    pub fn chn2(&mut self) -> CHN2_W<18> {
        CHN2_W::new(self)
    }
    #[doc = "Bit 19 - Channel 3 input enable"]
    #[inline(always)]
    #[must_use]
    pub fn chn3(&mut self) -> CHN3_W<19> {
        CHN3_W::new(self)
    }
    #[doc = "Bit 20 - Channel 4 input enable"]
    #[inline(always)]
    #[must_use]
    pub fn chn4(&mut self) -> CHN4_W<20> {
        CHN4_W::new(self)
    }
    #[doc = "Bit 21 - Channel 5 input enable"]
    #[inline(always)]
    #[must_use]
    pub fn chn5(&mut self) -> CHN5_W<21> {
        CHN5_W::new(self)
    }
    #[doc = "Bits 24:26 - Minus Input MUX Control"]
    #[inline(always)]
    #[must_use]
    pub fn msel(&mut self) -> MSEL_W<24> {
        MSEL_W::new(self)
    }
    #[doc = "Bits 28:30 - Plus Input MUX Control"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PSEL_W<28> {
        PSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMP Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1](index.html) module"]
pub struct C1_SPEC;
impl crate::RegisterSpec for C1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1::R](R) reader structure"]
impl crate::Readable for C1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1::W](W) writer structure"]
impl crate::Writable for C1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1 to value 0"]
impl crate::Resettable for C1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
