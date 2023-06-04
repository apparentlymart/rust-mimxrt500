#[doc = "Register `CMDL11` reader"]
pub struct R(crate::R<CMDL11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDL11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDL11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDL11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDL11` writer"]
pub struct W(crate::W<CMDL11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDL11_SPEC>;
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
impl From<crate::W<CMDL11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDL11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCH` reader - Input Channel Select"]
pub type ADCH_R = crate::FieldReader<u8, ADCH_A>;
#[doc = "Input Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCH_A {
    #[doc = "0: CH0A or CH0B or CH0A/CH0B pair."]
    SELECT_CH0 = 0,
    #[doc = "1: CH1A or CH1B or CH1A/CH1B pair."]
    SELECT_CH1 = 1,
    #[doc = "2: CH2A or CH2B or CH2A/CH2B pair."]
    SELECT_CH2 = 2,
    #[doc = "3: CH3A or CH3B or CH3A/CH3B pair."]
    SELECT_CH3 = 3,
    #[doc = "4: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_4 = 4,
    #[doc = "5: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_5 = 5,
    #[doc = "6: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_6 = 6,
    #[doc = "7: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_7 = 7,
    #[doc = "8: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_8 = 8,
    #[doc = "9: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SELECT_CORRESPONDING_CHANNEL_9 = 9,
    #[doc = "30: CH30A or CH30B or CH30A/CH30B pair."]
    SELECT_CH30 = 30,
    #[doc = "31: CH31A or CH31B or CH31A/CH31B pair."]
    SELECT_CH31 = 31,
}
impl From<ADCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCH_A) -> Self {
        variant as _
    }
}
impl ADCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADCH_A> {
        match self.bits {
            0 => Some(ADCH_A::SELECT_CH0),
            1 => Some(ADCH_A::SELECT_CH1),
            2 => Some(ADCH_A::SELECT_CH2),
            3 => Some(ADCH_A::SELECT_CH3),
            4 => Some(ADCH_A::SELECT_CORRESPONDING_CHANNEL_4),
            5 => Some(ADCH_A::SELECT_CORRESPONDING_CHANNEL_5),
            6 => Some(ADCH_A::SELECT_CORRESPONDING_CHANNEL_6),
            7 => Some(ADCH_A::SELECT_CORRESPONDING_CHANNEL_7),
            8 => Some(ADCH_A::SELECT_CORRESPONDING_CHANNEL_8),
            9 => Some(ADCH_A::SELECT_CORRESPONDING_CHANNEL_9),
            30 => Some(ADCH_A::SELECT_CH30),
            31 => Some(ADCH_A::SELECT_CH31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SELECT_CH0`"]
    #[inline(always)]
    pub fn is_select_ch0(&self) -> bool {
        *self == ADCH_A::SELECT_CH0
    }
    #[doc = "Checks if the value of the field is `SELECT_CH1`"]
    #[inline(always)]
    pub fn is_select_ch1(&self) -> bool {
        *self == ADCH_A::SELECT_CH1
    }
    #[doc = "Checks if the value of the field is `SELECT_CH2`"]
    #[inline(always)]
    pub fn is_select_ch2(&self) -> bool {
        *self == ADCH_A::SELECT_CH2
    }
    #[doc = "Checks if the value of the field is `SELECT_CH3`"]
    #[inline(always)]
    pub fn is_select_ch3(&self) -> bool {
        *self == ADCH_A::SELECT_CH3
    }
    #[doc = "Checks if the value of the field is `SELECT_CORRESPONDING_CHANNEL_4`"]
    #[inline(always)]
    pub fn is_select_corresponding_channel_4(&self) -> bool {
        *self == ADCH_A::SELECT_CORRESPONDING_CHANNEL_4
    }
    #[doc = "Checks if the value of the field is `SELECT_CORRESPONDING_CHANNEL_5`"]
    #[inline(always)]
    pub fn is_select_corresponding_channel_5(&self) -> bool {
        *self == ADCH_A::SELECT_CORRESPONDING_CHANNEL_5
    }
    #[doc = "Checks if the value of the field is `SELECT_CORRESPONDING_CHANNEL_6`"]
    #[inline(always)]
    pub fn is_select_corresponding_channel_6(&self) -> bool {
        *self == ADCH_A::SELECT_CORRESPONDING_CHANNEL_6
    }
    #[doc = "Checks if the value of the field is `SELECT_CORRESPONDING_CHANNEL_7`"]
    #[inline(always)]
    pub fn is_select_corresponding_channel_7(&self) -> bool {
        *self == ADCH_A::SELECT_CORRESPONDING_CHANNEL_7
    }
    #[doc = "Checks if the value of the field is `SELECT_CORRESPONDING_CHANNEL_8`"]
    #[inline(always)]
    pub fn is_select_corresponding_channel_8(&self) -> bool {
        *self == ADCH_A::SELECT_CORRESPONDING_CHANNEL_8
    }
    #[doc = "Checks if the value of the field is `SELECT_CORRESPONDING_CHANNEL_9`"]
    #[inline(always)]
    pub fn is_select_corresponding_channel_9(&self) -> bool {
        *self == ADCH_A::SELECT_CORRESPONDING_CHANNEL_9
    }
    #[doc = "Checks if the value of the field is `SELECT_CH30`"]
    #[inline(always)]
    pub fn is_select_ch30(&self) -> bool {
        *self == ADCH_A::SELECT_CH30
    }
    #[doc = "Checks if the value of the field is `SELECT_CH31`"]
    #[inline(always)]
    pub fn is_select_ch31(&self) -> bool {
        *self == ADCH_A::SELECT_CH31
    }
}
#[doc = "Field `ADCH` writer - Input Channel Select"]
pub type ADCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDL11_SPEC, u8, ADCH_A, 5, O>;
impl<'a, const O: u8> ADCH_W<'a, O> {
    #[doc = "CH0A or CH0B or CH0A/CH0B pair."]
    #[inline(always)]
    pub fn select_ch0(self) -> &'a mut W {
        self.variant(ADCH_A::SELECT_CH0)
    }
    #[doc = "CH1A or CH1B or CH1A/CH1B pair."]
    #[inline(always)]
    pub fn select_ch1(self) -> &'a mut W {
        self.variant(ADCH_A::SELECT_CH1)
    }
    #[doc = "CH2A or CH2B or CH2A/CH2B pair."]
    #[inline(always)]
    pub fn select_ch2(self) -> &'a mut W {
        self.variant(ADCH_A::SELECT_CH2)
    }
    #[doc = "CH3A or CH3B or CH3A/CH3B pair."]
    #[inline(always)]
    pub fn select_ch3(self) -> &'a mut W {
        self.variant(ADCH_A::SELECT_CH3)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn select_corresponding_channel_4(self) -> &'a mut W {
        self.variant(ADCH_A::SELECT_CORRESPONDING_CHANNEL_4)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn select_corresponding_channel_5(self) -> &'a mut W {
        self.variant(ADCH_A::SELECT_CORRESPONDING_CHANNEL_5)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn select_corresponding_channel_6(self) -> &'a mut W {
        self.variant(ADCH_A::SELECT_CORRESPONDING_CHANNEL_6)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn select_corresponding_channel_7(self) -> &'a mut W {
        self.variant(ADCH_A::SELECT_CORRESPONDING_CHANNEL_7)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn select_corresponding_channel_8(self) -> &'a mut W {
        self.variant(ADCH_A::SELECT_CORRESPONDING_CHANNEL_8)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn select_corresponding_channel_9(self) -> &'a mut W {
        self.variant(ADCH_A::SELECT_CORRESPONDING_CHANNEL_9)
    }
    #[doc = "CH30A or CH30B or CH30A/CH30B pair."]
    #[inline(always)]
    pub fn select_ch30(self) -> &'a mut W {
        self.variant(ADCH_A::SELECT_CH30)
    }
    #[doc = "CH31A or CH31B or CH31A/CH31B pair."]
    #[inline(always)]
    pub fn select_ch31(self) -> &'a mut W {
        self.variant(ADCH_A::SELECT_CH31)
    }
}
#[doc = "Field `ABSEL` reader - A-side or B-side Select"]
pub type ABSEL_R = crate::BitReader<ABSEL_A>;
#[doc = "A-side or B-side Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABSEL_A {
    #[doc = "0: When CMDLn\\[DIFF\\]
= 0b, the associated A-side channel is converted as single-ended. When CMDLn\\[DIFF\\]
= 1b, the ADC result is (CHnA - CHnB)."]
    SELECT_A_SIDE_CHANNEL = 0,
    #[doc = "1: When CMDLn\\[DIFF\\]
= 0b, the associated B-side channel is converted as single-ended. When CMDLn\\[DIFF\\]
= 1b, the ADC result is (CHnB - CHnA)."]
    SELECT_B_SIDE_CHANNEL = 1,
}
impl From<ABSEL_A> for bool {
    #[inline(always)]
    fn from(variant: ABSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl ABSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABSEL_A {
        match self.bits {
            false => ABSEL_A::SELECT_A_SIDE_CHANNEL,
            true => ABSEL_A::SELECT_B_SIDE_CHANNEL,
        }
    }
    #[doc = "Checks if the value of the field is `SELECT_A_SIDE_CHANNEL`"]
    #[inline(always)]
    pub fn is_select_a_side_channel(&self) -> bool {
        *self == ABSEL_A::SELECT_A_SIDE_CHANNEL
    }
    #[doc = "Checks if the value of the field is `SELECT_B_SIDE_CHANNEL`"]
    #[inline(always)]
    pub fn is_select_b_side_channel(&self) -> bool {
        *self == ABSEL_A::SELECT_B_SIDE_CHANNEL
    }
}
#[doc = "Field `ABSEL` writer - A-side or B-side Select"]
pub type ABSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDL11_SPEC, ABSEL_A, O>;
impl<'a, const O: u8> ABSEL_W<'a, O> {
    #[doc = "When CMDLn\\[DIFF\\]
= 0b, the associated A-side channel is converted as single-ended. When CMDLn\\[DIFF\\]
= 1b, the ADC result is (CHnA - CHnB)."]
    #[inline(always)]
    pub fn select_a_side_channel(self) -> &'a mut W {
        self.variant(ABSEL_A::SELECT_A_SIDE_CHANNEL)
    }
    #[doc = "When CMDLn\\[DIFF\\]
= 0b, the associated B-side channel is converted as single-ended. When CMDLn\\[DIFF\\]
= 1b, the ADC result is (CHnB - CHnA)."]
    #[inline(always)]
    pub fn select_b_side_channel(self) -> &'a mut W {
        self.variant(ABSEL_A::SELECT_B_SIDE_CHANNEL)
    }
}
#[doc = "Field `DIFF` reader - Differential Mode Enable"]
pub type DIFF_R = crate::BitReader<DIFF_A>;
#[doc = "Differential Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIFF_A {
    #[doc = "0: Dual-single-ended mode"]
    SINGLE_ENDED = 0,
    #[doc = "1: Differential mode"]
    DIFFERENTIAL = 1,
}
impl From<DIFF_A> for bool {
    #[inline(always)]
    fn from(variant: DIFF_A) -> Self {
        variant as u8 != 0
    }
}
impl DIFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIFF_A {
        match self.bits {
            false => DIFF_A::SINGLE_ENDED,
            true => DIFF_A::DIFFERENTIAL,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_ENDED`"]
    #[inline(always)]
    pub fn is_single_ended(&self) -> bool {
        *self == DIFF_A::SINGLE_ENDED
    }
    #[doc = "Checks if the value of the field is `DIFFERENTIAL`"]
    #[inline(always)]
    pub fn is_differential(&self) -> bool {
        *self == DIFF_A::DIFFERENTIAL
    }
}
#[doc = "Field `DIFF` writer - Differential Mode Enable"]
pub type DIFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDL11_SPEC, DIFF_A, O>;
impl<'a, const O: u8> DIFF_W<'a, O> {
    #[doc = "Dual-single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFF_A::SINGLE_ENDED)
    }
    #[doc = "Differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFF_A::DIFFERENTIAL)
    }
}
#[doc = "Field `CSCALE` reader - Channel Scale"]
pub type CSCALE_R = crate::BitReader<CSCALE_A>;
#[doc = "Channel Scale\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCALE_A {
    #[doc = "0: Scale selected analog channel (factor of 30/64)"]
    HALF_SCALE = 0,
    #[doc = "1: Full-scale (factor of 1)"]
    FULL_SCALE = 1,
}
impl From<CSCALE_A> for bool {
    #[inline(always)]
    fn from(variant: CSCALE_A) -> Self {
        variant as u8 != 0
    }
}
impl CSCALE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSCALE_A {
        match self.bits {
            false => CSCALE_A::HALF_SCALE,
            true => CSCALE_A::FULL_SCALE,
        }
    }
    #[doc = "Checks if the value of the field is `HALF_SCALE`"]
    #[inline(always)]
    pub fn is_half_scale(&self) -> bool {
        *self == CSCALE_A::HALF_SCALE
    }
    #[doc = "Checks if the value of the field is `FULL_SCALE`"]
    #[inline(always)]
    pub fn is_full_scale(&self) -> bool {
        *self == CSCALE_A::FULL_SCALE
    }
}
#[doc = "Field `CSCALE` writer - Channel Scale"]
pub type CSCALE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDL11_SPEC, CSCALE_A, O>;
impl<'a, const O: u8> CSCALE_W<'a, O> {
    #[doc = "Scale selected analog channel (factor of 30/64)"]
    #[inline(always)]
    pub fn half_scale(self) -> &'a mut W {
        self.variant(CSCALE_A::HALF_SCALE)
    }
    #[doc = "Full-scale (factor of 1)"]
    #[inline(always)]
    pub fn full_scale(self) -> &'a mut W {
        self.variant(CSCALE_A::FULL_SCALE)
    }
}
impl R {
    #[doc = "Bits 0:4 - Input Channel Select"]
    #[inline(always)]
    pub fn adch(&self) -> ADCH_R {
        ADCH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - A-side or B-side Select"]
    #[inline(always)]
    pub fn absel(&self) -> ABSEL_R {
        ABSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Differential Mode Enable"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel Scale"]
    #[inline(always)]
    pub fn cscale(&self) -> CSCALE_R {
        CSCALE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn adch(&mut self) -> ADCH_W<0> {
        ADCH_W::new(self)
    }
    #[doc = "Bit 5 - A-side or B-side Select"]
    #[inline(always)]
    #[must_use]
    pub fn absel(&mut self) -> ABSEL_W<5> {
        ABSEL_W::new(self)
    }
    #[doc = "Bit 6 - Differential Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn diff(&mut self) -> DIFF_W<6> {
        DIFF_W::new(self)
    }
    #[doc = "Bit 13 - Channel Scale"]
    #[inline(always)]
    #[must_use]
    pub fn cscale(&mut self) -> CSCALE_W<13> {
        CSCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Low Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdl11](index.html) module"]
pub struct CMDL11_SPEC;
impl crate::RegisterSpec for CMDL11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdl11::R](R) reader structure"]
impl crate::Readable for CMDL11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdl11::W](W) writer structure"]
impl crate::Writable for CMDL11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDL11 to value 0x2000"]
impl crate::Resettable for CMDL11_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000;
}
