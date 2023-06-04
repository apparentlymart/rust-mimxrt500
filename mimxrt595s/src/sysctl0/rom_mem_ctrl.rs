#[doc = "Register `ROM_MEM_CTRL` reader"]
pub struct R(crate::R<ROM_MEM_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_MEM_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_MEM_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_MEM_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROM_MEM_CTRL` writer"]
pub struct W(crate::W<ROM_MEM_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_MEM_CTRL_SPEC>;
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
impl From<crate::W<ROM_MEM_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_MEM_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_IG` reader - Memory Input Gate - Blocks the write input signals to dual-port memory"]
pub type MEM_IG_R = crate::BitReader<MEM_IG_A>;
#[doc = "Memory Input Gate - Blocks the write input signals to dual-port memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEM_IG_A {
    #[doc = "0: Enable"]
    MEM_IG_0 = 0,
    #[doc = "1: Disable"]
    MEM_IG_1 = 1,
}
impl From<MEM_IG_A> for bool {
    #[inline(always)]
    fn from(variant: MEM_IG_A) -> Self {
        variant as u8 != 0
    }
}
impl MEM_IG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEM_IG_A {
        match self.bits {
            false => MEM_IG_A::MEM_IG_0,
            true => MEM_IG_A::MEM_IG_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEM_IG_0`"]
    #[inline(always)]
    pub fn is_mem_ig_0(&self) -> bool {
        *self == MEM_IG_A::MEM_IG_0
    }
    #[doc = "Checks if the value of the field is `MEM_IG_1`"]
    #[inline(always)]
    pub fn is_mem_ig_1(&self) -> bool {
        *self == MEM_IG_A::MEM_IG_1
    }
}
#[doc = "Field `MEM_IG` writer - Memory Input Gate - Blocks the write input signals to dual-port memory"]
pub type MEM_IG_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROM_MEM_CTRL_SPEC, MEM_IG_A, O>;
impl<'a, const O: u8> MEM_IG_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn mem_ig_0(self) -> &'a mut W {
        self.variant(MEM_IG_A::MEM_IG_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn mem_ig_1(self) -> &'a mut W {
        self.variant(MEM_IG_A::MEM_IG_1)
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
    crate::BitWriter<'a, u32, ROM_MEM_CTRL_SPEC, MEM_STDBY_A, O>;
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
#[doc = "Field `MEM_LS` reader - MEM LS"]
pub type MEM_LS_R = crate::BitReader<MEM_LS_A>;
#[doc = "MEM LS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEM_LS_A {
    #[doc = "0: Enable"]
    MEM_LS_0 = 0,
    #[doc = "1: Disable"]
    MEM_LS_1 = 1,
}
impl From<MEM_LS_A> for bool {
    #[inline(always)]
    fn from(variant: MEM_LS_A) -> Self {
        variant as u8 != 0
    }
}
impl MEM_LS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEM_LS_A {
        match self.bits {
            false => MEM_LS_A::MEM_LS_0,
            true => MEM_LS_A::MEM_LS_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEM_LS_0`"]
    #[inline(always)]
    pub fn is_mem_ls_0(&self) -> bool {
        *self == MEM_LS_A::MEM_LS_0
    }
    #[doc = "Checks if the value of the field is `MEM_LS_1`"]
    #[inline(always)]
    pub fn is_mem_ls_1(&self) -> bool {
        *self == MEM_LS_A::MEM_LS_1
    }
}
#[doc = "Field `MEM_LS` writer - MEM LS"]
pub type MEM_LS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROM_MEM_CTRL_SPEC, MEM_LS_A, O>;
impl<'a, const O: u8> MEM_LS_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn mem_ls_0(self) -> &'a mut W {
        self.variant(MEM_LS_A::MEM_LS_0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn mem_ls_1(self) -> &'a mut W {
        self.variant(MEM_LS_A::MEM_LS_1)
    }
}
impl R {
    #[doc = "Bit 0 - Memory Input Gate - Blocks the write input signals to dual-port memory"]
    #[inline(always)]
    pub fn mem_ig(&self) -> MEM_IG_R {
        MEM_IG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Memory Standby - Powers the driver to dual-port memory"]
    #[inline(always)]
    pub fn mem_stdby(&self) -> MEM_STDBY_R {
        MEM_STDBY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MEM LS"]
    #[inline(always)]
    pub fn mem_ls(&self) -> MEM_LS_R {
        MEM_LS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Memory Input Gate - Blocks the write input signals to dual-port memory"]
    #[inline(always)]
    #[must_use]
    pub fn mem_ig(&mut self) -> MEM_IG_W<0> {
        MEM_IG_W::new(self)
    }
    #[doc = "Bit 1 - Memory Standby - Powers the driver to dual-port memory"]
    #[inline(always)]
    #[must_use]
    pub fn mem_stdby(&mut self) -> MEM_STDBY_W<1> {
        MEM_STDBY_W::new(self)
    }
    #[doc = "Bit 2 - MEM LS"]
    #[inline(always)]
    #[must_use]
    pub fn mem_ls(&mut self) -> MEM_LS_W<2> {
        MEM_LS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ROM Memory Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_mem_ctrl](index.html) module"]
pub struct ROM_MEM_CTRL_SPEC;
impl crate::RegisterSpec for ROM_MEM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_mem_ctrl::R](R) reader structure"]
impl crate::Readable for ROM_MEM_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_mem_ctrl::W](W) writer structure"]
impl crate::Writable for ROM_MEM_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROM_MEM_CTRL to value 0"]
impl crate::Resettable for ROM_MEM_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
