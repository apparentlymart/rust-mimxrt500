#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPRICTRL` reader - ADC Trigger Priority Control"]
pub type TPRICTRL_R = crate::FieldReader<u8, TPRICTRL_A>;
#[doc = "ADC Trigger Priority Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TPRICTRL_A {
    #[doc = "0: Current conversion is aborted and the new command specified by the trigger is started."]
    ABORT_CURRENT_ON_PRIORITY = 0,
    #[doc = "1: Current command is stopped after completing the current conversion. If averaging is enabled, the averaging loop is completed. CMDHn\\[LOOP\\]
is ignored and the higher-priority trigger is serviced."]
    FINISH_CURRENT_ON_PRIORITY = 1,
    #[doc = "2: Current command is completed (averaging, looping, compare) before servicing the higher-priority trigger."]
    FINISH_SEQUENCE_ON_PRIORITY = 2,
}
impl From<TPRICTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: TPRICTRL_A) -> Self {
        variant as _
    }
}
impl TPRICTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TPRICTRL_A> {
        match self.bits {
            0 => Some(TPRICTRL_A::ABORT_CURRENT_ON_PRIORITY),
            1 => Some(TPRICTRL_A::FINISH_CURRENT_ON_PRIORITY),
            2 => Some(TPRICTRL_A::FINISH_SEQUENCE_ON_PRIORITY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ABORT_CURRENT_ON_PRIORITY`"]
    #[inline(always)]
    pub fn is_abort_current_on_priority(&self) -> bool {
        *self == TPRICTRL_A::ABORT_CURRENT_ON_PRIORITY
    }
    #[doc = "Checks if the value of the field is `FINISH_CURRENT_ON_PRIORITY`"]
    #[inline(always)]
    pub fn is_finish_current_on_priority(&self) -> bool {
        *self == TPRICTRL_A::FINISH_CURRENT_ON_PRIORITY
    }
    #[doc = "Checks if the value of the field is `FINISH_SEQUENCE_ON_PRIORITY`"]
    #[inline(always)]
    pub fn is_finish_sequence_on_priority(&self) -> bool {
        *self == TPRICTRL_A::FINISH_SEQUENCE_ON_PRIORITY
    }
}
#[doc = "Field `TPRICTRL` writer - ADC Trigger Priority Control"]
pub type TPRICTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, TPRICTRL_A, 2, O>;
impl<'a, const O: u8> TPRICTRL_W<'a, O> {
    #[doc = "Current conversion is aborted and the new command specified by the trigger is started."]
    #[inline(always)]
    pub fn abort_current_on_priority(self) -> &'a mut W {
        self.variant(TPRICTRL_A::ABORT_CURRENT_ON_PRIORITY)
    }
    #[doc = "Current command is stopped after completing the current conversion. If averaging is enabled, the averaging loop is completed. CMDHn\\[LOOP\\]
is ignored and the higher-priority trigger is serviced."]
    #[inline(always)]
    pub fn finish_current_on_priority(self) -> &'a mut W {
        self.variant(TPRICTRL_A::FINISH_CURRENT_ON_PRIORITY)
    }
    #[doc = "Current command is completed (averaging, looping, compare) before servicing the higher-priority trigger."]
    #[inline(always)]
    pub fn finish_sequence_on_priority(self) -> &'a mut W {
        self.variant(TPRICTRL_A::FINISH_SEQUENCE_ON_PRIORITY)
    }
}
#[doc = "Field `PWRSEL` reader - Power Configuration Select"]
pub type PWRSEL_R = crate::FieldReader<u8, PWRSEL_A>;
#[doc = "Power Configuration Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWRSEL_A {
    #[doc = "0: Lowest power"]
    LOWEST = 0,
    #[doc = "1: Higher power than 00b"]
    INTERMEDIATE_1 = 1,
    #[doc = "2: Higher power than 01b"]
    INTERMEDIATE_2 = 2,
    #[doc = "3: Highest power"]
    HIGHEST = 3,
}
impl From<PWRSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRSEL_A) -> Self {
        variant as _
    }
}
impl PWRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRSEL_A {
        match self.bits {
            0 => PWRSEL_A::LOWEST,
            1 => PWRSEL_A::INTERMEDIATE_1,
            2 => PWRSEL_A::INTERMEDIATE_2,
            3 => PWRSEL_A::HIGHEST,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOWEST`"]
    #[inline(always)]
    pub fn is_lowest(&self) -> bool {
        *self == PWRSEL_A::LOWEST
    }
    #[doc = "Checks if the value of the field is `INTERMEDIATE_1`"]
    #[inline(always)]
    pub fn is_intermediate_1(&self) -> bool {
        *self == PWRSEL_A::INTERMEDIATE_1
    }
    #[doc = "Checks if the value of the field is `INTERMEDIATE_2`"]
    #[inline(always)]
    pub fn is_intermediate_2(&self) -> bool {
        *self == PWRSEL_A::INTERMEDIATE_2
    }
    #[doc = "Checks if the value of the field is `HIGHEST`"]
    #[inline(always)]
    pub fn is_highest(&self) -> bool {
        *self == PWRSEL_A::HIGHEST
    }
}
#[doc = "Field `PWRSEL` writer - Power Configuration Select"]
pub type PWRSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG_SPEC, u8, PWRSEL_A, 2, O>;
impl<'a, const O: u8> PWRSEL_W<'a, O> {
    #[doc = "Lowest power"]
    #[inline(always)]
    pub fn lowest(self) -> &'a mut W {
        self.variant(PWRSEL_A::LOWEST)
    }
    #[doc = "Higher power than 00b"]
    #[inline(always)]
    pub fn intermediate_1(self) -> &'a mut W {
        self.variant(PWRSEL_A::INTERMEDIATE_1)
    }
    #[doc = "Higher power than 01b"]
    #[inline(always)]
    pub fn intermediate_2(self) -> &'a mut W {
        self.variant(PWRSEL_A::INTERMEDIATE_2)
    }
    #[doc = "Highest power"]
    #[inline(always)]
    pub fn highest(self) -> &'a mut W {
        self.variant(PWRSEL_A::HIGHEST)
    }
}
#[doc = "Field `REFSEL` reader - Voltage Reference Selection"]
pub type REFSEL_R = crate::FieldReader<u8, REFSEL_A>;
#[doc = "Voltage Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: Option 1"]
    OPTION_1 = 0,
    #[doc = "1: Option 2"]
    OPTION_2 = 1,
    #[doc = "2: Option 3"]
    OPTION_3 = 2,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
impl REFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REFSEL_A> {
        match self.bits {
            0 => Some(REFSEL_A::OPTION_1),
            1 => Some(REFSEL_A::OPTION_2),
            2 => Some(REFSEL_A::OPTION_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OPTION_1`"]
    #[inline(always)]
    pub fn is_option_1(&self) -> bool {
        *self == REFSEL_A::OPTION_1
    }
    #[doc = "Checks if the value of the field is `OPTION_2`"]
    #[inline(always)]
    pub fn is_option_2(&self) -> bool {
        *self == REFSEL_A::OPTION_2
    }
    #[doc = "Checks if the value of the field is `OPTION_3`"]
    #[inline(always)]
    pub fn is_option_3(&self) -> bool {
        *self == REFSEL_A::OPTION_3
    }
}
#[doc = "Field `REFSEL` writer - Voltage Reference Selection"]
pub type REFSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, REFSEL_A, 2, O>;
impl<'a, const O: u8> REFSEL_W<'a, O> {
    #[doc = "Option 1"]
    #[inline(always)]
    pub fn option_1(self) -> &'a mut W {
        self.variant(REFSEL_A::OPTION_1)
    }
    #[doc = "Option 2"]
    #[inline(always)]
    pub fn option_2(self) -> &'a mut W {
        self.variant(REFSEL_A::OPTION_2)
    }
    #[doc = "Option 3"]
    #[inline(always)]
    pub fn option_3(self) -> &'a mut W {
        self.variant(REFSEL_A::OPTION_3)
    }
}
#[doc = "Field `TRES` reader - Trigger Resume Enable"]
pub type TRES_R = crate::BitReader<TRES_A>;
#[doc = "Trigger Resume Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRES_A {
    #[doc = "0: Not automatically resumed or restarted"]
    DISABLED = 0,
    #[doc = "1: Automatically resumed or restarted"]
    ENABLED = 1,
}
impl From<TRES_A> for bool {
    #[inline(always)]
    fn from(variant: TRES_A) -> Self {
        variant as u8 != 0
    }
}
impl TRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRES_A {
        match self.bits {
            false => TRES_A::DISABLED,
            true => TRES_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRES_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRES_A::ENABLED
    }
}
#[doc = "Field `TRES` writer - Trigger Resume Enable"]
pub type TRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, TRES_A, O>;
impl<'a, const O: u8> TRES_W<'a, O> {
    #[doc = "Not automatically resumed or restarted"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRES_A::DISABLED)
    }
    #[doc = "Automatically resumed or restarted"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRES_A::ENABLED)
    }
}
#[doc = "Field `TCMDRES` reader - Trigger Command Resume"]
pub type TCMDRES_R = crate::BitReader<TCMDRES_A>;
#[doc = "Trigger Command Resume\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCMDRES_A {
    #[doc = "0: Trigger sequence automatically restarted."]
    DISABLED = 0,
    #[doc = "1: Trigger sequence resumed from the command that was executed prior to the exception."]
    ENABLED = 1,
}
impl From<TCMDRES_A> for bool {
    #[inline(always)]
    fn from(variant: TCMDRES_A) -> Self {
        variant as u8 != 0
    }
}
impl TCMDRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCMDRES_A {
        match self.bits {
            false => TCMDRES_A::DISABLED,
            true => TCMDRES_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCMDRES_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCMDRES_A::ENABLED
    }
}
#[doc = "Field `TCMDRES` writer - Trigger Command Resume"]
pub type TCMDRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, TCMDRES_A, O>;
impl<'a, const O: u8> TCMDRES_W<'a, O> {
    #[doc = "Trigger sequence automatically restarted."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCMDRES_A::DISABLED)
    }
    #[doc = "Trigger sequence resumed from the command that was executed prior to the exception."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCMDRES_A::ENABLED)
    }
}
#[doc = "Field `HPT_EXDI` reader - High-Priority Trigger Exception Disable"]
pub type HPT_EXDI_R = crate::BitReader<HPT_EXDI_A>;
#[doc = "High-Priority Trigger Exception Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPT_EXDI_A {
    #[doc = "0: Enabled"]
    ENABLED = 0,
    #[doc = "1: Disabled"]
    DISABLED = 1,
}
impl From<HPT_EXDI_A> for bool {
    #[inline(always)]
    fn from(variant: HPT_EXDI_A) -> Self {
        variant as u8 != 0
    }
}
impl HPT_EXDI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPT_EXDI_A {
        match self.bits {
            false => HPT_EXDI_A::ENABLED,
            true => HPT_EXDI_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HPT_EXDI_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HPT_EXDI_A::DISABLED
    }
}
#[doc = "Field `HPT_EXDI` writer - High-Priority Trigger Exception Disable"]
pub type HPT_EXDI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, HPT_EXDI_A, O>;
impl<'a, const O: u8> HPT_EXDI_W<'a, O> {
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HPT_EXDI_A::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HPT_EXDI_A::DISABLED)
    }
}
#[doc = "Field `PUDLY` reader - Power-up Delay"]
pub type PUDLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUDLY` writer - Power-up Delay"]
pub type PUDLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `PWREN` reader - ADC Analog Pre-Enable"]
pub type PWREN_R = crate::BitReader<PWREN_A>;
#[doc = "ADC Analog Pre-Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWREN_A {
    #[doc = "0: ADC analog circuits are only enabled while conversions are active. Analog startup delays affect performance."]
    NOT_PRE_ENABLED = 0,
    #[doc = "1: ADC analog circuits are pre-enabled and ready to execute conversions without startup delays, at the cost of higher DC current consumption. A single power-up delay (CFG\\[PUDLY\\]) is executed immediately once PWREN is set. No detected triggers begin ADC operation until the power-up delay time has passed. After this initial delay expires, the analog circuits remain pre-enabled, and no additional delays are executed."]
    PRE_ENABLED = 1,
}
impl From<PWREN_A> for bool {
    #[inline(always)]
    fn from(variant: PWREN_A) -> Self {
        variant as u8 != 0
    }
}
impl PWREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWREN_A {
        match self.bits {
            false => PWREN_A::NOT_PRE_ENABLED,
            true => PWREN_A::PRE_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRE_ENABLED`"]
    #[inline(always)]
    pub fn is_not_pre_enabled(&self) -> bool {
        *self == PWREN_A::NOT_PRE_ENABLED
    }
    #[doc = "Checks if the value of the field is `PRE_ENABLED`"]
    #[inline(always)]
    pub fn is_pre_enabled(&self) -> bool {
        *self == PWREN_A::PRE_ENABLED
    }
}
#[doc = "Field `PWREN` writer - ADC Analog Pre-Enable"]
pub type PWREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, PWREN_A, O>;
impl<'a, const O: u8> PWREN_W<'a, O> {
    #[doc = "ADC analog circuits are only enabled while conversions are active. Analog startup delays affect performance."]
    #[inline(always)]
    pub fn not_pre_enabled(self) -> &'a mut W {
        self.variant(PWREN_A::NOT_PRE_ENABLED)
    }
    #[doc = "ADC analog circuits are pre-enabled and ready to execute conversions without startup delays, at the cost of higher DC current consumption. A single power-up delay (CFG\\[PUDLY\\]) is executed immediately once PWREN is set. No detected triggers begin ADC operation until the power-up delay time has passed. After this initial delay expires, the analog circuits remain pre-enabled, and no additional delays are executed."]
    #[inline(always)]
    pub fn pre_enabled(self) -> &'a mut W {
        self.variant(PWREN_A::PRE_ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:1 - ADC Trigger Priority Control"]
    #[inline(always)]
    pub fn tprictrl(&self) -> TPRICTRL_R {
        TPRICTRL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Power Configuration Select"]
    #[inline(always)]
    pub fn pwrsel(&self) -> PWRSEL_R {
        PWRSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Voltage Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Trigger Resume Enable"]
    #[inline(always)]
    pub fn tres(&self) -> TRES_R {
        TRES_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Trigger Command Resume"]
    #[inline(always)]
    pub fn tcmdres(&self) -> TCMDRES_R {
        TCMDRES_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - High-Priority Trigger Exception Disable"]
    #[inline(always)]
    pub fn hpt_exdi(&self) -> HPT_EXDI_R {
        HPT_EXDI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Power-up Delay"]
    #[inline(always)]
    pub fn pudly(&self) -> PUDLY_R {
        PUDLY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 28 - ADC Analog Pre-Enable"]
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC Trigger Priority Control"]
    #[inline(always)]
    #[must_use]
    pub fn tprictrl(&mut self) -> TPRICTRL_W<0> {
        TPRICTRL_W::new(self)
    }
    #[doc = "Bits 4:5 - Power Configuration Select"]
    #[inline(always)]
    #[must_use]
    pub fn pwrsel(&mut self) -> PWRSEL_W<4> {
        PWRSEL_W::new(self)
    }
    #[doc = "Bits 6:7 - Voltage Reference Selection"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> REFSEL_W<6> {
        REFSEL_W::new(self)
    }
    #[doc = "Bit 8 - Trigger Resume Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tres(&mut self) -> TRES_W<8> {
        TRES_W::new(self)
    }
    #[doc = "Bit 9 - Trigger Command Resume"]
    #[inline(always)]
    #[must_use]
    pub fn tcmdres(&mut self) -> TCMDRES_W<9> {
        TCMDRES_W::new(self)
    }
    #[doc = "Bit 10 - High-Priority Trigger Exception Disable"]
    #[inline(always)]
    #[must_use]
    pub fn hpt_exdi(&mut self) -> HPT_EXDI_W<10> {
        HPT_EXDI_W::new(self)
    }
    #[doc = "Bits 16:23 - Power-up Delay"]
    #[inline(always)]
    #[must_use]
    pub fn pudly(&mut self) -> PUDLY_W<16> {
        PUDLY_W::new(self)
    }
    #[doc = "Bit 28 - ADC Analog Pre-Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwren(&mut self) -> PWREN_W<28> {
        PWREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0x0080_0000"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0080_0000;
}
