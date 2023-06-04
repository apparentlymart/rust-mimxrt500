#[doc = "Register `FREQMECTRL_W` writer"]
pub struct W(crate::W<FREQME_FREQMECTRL_W_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FREQME_FREQMECTRL_W_SPEC>;
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
impl From<crate::W<FREQME_FREQMECTRL_W_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FREQME_FREQMECTRL_W_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Reference Clock Scaling Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REF_SCALE_AW {
    #[doc = "0: Count cycle = 2 ^ 0 = 1"]
    COUNTCYCLE_1 = 0,
    #[doc = "1: Count cycle = 2 ^ 1 = 2"]
    COUNTCYCLE_2 = 1,
    #[doc = "2: Count cycle = 2 ^ 2 = 4"]
    COUNTCYCLE_4 = 2,
    #[doc = "31: Count cycle = 2 ^ 31 = 2,147,483,648"]
    COUNTCYCLE_MAX = 31,
}
impl From<REF_SCALE_AW> for u8 {
    #[inline(always)]
    fn from(variant: REF_SCALE_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `REF_SCALE` writer - Reference Clock Scaling Factor"]
pub type REF_SCALE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FREQME_FREQMECTRL_W_SPEC, u8, REF_SCALE_AW, 5, O>;
impl<'a, const O: u8> REF_SCALE_W<'a, O> {
    #[doc = "Count cycle = 2 ^ 0 = 1"]
    #[inline(always)]
    pub fn countcycle_1(self) -> &'a mut W {
        self.variant(REF_SCALE_AW::COUNTCYCLE_1)
    }
    #[doc = "Count cycle = 2 ^ 1 = 2"]
    #[inline(always)]
    pub fn countcycle_2(self) -> &'a mut W {
        self.variant(REF_SCALE_AW::COUNTCYCLE_2)
    }
    #[doc = "Count cycle = 2 ^ 2 = 4"]
    #[inline(always)]
    pub fn countcycle_4(self) -> &'a mut W {
        self.variant(REF_SCALE_AW::COUNTCYCLE_4)
    }
    #[doc = "Count cycle = 2 ^ 31 = 2,147,483,648"]
    #[inline(always)]
    pub fn countcycle_max(self) -> &'a mut W {
        self.variant(REF_SCALE_AW::COUNTCYCLE_MAX)
    }
}
#[doc = "Pulse Width Measurement mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PULSE_MODE_AW {
    #[doc = "0: Frequency Measurement Mode. FREQMECTRL works in a Frequency Measurement mode. Once the measurement starts (real count start is aligned at rising edge arrival on reference clock), the target counter increments by the target clock until the reference counter running by the reference clock reaches the count end point selected by REF_SCALE."]
    FREQ_ME_MODE = 0,
    #[doc = "1: Pulse Width Measurement mode. FREQMECTRL works in a Pulse Width Measurement mode, measuring the high or low period of reference clock input selected by PULSE_POL. The target counter starts incrementing by the target clock once a corresponding trigger edge (rising edge for high period measurement and falling edge for low period) occurs."]
    PULSE_ME_MODE = 1,
}
impl From<PULSE_MODE_AW> for bool {
    #[inline(always)]
    fn from(variant: PULSE_MODE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PULSE_MODE` writer - Pulse Width Measurement mode select"]
pub type PULSE_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FREQME_FREQMECTRL_W_SPEC, PULSE_MODE_AW, O>;
impl<'a, const O: u8> PULSE_MODE_W<'a, O> {
    #[doc = "Frequency Measurement Mode. FREQMECTRL works in a Frequency Measurement mode. Once the measurement starts (real count start is aligned at rising edge arrival on reference clock), the target counter increments by the target clock until the reference counter running by the reference clock reaches the count end point selected by REF_SCALE."]
    #[inline(always)]
    pub fn freq_me_mode(self) -> &'a mut W {
        self.variant(PULSE_MODE_AW::FREQ_ME_MODE)
    }
    #[doc = "Pulse Width Measurement mode. FREQMECTRL works in a Pulse Width Measurement mode, measuring the high or low period of reference clock input selected by PULSE_POL. The target counter starts incrementing by the target clock once a corresponding trigger edge (rising edge for high period measurement and falling edge for low period) occurs."]
    #[inline(always)]
    pub fn pulse_me_mode(self) -> &'a mut W {
        self.variant(PULSE_MODE_AW::PULSE_ME_MODE)
    }
}
#[doc = "Pulse Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PULSE_POL_AW {
    #[doc = "0: High Period. High period of reference clock is measured in Pulse Width Measurement mode triggered by the rising edge on the reference clock input."]
    HIGH_PERIOD = 0,
    #[doc = "1: Low Period. Low period of reference clock is measured in Pulse Width Measurement mode triggered by the falling edge on the reference clock input."]
    LOW_PERIOD = 1,
}
impl From<PULSE_POL_AW> for bool {
    #[inline(always)]
    fn from(variant: PULSE_POL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PULSE_POL` writer - Pulse Polarity"]
pub type PULSE_POL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FREQME_FREQMECTRL_W_SPEC, PULSE_POL_AW, O>;
impl<'a, const O: u8> PULSE_POL_W<'a, O> {
    #[doc = "High Period. High period of reference clock is measured in Pulse Width Measurement mode triggered by the rising edge on the reference clock input."]
    #[inline(always)]
    pub fn high_period(self) -> &'a mut W {
        self.variant(PULSE_POL_AW::HIGH_PERIOD)
    }
    #[doc = "Low Period. Low period of reference clock is measured in Pulse Width Measurement mode triggered by the falling edge on the reference clock input."]
    #[inline(always)]
    pub fn low_period(self) -> &'a mut W {
        self.variant(PULSE_POL_AW::LOW_PERIOD)
    }
}
#[doc = "Measure in Progress\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEASURE_IN_PROGRESS_AW {
    #[doc = "0: Force Terminate"]
    FORCE_TERMINATE = 0,
    #[doc = "1: Initiates Measurement Cycle"]
    INITIATE_A_FREQME_CYCLE = 1,
}
impl From<MEASURE_IN_PROGRESS_AW> for bool {
    #[inline(always)]
    fn from(variant: MEASURE_IN_PROGRESS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEASURE_IN_PROGRESS` writer - Measure in Progress"]
pub type MEASURE_IN_PROGRESS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FREQME_FREQMECTRL_W_SPEC, MEASURE_IN_PROGRESS_AW, O>;
impl<'a, const O: u8> MEASURE_IN_PROGRESS_W<'a, O> {
    #[doc = "Force Terminate"]
    #[inline(always)]
    pub fn force_terminate(self) -> &'a mut W {
        self.variant(MEASURE_IN_PROGRESS_AW::FORCE_TERMINATE)
    }
    #[doc = "Initiates Measurement Cycle"]
    #[inline(always)]
    pub fn initiate_a_freqme_cycle(self) -> &'a mut W {
        self.variant(MEASURE_IN_PROGRESS_AW::INITIATE_A_FREQME_CYCLE)
    }
}
impl W {
    #[doc = "Bits 0:4 - Reference Clock Scaling Factor"]
    #[inline(always)]
    #[must_use]
    pub fn ref_scale(&mut self) -> REF_SCALE_W<0> {
        REF_SCALE_W::new(self)
    }
    #[doc = "Bit 8 - Pulse Width Measurement mode select"]
    #[inline(always)]
    #[must_use]
    pub fn pulse_mode(&mut self) -> PULSE_MODE_W<8> {
        PULSE_MODE_W::new(self)
    }
    #[doc = "Bit 9 - Pulse Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pulse_pol(&mut self) -> PULSE_POL_W<9> {
        PULSE_POL_W::new(self)
    }
    #[doc = "Bit 31 - Measure in Progress"]
    #[inline(always)]
    #[must_use]
    pub fn measure_in_progress(&mut self) -> MEASURE_IN_PROGRESS_W<31> {
        MEASURE_IN_PROGRESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frequency Measurement (in Write mode)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freqme_freqmectrl_w](index.html) module"]
pub struct FREQME_FREQMECTRL_W_SPEC;
impl crate::RegisterSpec for FREQME_FREQMECTRL_W_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [freqme_freqmectrl_w::W](W) writer structure"]
impl crate::Writable for FREQME_FREQMECTRL_W_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FREQMECTRL_W to value 0"]
impl crate::Resettable for FREQME_FREQMECTRL_W_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
