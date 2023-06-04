#[doc = "Register `SEC_VIO_MISC_INFO[%s]` reader"]
pub struct R(crate::R<SEC_VIO_MISC_INFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_VIO_MISC_INFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_VIO_MISC_INFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_VIO_MISC_INFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEC_VIO_INFO_WRITE` reader - Security violation access read/write indicator"]
pub type SEC_VIO_INFO_WRITE_R = crate::BitReader<SEC_VIO_INFO_WRITE_A>;
#[doc = "Security violation access read/write indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEC_VIO_INFO_WRITE_A {
    #[doc = "0: Read access"]
    READ = 0,
    #[doc = "1: Write access"]
    WRITE = 1,
}
impl From<SEC_VIO_INFO_WRITE_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_VIO_INFO_WRITE_A) -> Self {
        variant as u8 != 0
    }
}
impl SEC_VIO_INFO_WRITE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_VIO_INFO_WRITE_A {
        match self.bits {
            false => SEC_VIO_INFO_WRITE_A::READ,
            true => SEC_VIO_INFO_WRITE_A::WRITE,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == SEC_VIO_INFO_WRITE_A::READ
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == SEC_VIO_INFO_WRITE_A::WRITE
    }
}
#[doc = "Field `SEC_VIO_INFO_DATA_ACCESS` reader - Security Violation Info Data Access"]
pub type SEC_VIO_INFO_DATA_ACCESS_R = crate::BitReader<SEC_VIO_INFO_DATA_ACCESS_A>;
#[doc = "Security Violation Info Data Access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEC_VIO_INFO_DATA_ACCESS_A {
    #[doc = "0: Code"]
    CODE = 0,
    #[doc = "1: Data"]
    DATA = 1,
}
impl From<SEC_VIO_INFO_DATA_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_VIO_INFO_DATA_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
impl SEC_VIO_INFO_DATA_ACCESS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_VIO_INFO_DATA_ACCESS_A {
        match self.bits {
            false => SEC_VIO_INFO_DATA_ACCESS_A::CODE,
            true => SEC_VIO_INFO_DATA_ACCESS_A::DATA,
        }
    }
    #[doc = "Checks if the value of the field is `CODE`"]
    #[inline(always)]
    pub fn is_code(&self) -> bool {
        *self == SEC_VIO_INFO_DATA_ACCESS_A::CODE
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == SEC_VIO_INFO_DATA_ACCESS_A::DATA
    }
}
#[doc = "Field `SEC_VIO_INFO_MASTER_SEC_LEVEL` reader - Security Violation Info Master Security Level"]
pub type SEC_VIO_INFO_MASTER_SEC_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEC_VIO_INFO_MASTER` reader - Security violation master number"]
pub type SEC_VIO_INFO_MASTER_R = crate::FieldReader<u8, SEC_VIO_INFO_MASTER_A>;
#[doc = "Security violation master number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEC_VIO_INFO_MASTER_A {
    #[doc = "0: M33 Code"]
    M33_CODE = 0,
    #[doc = "1: M33 System"]
    M33_SYS = 1,
    #[doc = "2: Powerquad"]
    POWERQUAD = 2,
    #[doc = "3: DSP"]
    DSP = 3,
    #[doc = "4: DMA0"]
    DMA0 = 4,
    #[doc = "5: DMA1"]
    DMA1 = 5,
    #[doc = "6: SDMA Instruction"]
    SDMA_INSTR = 6,
    #[doc = "7: SDMA Data"]
    SDMA_DATA = 7,
    #[doc = "8: SDIO0"]
    SDIO0 = 8,
    #[doc = "9: SDIO1"]
    SDIO1 = 9,
    #[doc = "10: HASH"]
    HASH = 10,
    #[doc = "11: GPU"]
    GPU = 11,
}
impl From<SEC_VIO_INFO_MASTER_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_VIO_INFO_MASTER_A) -> Self {
        variant as _
    }
}
impl SEC_VIO_INFO_MASTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEC_VIO_INFO_MASTER_A> {
        match self.bits {
            0 => Some(SEC_VIO_INFO_MASTER_A::M33_CODE),
            1 => Some(SEC_VIO_INFO_MASTER_A::M33_SYS),
            2 => Some(SEC_VIO_INFO_MASTER_A::POWERQUAD),
            3 => Some(SEC_VIO_INFO_MASTER_A::DSP),
            4 => Some(SEC_VIO_INFO_MASTER_A::DMA0),
            5 => Some(SEC_VIO_INFO_MASTER_A::DMA1),
            6 => Some(SEC_VIO_INFO_MASTER_A::SDMA_INSTR),
            7 => Some(SEC_VIO_INFO_MASTER_A::SDMA_DATA),
            8 => Some(SEC_VIO_INFO_MASTER_A::SDIO0),
            9 => Some(SEC_VIO_INFO_MASTER_A::SDIO1),
            10 => Some(SEC_VIO_INFO_MASTER_A::HASH),
            11 => Some(SEC_VIO_INFO_MASTER_A::GPU),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `M33_CODE`"]
    #[inline(always)]
    pub fn is_m33_code(&self) -> bool {
        *self == SEC_VIO_INFO_MASTER_A::M33_CODE
    }
    #[doc = "Checks if the value of the field is `M33_SYS`"]
    #[inline(always)]
    pub fn is_m33_sys(&self) -> bool {
        *self == SEC_VIO_INFO_MASTER_A::M33_SYS
    }
    #[doc = "Checks if the value of the field is `POWERQUAD`"]
    #[inline(always)]
    pub fn is_powerquad(&self) -> bool {
        *self == SEC_VIO_INFO_MASTER_A::POWERQUAD
    }
    #[doc = "Checks if the value of the field is `DSP`"]
    #[inline(always)]
    pub fn is_dsp(&self) -> bool {
        *self == SEC_VIO_INFO_MASTER_A::DSP
    }
    #[doc = "Checks if the value of the field is `DMA0`"]
    #[inline(always)]
    pub fn is_dma0(&self) -> bool {
        *self == SEC_VIO_INFO_MASTER_A::DMA0
    }
    #[doc = "Checks if the value of the field is `DMA1`"]
    #[inline(always)]
    pub fn is_dma1(&self) -> bool {
        *self == SEC_VIO_INFO_MASTER_A::DMA1
    }
    #[doc = "Checks if the value of the field is `SDMA_INSTR`"]
    #[inline(always)]
    pub fn is_sdma_instr(&self) -> bool {
        *self == SEC_VIO_INFO_MASTER_A::SDMA_INSTR
    }
    #[doc = "Checks if the value of the field is `SDMA_DATA`"]
    #[inline(always)]
    pub fn is_sdma_data(&self) -> bool {
        *self == SEC_VIO_INFO_MASTER_A::SDMA_DATA
    }
    #[doc = "Checks if the value of the field is `SDIO0`"]
    #[inline(always)]
    pub fn is_sdio0(&self) -> bool {
        *self == SEC_VIO_INFO_MASTER_A::SDIO0
    }
    #[doc = "Checks if the value of the field is `SDIO1`"]
    #[inline(always)]
    pub fn is_sdio1(&self) -> bool {
        *self == SEC_VIO_INFO_MASTER_A::SDIO1
    }
    #[doc = "Checks if the value of the field is `HASH`"]
    #[inline(always)]
    pub fn is_hash(&self) -> bool {
        *self == SEC_VIO_INFO_MASTER_A::HASH
    }
    #[doc = "Checks if the value of the field is `GPU`"]
    #[inline(always)]
    pub fn is_gpu(&self) -> bool {
        *self == SEC_VIO_INFO_MASTER_A::GPU
    }
}
impl R {
    #[doc = "Bit 0 - Security violation access read/write indicator"]
    #[inline(always)]
    pub fn sec_vio_info_write(&self) -> SEC_VIO_INFO_WRITE_R {
        SEC_VIO_INFO_WRITE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Security Violation Info Data Access"]
    #[inline(always)]
    pub fn sec_vio_info_data_access(&self) -> SEC_VIO_INFO_DATA_ACCESS_R {
        SEC_VIO_INFO_DATA_ACCESS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Security Violation Info Master Security Level"]
    #[inline(always)]
    pub fn sec_vio_info_master_sec_level(&self) -> SEC_VIO_INFO_MASTER_SEC_LEVEL_R {
        SEC_VIO_INFO_MASTER_SEC_LEVEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Security violation master number"]
    #[inline(always)]
    pub fn sec_vio_info_master(&self) -> SEC_VIO_INFO_MASTER_R {
        SEC_VIO_INFO_MASTER_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "Security Violation Miscellaneous Information at Address(n) Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_vio_misc_info](index.html) module"]
pub struct SEC_VIO_MISC_INFO_SPEC;
impl crate::RegisterSpec for SEC_VIO_MISC_INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_vio_misc_info::R](R) reader structure"]
impl crate::Readable for SEC_VIO_MISC_INFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SEC_VIO_MISC_INFO[%s]
to value 0"]
impl crate::Resettable for SEC_VIO_MISC_INFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
