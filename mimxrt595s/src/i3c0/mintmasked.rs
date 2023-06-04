#[doc = "Register `MINTMASKED` reader"]
pub struct R(crate::R<MINTMASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MINTMASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MINTMASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MINTMASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLVSTART` reader - SLVSTART interrupt mask"]
pub type SLVSTART_R = crate::BitReader<bool>;
#[doc = "Field `MCTRLDONE` reader - MCTRLDONE interrupt mask"]
pub type MCTRLDONE_R = crate::BitReader<bool>;
#[doc = "Field `COMPLETE` reader - COMPLETE interrupt mask"]
pub type COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `RXPEND` reader - RXPEND interrupt mask"]
pub type RXPEND_R = crate::BitReader<bool>;
#[doc = "Field `TXNOTFULL` reader - TXNOTFULL interrupt mask"]
pub type TXNOTFULL_R = crate::BitReader<bool>;
#[doc = "Field `IBIWON` reader - IBIWON interrupt mask"]
pub type IBIWON_R = crate::BitReader<bool>;
#[doc = "Field `ERRWARN` reader - ERRWARN interrupt mask"]
pub type ERRWARN_R = crate::BitReader<bool>;
#[doc = "Field `NOWMASTER` reader - NOWMASTER interrupt mask"]
pub type NOWMASTER_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 8 - SLVSTART interrupt mask"]
    #[inline(always)]
    pub fn slvstart(&self) -> SLVSTART_R {
        SLVSTART_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MCTRLDONE interrupt mask"]
    #[inline(always)]
    pub fn mctrldone(&self) -> MCTRLDONE_R {
        MCTRLDONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - COMPLETE interrupt mask"]
    #[inline(always)]
    pub fn complete(&self) -> COMPLETE_R {
        COMPLETE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RXPEND interrupt mask"]
    #[inline(always)]
    pub fn rxpend(&self) -> RXPEND_R {
        RXPEND_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TXNOTFULL interrupt mask"]
    #[inline(always)]
    pub fn txnotfull(&self) -> TXNOTFULL_R {
        TXNOTFULL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - IBIWON interrupt mask"]
    #[inline(always)]
    pub fn ibiwon(&self) -> IBIWON_R {
        IBIWON_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - ERRWARN interrupt mask"]
    #[inline(always)]
    pub fn errwarn(&self) -> ERRWARN_R {
        ERRWARN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - NOWMASTER interrupt mask"]
    #[inline(always)]
    pub fn nowmaster(&self) -> NOWMASTER_R {
        NOWMASTER_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Master Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mintmasked](index.html) module"]
pub struct MINTMASKED_SPEC;
impl crate::RegisterSpec for MINTMASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mintmasked::R](R) reader structure"]
impl crate::Readable for MINTMASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MINTMASKED to value 0x1000"]
impl crate::Resettable for MINTMASKED_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
