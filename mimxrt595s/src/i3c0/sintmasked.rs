#[doc = "Register `SINTMASKED` reader"]
pub struct R(crate::R<SINTMASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINTMASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SINTMASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SINTMASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `START` reader - START interrupt mask"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `MATCHED` reader - MATCHED interrupt mask"]
pub type MATCHED_R = crate::BitReader<bool>;
#[doc = "Field `STOP` reader - STOP interrupt mask"]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `RXPEND` reader - RXPEND interrupt mask"]
pub type RXPEND_R = crate::BitReader<bool>;
#[doc = "Field `TXSEND` reader - TXSEND interrupt mask"]
pub type TXSEND_R = crate::BitReader<bool>;
#[doc = "Field `DACHG` reader - DACHG interrupt mask"]
pub type DACHG_R = crate::BitReader<bool>;
#[doc = "Field `CCC` reader - CCC interrupt mask"]
pub type CCC_R = crate::BitReader<bool>;
#[doc = "Field `ERRWARN` reader - ERRWARN interrupt mask"]
pub type ERRWARN_R = crate::BitReader<bool>;
#[doc = "Field `DDRMATCHED` reader - DDRMATCHED interrupt mask"]
pub type DDRMATCHED_R = crate::BitReader<bool>;
#[doc = "Field `CHANDLED` reader - CHANDLED interrupt mask"]
pub type CHANDLED_R = crate::BitReader<bool>;
#[doc = "Field `EVENT` reader - EVENT interrupt mask"]
pub type EVENT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 8 - START interrupt mask"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MATCHED interrupt mask"]
    #[inline(always)]
    pub fn matched(&self) -> MATCHED_R {
        MATCHED_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - STOP interrupt mask"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RXPEND interrupt mask"]
    #[inline(always)]
    pub fn rxpend(&self) -> RXPEND_R {
        RXPEND_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TXSEND interrupt mask"]
    #[inline(always)]
    pub fn txsend(&self) -> TXSEND_R {
        TXSEND_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DACHG interrupt mask"]
    #[inline(always)]
    pub fn dachg(&self) -> DACHG_R {
        DACHG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CCC interrupt mask"]
    #[inline(always)]
    pub fn ccc(&self) -> CCC_R {
        CCC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ERRWARN interrupt mask"]
    #[inline(always)]
    pub fn errwarn(&self) -> ERRWARN_R {
        ERRWARN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DDRMATCHED interrupt mask"]
    #[inline(always)]
    pub fn ddrmatched(&self) -> DDRMATCHED_R {
        DDRMATCHED_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CHANDLED interrupt mask"]
    #[inline(always)]
    pub fn chandled(&self) -> CHANDLED_R {
        CHANDLED_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - EVENT interrupt mask"]
    #[inline(always)]
    pub fn event(&self) -> EVENT_R {
        EVENT_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[doc = "Slave Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sintmasked](index.html) module"]
pub struct SINTMASKED_SPEC;
impl crate::RegisterSpec for SINTMASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sintmasked::R](R) reader structure"]
impl crate::Readable for SINTMASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SINTMASKED to value 0x1000"]
impl crate::Resettable for SINTMASKED_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
