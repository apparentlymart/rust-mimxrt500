#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DUAL` reader - Reads 1 if 2 x 512 bit buffers, 0 if only 1 x 512 bit"]
pub type DUAL_R = crate::BitReader<bool>;
#[doc = "Field `DMA` reader - Reads 1 if DMA is connected"]
pub type DMA_R = crate::BitReader<bool>;
#[doc = "Field `AHB` reader - Reads 1 if AHB Master is enabled"]
pub type AHB_R = crate::BitReader<bool>;
#[doc = "Field `AES` reader - Reads 1 if AES 128 is included"]
pub type AES_R = crate::BitReader<bool>;
#[doc = "Field `AESKEY` reader - Reads 1 if AES 192 and 256 also included"]
pub type AESKEY_R = crate::BitReader<bool>;
#[doc = "Field `SECRET` reader - Reads 1 if AES Secret key is available"]
pub type SECRET_R = crate::BitReader<bool>;
#[doc = "Field `ICB` reader - ICB"]
pub type ICB_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Reads 1 if 2 x 512 bit buffers, 0 if only 1 x 512 bit"]
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reads 1 if DMA is connected"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Reads 1 if AHB Master is enabled"]
    #[inline(always)]
    pub fn ahb(&self) -> AHB_R {
        AHB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Reads 1 if AES 128 is included"]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reads 1 if AES 192 and 256 also included"]
    #[inline(always)]
    pub fn aeskey(&self) -> AESKEY_R {
        AESKEY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reads 1 if AES Secret key is available"]
    #[inline(always)]
    pub fn secret(&self) -> SECRET_R {
        SECRET_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - ICB"]
    #[inline(always)]
    pub fn icb(&self) -> ICB_R {
        ICB_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CONFIG to value 0x09cb"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x09cb;
}
