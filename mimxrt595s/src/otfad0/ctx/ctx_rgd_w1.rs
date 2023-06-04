#[doc = "Register `CTX_RGD_W1` reader"]
pub struct R(crate::R<CTX_RGD_W1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTX_RGD_W1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTX_RGD_W1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTX_RGD_W1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTX_RGD_W1` writer"]
pub struct W(crate::W<CTX_RGD_W1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTX_RGD_W1_SPEC>;
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
impl From<crate::W<CTX_RGD_W1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTX_RGD_W1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLD` reader - Valid"]
pub type VLD_R = crate::BitReader<VLD_A>;
#[doc = "Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VLD_A {
    #[doc = "0: Context is invalid."]
    NOT_VALID = 0,
    #[doc = "1: Context is valid."]
    VALID = 1,
}
impl From<VLD_A> for bool {
    #[inline(always)]
    fn from(variant: VLD_A) -> Self {
        variant as u8 != 0
    }
}
impl VLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLD_A {
        match self.bits {
            false => VLD_A::NOT_VALID,
            true => VLD_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == VLD_A::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VLD_A::VALID
    }
}
#[doc = "Field `VLD` writer - Valid"]
pub type VLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTX_RGD_W1_SPEC, VLD_A, O>;
impl<'a, const O: u8> VLD_W<'a, O> {
    #[doc = "Context is invalid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(VLD_A::NOT_VALID)
    }
    #[doc = "Context is valid."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(VLD_A::VALID)
    }
}
#[doc = "Field `ADE` reader - AES Decryption Enable."]
pub type ADE_R = crate::BitReader<ADE_A>;
#[doc = "AES Decryption Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADE_A {
    #[doc = "0: Bypass the fetched data."]
    BYPASS = 0,
    #[doc = "1: Perform the CTR-AES128 mode decryption on the fetched data."]
    DECRYPT = 1,
}
impl From<ADE_A> for bool {
    #[inline(always)]
    fn from(variant: ADE_A) -> Self {
        variant as u8 != 0
    }
}
impl ADE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADE_A {
        match self.bits {
            false => ADE_A::BYPASS,
            true => ADE_A::DECRYPT,
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == ADE_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `DECRYPT`"]
    #[inline(always)]
    pub fn is_decrypt(&self) -> bool {
        *self == ADE_A::DECRYPT
    }
}
#[doc = "Field `ADE` writer - AES Decryption Enable."]
pub type ADE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTX_RGD_W1_SPEC, ADE_A, O>;
impl<'a, const O: u8> ADE_W<'a, O> {
    #[doc = "Bypass the fetched data."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(ADE_A::BYPASS)
    }
    #[doc = "Perform the CTR-AES128 mode decryption on the fetched data."]
    #[inline(always)]
    pub fn decrypt(self) -> &'a mut W {
        self.variant(ADE_A::DECRYPT)
    }
}
#[doc = "Field `RO` reader - Read-Only"]
pub type RO_R = crate::BitReader<RO_A>;
#[doc = "Read-Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RO_A {
    #[doc = "0: The context registers can be accessed normally (as defined by SR\\[RRAM\\])."]
    NORMAL = 0,
    #[doc = "1: The context registers are read-only and accesses may be further restricted based on SR\\[RRAM\\]."]
    RESTRICT = 1,
}
impl From<RO_A> for bool {
    #[inline(always)]
    fn from(variant: RO_A) -> Self {
        variant as u8 != 0
    }
}
impl RO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RO_A {
        match self.bits {
            false => RO_A::NORMAL,
            true => RO_A::RESTRICT,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == RO_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `RESTRICT`"]
    #[inline(always)]
    pub fn is_restrict(&self) -> bool {
        *self == RO_A::RESTRICT
    }
}
#[doc = "Field `RO` writer - Read-Only"]
pub type RO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTX_RGD_W1_SPEC, RO_A, O>;
impl<'a, const O: u8> RO_W<'a, O> {
    #[doc = "The context registers can be accessed normally (as defined by SR\\[RRAM\\])."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(RO_A::NORMAL)
    }
    #[doc = "The context registers are read-only and accesses may be further restricted based on SR\\[RRAM\\]."]
    #[inline(always)]
    pub fn restrict(self) -> &'a mut W {
        self.variant(RO_A::RESTRICT)
    }
}
#[doc = "Field `ENDADDR` reader - End Address"]
pub type ENDADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ENDADDR` writer - End Address"]
pub type ENDADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTX_RGD_W1_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bit 0 - Valid"]
    #[inline(always)]
    pub fn vld(&self) -> VLD_R {
        VLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AES Decryption Enable."]
    #[inline(always)]
    pub fn ade(&self) -> ADE_R {
        ADE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read-Only"]
    #[inline(always)]
    pub fn ro(&self) -> RO_R {
        RO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 10:31 - End Address"]
    #[inline(always)]
    pub fn endaddr(&self) -> ENDADDR_R {
        ENDADDR_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Valid"]
    #[inline(always)]
    #[must_use]
    pub fn vld(&mut self) -> VLD_W<0> {
        VLD_W::new(self)
    }
    #[doc = "Bit 1 - AES Decryption Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ade(&mut self) -> ADE_W<1> {
        ADE_W::new(self)
    }
    #[doc = "Bit 2 - Read-Only"]
    #[inline(always)]
    #[must_use]
    pub fn ro(&mut self) -> RO_W<2> {
        RO_W::new(self)
    }
    #[doc = "Bits 10:31 - End Address"]
    #[inline(always)]
    #[must_use]
    pub fn endaddr(&mut self) -> ENDADDR_W<10> {
        ENDADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Region Descriptor Word1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctx_rgd_w1](index.html) module"]
pub struct CTX_RGD_W1_SPEC;
impl crate::RegisterSpec for CTX_RGD_W1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctx_rgd_w1::R](R) reader structure"]
impl crate::Readable for CTX_RGD_W1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctx_rgd_w1::W](W) writer structure"]
impl crate::Writable for CTX_RGD_W1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTX_RGD_W1 to value 0x03f8"]
impl crate::Resettable for CTX_RGD_W1_SPEC {
    const RESET_VALUE: Self::Ux = 0x03f8;
}
