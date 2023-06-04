#[doc = "Register `MIPI_MEM_CTRL` reader"]
pub struct R(crate::R<MIPI_MEM_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIPI_MEM_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIPI_MEM_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIPI_MEM_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIPI_MEM_CTRL` writer"]
pub struct W(crate::W<MIPI_MEM_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIPI_MEM_CTRL_SPEC>;
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
impl From<crate::W<MIPI_MEM_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIPI_MEM_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_RIG` reader - Memory Read Input Gate - Blocks the read input signals to dual-port memory"]
pub type MEM_RIG_R = crate::BitReader<MEM_RIG_A>;
#[doc = "Memory Read Input Gate - Blocks the read input signals to dual-port memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEM_RIG_A {
    #[doc = "0: Enable"]
    MEM_RIG_0 = 0,
    #[doc = "1: Disable"]
    MEM_RIG_1 = 1,
}
impl From<MEM_RIG_A> for bool {
    #[inline(always)]
    fn from(variant: MEM_RIG_A) -> Self {
        variant as u8 != 0
    }
}
impl MEM_RIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEM_RIG_A {
        match self.bits {
            false => MEM_RIG_A::MEM_RIG_0,
            true => MEM_RIG_A::MEM_RIG_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEM_RIG_0`"]
    #[inline(always)]
    pub fn is_mem_rig_0(&self) -> bool {
        *self == MEM_RIG_A::MEM_RIG_0
    }
    #[doc = "Checks if the value of the field is `MEM_RIG_1`"]
    #[inline(always)]
    pub fn is_mem_rig_1(&self) -> bool {
        *self == MEM_RIG_A::MEM_RIG_1
    }
}
#[doc = "Field `MEM_RIG` writer - Memory Read Input Gate - Blocks the read input signals to dual-port memory"]
pub type MEM_RIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIPI_MEM_CTRL_SPEC, MEM_RIG_A, O>;
impl<'a, const O: u8> MEM_RIG_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn mem_rig_0(self) -> &'a mut W {
        self.variant(MEM_RIG_A::MEM_RIG_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn mem_rig_1(self) -> &'a mut W {
        self.variant(MEM_RIG_A::MEM_RIG_1)
    }
}
#[doc = "Field `MEM_WIG` reader - Memory Write Input Gate - Blocks the write input signals to dual-port memory"]
pub type MEM_WIG_R = crate::BitReader<MEM_WIG_A>;
#[doc = "Memory Write Input Gate - Blocks the write input signals to dual-port memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEM_WIG_A {
    #[doc = "0: Enable"]
    MEM_WIG_0 = 0,
    #[doc = "1: Disable"]
    MEM_WIG_1 = 1,
}
impl From<MEM_WIG_A> for bool {
    #[inline(always)]
    fn from(variant: MEM_WIG_A) -> Self {
        variant as u8 != 0
    }
}
impl MEM_WIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEM_WIG_A {
        match self.bits {
            false => MEM_WIG_A::MEM_WIG_0,
            true => MEM_WIG_A::MEM_WIG_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEM_WIG_0`"]
    #[inline(always)]
    pub fn is_mem_wig_0(&self) -> bool {
        *self == MEM_WIG_A::MEM_WIG_0
    }
    #[doc = "Checks if the value of the field is `MEM_WIG_1`"]
    #[inline(always)]
    pub fn is_mem_wig_1(&self) -> bool {
        *self == MEM_WIG_A::MEM_WIG_1
    }
}
#[doc = "Field `MEM_WIG` writer - Memory Write Input Gate - Blocks the write input signals to dual-port memory"]
pub type MEM_WIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIPI_MEM_CTRL_SPEC, MEM_WIG_A, O>;
impl<'a, const O: u8> MEM_WIG_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn mem_wig_0(self) -> &'a mut W {
        self.variant(MEM_WIG_A::MEM_WIG_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn mem_wig_1(self) -> &'a mut W {
        self.variant(MEM_WIG_A::MEM_WIG_1)
    }
}
#[doc = "Field `MEM_STDBY` reader - Memory Standby - Powers the driver to dual-port memory"]
pub type MEM_STDBY_R = crate::BitReader<MEM_STDBY_A>;
#[doc = "Memory Standby - Powers the driver to dual-port memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEM_STDBY_A {
    #[doc = "0: Enable"]
    MEM_STDBY_0 = 0,
    #[doc = "1: Disable"]
    MEM_STDBY_1 = 1,
}
impl From<MEM_STDBY_A> for bool {
    #[inline(always)]
    fn from(variant: MEM_STDBY_A) -> Self {
        variant as u8 != 0
    }
}
impl MEM_STDBY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEM_STDBY_A {
        match self.bits {
            false => MEM_STDBY_A::MEM_STDBY_0,
            true => MEM_STDBY_A::MEM_STDBY_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEM_STDBY_0`"]
    #[inline(always)]
    pub fn is_mem_stdby_0(&self) -> bool {
        *self == MEM_STDBY_A::MEM_STDBY_0
    }
    #[doc = "Checks if the value of the field is `MEM_STDBY_1`"]
    #[inline(always)]
    pub fn is_mem_stdby_1(&self) -> bool {
        *self == MEM_STDBY_A::MEM_STDBY_1
    }
}
#[doc = "Field `MEM_STDBY` writer - Memory Standby - Powers the driver to dual-port memory"]
pub type MEM_STDBY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MIPI_MEM_CTRL_SPEC, MEM_STDBY_A, O>;
impl<'a, const O: u8> MEM_STDBY_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn mem_stdby_0(self) -> &'a mut W {
        self.variant(MEM_STDBY_A::MEM_STDBY_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn mem_stdby_1(self) -> &'a mut W {
        self.variant(MEM_STDBY_A::MEM_STDBY_1)
    }
}
impl R {
    #[doc = "Bit 0 - Memory Read Input Gate - Blocks the read input signals to dual-port memory"]
    #[inline(always)]
    pub fn mem_rig(&self) -> MEM_RIG_R {
        MEM_RIG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Memory Write Input Gate - Blocks the write input signals to dual-port memory"]
    #[inline(always)]
    pub fn mem_wig(&self) -> MEM_WIG_R {
        MEM_WIG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Memory Standby - Powers the driver to dual-port memory"]
    #[inline(always)]
    pub fn mem_stdby(&self) -> MEM_STDBY_R {
        MEM_STDBY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Memory Read Input Gate - Blocks the read input signals to dual-port memory"]
    #[inline(always)]
    #[must_use]
    pub fn mem_rig(&mut self) -> MEM_RIG_W<0> {
        MEM_RIG_W::new(self)
    }
    #[doc = "Bit 1 - Memory Write Input Gate - Blocks the write input signals to dual-port memory"]
    #[inline(always)]
    #[must_use]
    pub fn mem_wig(&mut self) -> MEM_WIG_W<1> {
        MEM_WIG_W::new(self)
    }
    #[doc = "Bit 2 - Memory Standby - Powers the driver to dual-port memory"]
    #[inline(always)]
    #[must_use]
    pub fn mem_stdby(&mut self) -> MEM_STDBY_W<2> {
        MEM_STDBY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIPI Memory Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mipi_mem_ctrl](index.html) module"]
pub struct MIPI_MEM_CTRL_SPEC;
impl crate::RegisterSpec for MIPI_MEM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mipi_mem_ctrl::R](R) reader structure"]
impl crate::Readable for MIPI_MEM_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mipi_mem_ctrl::W](W) writer structure"]
impl crate::Writable for MIPI_MEM_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIPI_MEM_CTRL to value 0"]
impl crate::Resettable for MIPI_MEM_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
