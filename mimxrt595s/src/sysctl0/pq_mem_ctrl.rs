#[doc = "Register `PQ_MEM_CTRL` reader"]
pub struct R(crate::R<PQ_MEM_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PQ_MEM_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PQ_MEM_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PQ_MEM_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PQ_MEM_CTRL` writer"]
pub struct W(crate::W<PQ_MEM_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PQ_MEM_CTRL_SPEC>;
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
impl From<crate::W<PQ_MEM_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PQ_MEM_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM_IG` reader - SRAM Input Gate - Blocks the read input signals to dual-port memory"]
pub type SRAM_IG_R = crate::BitReader<SRAM_IG_A>;
#[doc = "SRAM Input Gate - Blocks the read input signals to dual-port memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_IG_A {
    #[doc = "0: Enable"]
    SRAM_IG_0 = 0,
    #[doc = "1: Disable"]
    SRAM_IG_1 = 1,
}
impl From<SRAM_IG_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_IG_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_IG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_IG_A {
        match self.bits {
            false => SRAM_IG_A::SRAM_IG_0,
            true => SRAM_IG_A::SRAM_IG_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM_IG_0`"]
    #[inline(always)]
    pub fn is_sram_ig_0(&self) -> bool {
        *self == SRAM_IG_A::SRAM_IG_0
    }
    #[doc = "Checks if the value of the field is `SRAM_IG_1`"]
    #[inline(always)]
    pub fn is_sram_ig_1(&self) -> bool {
        *self == SRAM_IG_A::SRAM_IG_1
    }
}
#[doc = "Field `SRAM_IG` writer - SRAM Input Gate - Blocks the read input signals to dual-port memory"]
pub type SRAM_IG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PQ_MEM_CTRL_SPEC, SRAM_IG_A, O>;
impl<'a, const O: u8> SRAM_IG_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram_ig_0(self) -> &'a mut W {
        self.variant(SRAM_IG_A::SRAM_IG_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram_ig_1(self) -> &'a mut W {
        self.variant(SRAM_IG_A::SRAM_IG_1)
    }
}
#[doc = "Field `SRAM_STDBY` reader - SRAM Standby - Powers the driver to dual-port memory"]
pub type SRAM_STDBY_R = crate::BitReader<SRAM_STDBY_A>;
#[doc = "SRAM Standby - Powers the driver to dual-port memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_STDBY_A {
    #[doc = "0: Enable"]
    SRAM_STDBY_0 = 0,
    #[doc = "1: Disable"]
    SRAM_STDBY_1 = 1,
}
impl From<SRAM_STDBY_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_STDBY_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_STDBY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_STDBY_A {
        match self.bits {
            false => SRAM_STDBY_A::SRAM_STDBY_0,
            true => SRAM_STDBY_A::SRAM_STDBY_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM_STDBY_0`"]
    #[inline(always)]
    pub fn is_sram_stdby_0(&self) -> bool {
        *self == SRAM_STDBY_A::SRAM_STDBY_0
    }
    #[doc = "Checks if the value of the field is `SRAM_STDBY_1`"]
    #[inline(always)]
    pub fn is_sram_stdby_1(&self) -> bool {
        *self == SRAM_STDBY_A::SRAM_STDBY_1
    }
}
#[doc = "Field `SRAM_STDBY` writer - SRAM Standby - Powers the driver to dual-port memory"]
pub type SRAM_STDBY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PQ_MEM_CTRL_SPEC, SRAM_STDBY_A, O>;
impl<'a, const O: u8> SRAM_STDBY_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn sram_stdby_0(self) -> &'a mut W {
        self.variant(SRAM_STDBY_A::SRAM_STDBY_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn sram_stdby_1(self) -> &'a mut W {
        self.variant(SRAM_STDBY_A::SRAM_STDBY_1)
    }
}
impl R {
    #[doc = "Bit 0 - SRAM Input Gate - Blocks the read input signals to dual-port memory"]
    #[inline(always)]
    pub fn sram_ig(&self) -> SRAM_IG_R {
        SRAM_IG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM Standby - Powers the driver to dual-port memory"]
    #[inline(always)]
    pub fn sram_stdby(&self) -> SRAM_STDBY_R {
        SRAM_STDBY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM Input Gate - Blocks the read input signals to dual-port memory"]
    #[inline(always)]
    #[must_use]
    pub fn sram_ig(&mut self) -> SRAM_IG_W<0> {
        SRAM_IG_W::new(self)
    }
    #[doc = "Bit 1 - SRAM Standby - Powers the driver to dual-port memory"]
    #[inline(always)]
    #[must_use]
    pub fn sram_stdby(&mut self) -> SRAM_STDBY_W<1> {
        SRAM_STDBY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power-Quad Memory Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pq_mem_ctrl](index.html) module"]
pub struct PQ_MEM_CTRL_SPEC;
impl crate::RegisterSpec for PQ_MEM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pq_mem_ctrl::R](R) reader structure"]
impl crate::Readable for PQ_MEM_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pq_mem_ctrl::W](W) writer structure"]
impl crate::Writable for PQ_MEM_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PQ_MEM_CTRL to value 0"]
impl crate::Resettable for PQ_MEM_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
