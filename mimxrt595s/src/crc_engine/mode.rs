#[doc = "Register `MODE` reader"]
pub struct R(crate::R<MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE` writer"]
pub struct W(crate::W<MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_SPEC>;
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
impl From<crate::W<MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC_POLY` reader - CRC Polynomial"]
pub type CRC_POLY_R = crate::FieldReader<u8, CRC_POLY_A>;
#[doc = "CRC Polynomial\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CRC_POLY_A {
    #[doc = "0: Use CRC-CCITT polynomial"]
    CRCCCITT = 0,
    #[doc = "1: Use CRC-16 polynomial"]
    CRC16 = 1,
    #[doc = "2: Use CRC-32 polynomial"]
    CRC32 = 2,
}
impl From<CRC_POLY_A> for u8 {
    #[inline(always)]
    fn from(variant: CRC_POLY_A) -> Self {
        variant as _
    }
}
impl CRC_POLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CRC_POLY_A> {
        match self.bits {
            0 => Some(CRC_POLY_A::CRCCCITT),
            1 => Some(CRC_POLY_A::CRC16),
            2 => Some(CRC_POLY_A::CRC32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CRCCCITT`"]
    #[inline(always)]
    pub fn is_crcccitt(&self) -> bool {
        *self == CRC_POLY_A::CRCCCITT
    }
    #[doc = "Checks if the value of the field is `CRC16`"]
    #[inline(always)]
    pub fn is_crc16(&self) -> bool {
        *self == CRC_POLY_A::CRC16
    }
    #[doc = "Checks if the value of the field is `CRC32`"]
    #[inline(always)]
    pub fn is_crc32(&self) -> bool {
        *self == CRC_POLY_A::CRC32
    }
}
#[doc = "Field `CRC_POLY` writer - CRC Polynomial"]
pub type CRC_POLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE_SPEC, u8, CRC_POLY_A, 2, O>;
impl<'a, const O: u8> CRC_POLY_W<'a, O> {
    #[doc = "Use CRC-CCITT polynomial"]
    #[inline(always)]
    pub fn crcccitt(self) -> &'a mut W {
        self.variant(CRC_POLY_A::CRCCCITT)
    }
    #[doc = "Use CRC-16 polynomial"]
    #[inline(always)]
    pub fn crc16(self) -> &'a mut W {
        self.variant(CRC_POLY_A::CRC16)
    }
    #[doc = "Use CRC-32 polynomial"]
    #[inline(always)]
    pub fn crc32(self) -> &'a mut W {
        self.variant(CRC_POLY_A::CRC32)
    }
}
#[doc = "Field `BIT_RVS_WR` reader - Bit-order Reverse for Write Data"]
pub type BIT_RVS_WR_R = crate::BitReader<BIT_RVS_WR_A>;
#[doc = "Bit-order Reverse for Write Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIT_RVS_WR_A {
    #[doc = "0: Do not use bit-order reverse for WR_DATA (per byte)"]
    NOBITORDERREVERSEFORWRITEDATA = 0,
    #[doc = "1: Use bit-order reverse for WR_DATA (per byte)"]
    BITORDERREVERSEFORWRITEDATA = 1,
}
impl From<BIT_RVS_WR_A> for bool {
    #[inline(always)]
    fn from(variant: BIT_RVS_WR_A) -> Self {
        variant as u8 != 0
    }
}
impl BIT_RVS_WR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIT_RVS_WR_A {
        match self.bits {
            false => BIT_RVS_WR_A::NOBITORDERREVERSEFORWRITEDATA,
            true => BIT_RVS_WR_A::BITORDERREVERSEFORWRITEDATA,
        }
    }
    #[doc = "Checks if the value of the field is `NOBITORDERREVERSEFORWRITEDATA`"]
    #[inline(always)]
    pub fn is_nobitorderreverseforwritedata(&self) -> bool {
        *self == BIT_RVS_WR_A::NOBITORDERREVERSEFORWRITEDATA
    }
    #[doc = "Checks if the value of the field is `BITORDERREVERSEFORWRITEDATA`"]
    #[inline(always)]
    pub fn is_bitorderreverseforwritedata(&self) -> bool {
        *self == BIT_RVS_WR_A::BITORDERREVERSEFORWRITEDATA
    }
}
#[doc = "Field `BIT_RVS_WR` writer - Bit-order Reverse for Write Data"]
pub type BIT_RVS_WR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, BIT_RVS_WR_A, O>;
impl<'a, const O: u8> BIT_RVS_WR_W<'a, O> {
    #[doc = "Do not use bit-order reverse for WR_DATA (per byte)"]
    #[inline(always)]
    pub fn nobitorderreverseforwritedata(self) -> &'a mut W {
        self.variant(BIT_RVS_WR_A::NOBITORDERREVERSEFORWRITEDATA)
    }
    #[doc = "Use bit-order reverse for WR_DATA (per byte)"]
    #[inline(always)]
    pub fn bitorderreverseforwritedata(self) -> &'a mut W {
        self.variant(BIT_RVS_WR_A::BITORDERREVERSEFORWRITEDATA)
    }
}
#[doc = "Field `CMPL_WR` reader - 1's Complement for Write Data"]
pub type CMPL_WR_R = crate::BitReader<CMPL_WR_A>;
#[doc = "1's Complement for Write Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPL_WR_A {
    #[doc = "0: Do not use 1's complement for WR_DATA"]
    NOONESCOMPLEMENTFORWRITEDATA = 0,
    #[doc = "1: Use 1's complement for WR_DATA"]
    ONESCOMPLEMENTFORWRITEDATA = 1,
}
impl From<CMPL_WR_A> for bool {
    #[inline(always)]
    fn from(variant: CMPL_WR_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPL_WR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPL_WR_A {
        match self.bits {
            false => CMPL_WR_A::NOONESCOMPLEMENTFORWRITEDATA,
            true => CMPL_WR_A::ONESCOMPLEMENTFORWRITEDATA,
        }
    }
    #[doc = "Checks if the value of the field is `NOONESCOMPLEMENTFORWRITEDATA`"]
    #[inline(always)]
    pub fn is_noonescomplementforwritedata(&self) -> bool {
        *self == CMPL_WR_A::NOONESCOMPLEMENTFORWRITEDATA
    }
    #[doc = "Checks if the value of the field is `ONESCOMPLEMENTFORWRITEDATA`"]
    #[inline(always)]
    pub fn is_onescomplementforwritedata(&self) -> bool {
        *self == CMPL_WR_A::ONESCOMPLEMENTFORWRITEDATA
    }
}
#[doc = "Field `CMPL_WR` writer - 1's Complement for Write Data"]
pub type CMPL_WR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, CMPL_WR_A, O>;
impl<'a, const O: u8> CMPL_WR_W<'a, O> {
    #[doc = "Do not use 1's complement for WR_DATA"]
    #[inline(always)]
    pub fn noonescomplementforwritedata(self) -> &'a mut W {
        self.variant(CMPL_WR_A::NOONESCOMPLEMENTFORWRITEDATA)
    }
    #[doc = "Use 1's complement for WR_DATA"]
    #[inline(always)]
    pub fn onescomplementforwritedata(self) -> &'a mut W {
        self.variant(CMPL_WR_A::ONESCOMPLEMENTFORWRITEDATA)
    }
}
#[doc = "Field `BIT_RVS_SUM` reader - Bit-order Reverse for CRC Sum"]
pub type BIT_RVS_SUM_R = crate::BitReader<BIT_RVS_SUM_A>;
#[doc = "Bit-order Reverse for CRC Sum\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIT_RVS_SUM_A {
    #[doc = "0: Do not use bit-order reverse for CRC Sum"]
    NOBITORDERREVERSEFORCRCSUM = 0,
    #[doc = "1: Use bit-order reverse for CRC Sum"]
    BITORDERREVERSEFORCRCSUM = 1,
}
impl From<BIT_RVS_SUM_A> for bool {
    #[inline(always)]
    fn from(variant: BIT_RVS_SUM_A) -> Self {
        variant as u8 != 0
    }
}
impl BIT_RVS_SUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIT_RVS_SUM_A {
        match self.bits {
            false => BIT_RVS_SUM_A::NOBITORDERREVERSEFORCRCSUM,
            true => BIT_RVS_SUM_A::BITORDERREVERSEFORCRCSUM,
        }
    }
    #[doc = "Checks if the value of the field is `NOBITORDERREVERSEFORCRCSUM`"]
    #[inline(always)]
    pub fn is_nobitorderreverseforcrcsum(&self) -> bool {
        *self == BIT_RVS_SUM_A::NOBITORDERREVERSEFORCRCSUM
    }
    #[doc = "Checks if the value of the field is `BITORDERREVERSEFORCRCSUM`"]
    #[inline(always)]
    pub fn is_bitorderreverseforcrcsum(&self) -> bool {
        *self == BIT_RVS_SUM_A::BITORDERREVERSEFORCRCSUM
    }
}
#[doc = "Field `BIT_RVS_SUM` writer - Bit-order Reverse for CRC Sum"]
pub type BIT_RVS_SUM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, BIT_RVS_SUM_A, O>;
impl<'a, const O: u8> BIT_RVS_SUM_W<'a, O> {
    #[doc = "Do not use bit-order reverse for CRC Sum"]
    #[inline(always)]
    pub fn nobitorderreverseforcrcsum(self) -> &'a mut W {
        self.variant(BIT_RVS_SUM_A::NOBITORDERREVERSEFORCRCSUM)
    }
    #[doc = "Use bit-order reverse for CRC Sum"]
    #[inline(always)]
    pub fn bitorderreverseforcrcsum(self) -> &'a mut W {
        self.variant(BIT_RVS_SUM_A::BITORDERREVERSEFORCRCSUM)
    }
}
#[doc = "Field `CMPL_SUM` reader - 1's Complement for CRC Sum"]
pub type CMPL_SUM_R = crate::BitReader<CMPL_SUM_A>;
#[doc = "1's Complement for CRC Sum\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPL_SUM_A {
    #[doc = "0: Do not use 1's complement for CRC Sum"]
    NOTONESCOMPLEMENTFORCRCSUM = 0,
    #[doc = "1: Use 1's complement for CRC Sum"]
    ONESCOMPLEMENTFORCRCSUM = 1,
}
impl From<CMPL_SUM_A> for bool {
    #[inline(always)]
    fn from(variant: CMPL_SUM_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPL_SUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPL_SUM_A {
        match self.bits {
            false => CMPL_SUM_A::NOTONESCOMPLEMENTFORCRCSUM,
            true => CMPL_SUM_A::ONESCOMPLEMENTFORCRCSUM,
        }
    }
    #[doc = "Checks if the value of the field is `NOTONESCOMPLEMENTFORCRCSUM`"]
    #[inline(always)]
    pub fn is_notonescomplementforcrcsum(&self) -> bool {
        *self == CMPL_SUM_A::NOTONESCOMPLEMENTFORCRCSUM
    }
    #[doc = "Checks if the value of the field is `ONESCOMPLEMENTFORCRCSUM`"]
    #[inline(always)]
    pub fn is_onescomplementforcrcsum(&self) -> bool {
        *self == CMPL_SUM_A::ONESCOMPLEMENTFORCRCSUM
    }
}
#[doc = "Field `CMPL_SUM` writer - 1's Complement for CRC Sum"]
pub type CMPL_SUM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, CMPL_SUM_A, O>;
impl<'a, const O: u8> CMPL_SUM_W<'a, O> {
    #[doc = "Do not use 1's complement for CRC Sum"]
    #[inline(always)]
    pub fn notonescomplementforcrcsum(self) -> &'a mut W {
        self.variant(CMPL_SUM_A::NOTONESCOMPLEMENTFORCRCSUM)
    }
    #[doc = "Use 1's complement for CRC Sum"]
    #[inline(always)]
    pub fn onescomplementforcrcsum(self) -> &'a mut W {
        self.variant(CMPL_SUM_A::ONESCOMPLEMENTFORCRCSUM)
    }
}
impl R {
    #[doc = "Bits 0:1 - CRC Polynomial"]
    #[inline(always)]
    pub fn crc_poly(&self) -> CRC_POLY_R {
        CRC_POLY_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Bit-order Reverse for Write Data"]
    #[inline(always)]
    pub fn bit_rvs_wr(&self) -> BIT_RVS_WR_R {
        BIT_RVS_WR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1's Complement for Write Data"]
    #[inline(always)]
    pub fn cmpl_wr(&self) -> CMPL_WR_R {
        CMPL_WR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit-order Reverse for CRC Sum"]
    #[inline(always)]
    pub fn bit_rvs_sum(&self) -> BIT_RVS_SUM_R {
        BIT_RVS_SUM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1's Complement for CRC Sum"]
    #[inline(always)]
    pub fn cmpl_sum(&self) -> CMPL_SUM_R {
        CMPL_SUM_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CRC Polynomial"]
    #[inline(always)]
    #[must_use]
    pub fn crc_poly(&mut self) -> CRC_POLY_W<0> {
        CRC_POLY_W::new(self)
    }
    #[doc = "Bit 2 - Bit-order Reverse for Write Data"]
    #[inline(always)]
    #[must_use]
    pub fn bit_rvs_wr(&mut self) -> BIT_RVS_WR_W<2> {
        BIT_RVS_WR_W::new(self)
    }
    #[doc = "Bit 3 - 1's Complement for Write Data"]
    #[inline(always)]
    #[must_use]
    pub fn cmpl_wr(&mut self) -> CMPL_WR_W<3> {
        CMPL_WR_W::new(self)
    }
    #[doc = "Bit 4 - Bit-order Reverse for CRC Sum"]
    #[inline(always)]
    #[must_use]
    pub fn bit_rvs_sum(&mut self) -> BIT_RVS_SUM_W<4> {
        BIT_RVS_SUM_W::new(self)
    }
    #[doc = "Bit 5 - 1's Complement for CRC Sum"]
    #[inline(always)]
    #[must_use]
    pub fn cmpl_sum(&mut self) -> CMPL_SUM_W<5> {
        CMPL_SUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MODE Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode::R](R) reader structure"]
impl crate::Readable for MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode::W](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODE to value 0x0100_1000"]
impl crate::Resettable for MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_1000;
}
