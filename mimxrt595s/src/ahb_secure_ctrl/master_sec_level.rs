#[doc = "Register `MASTER_SEC_LEVEL` reader"]
pub struct R(crate::R<MASTER_SEC_LEVEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASTER_SEC_LEVEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASTER_SEC_LEVEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASTER_SEC_LEVEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASTER_SEC_LEVEL` writer"]
pub struct W(crate::W<MASTER_SEC_LEVEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASTER_SEC_LEVEL_SPEC>;
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
impl From<crate::W<MASTER_SEC_LEVEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASTER_SEC_LEVEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PQ` reader - Power Quad"]
pub type PQ_R = crate::FieldReader<u8, PQ_A>;
#[doc = "Power Quad\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PQ_A {
    #[doc = "0: Non-secure and non-privileged Master"]
    NONSECURE_NONPRIV_MASTER = 0,
    #[doc = "1: Non-secure and privileged Master"]
    NONSECURE_PRIV_MASTER = 1,
    #[doc = "2: Secure and non-privileged Master"]
    SECURE_NONPRIV_MASTER = 2,
    #[doc = "3: Secure and privileged Master"]
    SECURE_PRIV_MASTER = 3,
}
impl From<PQ_A> for u8 {
    #[inline(always)]
    fn from(variant: PQ_A) -> Self {
        variant as _
    }
}
impl PQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PQ_A {
        match self.bits {
            0 => PQ_A::NONSECURE_NONPRIV_MASTER,
            1 => PQ_A::NONSECURE_PRIV_MASTER,
            2 => PQ_A::SECURE_NONPRIV_MASTER,
            3 => PQ_A::SECURE_PRIV_MASTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_MASTER`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_master(&self) -> bool {
        *self == PQ_A::NONSECURE_NONPRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_MASTER`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_master(&self) -> bool {
        *self == PQ_A::NONSECURE_PRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_MASTER`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_master(&self) -> bool {
        *self == PQ_A::SECURE_NONPRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_MASTER`"]
    #[inline(always)]
    pub fn is_secure_priv_master(&self) -> bool {
        *self == PQ_A::SECURE_PRIV_MASTER
    }
}
#[doc = "Field `PQ` writer - Power Quad"]
pub type PQ_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MASTER_SEC_LEVEL_SPEC, u8, PQ_A, 2, O>;
impl<'a, const O: u8> PQ_W<'a, O> {
    #[doc = "Non-secure and non-privileged Master"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_master(self) -> &'a mut W {
        self.variant(PQ_A::NONSECURE_NONPRIV_MASTER)
    }
    #[doc = "Non-secure and privileged Master"]
    #[inline(always)]
    pub fn nonsecure_priv_master(self) -> &'a mut W {
        self.variant(PQ_A::NONSECURE_PRIV_MASTER)
    }
    #[doc = "Secure and non-privileged Master"]
    #[inline(always)]
    pub fn secure_nonpriv_master(self) -> &'a mut W {
        self.variant(PQ_A::SECURE_NONPRIV_MASTER)
    }
    #[doc = "Secure and privileged Master"]
    #[inline(always)]
    pub fn secure_priv_master(self) -> &'a mut W {
        self.variant(PQ_A::SECURE_PRIV_MASTER)
    }
}
#[doc = "Field `DSP` reader - DSP"]
pub type DSP_R = crate::FieldReader<u8, DSP_A>;
#[doc = "DSP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DSP_A {
    #[doc = "0: Non-secure and non-privileged Master"]
    NONSECURE_NONPRIV_MASTER = 0,
    #[doc = "1: Non-secure and privileged Master"]
    NONSECURE_PRIV_MASTER = 1,
    #[doc = "2: Secure and non-privileged Master"]
    SECURE_NONPRIV_MASTER = 2,
    #[doc = "3: Secure and privileged Master"]
    SECURE_PRIV_MASTER = 3,
}
impl From<DSP_A> for u8 {
    #[inline(always)]
    fn from(variant: DSP_A) -> Self {
        variant as _
    }
}
impl DSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_A {
        match self.bits {
            0 => DSP_A::NONSECURE_NONPRIV_MASTER,
            1 => DSP_A::NONSECURE_PRIV_MASTER,
            2 => DSP_A::SECURE_NONPRIV_MASTER,
            3 => DSP_A::SECURE_PRIV_MASTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_MASTER`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_master(&self) -> bool {
        *self == DSP_A::NONSECURE_NONPRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_MASTER`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_master(&self) -> bool {
        *self == DSP_A::NONSECURE_PRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_MASTER`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_master(&self) -> bool {
        *self == DSP_A::SECURE_NONPRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_MASTER`"]
    #[inline(always)]
    pub fn is_secure_priv_master(&self) -> bool {
        *self == DSP_A::SECURE_PRIV_MASTER
    }
}
#[doc = "Field `DSP` writer - DSP"]
pub type DSP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MASTER_SEC_LEVEL_SPEC, u8, DSP_A, 2, O>;
impl<'a, const O: u8> DSP_W<'a, O> {
    #[doc = "Non-secure and non-privileged Master"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_master(self) -> &'a mut W {
        self.variant(DSP_A::NONSECURE_NONPRIV_MASTER)
    }
    #[doc = "Non-secure and privileged Master"]
    #[inline(always)]
    pub fn nonsecure_priv_master(self) -> &'a mut W {
        self.variant(DSP_A::NONSECURE_PRIV_MASTER)
    }
    #[doc = "Secure and non-privileged Master"]
    #[inline(always)]
    pub fn secure_nonpriv_master(self) -> &'a mut W {
        self.variant(DSP_A::SECURE_NONPRIV_MASTER)
    }
    #[doc = "Secure and privileged Master"]
    #[inline(always)]
    pub fn secure_priv_master(self) -> &'a mut W {
        self.variant(DSP_A::SECURE_PRIV_MASTER)
    }
}
#[doc = "Field `DMA0` reader - DMA 0"]
pub type DMA0_R = crate::FieldReader<u8, DMA0_A>;
#[doc = "DMA 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA0_A {
    #[doc = "0: Non-secure and non-privileged Master"]
    NONSECURE_NONPRIV_MASTER = 0,
    #[doc = "1: Non-secure and privileged Master"]
    NONSECURE_PRIV_MASTER = 1,
    #[doc = "2: Secure and non-privileged Master"]
    SECURE_NONPRIV_MASTER = 2,
    #[doc = "3: Secure and privileged Master"]
    SECURE_PRIV_MASTER = 3,
}
impl From<DMA0_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA0_A) -> Self {
        variant as _
    }
}
impl DMA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA0_A {
        match self.bits {
            0 => DMA0_A::NONSECURE_NONPRIV_MASTER,
            1 => DMA0_A::NONSECURE_PRIV_MASTER,
            2 => DMA0_A::SECURE_NONPRIV_MASTER,
            3 => DMA0_A::SECURE_PRIV_MASTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_MASTER`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_master(&self) -> bool {
        *self == DMA0_A::NONSECURE_NONPRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_MASTER`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_master(&self) -> bool {
        *self == DMA0_A::NONSECURE_PRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_MASTER`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_master(&self) -> bool {
        *self == DMA0_A::SECURE_NONPRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_MASTER`"]
    #[inline(always)]
    pub fn is_secure_priv_master(&self) -> bool {
        *self == DMA0_A::SECURE_PRIV_MASTER
    }
}
#[doc = "Field `DMA0` writer - DMA 0"]
pub type DMA0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MASTER_SEC_LEVEL_SPEC, u8, DMA0_A, 2, O>;
impl<'a, const O: u8> DMA0_W<'a, O> {
    #[doc = "Non-secure and non-privileged Master"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_master(self) -> &'a mut W {
        self.variant(DMA0_A::NONSECURE_NONPRIV_MASTER)
    }
    #[doc = "Non-secure and privileged Master"]
    #[inline(always)]
    pub fn nonsecure_priv_master(self) -> &'a mut W {
        self.variant(DMA0_A::NONSECURE_PRIV_MASTER)
    }
    #[doc = "Secure and non-privileged Master"]
    #[inline(always)]
    pub fn secure_nonpriv_master(self) -> &'a mut W {
        self.variant(DMA0_A::SECURE_NONPRIV_MASTER)
    }
    #[doc = "Secure and privileged Master"]
    #[inline(always)]
    pub fn secure_priv_master(self) -> &'a mut W {
        self.variant(DMA0_A::SECURE_PRIV_MASTER)
    }
}
#[doc = "Field `DMA1` reader - DMA 1"]
pub type DMA1_R = crate::FieldReader<u8, DMA1_A>;
#[doc = "DMA 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA1_A {
    #[doc = "0: Non-secure and non-privileged Master"]
    NONSECURE_NONPRIV_MASTER = 0,
    #[doc = "1: Non-secure and privileged Master"]
    NONSECURE_PRIV_MASTER = 1,
    #[doc = "2: Secure and non-privileged Master"]
    SECURE_NONPRIV_MASTER = 2,
    #[doc = "3: Secure and privileged Master"]
    SECURE_PRIV_MASTER = 3,
}
impl From<DMA1_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA1_A) -> Self {
        variant as _
    }
}
impl DMA1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1_A {
        match self.bits {
            0 => DMA1_A::NONSECURE_NONPRIV_MASTER,
            1 => DMA1_A::NONSECURE_PRIV_MASTER,
            2 => DMA1_A::SECURE_NONPRIV_MASTER,
            3 => DMA1_A::SECURE_PRIV_MASTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_MASTER`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_master(&self) -> bool {
        *self == DMA1_A::NONSECURE_NONPRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_MASTER`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_master(&self) -> bool {
        *self == DMA1_A::NONSECURE_PRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_MASTER`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_master(&self) -> bool {
        *self == DMA1_A::SECURE_NONPRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_MASTER`"]
    #[inline(always)]
    pub fn is_secure_priv_master(&self) -> bool {
        *self == DMA1_A::SECURE_PRIV_MASTER
    }
}
#[doc = "Field `DMA1` writer - DMA 1"]
pub type DMA1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MASTER_SEC_LEVEL_SPEC, u8, DMA1_A, 2, O>;
impl<'a, const O: u8> DMA1_W<'a, O> {
    #[doc = "Non-secure and non-privileged Master"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_master(self) -> &'a mut W {
        self.variant(DMA1_A::NONSECURE_NONPRIV_MASTER)
    }
    #[doc = "Non-secure and privileged Master"]
    #[inline(always)]
    pub fn nonsecure_priv_master(self) -> &'a mut W {
        self.variant(DMA1_A::NONSECURE_PRIV_MASTER)
    }
    #[doc = "Secure and non-privileged Master"]
    #[inline(always)]
    pub fn secure_nonpriv_master(self) -> &'a mut W {
        self.variant(DMA1_A::SECURE_NONPRIV_MASTER)
    }
    #[doc = "Secure and privileged Master"]
    #[inline(always)]
    pub fn secure_priv_master(self) -> &'a mut W {
        self.variant(DMA1_A::SECURE_PRIV_MASTER)
    }
}
#[doc = "Field `SDMA_I` reader - Smart DMA (SDMA) Instruction"]
pub type SDMA_I_R = crate::FieldReader<u8, SDMA_I_A>;
#[doc = "Smart DMA (SDMA) Instruction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDMA_I_A {
    #[doc = "0: Non-secure and non-privileged Master"]
    NONSECURE_NONPRIV_MASTER = 0,
    #[doc = "1: Non-secure and privileged Master"]
    NONSECURE_PRIV_MASTER = 1,
    #[doc = "2: Secure and non-privileged Master"]
    SECURE_NONPRIV_MASTER = 2,
    #[doc = "3: Secure and privileged Master"]
    SECURE_PRIV_MASTER = 3,
}
impl From<SDMA_I_A> for u8 {
    #[inline(always)]
    fn from(variant: SDMA_I_A) -> Self {
        variant as _
    }
}
impl SDMA_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMA_I_A {
        match self.bits {
            0 => SDMA_I_A::NONSECURE_NONPRIV_MASTER,
            1 => SDMA_I_A::NONSECURE_PRIV_MASTER,
            2 => SDMA_I_A::SECURE_NONPRIV_MASTER,
            3 => SDMA_I_A::SECURE_PRIV_MASTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_MASTER`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_master(&self) -> bool {
        *self == SDMA_I_A::NONSECURE_NONPRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_MASTER`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_master(&self) -> bool {
        *self == SDMA_I_A::NONSECURE_PRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_MASTER`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_master(&self) -> bool {
        *self == SDMA_I_A::SECURE_NONPRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_MASTER`"]
    #[inline(always)]
    pub fn is_secure_priv_master(&self) -> bool {
        *self == SDMA_I_A::SECURE_PRIV_MASTER
    }
}
#[doc = "Field `SDMA_I` writer - Smart DMA (SDMA) Instruction"]
pub type SDMA_I_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MASTER_SEC_LEVEL_SPEC, u8, SDMA_I_A, 2, O>;
impl<'a, const O: u8> SDMA_I_W<'a, O> {
    #[doc = "Non-secure and non-privileged Master"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_master(self) -> &'a mut W {
        self.variant(SDMA_I_A::NONSECURE_NONPRIV_MASTER)
    }
    #[doc = "Non-secure and privileged Master"]
    #[inline(always)]
    pub fn nonsecure_priv_master(self) -> &'a mut W {
        self.variant(SDMA_I_A::NONSECURE_PRIV_MASTER)
    }
    #[doc = "Secure and non-privileged Master"]
    #[inline(always)]
    pub fn secure_nonpriv_master(self) -> &'a mut W {
        self.variant(SDMA_I_A::SECURE_NONPRIV_MASTER)
    }
    #[doc = "Secure and privileged Master"]
    #[inline(always)]
    pub fn secure_priv_master(self) -> &'a mut W {
        self.variant(SDMA_I_A::SECURE_PRIV_MASTER)
    }
}
#[doc = "Field `SDMA_D` reader - Smart DMA (SDMA) Data"]
pub type SDMA_D_R = crate::FieldReader<u8, SDMA_D_A>;
#[doc = "Smart DMA (SDMA) Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDMA_D_A {
    #[doc = "0: Non-secure and non-privileged Master"]
    NONSECURE_NONPRIV_MASTER = 0,
    #[doc = "1: Non-secure and privileged Master"]
    NONSECURE_PRIV_MASTER = 1,
    #[doc = "2: Secure and non-privileged Master"]
    SECURE_NONPRIV_MASTER = 2,
    #[doc = "3: Secure and privileged Master"]
    SECURE_PRIV_MASTER = 3,
}
impl From<SDMA_D_A> for u8 {
    #[inline(always)]
    fn from(variant: SDMA_D_A) -> Self {
        variant as _
    }
}
impl SDMA_D_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMA_D_A {
        match self.bits {
            0 => SDMA_D_A::NONSECURE_NONPRIV_MASTER,
            1 => SDMA_D_A::NONSECURE_PRIV_MASTER,
            2 => SDMA_D_A::SECURE_NONPRIV_MASTER,
            3 => SDMA_D_A::SECURE_PRIV_MASTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_MASTER`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_master(&self) -> bool {
        *self == SDMA_D_A::NONSECURE_NONPRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_MASTER`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_master(&self) -> bool {
        *self == SDMA_D_A::NONSECURE_PRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_MASTER`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_master(&self) -> bool {
        *self == SDMA_D_A::SECURE_NONPRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_MASTER`"]
    #[inline(always)]
    pub fn is_secure_priv_master(&self) -> bool {
        *self == SDMA_D_A::SECURE_PRIV_MASTER
    }
}
#[doc = "Field `SDMA_D` writer - Smart DMA (SDMA) Data"]
pub type SDMA_D_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MASTER_SEC_LEVEL_SPEC, u8, SDMA_D_A, 2, O>;
impl<'a, const O: u8> SDMA_D_W<'a, O> {
    #[doc = "Non-secure and non-privileged Master"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_master(self) -> &'a mut W {
        self.variant(SDMA_D_A::NONSECURE_NONPRIV_MASTER)
    }
    #[doc = "Non-secure and privileged Master"]
    #[inline(always)]
    pub fn nonsecure_priv_master(self) -> &'a mut W {
        self.variant(SDMA_D_A::NONSECURE_PRIV_MASTER)
    }
    #[doc = "Secure and non-privileged Master"]
    #[inline(always)]
    pub fn secure_nonpriv_master(self) -> &'a mut W {
        self.variant(SDMA_D_A::SECURE_NONPRIV_MASTER)
    }
    #[doc = "Secure and privileged Master"]
    #[inline(always)]
    pub fn secure_priv_master(self) -> &'a mut W {
        self.variant(SDMA_D_A::SECURE_PRIV_MASTER)
    }
}
#[doc = "Field `SDIO0` reader - SDIO 0"]
pub type SDIO0_R = crate::FieldReader<u8, SDIO0_A>;
#[doc = "SDIO 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDIO0_A {
    #[doc = "0: Non-secure and non-privileged Master"]
    NONSECURE_NONPRIV_MASTER = 0,
    #[doc = "1: Non-secure and privileged Master"]
    NONSECURE_PRIV_MASTER = 1,
    #[doc = "2: Secure and non-privileged Master"]
    SECURE_NONPRIV_MASTER = 2,
    #[doc = "3: Secure and privileged Master"]
    SECURE_PRIV_MASTER = 3,
}
impl From<SDIO0_A> for u8 {
    #[inline(always)]
    fn from(variant: SDIO0_A) -> Self {
        variant as _
    }
}
impl SDIO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIO0_A {
        match self.bits {
            0 => SDIO0_A::NONSECURE_NONPRIV_MASTER,
            1 => SDIO0_A::NONSECURE_PRIV_MASTER,
            2 => SDIO0_A::SECURE_NONPRIV_MASTER,
            3 => SDIO0_A::SECURE_PRIV_MASTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_MASTER`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_master(&self) -> bool {
        *self == SDIO0_A::NONSECURE_NONPRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_MASTER`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_master(&self) -> bool {
        *self == SDIO0_A::NONSECURE_PRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_MASTER`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_master(&self) -> bool {
        *self == SDIO0_A::SECURE_NONPRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_MASTER`"]
    #[inline(always)]
    pub fn is_secure_priv_master(&self) -> bool {
        *self == SDIO0_A::SECURE_PRIV_MASTER
    }
}
#[doc = "Field `SDIO0` writer - SDIO 0"]
pub type SDIO0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MASTER_SEC_LEVEL_SPEC, u8, SDIO0_A, 2, O>;
impl<'a, const O: u8> SDIO0_W<'a, O> {
    #[doc = "Non-secure and non-privileged Master"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_master(self) -> &'a mut W {
        self.variant(SDIO0_A::NONSECURE_NONPRIV_MASTER)
    }
    #[doc = "Non-secure and privileged Master"]
    #[inline(always)]
    pub fn nonsecure_priv_master(self) -> &'a mut W {
        self.variant(SDIO0_A::NONSECURE_PRIV_MASTER)
    }
    #[doc = "Secure and non-privileged Master"]
    #[inline(always)]
    pub fn secure_nonpriv_master(self) -> &'a mut W {
        self.variant(SDIO0_A::SECURE_NONPRIV_MASTER)
    }
    #[doc = "Secure and privileged Master"]
    #[inline(always)]
    pub fn secure_priv_master(self) -> &'a mut W {
        self.variant(SDIO0_A::SECURE_PRIV_MASTER)
    }
}
#[doc = "Field `SDIO1` reader - SDIO 1"]
pub type SDIO1_R = crate::FieldReader<u8, SDIO1_A>;
#[doc = "SDIO 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDIO1_A {
    #[doc = "0: Non-secure and non-privileged Master"]
    NONSECURE_NONPRIV_MASTER = 0,
    #[doc = "1: Non-secure and privileged Master"]
    NONSECURE_PRIV_MASTER = 1,
    #[doc = "2: Secure and non-privileged Master"]
    SECURE_NONPRIV_MASTER = 2,
    #[doc = "3: Secure and privileged Master"]
    SECURE_PRIV_MASTER = 3,
}
impl From<SDIO1_A> for u8 {
    #[inline(always)]
    fn from(variant: SDIO1_A) -> Self {
        variant as _
    }
}
impl SDIO1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIO1_A {
        match self.bits {
            0 => SDIO1_A::NONSECURE_NONPRIV_MASTER,
            1 => SDIO1_A::NONSECURE_PRIV_MASTER,
            2 => SDIO1_A::SECURE_NONPRIV_MASTER,
            3 => SDIO1_A::SECURE_PRIV_MASTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_MASTER`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_master(&self) -> bool {
        *self == SDIO1_A::NONSECURE_NONPRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_MASTER`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_master(&self) -> bool {
        *self == SDIO1_A::NONSECURE_PRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_MASTER`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_master(&self) -> bool {
        *self == SDIO1_A::SECURE_NONPRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_MASTER`"]
    #[inline(always)]
    pub fn is_secure_priv_master(&self) -> bool {
        *self == SDIO1_A::SECURE_PRIV_MASTER
    }
}
#[doc = "Field `SDIO1` writer - SDIO 1"]
pub type SDIO1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MASTER_SEC_LEVEL_SPEC, u8, SDIO1_A, 2, O>;
impl<'a, const O: u8> SDIO1_W<'a, O> {
    #[doc = "Non-secure and non-privileged Master"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_master(self) -> &'a mut W {
        self.variant(SDIO1_A::NONSECURE_NONPRIV_MASTER)
    }
    #[doc = "Non-secure and privileged Master"]
    #[inline(always)]
    pub fn nonsecure_priv_master(self) -> &'a mut W {
        self.variant(SDIO1_A::NONSECURE_PRIV_MASTER)
    }
    #[doc = "Secure and non-privileged Master"]
    #[inline(always)]
    pub fn secure_nonpriv_master(self) -> &'a mut W {
        self.variant(SDIO1_A::SECURE_NONPRIV_MASTER)
    }
    #[doc = "Secure and privileged Master"]
    #[inline(always)]
    pub fn secure_priv_master(self) -> &'a mut W {
        self.variant(SDIO1_A::SECURE_PRIV_MASTER)
    }
}
#[doc = "Field `GPU` reader - GPU"]
pub type GPU_R = crate::FieldReader<u8, GPU_A>;
#[doc = "GPU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPU_A {
    #[doc = "0: Non-secure and non-privileged Master"]
    NONSECURE_NONPRIV_MASTER = 0,
    #[doc = "1: Non-secure and privileged Master"]
    NONSECURE_PRIV_MASTER = 1,
    #[doc = "2: Secure and non-privileged Master"]
    SECURE_NONPRIV_MASTER = 2,
    #[doc = "3: Secure and privileged Master"]
    SECURE_PRIV_MASTER = 3,
}
impl From<GPU_A> for u8 {
    #[inline(always)]
    fn from(variant: GPU_A) -> Self {
        variant as _
    }
}
impl GPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPU_A {
        match self.bits {
            0 => GPU_A::NONSECURE_NONPRIV_MASTER,
            1 => GPU_A::NONSECURE_PRIV_MASTER,
            2 => GPU_A::SECURE_NONPRIV_MASTER,
            3 => GPU_A::SECURE_PRIV_MASTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE_NONPRIV_MASTER`"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_master(&self) -> bool {
        *self == GPU_A::NONSECURE_NONPRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `NONSECURE_PRIV_MASTER`"]
    #[inline(always)]
    pub fn is_nonsecure_priv_master(&self) -> bool {
        *self == GPU_A::NONSECURE_PRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `SECURE_NONPRIV_MASTER`"]
    #[inline(always)]
    pub fn is_secure_nonpriv_master(&self) -> bool {
        *self == GPU_A::SECURE_NONPRIV_MASTER
    }
    #[doc = "Checks if the value of the field is `SECURE_PRIV_MASTER`"]
    #[inline(always)]
    pub fn is_secure_priv_master(&self) -> bool {
        *self == GPU_A::SECURE_PRIV_MASTER
    }
}
#[doc = "Field `GPU` writer - GPU"]
pub type GPU_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MASTER_SEC_LEVEL_SPEC, u8, GPU_A, 2, O>;
impl<'a, const O: u8> GPU_W<'a, O> {
    #[doc = "Non-secure and non-privileged Master"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_master(self) -> &'a mut W {
        self.variant(GPU_A::NONSECURE_NONPRIV_MASTER)
    }
    #[doc = "Non-secure and privileged Master"]
    #[inline(always)]
    pub fn nonsecure_priv_master(self) -> &'a mut W {
        self.variant(GPU_A::NONSECURE_PRIV_MASTER)
    }
    #[doc = "Secure and non-privileged Master"]
    #[inline(always)]
    pub fn secure_nonpriv_master(self) -> &'a mut W {
        self.variant(GPU_A::SECURE_NONPRIV_MASTER)
    }
    #[doc = "Secure and privileged Master"]
    #[inline(always)]
    pub fn secure_priv_master(self) -> &'a mut W {
        self.variant(GPU_A::SECURE_PRIV_MASTER)
    }
}
#[doc = "Field `MASTER_SEC_LEVEL_LOCK` reader - Master Security Level Lock"]
pub type MASTER_SEC_LEVEL_LOCK_R = crate::FieldReader<u8, MASTER_SEC_LEVEL_LOCK_A>;
#[doc = "Master Security Level Lock\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MASTER_SEC_LEVEL_LOCK_A {
    #[doc = "1: Lock writing to this register, including these (MASTER_SEC_LEVEL_LOCK) bits"]
    MASTER_SEC_LEVEL_LOCK_CANNOT_BE_WRITTEN1 = 1,
    #[doc = "2: This register can be written"]
    MASTER_SEC_LEVEL_LOCK_CAN_BE_WRITTEN = 2,
}
impl From<MASTER_SEC_LEVEL_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: MASTER_SEC_LEVEL_LOCK_A) -> Self {
        variant as _
    }
}
impl MASTER_SEC_LEVEL_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MASTER_SEC_LEVEL_LOCK_A> {
        match self.bits {
            1 => Some(MASTER_SEC_LEVEL_LOCK_A::MASTER_SEC_LEVEL_LOCK_CANNOT_BE_WRITTEN1),
            2 => Some(MASTER_SEC_LEVEL_LOCK_A::MASTER_SEC_LEVEL_LOCK_CAN_BE_WRITTEN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MASTER_SEC_LEVEL_LOCK_CANNOT_BE_WRITTEN1`"]
    #[inline(always)]
    pub fn is_master_sec_level_lock_cannot_be_written1(&self) -> bool {
        *self == MASTER_SEC_LEVEL_LOCK_A::MASTER_SEC_LEVEL_LOCK_CANNOT_BE_WRITTEN1
    }
    #[doc = "Checks if the value of the field is `MASTER_SEC_LEVEL_LOCK_CAN_BE_WRITTEN`"]
    #[inline(always)]
    pub fn is_master_sec_level_lock_can_be_written(&self) -> bool {
        *self == MASTER_SEC_LEVEL_LOCK_A::MASTER_SEC_LEVEL_LOCK_CAN_BE_WRITTEN
    }
}
#[doc = "Field `MASTER_SEC_LEVEL_LOCK` writer - Master Security Level Lock"]
pub type MASTER_SEC_LEVEL_LOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MASTER_SEC_LEVEL_SPEC, u8, MASTER_SEC_LEVEL_LOCK_A, 2, O>;
impl<'a, const O: u8> MASTER_SEC_LEVEL_LOCK_W<'a, O> {
    #[doc = "Lock writing to this register, including these (MASTER_SEC_LEVEL_LOCK) bits"]
    #[inline(always)]
    pub fn master_sec_level_lock_cannot_be_written1(self) -> &'a mut W {
        self.variant(MASTER_SEC_LEVEL_LOCK_A::MASTER_SEC_LEVEL_LOCK_CANNOT_BE_WRITTEN1)
    }
    #[doc = "This register can be written"]
    #[inline(always)]
    pub fn master_sec_level_lock_can_be_written(self) -> &'a mut W {
        self.variant(MASTER_SEC_LEVEL_LOCK_A::MASTER_SEC_LEVEL_LOCK_CAN_BE_WRITTEN)
    }
}
impl R {
    #[doc = "Bits 4:5 - Power Quad"]
    #[inline(always)]
    pub fn pq(&self) -> PQ_R {
        PQ_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - DSP"]
    #[inline(always)]
    pub fn dsp(&self) -> DSP_R {
        DSP_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - DMA 0"]
    #[inline(always)]
    pub fn dma0(&self) -> DMA0_R {
        DMA0_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - DMA 1"]
    #[inline(always)]
    pub fn dma1(&self) -> DMA1_R {
        DMA1_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Smart DMA (SDMA) Instruction"]
    #[inline(always)]
    pub fn sdma_i(&self) -> SDMA_I_R {
        SDMA_I_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Smart DMA (SDMA) Data"]
    #[inline(always)]
    pub fn sdma_d(&self) -> SDMA_D_R {
        SDMA_D_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - SDIO 0"]
    #[inline(always)]
    pub fn sdio0(&self) -> SDIO0_R {
        SDIO0_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - SDIO 1"]
    #[inline(always)]
    pub fn sdio1(&self) -> SDIO1_R {
        SDIO1_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - GPU"]
    #[inline(always)]
    pub fn gpu(&self) -> GPU_R {
        GPU_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Master Security Level Lock"]
    #[inline(always)]
    pub fn master_sec_level_lock(&self) -> MASTER_SEC_LEVEL_LOCK_R {
        MASTER_SEC_LEVEL_LOCK_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - Power Quad"]
    #[inline(always)]
    #[must_use]
    pub fn pq(&mut self) -> PQ_W<4> {
        PQ_W::new(self)
    }
    #[doc = "Bits 6:7 - DSP"]
    #[inline(always)]
    #[must_use]
    pub fn dsp(&mut self) -> DSP_W<6> {
        DSP_W::new(self)
    }
    #[doc = "Bits 8:9 - DMA 0"]
    #[inline(always)]
    #[must_use]
    pub fn dma0(&mut self) -> DMA0_W<8> {
        DMA0_W::new(self)
    }
    #[doc = "Bits 10:11 - DMA 1"]
    #[inline(always)]
    #[must_use]
    pub fn dma1(&mut self) -> DMA1_W<10> {
        DMA1_W::new(self)
    }
    #[doc = "Bits 12:13 - Smart DMA (SDMA) Instruction"]
    #[inline(always)]
    #[must_use]
    pub fn sdma_i(&mut self) -> SDMA_I_W<12> {
        SDMA_I_W::new(self)
    }
    #[doc = "Bits 14:15 - Smart DMA (SDMA) Data"]
    #[inline(always)]
    #[must_use]
    pub fn sdma_d(&mut self) -> SDMA_D_W<14> {
        SDMA_D_W::new(self)
    }
    #[doc = "Bits 16:17 - SDIO 0"]
    #[inline(always)]
    #[must_use]
    pub fn sdio0(&mut self) -> SDIO0_W<16> {
        SDIO0_W::new(self)
    }
    #[doc = "Bits 18:19 - SDIO 1"]
    #[inline(always)]
    #[must_use]
    pub fn sdio1(&mut self) -> SDIO1_W<18> {
        SDIO1_W::new(self)
    }
    #[doc = "Bits 20:21 - GPU"]
    #[inline(always)]
    #[must_use]
    pub fn gpu(&mut self) -> GPU_W<20> {
        GPU_W::new(self)
    }
    #[doc = "Bits 30:31 - Master Security Level Lock"]
    #[inline(always)]
    #[must_use]
    pub fn master_sec_level_lock(&mut self) -> MASTER_SEC_LEVEL_LOCK_W<30> {
        MASTER_SEC_LEVEL_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Secure Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [master_sec_level](index.html) module"]
pub struct MASTER_SEC_LEVEL_SPEC;
impl crate::RegisterSpec for MASTER_SEC_LEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [master_sec_level::R](R) reader structure"]
impl crate::Readable for MASTER_SEC_LEVEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [master_sec_level::W](W) writer structure"]
impl crate::Writable for MASTER_SEC_LEVEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MASTER_SEC_LEVEL to value 0x8000_0000"]
impl crate::Resettable for MASTER_SEC_LEVEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
