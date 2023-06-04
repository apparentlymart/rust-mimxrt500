#[doc = "Register `PORTSC1` reader"]
pub struct R(crate::R<PORTSC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORTSC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORTSC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORTSC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORTSC1` writer"]
pub struct W(crate::W<PORTSC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORTSC1_SPEC>;
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
impl From<crate::W<PORTSC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORTSC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCS` reader - Current Connect Status"]
pub type CCS_R = crate::BitReader<CCS_A>;
#[doc = "Current Connect Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCS_A {
    #[doc = "0: No Device is present"]
    DISABLE = 0,
    #[doc = "1: Device is present"]
    ENABLE = 1,
}
impl From<CCS_A> for bool {
    #[inline(always)]
    fn from(variant: CCS_A) -> Self {
        variant as u8 != 0
    }
}
impl CCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCS_A {
        match self.bits {
            false => CCS_A::DISABLE,
            true => CCS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CCS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CCS_A::ENABLE
    }
}
#[doc = "Field `CSC` reader - Connect Status Change"]
pub type CSC_R = crate::BitReader<CSC_A>;
#[doc = "Connect Status Change\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSC_A {
    #[doc = "0: CCS value has not changed"]
    DISABLE = 0,
    #[doc = "1: CCS value has changed"]
    ENABLE = 1,
}
impl From<CSC_A> for bool {
    #[inline(always)]
    fn from(variant: CSC_A) -> Self {
        variant as u8 != 0
    }
}
impl CSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSC_A {
        match self.bits {
            false => CSC_A::DISABLE,
            true => CSC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CSC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CSC_A::ENABLE
    }
}
#[doc = "Field `CSC` writer - Connect Status Change"]
pub type CSC_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, PORTSC1_SPEC, CSC_A, O>;
impl<'a, const O: u8> CSC_W<'a, O> {
    #[doc = "CCS value has not changed"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CSC_A::DISABLE)
    }
    #[doc = "CCS value has changed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CSC_A::ENABLE)
    }
}
#[doc = "Field `PED` reader - Port Enabled/Disabled"]
pub type PED_R = crate::BitReader<PED_A>;
#[doc = "Port Enabled/Disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PED_A {
    #[doc = "0: Port Disabled"]
    DISABLE = 0,
    #[doc = "1: Port Enabled"]
    ENABLE = 1,
}
impl From<PED_A> for bool {
    #[inline(always)]
    fn from(variant: PED_A) -> Self {
        variant as u8 != 0
    }
}
impl PED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PED_A {
        match self.bits {
            false => PED_A::DISABLE,
            true => PED_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PED_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PED_A::ENABLE
    }
}
#[doc = "Field `PED` writer - Port Enabled/Disabled"]
pub type PED_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, PED_A, O>;
impl<'a, const O: u8> PED_W<'a, O> {
    #[doc = "Port Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PED_A::DISABLE)
    }
    #[doc = "Port Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PED_A::ENABLE)
    }
}
#[doc = "Field `PEDC` reader - Port Enabled/Disabled Change"]
pub type PEDC_R = crate::BitReader<PEDC_A>;
#[doc = "Port Enabled/Disabled Change\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEDC_A {
    #[doc = "0: PED value has not changed"]
    DISABLE = 0,
    #[doc = "1: PED value has changed"]
    ENABLE = 1,
}
impl From<PEDC_A> for bool {
    #[inline(always)]
    fn from(variant: PEDC_A) -> Self {
        variant as u8 != 0
    }
}
impl PEDC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEDC_A {
        match self.bits {
            false => PEDC_A::DISABLE,
            true => PEDC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PEDC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PEDC_A::ENABLE
    }
}
#[doc = "Field `PEDC` writer - Port Enabled/Disabled Change"]
pub type PEDC_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, PORTSC1_SPEC, PEDC_A, O>;
impl<'a, const O: u8> PEDC_W<'a, O> {
    #[doc = "PED value has not changed"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PEDC_A::DISABLE)
    }
    #[doc = "PED value has changed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PEDC_A::ENABLE)
    }
}
#[doc = "Field `OCA` reader - Over-current active"]
pub type OCA_R = crate::BitReader<OCA_A>;
#[doc = "Over-current active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCA_A {
    #[doc = "0: Port does not have an over-current condition"]
    DISABLE = 0,
    #[doc = "1: Port has an over-current condition"]
    ENABLE = 1,
}
impl From<OCA_A> for bool {
    #[inline(always)]
    fn from(variant: OCA_A) -> Self {
        variant as u8 != 0
    }
}
impl OCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCA_A {
        match self.bits {
            false => OCA_A::DISABLE,
            true => OCA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OCA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OCA_A::ENABLE
    }
}
#[doc = "Field `OCC` reader - Over-current active"]
pub type OCC_R = crate::BitReader<OCC_A>;
#[doc = "Over-current active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCC_A {
    #[doc = "0: OCA value has not changed"]
    DISABLE = 0,
    #[doc = "1: OCA value has changed"]
    ENABLE = 1,
}
impl From<OCC_A> for bool {
    #[inline(always)]
    fn from(variant: OCC_A) -> Self {
        variant as u8 != 0
    }
}
impl OCC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCC_A {
        match self.bits {
            false => OCC_A::DISABLE,
            true => OCC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OCC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OCC_A::ENABLE
    }
}
#[doc = "Field `OCC` writer - Over-current active"]
pub type OCC_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, PORTSC1_SPEC, OCC_A, O>;
impl<'a, const O: u8> OCC_W<'a, O> {
    #[doc = "OCA value has not changed"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OCC_A::DISABLE)
    }
    #[doc = "OCA value has changed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OCC_A::ENABLE)
    }
}
#[doc = "Field `FPR` reader - Force Port Resume"]
pub type FPR_R = crate::BitReader<FPR_A>;
#[doc = "Force Port Resume\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPR_A {
    #[doc = "0: No Resume (K-state) detected or driven on the port."]
    DISABLE = 0,
    #[doc = "1: Resume (K-state) detected or driven on the port."]
    ENABLE = 1,
}
impl From<FPR_A> for bool {
    #[inline(always)]
    fn from(variant: FPR_A) -> Self {
        variant as u8 != 0
    }
}
impl FPR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPR_A {
        match self.bits {
            false => FPR_A::DISABLE,
            true => FPR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FPR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FPR_A::ENABLE
    }
}
#[doc = "Field `FPR` writer - Force Port Resume"]
pub type FPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, FPR_A, O>;
impl<'a, const O: u8> FPR_W<'a, O> {
    #[doc = "No Resume (K-state) detected or driven on the port."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FPR_A::DISABLE)
    }
    #[doc = "Resume (K-state) detected or driven on the port."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FPR_A::ENABLE)
    }
}
#[doc = "Field `SUSP` reader - Suspend"]
pub type SUSP_R = crate::BitReader<SUSP_A>;
#[doc = "Suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSP_A {
    #[doc = "0: Enabled port is not suspended"]
    DISABLE = 0,
    #[doc = "1: Enabled port is in the L1 or L2 suspend state"]
    ENABLE = 1,
}
impl From<SUSP_A> for bool {
    #[inline(always)]
    fn from(variant: SUSP_A) -> Self {
        variant as u8 != 0
    }
}
impl SUSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSP_A {
        match self.bits {
            false => SUSP_A::DISABLE,
            true => SUSP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SUSP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SUSP_A::ENABLE
    }
}
#[doc = "Field `SUSP` writer - Suspend"]
pub type SUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, SUSP_A, O>;
impl<'a, const O: u8> SUSP_W<'a, O> {
    #[doc = "Enabled port is not suspended"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SUSP_A::DISABLE)
    }
    #[doc = "Enabled port is in the L1 or L2 suspend state"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SUSP_A::ENABLE)
    }
}
#[doc = "Field `PR` reader - Port Reset"]
pub type PR_R = crate::BitReader<PR_A>;
#[doc = "Port Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR_A {
    #[doc = "0: Port is not in the reset state"]
    DISABLE = 0,
    #[doc = "1: Port is in the reset state"]
    ENABLE = 1,
}
impl From<PR_A> for bool {
    #[inline(always)]
    fn from(variant: PR_A) -> Self {
        variant as u8 != 0
    }
}
impl PR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PR_A {
        match self.bits {
            false => PR_A::DISABLE,
            true => PR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PR_A::ENABLE
    }
}
#[doc = "Field `PR` writer - Port Reset"]
pub type PR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, PR_A, O>;
impl<'a, const O: u8> PR_W<'a, O> {
    #[doc = "Port is not in the reset state"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PR_A::DISABLE)
    }
    #[doc = "Port is in the reset state"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PR_A::ENABLE)
    }
}
#[doc = "Field `SUS_L1` reader - Suspend using L1"]
pub type SUS_L1_R = crate::BitReader<SUS_L1_A>;
#[doc = "Suspend using L1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUS_L1_A {
    #[doc = "0: Suspend using L2"]
    DISABLE = 0,
    #[doc = "1: Suspend using L1"]
    ENABLE = 1,
}
impl From<SUS_L1_A> for bool {
    #[inline(always)]
    fn from(variant: SUS_L1_A) -> Self {
        variant as u8 != 0
    }
}
impl SUS_L1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUS_L1_A {
        match self.bits {
            false => SUS_L1_A::DISABLE,
            true => SUS_L1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SUS_L1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SUS_L1_A::ENABLE
    }
}
#[doc = "Field `SUS_L1` writer - Suspend using L1"]
pub type SUS_L1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, SUS_L1_A, O>;
impl<'a, const O: u8> SUS_L1_W<'a, O> {
    #[doc = "Suspend using L2"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SUS_L1_A::DISABLE)
    }
    #[doc = "Suspend using L1"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SUS_L1_A::ENABLE)
    }
}
#[doc = "Field `LS` reader - Line Status"]
pub type LS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PP` reader - Port Power"]
pub type PP_R = crate::BitReader<bool>;
#[doc = "Field `PP` writer - Port Power"]
pub type PP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `PIC` reader - Port Indicator Control"]
pub type PIC_R = crate::FieldReader<u8, PIC_A>;
#[doc = "Port Indicator Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PIC_A {
    #[doc = "0: Port Indicators are off"]
    DISABLE = 0,
    #[doc = "1: Amber"]
    ENABLE1 = 1,
    #[doc = "2: Green"]
    ENABLE2 = 2,
    #[doc = "3: Undefined"]
    ENABLE3 = 3,
}
impl From<PIC_A> for u8 {
    #[inline(always)]
    fn from(variant: PIC_A) -> Self {
        variant as _
    }
}
impl PIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIC_A {
        match self.bits {
            0 => PIC_A::DISABLE,
            1 => PIC_A::ENABLE1,
            2 => PIC_A::ENABLE2,
            3 => PIC_A::ENABLE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PIC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE1`"]
    #[inline(always)]
    pub fn is_enable1(&self) -> bool {
        *self == PIC_A::ENABLE1
    }
    #[doc = "Checks if the value of the field is `ENABLE2`"]
    #[inline(always)]
    pub fn is_enable2(&self) -> bool {
        *self == PIC_A::ENABLE2
    }
    #[doc = "Checks if the value of the field is `ENABLE3`"]
    #[inline(always)]
    pub fn is_enable3(&self) -> bool {
        *self == PIC_A::ENABLE3
    }
}
#[doc = "Field `PIC` writer - Port Indicator Control"]
pub type PIC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PORTSC1_SPEC, u8, PIC_A, 2, O>;
impl<'a, const O: u8> PIC_W<'a, O> {
    #[doc = "Port Indicators are off"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PIC_A::DISABLE)
    }
    #[doc = "Amber"]
    #[inline(always)]
    pub fn enable1(self) -> &'a mut W {
        self.variant(PIC_A::ENABLE1)
    }
    #[doc = "Green"]
    #[inline(always)]
    pub fn enable2(self) -> &'a mut W {
        self.variant(PIC_A::ENABLE2)
    }
    #[doc = "Undefined"]
    #[inline(always)]
    pub fn enable3(self) -> &'a mut W {
        self.variant(PIC_A::ENABLE3)
    }
}
#[doc = "Field `PTC` reader - Port Test Control"]
pub type PTC_R = crate::FieldReader<u8, PTC_A>;
#[doc = "Port Test Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PTC_A {
    #[doc = "0: Test mode not enabled"]
    DISABLE = 0,
    #[doc = "1: Test J_STATE"]
    ENABLE1 = 1,
    #[doc = "2: Test K_STATE"]
    ENABLE2 = 2,
    #[doc = "3: TEST SE0_NAK"]
    ENABLE3 = 3,
    #[doc = "4: Test_Packet"]
    ENABLE4 = 4,
    #[doc = "5: Test Force_Enable"]
    ENABLE5 = 5,
}
impl From<PTC_A> for u8 {
    #[inline(always)]
    fn from(variant: PTC_A) -> Self {
        variant as _
    }
}
impl PTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PTC_A> {
        match self.bits {
            0 => Some(PTC_A::DISABLE),
            1 => Some(PTC_A::ENABLE1),
            2 => Some(PTC_A::ENABLE2),
            3 => Some(PTC_A::ENABLE3),
            4 => Some(PTC_A::ENABLE4),
            5 => Some(PTC_A::ENABLE5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PTC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE1`"]
    #[inline(always)]
    pub fn is_enable1(&self) -> bool {
        *self == PTC_A::ENABLE1
    }
    #[doc = "Checks if the value of the field is `ENABLE2`"]
    #[inline(always)]
    pub fn is_enable2(&self) -> bool {
        *self == PTC_A::ENABLE2
    }
    #[doc = "Checks if the value of the field is `ENABLE3`"]
    #[inline(always)]
    pub fn is_enable3(&self) -> bool {
        *self == PTC_A::ENABLE3
    }
    #[doc = "Checks if the value of the field is `ENABLE4`"]
    #[inline(always)]
    pub fn is_enable4(&self) -> bool {
        *self == PTC_A::ENABLE4
    }
    #[doc = "Checks if the value of the field is `ENABLE5`"]
    #[inline(always)]
    pub fn is_enable5(&self) -> bool {
        *self == PTC_A::ENABLE5
    }
}
#[doc = "Field `PTC` writer - Port Test Control"]
pub type PTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PORTSC1_SPEC, u8, PTC_A, 4, O>;
impl<'a, const O: u8> PTC_W<'a, O> {
    #[doc = "Test mode not enabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PTC_A::DISABLE)
    }
    #[doc = "Test J_STATE"]
    #[inline(always)]
    pub fn enable1(self) -> &'a mut W {
        self.variant(PTC_A::ENABLE1)
    }
    #[doc = "Test K_STATE"]
    #[inline(always)]
    pub fn enable2(self) -> &'a mut W {
        self.variant(PTC_A::ENABLE2)
    }
    #[doc = "TEST SE0_NAK"]
    #[inline(always)]
    pub fn enable3(self) -> &'a mut W {
        self.variant(PTC_A::ENABLE3)
    }
    #[doc = "Test_Packet"]
    #[inline(always)]
    pub fn enable4(self) -> &'a mut W {
        self.variant(PTC_A::ENABLE4)
    }
    #[doc = "Test Force_Enable"]
    #[inline(always)]
    pub fn enable5(self) -> &'a mut W {
        self.variant(PTC_A::ENABLE5)
    }
}
#[doc = "Field `PSPD` reader - Port Speed"]
pub type PSPD_R = crate::FieldReader<u8, PSPD_A>;
#[doc = "Port Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSPD_A {
    #[doc = "1: Full-speed"]
    ENABLE1 = 1,
    #[doc = "2: High-speed"]
    ENABLE2 = 2,
}
impl From<PSPD_A> for u8 {
    #[inline(always)]
    fn from(variant: PSPD_A) -> Self {
        variant as _
    }
}
impl PSPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PSPD_A> {
        match self.bits {
            1 => Some(PSPD_A::ENABLE1),
            2 => Some(PSPD_A::ENABLE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE1`"]
    #[inline(always)]
    pub fn is_enable1(&self) -> bool {
        *self == PSPD_A::ENABLE1
    }
    #[doc = "Checks if the value of the field is `ENABLE2`"]
    #[inline(always)]
    pub fn is_enable2(&self) -> bool {
        *self == PSPD_A::ENABLE2
    }
}
#[doc = "Field `WOO` reader - Wake on overcurrent enable"]
pub type WOO_R = crate::BitReader<bool>;
#[doc = "Field `WOO` writer - Wake on overcurrent enable"]
pub type WOO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `SUS_STAT` reader - Suspend Status"]
pub type SUS_STAT_R = crate::FieldReader<u8, SUS_STAT_A>;
#[doc = "Suspend Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SUS_STAT_A {
    #[doc = "0: State transition was successful (ACK)"]
    DISABLE = 0,
    #[doc = "1: Device was unable to enter the L1 state at this time (NYET)"]
    ENABLE1 = 1,
    #[doc = "2: Device does not support the L1 state (STALL)"]
    ENABLE2 = 2,
    #[doc = "3: Timeout/Error - Device failed to respond or an error occurred."]
    ENABLE3 = 3,
}
impl From<SUS_STAT_A> for u8 {
    #[inline(always)]
    fn from(variant: SUS_STAT_A) -> Self {
        variant as _
    }
}
impl SUS_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUS_STAT_A {
        match self.bits {
            0 => SUS_STAT_A::DISABLE,
            1 => SUS_STAT_A::ENABLE1,
            2 => SUS_STAT_A::ENABLE2,
            3 => SUS_STAT_A::ENABLE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SUS_STAT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE1`"]
    #[inline(always)]
    pub fn is_enable1(&self) -> bool {
        *self == SUS_STAT_A::ENABLE1
    }
    #[doc = "Checks if the value of the field is `ENABLE2`"]
    #[inline(always)]
    pub fn is_enable2(&self) -> bool {
        *self == SUS_STAT_A::ENABLE2
    }
    #[doc = "Checks if the value of the field is `ENABLE3`"]
    #[inline(always)]
    pub fn is_enable3(&self) -> bool {
        *self == SUS_STAT_A::ENABLE3
    }
}
#[doc = "Field `DEV_ADD` reader - Device Address for LPM tokens"]
pub type DEV_ADD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEV_ADD` writer - Device Address for LPM tokens"]
pub type DEV_ADD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PORTSC1_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - Current Connect Status"]
    #[inline(always)]
    pub fn ccs(&self) -> CCS_R {
        CCS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Connect Status Change"]
    #[inline(always)]
    pub fn csc(&self) -> CSC_R {
        CSC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Enabled/Disabled"]
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Enabled/Disabled Change"]
    #[inline(always)]
    pub fn pedc(&self) -> PEDC_R {
        PEDC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Over-current active"]
    #[inline(always)]
    pub fn oca(&self) -> OCA_R {
        OCA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Over-current active"]
    #[inline(always)]
    pub fn occ(&self) -> OCC_R {
        OCC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Force Port Resume"]
    #[inline(always)]
    pub fn fpr(&self) -> FPR_R {
        FPR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Suspend"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Suspend using L1"]
    #[inline(always)]
    pub fn sus_l1(&self) -> SUS_L1_R {
        SUS_L1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Line Status"]
    #[inline(always)]
    pub fn ls(&self) -> LS_R {
        LS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline(always)]
    pub fn pp(&self) -> PP_R {
        PP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Port Indicator Control"]
    #[inline(always)]
    pub fn pic(&self) -> PIC_R {
        PIC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Port Test Control"]
    #[inline(always)]
    pub fn ptc(&self) -> PTC_R {
        PTC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Port Speed"]
    #[inline(always)]
    pub fn pspd(&self) -> PSPD_R {
        PSPD_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Wake on overcurrent enable"]
    #[inline(always)]
    pub fn woo(&self) -> WOO_R {
        WOO_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:24 - Suspend Status"]
    #[inline(always)]
    pub fn sus_stat(&self) -> SUS_STAT_R {
        SUS_STAT_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:31 - Device Address for LPM tokens"]
    #[inline(always)]
    pub fn dev_add(&self) -> DEV_ADD_R {
        DEV_ADD_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Connect Status Change"]
    #[inline(always)]
    #[must_use]
    pub fn csc(&mut self) -> CSC_W<1> {
        CSC_W::new(self)
    }
    #[doc = "Bit 2 - Port Enabled/Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn ped(&mut self) -> PED_W<2> {
        PED_W::new(self)
    }
    #[doc = "Bit 3 - Port Enabled/Disabled Change"]
    #[inline(always)]
    #[must_use]
    pub fn pedc(&mut self) -> PEDC_W<3> {
        PEDC_W::new(self)
    }
    #[doc = "Bit 5 - Over-current active"]
    #[inline(always)]
    #[must_use]
    pub fn occ(&mut self) -> OCC_W<5> {
        OCC_W::new(self)
    }
    #[doc = "Bit 6 - Force Port Resume"]
    #[inline(always)]
    #[must_use]
    pub fn fpr(&mut self) -> FPR_W<6> {
        FPR_W::new(self)
    }
    #[doc = "Bit 7 - Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<7> {
        SUSP_W::new(self)
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<8> {
        PR_W::new(self)
    }
    #[doc = "Bit 9 - Suspend using L1"]
    #[inline(always)]
    #[must_use]
    pub fn sus_l1(&mut self) -> SUS_L1_W<9> {
        SUS_L1_W::new(self)
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline(always)]
    #[must_use]
    pub fn pp(&mut self) -> PP_W<12> {
        PP_W::new(self)
    }
    #[doc = "Bits 14:15 - Port Indicator Control"]
    #[inline(always)]
    #[must_use]
    pub fn pic(&mut self) -> PIC_W<14> {
        PIC_W::new(self)
    }
    #[doc = "Bits 16:19 - Port Test Control"]
    #[inline(always)]
    #[must_use]
    pub fn ptc(&mut self) -> PTC_W<16> {
        PTC_W::new(self)
    }
    #[doc = "Bit 22 - Wake on overcurrent enable"]
    #[inline(always)]
    #[must_use]
    pub fn woo(&mut self) -> WOO_W<22> {
        WOO_W::new(self)
    }
    #[doc = "Bits 25:31 - Device Address for LPM tokens"]
    #[inline(always)]
    #[must_use]
    pub fn dev_add(&mut self) -> DEV_ADD_W<25> {
        DEV_ADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Status and Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portsc1](index.html) module"]
pub struct PORTSC1_SPEC;
impl crate::RegisterSpec for PORTSC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [portsc1::R](R) reader structure"]
impl crate::Readable for PORTSC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [portsc1::W](W) writer structure"]
impl crate::Writable for PORTSC1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x2a;
}
#[doc = "`reset()` method sets PORTSC1 to value 0"]
impl crate::Resettable for PORTSC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
