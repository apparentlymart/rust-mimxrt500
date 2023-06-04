#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IPCMDDONEEN` reader - IP triggered Command Sequences Execution finished interrupt enable."]
pub type IPCMDDONEEN_R = crate::BitReader<IPCMDDONEEN_A>;
#[doc = "IP triggered Command Sequences Execution finished interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPCMDDONEEN_A {
    #[doc = "0: Disable interrupt or no impact"]
    VALUE0 = 0,
    #[doc = "1: Enable interrupt"]
    VALUE1 = 1,
}
impl From<IPCMDDONEEN_A> for bool {
    #[inline(always)]
    fn from(variant: IPCMDDONEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IPCMDDONEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPCMDDONEEN_A {
        match self.bits {
            false => IPCMDDONEEN_A::VALUE0,
            true => IPCMDDONEEN_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == IPCMDDONEEN_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IPCMDDONEEN_A::VALUE1
    }
}
#[doc = "Field `IPCMDDONEEN` writer - IP triggered Command Sequences Execution finished interrupt enable."]
pub type IPCMDDONEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, IPCMDDONEEN_A, O>;
impl<'a, const O: u8> IPCMDDONEEN_W<'a, O> {
    #[doc = "Disable interrupt or no impact"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(IPCMDDONEEN_A::VALUE0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IPCMDDONEEN_A::VALUE1)
    }
}
#[doc = "Field `IPCMDGEEN` reader - IP triggered Command Sequences Grant Timeout interrupt enable."]
pub type IPCMDGEEN_R = crate::BitReader<IPCMDGEEN_A>;
#[doc = "IP triggered Command Sequences Grant Timeout interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPCMDGEEN_A {
    #[doc = "0: Disable interrupt or no impact"]
    VALUE0 = 0,
    #[doc = "1: Enable interrupt"]
    VALUE1 = 1,
}
impl From<IPCMDGEEN_A> for bool {
    #[inline(always)]
    fn from(variant: IPCMDGEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IPCMDGEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPCMDGEEN_A {
        match self.bits {
            false => IPCMDGEEN_A::VALUE0,
            true => IPCMDGEEN_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == IPCMDGEEN_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IPCMDGEEN_A::VALUE1
    }
}
#[doc = "Field `IPCMDGEEN` writer - IP triggered Command Sequences Grant Timeout interrupt enable."]
pub type IPCMDGEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, IPCMDGEEN_A, O>;
impl<'a, const O: u8> IPCMDGEEN_W<'a, O> {
    #[doc = "Disable interrupt or no impact"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(IPCMDGEEN_A::VALUE0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IPCMDGEEN_A::VALUE1)
    }
}
#[doc = "Field `AHBCMDGEEN` reader - AHB triggered Command Sequences Grant Timeout interrupt enable."]
pub type AHBCMDGEEN_R = crate::BitReader<AHBCMDGEEN_A>;
#[doc = "AHB triggered Command Sequences Grant Timeout interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHBCMDGEEN_A {
    #[doc = "0: Disable interrupt or no impact"]
    VALUE0 = 0,
    #[doc = "1: Enable interrupt"]
    VALUE1 = 1,
}
impl From<AHBCMDGEEN_A> for bool {
    #[inline(always)]
    fn from(variant: AHBCMDGEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AHBCMDGEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHBCMDGEEN_A {
        match self.bits {
            false => AHBCMDGEEN_A::VALUE0,
            true => AHBCMDGEEN_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == AHBCMDGEEN_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AHBCMDGEEN_A::VALUE1
    }
}
#[doc = "Field `AHBCMDGEEN` writer - AHB triggered Command Sequences Grant Timeout interrupt enable."]
pub type AHBCMDGEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, AHBCMDGEEN_A, O>;
impl<'a, const O: u8> AHBCMDGEEN_W<'a, O> {
    #[doc = "Disable interrupt or no impact"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(AHBCMDGEEN_A::VALUE0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AHBCMDGEEN_A::VALUE1)
    }
}
#[doc = "Field `IPCMDERREN` reader - IP triggered Command Sequences Error Detected interrupt enable."]
pub type IPCMDERREN_R = crate::BitReader<IPCMDERREN_A>;
#[doc = "IP triggered Command Sequences Error Detected interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPCMDERREN_A {
    #[doc = "0: Disable interrupt or no impact"]
    VALUE0 = 0,
    #[doc = "1: Enable interrupt"]
    VALUE1 = 1,
}
impl From<IPCMDERREN_A> for bool {
    #[inline(always)]
    fn from(variant: IPCMDERREN_A) -> Self {
        variant as u8 != 0
    }
}
impl IPCMDERREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPCMDERREN_A {
        match self.bits {
            false => IPCMDERREN_A::VALUE0,
            true => IPCMDERREN_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == IPCMDERREN_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IPCMDERREN_A::VALUE1
    }
}
#[doc = "Field `IPCMDERREN` writer - IP triggered Command Sequences Error Detected interrupt enable."]
pub type IPCMDERREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, IPCMDERREN_A, O>;
impl<'a, const O: u8> IPCMDERREN_W<'a, O> {
    #[doc = "Disable interrupt or no impact"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(IPCMDERREN_A::VALUE0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IPCMDERREN_A::VALUE1)
    }
}
#[doc = "Field `AHBCMDERREN` reader - AHB triggered Command Sequences Error Detected interrupt enable."]
pub type AHBCMDERREN_R = crate::BitReader<AHBCMDERREN_A>;
#[doc = "AHB triggered Command Sequences Error Detected interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHBCMDERREN_A {
    #[doc = "0: Disable interrupt or no impact"]
    VALUE0 = 0,
    #[doc = "1: Enable interrupt"]
    VALUE1 = 1,
}
impl From<AHBCMDERREN_A> for bool {
    #[inline(always)]
    fn from(variant: AHBCMDERREN_A) -> Self {
        variant as u8 != 0
    }
}
impl AHBCMDERREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHBCMDERREN_A {
        match self.bits {
            false => AHBCMDERREN_A::VALUE0,
            true => AHBCMDERREN_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == AHBCMDERREN_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AHBCMDERREN_A::VALUE1
    }
}
#[doc = "Field `AHBCMDERREN` writer - AHB triggered Command Sequences Error Detected interrupt enable."]
pub type AHBCMDERREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, AHBCMDERREN_A, O>;
impl<'a, const O: u8> AHBCMDERREN_W<'a, O> {
    #[doc = "Disable interrupt or no impact"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(AHBCMDERREN_A::VALUE0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AHBCMDERREN_A::VALUE1)
    }
}
#[doc = "Field `IPRXWAEN` reader - IP RX FIFO WaterMark available interrupt enable."]
pub type IPRXWAEN_R = crate::BitReader<IPRXWAEN_A>;
#[doc = "IP RX FIFO WaterMark available interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPRXWAEN_A {
    #[doc = "0: Disable interrupt or no impact"]
    VALUE0 = 0,
    #[doc = "1: Enable interrupt"]
    VALUE1 = 1,
}
impl From<IPRXWAEN_A> for bool {
    #[inline(always)]
    fn from(variant: IPRXWAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IPRXWAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPRXWAEN_A {
        match self.bits {
            false => IPRXWAEN_A::VALUE0,
            true => IPRXWAEN_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == IPRXWAEN_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IPRXWAEN_A::VALUE1
    }
}
#[doc = "Field `IPRXWAEN` writer - IP RX FIFO WaterMark available interrupt enable."]
pub type IPRXWAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, IPRXWAEN_A, O>;
impl<'a, const O: u8> IPRXWAEN_W<'a, O> {
    #[doc = "Disable interrupt or no impact"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(IPRXWAEN_A::VALUE0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IPRXWAEN_A::VALUE1)
    }
}
#[doc = "Field `IPTXWEEN` reader - IP TX FIFO WaterMark empty interrupt enable."]
pub type IPTXWEEN_R = crate::BitReader<IPTXWEEN_A>;
#[doc = "IP TX FIFO WaterMark empty interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPTXWEEN_A {
    #[doc = "0: Disable interrupt or no impact"]
    VALUE0 = 0,
    #[doc = "1: Enable interrupt"]
    VALUE1 = 1,
}
impl From<IPTXWEEN_A> for bool {
    #[inline(always)]
    fn from(variant: IPTXWEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IPTXWEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPTXWEEN_A {
        match self.bits {
            false => IPTXWEEN_A::VALUE0,
            true => IPTXWEEN_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == IPTXWEEN_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IPTXWEEN_A::VALUE1
    }
}
#[doc = "Field `IPTXWEEN` writer - IP TX FIFO WaterMark empty interrupt enable."]
pub type IPTXWEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, IPTXWEEN_A, O>;
impl<'a, const O: u8> IPTXWEEN_W<'a, O> {
    #[doc = "Disable interrupt or no impact"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(IPTXWEEN_A::VALUE0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IPTXWEEN_A::VALUE1)
    }
}
#[doc = "Field `DATALEARNFAILEN` reader - Data Learning failed interrupt enable."]
pub type DATALEARNFAILEN_R = crate::BitReader<DATALEARNFAILEN_A>;
#[doc = "Data Learning failed interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATALEARNFAILEN_A {
    #[doc = "0: Disable interrupt or no impact"]
    VALUE0 = 0,
    #[doc = "1: Enable interrupt"]
    VALUE1 = 1,
}
impl From<DATALEARNFAILEN_A> for bool {
    #[inline(always)]
    fn from(variant: DATALEARNFAILEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DATALEARNFAILEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATALEARNFAILEN_A {
        match self.bits {
            false => DATALEARNFAILEN_A::VALUE0,
            true => DATALEARNFAILEN_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == DATALEARNFAILEN_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DATALEARNFAILEN_A::VALUE1
    }
}
#[doc = "Field `DATALEARNFAILEN` writer - Data Learning failed interrupt enable."]
pub type DATALEARNFAILEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTEN_SPEC, DATALEARNFAILEN_A, O>;
impl<'a, const O: u8> DATALEARNFAILEN_W<'a, O> {
    #[doc = "Disable interrupt or no impact"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(DATALEARNFAILEN_A::VALUE0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATALEARNFAILEN_A::VALUE1)
    }
}
#[doc = "Field `SCKSTOPBYRDEN` reader - SCLK is stopped during command sequence because Async RX FIFO full interrupt enable."]
pub type SCKSTOPBYRDEN_R = crate::BitReader<SCKSTOPBYRDEN_A>;
#[doc = "SCLK is stopped during command sequence because Async RX FIFO full interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCKSTOPBYRDEN_A {
    #[doc = "0: Disable interrupt or no impact"]
    VALUE0 = 0,
    #[doc = "1: Enable interrupt"]
    VALUE1 = 1,
}
impl From<SCKSTOPBYRDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCKSTOPBYRDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SCKSTOPBYRDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCKSTOPBYRDEN_A {
        match self.bits {
            false => SCKSTOPBYRDEN_A::VALUE0,
            true => SCKSTOPBYRDEN_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == SCKSTOPBYRDEN_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SCKSTOPBYRDEN_A::VALUE1
    }
}
#[doc = "Field `SCKSTOPBYRDEN` writer - SCLK is stopped during command sequence because Async RX FIFO full interrupt enable."]
pub type SCKSTOPBYRDEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTEN_SPEC, SCKSTOPBYRDEN_A, O>;
impl<'a, const O: u8> SCKSTOPBYRDEN_W<'a, O> {
    #[doc = "Disable interrupt or no impact"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(SCKSTOPBYRDEN_A::VALUE0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCKSTOPBYRDEN_A::VALUE1)
    }
}
#[doc = "Field `SCKSTOPBYWREN` reader - SCLK is stopped during command sequence because Async TX FIFO empty interrupt enable."]
pub type SCKSTOPBYWREN_R = crate::BitReader<SCKSTOPBYWREN_A>;
#[doc = "SCLK is stopped during command sequence because Async TX FIFO empty interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCKSTOPBYWREN_A {
    #[doc = "0: Disable interrupt or no impact"]
    VALUE0 = 0,
    #[doc = "1: Enable interrupt"]
    VALUE1 = 1,
}
impl From<SCKSTOPBYWREN_A> for bool {
    #[inline(always)]
    fn from(variant: SCKSTOPBYWREN_A) -> Self {
        variant as u8 != 0
    }
}
impl SCKSTOPBYWREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCKSTOPBYWREN_A {
        match self.bits {
            false => SCKSTOPBYWREN_A::VALUE0,
            true => SCKSTOPBYWREN_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == SCKSTOPBYWREN_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SCKSTOPBYWREN_A::VALUE1
    }
}
#[doc = "Field `SCKSTOPBYWREN` writer - SCLK is stopped during command sequence because Async TX FIFO empty interrupt enable."]
pub type SCKSTOPBYWREN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTEN_SPEC, SCKSTOPBYWREN_A, O>;
impl<'a, const O: u8> SCKSTOPBYWREN_W<'a, O> {
    #[doc = "Disable interrupt or no impact"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(SCKSTOPBYWREN_A::VALUE0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCKSTOPBYWREN_A::VALUE1)
    }
}
#[doc = "Field `AHBBUSERROREN` reader - AHB Bus error interrupt enable."]
pub type AHBBUSERROREN_R = crate::BitReader<AHBBUSERROREN_A>;
#[doc = "AHB Bus error interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHBBUSERROREN_A {
    #[doc = "0: Disable interrupt or no impact"]
    VALUE0 = 0,
    #[doc = "1: Enable interrupt"]
    VALUE1 = 1,
}
impl From<AHBBUSERROREN_A> for bool {
    #[inline(always)]
    fn from(variant: AHBBUSERROREN_A) -> Self {
        variant as u8 != 0
    }
}
impl AHBBUSERROREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHBBUSERROREN_A {
        match self.bits {
            false => AHBBUSERROREN_A::VALUE0,
            true => AHBBUSERROREN_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == AHBBUSERROREN_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AHBBUSERROREN_A::VALUE1
    }
}
#[doc = "Field `AHBBUSERROREN` writer - AHB Bus error interrupt enable."]
pub type AHBBUSERROREN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTEN_SPEC, AHBBUSERROREN_A, O>;
impl<'a, const O: u8> AHBBUSERROREN_W<'a, O> {
    #[doc = "Disable interrupt or no impact"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(AHBBUSERROREN_A::VALUE0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AHBBUSERROREN_A::VALUE1)
    }
}
#[doc = "Field `SEQTIMEOUTEN` reader - Sequence execution timeout interrupt enable."]
pub type SEQTIMEOUTEN_R = crate::BitReader<SEQTIMEOUTEN_A>;
#[doc = "Sequence execution timeout interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEQTIMEOUTEN_A {
    #[doc = "0: Disable interrupt or no impact"]
    VALUE0 = 0,
    #[doc = "1: Enable interrupt"]
    VALUE1 = 1,
}
impl From<SEQTIMEOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SEQTIMEOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SEQTIMEOUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQTIMEOUTEN_A {
        match self.bits {
            false => SEQTIMEOUTEN_A::VALUE0,
            true => SEQTIMEOUTEN_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == SEQTIMEOUTEN_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SEQTIMEOUTEN_A::VALUE1
    }
}
#[doc = "Field `SEQTIMEOUTEN` writer - Sequence execution timeout interrupt enable."]
pub type SEQTIMEOUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, SEQTIMEOUTEN_A, O>;
impl<'a, const O: u8> SEQTIMEOUTEN_W<'a, O> {
    #[doc = "Disable interrupt or no impact"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(SEQTIMEOUTEN_A::VALUE0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEQTIMEOUTEN_A::VALUE1)
    }
}
#[doc = "Field `KEYDONEEN` reader - OTFAD key blob processing done interrupt enable."]
pub type KEYDONEEN_R = crate::BitReader<KEYDONEEN_A>;
#[doc = "OTFAD key blob processing done interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KEYDONEEN_A {
    #[doc = "0: Disable interrupt or no impact"]
    VALUE0 = 0,
    #[doc = "1: Enable interrupt"]
    VALUE1 = 1,
}
impl From<KEYDONEEN_A> for bool {
    #[inline(always)]
    fn from(variant: KEYDONEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl KEYDONEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEYDONEEN_A {
        match self.bits {
            false => KEYDONEEN_A::VALUE0,
            true => KEYDONEEN_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == KEYDONEEN_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == KEYDONEEN_A::VALUE1
    }
}
#[doc = "Field `KEYDONEEN` writer - OTFAD key blob processing done interrupt enable."]
pub type KEYDONEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, KEYDONEEN_A, O>;
impl<'a, const O: u8> KEYDONEEN_W<'a, O> {
    #[doc = "Disable interrupt or no impact"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(KEYDONEEN_A::VALUE0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(KEYDONEEN_A::VALUE1)
    }
}
#[doc = "Field `KEYERROREN` reader - OTFAD key blob processing error interrupt enable."]
pub type KEYERROREN_R = crate::BitReader<KEYERROREN_A>;
#[doc = "OTFAD key blob processing error interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KEYERROREN_A {
    #[doc = "0: Disable interrupt or no impact"]
    VALUE0 = 0,
    #[doc = "1: Enable interrupt"]
    VALUE1 = 1,
}
impl From<KEYERROREN_A> for bool {
    #[inline(always)]
    fn from(variant: KEYERROREN_A) -> Self {
        variant as u8 != 0
    }
}
impl KEYERROREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEYERROREN_A {
        match self.bits {
            false => KEYERROREN_A::VALUE0,
            true => KEYERROREN_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == KEYERROREN_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == KEYERROREN_A::VALUE1
    }
}
#[doc = "Field `KEYERROREN` writer - OTFAD key blob processing error interrupt enable."]
pub type KEYERROREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, KEYERROREN_A, O>;
impl<'a, const O: u8> KEYERROREN_W<'a, O> {
    #[doc = "Disable interrupt or no impact"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(KEYERROREN_A::VALUE0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(KEYERROREN_A::VALUE1)
    }
}
impl R {
    #[doc = "Bit 0 - IP triggered Command Sequences Execution finished interrupt enable."]
    #[inline(always)]
    pub fn ipcmddoneen(&self) -> IPCMDDONEEN_R {
        IPCMDDONEEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IP triggered Command Sequences Grant Timeout interrupt enable."]
    #[inline(always)]
    pub fn ipcmdgeen(&self) -> IPCMDGEEN_R {
        IPCMDGEEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB triggered Command Sequences Grant Timeout interrupt enable."]
    #[inline(always)]
    pub fn ahbcmdgeen(&self) -> AHBCMDGEEN_R {
        AHBCMDGEEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IP triggered Command Sequences Error Detected interrupt enable."]
    #[inline(always)]
    pub fn ipcmderren(&self) -> IPCMDERREN_R {
        IPCMDERREN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB triggered Command Sequences Error Detected interrupt enable."]
    #[inline(always)]
    pub fn ahbcmderren(&self) -> AHBCMDERREN_R {
        AHBCMDERREN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IP RX FIFO WaterMark available interrupt enable."]
    #[inline(always)]
    pub fn iprxwaen(&self) -> IPRXWAEN_R {
        IPRXWAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IP TX FIFO WaterMark empty interrupt enable."]
    #[inline(always)]
    pub fn iptxween(&self) -> IPTXWEEN_R {
        IPTXWEEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data Learning failed interrupt enable."]
    #[inline(always)]
    pub fn datalearnfailen(&self) -> DATALEARNFAILEN_R {
        DATALEARNFAILEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCLK is stopped during command sequence because Async RX FIFO full interrupt enable."]
    #[inline(always)]
    pub fn sckstopbyrden(&self) -> SCKSTOPBYRDEN_R {
        SCKSTOPBYRDEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCLK is stopped during command sequence because Async TX FIFO empty interrupt enable."]
    #[inline(always)]
    pub fn sckstopbywren(&self) -> SCKSTOPBYWREN_R {
        SCKSTOPBYWREN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AHB Bus error interrupt enable."]
    #[inline(always)]
    pub fn ahbbuserroren(&self) -> AHBBUSERROREN_R {
        AHBBUSERROREN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Sequence execution timeout interrupt enable."]
    #[inline(always)]
    pub fn seqtimeouten(&self) -> SEQTIMEOUTEN_R {
        SEQTIMEOUTEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - OTFAD key blob processing done interrupt enable."]
    #[inline(always)]
    pub fn keydoneen(&self) -> KEYDONEEN_R {
        KEYDONEEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - OTFAD key blob processing error interrupt enable."]
    #[inline(always)]
    pub fn keyerroren(&self) -> KEYERROREN_R {
        KEYERROREN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IP triggered Command Sequences Execution finished interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn ipcmddoneen(&mut self) -> IPCMDDONEEN_W<0> {
        IPCMDDONEEN_W::new(self)
    }
    #[doc = "Bit 1 - IP triggered Command Sequences Grant Timeout interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn ipcmdgeen(&mut self) -> IPCMDGEEN_W<1> {
        IPCMDGEEN_W::new(self)
    }
    #[doc = "Bit 2 - AHB triggered Command Sequences Grant Timeout interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn ahbcmdgeen(&mut self) -> AHBCMDGEEN_W<2> {
        AHBCMDGEEN_W::new(self)
    }
    #[doc = "Bit 3 - IP triggered Command Sequences Error Detected interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn ipcmderren(&mut self) -> IPCMDERREN_W<3> {
        IPCMDERREN_W::new(self)
    }
    #[doc = "Bit 4 - AHB triggered Command Sequences Error Detected interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn ahbcmderren(&mut self) -> AHBCMDERREN_W<4> {
        AHBCMDERREN_W::new(self)
    }
    #[doc = "Bit 5 - IP RX FIFO WaterMark available interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn iprxwaen(&mut self) -> IPRXWAEN_W<5> {
        IPRXWAEN_W::new(self)
    }
    #[doc = "Bit 6 - IP TX FIFO WaterMark empty interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn iptxween(&mut self) -> IPTXWEEN_W<6> {
        IPTXWEEN_W::new(self)
    }
    #[doc = "Bit 7 - Data Learning failed interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn datalearnfailen(&mut self) -> DATALEARNFAILEN_W<7> {
        DATALEARNFAILEN_W::new(self)
    }
    #[doc = "Bit 8 - SCLK is stopped during command sequence because Async RX FIFO full interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn sckstopbyrden(&mut self) -> SCKSTOPBYRDEN_W<8> {
        SCKSTOPBYRDEN_W::new(self)
    }
    #[doc = "Bit 9 - SCLK is stopped during command sequence because Async TX FIFO empty interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn sckstopbywren(&mut self) -> SCKSTOPBYWREN_W<9> {
        SCKSTOPBYWREN_W::new(self)
    }
    #[doc = "Bit 10 - AHB Bus error interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn ahbbuserroren(&mut self) -> AHBBUSERROREN_W<10> {
        AHBBUSERROREN_W::new(self)
    }
    #[doc = "Bit 11 - Sequence execution timeout interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn seqtimeouten(&mut self) -> SEQTIMEOUTEN_W<11> {
        SEQTIMEOUTEN_W::new(self)
    }
    #[doc = "Bit 12 - OTFAD key blob processing done interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn keydoneen(&mut self) -> KEYDONEEN_W<12> {
        KEYDONEEN_W::new(self)
    }
    #[doc = "Bit 13 - OTFAD key blob processing error interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn keyerroren(&mut self) -> KEYERROREN_W<13> {
        KEYERROREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
