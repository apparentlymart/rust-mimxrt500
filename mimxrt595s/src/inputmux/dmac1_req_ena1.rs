#[doc = "Register `DMAC1_REQ_ENA1` reader"]
pub struct R(crate::R<DMAC1_REQ_ENA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC1_REQ_ENA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC1_REQ_ENA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC1_REQ_ENA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC1_REQ_ENA1` writer"]
pub struct W(crate::W<DMAC1_REQ_ENA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC1_REQ_ENA1_SPEC>;
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
impl From<crate::W<DMAC1_REQ_ENA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC1_REQ_ENA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLEXCOMM11_RX` reader - FLEXCOMM11_RX"]
pub type FLEXCOMM11_RX_R = crate::BitReader<FLEXCOMM11_RX_A>;
#[doc = "FLEXCOMM11_RX\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM11_RX_A {
    #[doc = "0: Disable"]
    FLEXCOMM11_RX_0 = 0,
    #[doc = "1: Enable"]
    FLEXCOMM11_RX_1 = 1,
}
impl From<FLEXCOMM11_RX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM11_RX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM11_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM11_RX_A {
        match self.bits {
            false => FLEXCOMM11_RX_A::FLEXCOMM11_RX_0,
            true => FLEXCOMM11_RX_A::FLEXCOMM11_RX_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM11_RX_0`"]
    #[inline(always)]
    pub fn is_flexcomm11_rx_0(&self) -> bool {
        *self == FLEXCOMM11_RX_A::FLEXCOMM11_RX_0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM11_RX_1`"]
    #[inline(always)]
    pub fn is_flexcomm11_rx_1(&self) -> bool {
        *self == FLEXCOMM11_RX_A::FLEXCOMM11_RX_1
    }
}
#[doc = "Field `FLEXCOMM11_RX` writer - FLEXCOMM11_RX"]
pub type FLEXCOMM11_RX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC1_REQ_ENA1_SPEC, FLEXCOMM11_RX_A, O>;
impl<'a, const O: u8> FLEXCOMM11_RX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn flexcomm11_rx_0(self) -> &'a mut W {
        self.variant(FLEXCOMM11_RX_A::FLEXCOMM11_RX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn flexcomm11_rx_1(self) -> &'a mut W {
        self.variant(FLEXCOMM11_RX_A::FLEXCOMM11_RX_1)
    }
}
#[doc = "Field `FLEXCOMM11_TX` reader - FLEXCOMM11_TX"]
pub type FLEXCOMM11_TX_R = crate::BitReader<FLEXCOMM11_TX_A>;
#[doc = "FLEXCOMM11_TX\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM11_TX_A {
    #[doc = "0: Disable"]
    FLEXCOMM11_TX_0 = 0,
    #[doc = "1: Enable"]
    FLEXCOMM11_TX_1 = 1,
}
impl From<FLEXCOMM11_TX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM11_TX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM11_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM11_TX_A {
        match self.bits {
            false => FLEXCOMM11_TX_A::FLEXCOMM11_TX_0,
            true => FLEXCOMM11_TX_A::FLEXCOMM11_TX_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM11_TX_0`"]
    #[inline(always)]
    pub fn is_flexcomm11_tx_0(&self) -> bool {
        *self == FLEXCOMM11_TX_A::FLEXCOMM11_TX_0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM11_TX_1`"]
    #[inline(always)]
    pub fn is_flexcomm11_tx_1(&self) -> bool {
        *self == FLEXCOMM11_TX_A::FLEXCOMM11_TX_1
    }
}
#[doc = "Field `FLEXCOMM11_TX` writer - FLEXCOMM11_TX"]
pub type FLEXCOMM11_TX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC1_REQ_ENA1_SPEC, FLEXCOMM11_TX_A, O>;
impl<'a, const O: u8> FLEXCOMM11_TX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn flexcomm11_tx_0(self) -> &'a mut W {
        self.variant(FLEXCOMM11_TX_A::FLEXCOMM11_TX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn flexcomm11_tx_1(self) -> &'a mut W {
        self.variant(FLEXCOMM11_TX_A::FLEXCOMM11_TX_1)
    }
}
#[doc = "Field `FLEXCOMM12_RX` reader - FLEXCOMM12_RX"]
pub type FLEXCOMM12_RX_R = crate::BitReader<FLEXCOMM12_RX_A>;
#[doc = "FLEXCOMM12_RX\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM12_RX_A {
    #[doc = "0: Disable"]
    FLEXIO_SHFT2_FLEXCOMM12_RX_0 = 0,
    #[doc = "1: Enable"]
    FLEXIO_SHFT2_FLEXCOMM12_RX_1 = 1,
}
impl From<FLEXCOMM12_RX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM12_RX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM12_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM12_RX_A {
        match self.bits {
            false => FLEXCOMM12_RX_A::FLEXIO_SHFT2_FLEXCOMM12_RX_0,
            true => FLEXCOMM12_RX_A::FLEXIO_SHFT2_FLEXCOMM12_RX_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO_SHFT2_FLEXCOMM12_RX_0`"]
    #[inline(always)]
    pub fn is_flexio_shft2_flexcomm12_rx_0(&self) -> bool {
        *self == FLEXCOMM12_RX_A::FLEXIO_SHFT2_FLEXCOMM12_RX_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO_SHFT2_FLEXCOMM12_RX_1`"]
    #[inline(always)]
    pub fn is_flexio_shft2_flexcomm12_rx_1(&self) -> bool {
        *self == FLEXCOMM12_RX_A::FLEXIO_SHFT2_FLEXCOMM12_RX_1
    }
}
#[doc = "Field `FLEXCOMM12_RX` writer - FLEXCOMM12_RX"]
pub type FLEXCOMM12_RX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC1_REQ_ENA1_SPEC, FLEXCOMM12_RX_A, O>;
impl<'a, const O: u8> FLEXCOMM12_RX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn flexio_shft2_flexcomm12_rx_0(self) -> &'a mut W {
        self.variant(FLEXCOMM12_RX_A::FLEXIO_SHFT2_FLEXCOMM12_RX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn flexio_shft2_flexcomm12_rx_1(self) -> &'a mut W {
        self.variant(FLEXCOMM12_RX_A::FLEXIO_SHFT2_FLEXCOMM12_RX_1)
    }
}
#[doc = "Field `FLEXCOMM12_TX` reader - FLEXCOMM12_TX"]
pub type FLEXCOMM12_TX_R = crate::BitReader<FLEXCOMM12_TX_A>;
#[doc = "FLEXCOMM12_TX\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM12_TX_A {
    #[doc = "0: Disable"]
    FLEXCOMM12_TX_0 = 0,
    #[doc = "1: Enable"]
    FLEXCOMM12_TX_1 = 1,
}
impl From<FLEXCOMM12_TX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM12_TX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM12_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM12_TX_A {
        match self.bits {
            false => FLEXCOMM12_TX_A::FLEXCOMM12_TX_0,
            true => FLEXCOMM12_TX_A::FLEXCOMM12_TX_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM12_TX_0`"]
    #[inline(always)]
    pub fn is_flexcomm12_tx_0(&self) -> bool {
        *self == FLEXCOMM12_TX_A::FLEXCOMM12_TX_0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM12_TX_1`"]
    #[inline(always)]
    pub fn is_flexcomm12_tx_1(&self) -> bool {
        *self == FLEXCOMM12_TX_A::FLEXCOMM12_TX_1
    }
}
#[doc = "Field `FLEXCOMM12_TX` writer - FLEXCOMM12_TX"]
pub type FLEXCOMM12_TX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC1_REQ_ENA1_SPEC, FLEXCOMM12_TX_A, O>;
impl<'a, const O: u8> FLEXCOMM12_TX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn flexcomm12_tx_0(self) -> &'a mut W {
        self.variant(FLEXCOMM12_TX_A::FLEXCOMM12_TX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn flexcomm12_tx_1(self) -> &'a mut W {
        self.variant(FLEXCOMM12_TX_A::FLEXCOMM12_TX_1)
    }
}
#[doc = "Field `HASHCRYPT_IN` reader - HASHCRYPT_IN"]
pub type HASHCRYPT_IN_R = crate::BitReader<HASHCRYPT_IN_A>;
#[doc = "HASHCRYPT_IN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HASHCRYPT_IN_A {
    #[doc = "0: Disable"]
    HASHCRYPT_IN_0 = 0,
    #[doc = "1: Enable"]
    HASHCRYPT_IN_1 = 1,
}
impl From<HASHCRYPT_IN_A> for bool {
    #[inline(always)]
    fn from(variant: HASHCRYPT_IN_A) -> Self {
        variant as u8 != 0
    }
}
impl HASHCRYPT_IN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASHCRYPT_IN_A {
        match self.bits {
            false => HASHCRYPT_IN_A::HASHCRYPT_IN_0,
            true => HASHCRYPT_IN_A::HASHCRYPT_IN_1,
        }
    }
    #[doc = "Checks if the value of the field is `HASHCRYPT_IN_0`"]
    #[inline(always)]
    pub fn is_hashcrypt_in_0(&self) -> bool {
        *self == HASHCRYPT_IN_A::HASHCRYPT_IN_0
    }
    #[doc = "Checks if the value of the field is `HASHCRYPT_IN_1`"]
    #[inline(always)]
    pub fn is_hashcrypt_in_1(&self) -> bool {
        *self == HASHCRYPT_IN_A::HASHCRYPT_IN_1
    }
}
#[doc = "Field `HASHCRYPT_IN` writer - HASHCRYPT_IN"]
pub type HASHCRYPT_IN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC1_REQ_ENA1_SPEC, HASHCRYPT_IN_A, O>;
impl<'a, const O: u8> HASHCRYPT_IN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn hashcrypt_in_0(self) -> &'a mut W {
        self.variant(HASHCRYPT_IN_A::HASHCRYPT_IN_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn hashcrypt_in_1(self) -> &'a mut W {
        self.variant(HASHCRYPT_IN_A::HASHCRYPT_IN_1)
    }
}
impl R {
    #[doc = "Bit 0 - FLEXCOMM11_RX"]
    #[inline(always)]
    pub fn flexcomm11_rx(&self) -> FLEXCOMM11_RX_R {
        FLEXCOMM11_RX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLEXCOMM11_TX"]
    #[inline(always)]
    pub fn flexcomm11_tx(&self) -> FLEXCOMM11_TX_R {
        FLEXCOMM11_TX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FLEXCOMM12_RX"]
    #[inline(always)]
    pub fn flexcomm12_rx(&self) -> FLEXCOMM12_RX_R {
        FLEXCOMM12_RX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FLEXCOMM12_TX"]
    #[inline(always)]
    pub fn flexcomm12_tx(&self) -> FLEXCOMM12_TX_R {
        FLEXCOMM12_TX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HASHCRYPT_IN"]
    #[inline(always)]
    pub fn hashcrypt_in(&self) -> HASHCRYPT_IN_R {
        HASHCRYPT_IN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLEXCOMM11_RX"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm11_rx(&mut self) -> FLEXCOMM11_RX_W<0> {
        FLEXCOMM11_RX_W::new(self)
    }
    #[doc = "Bit 1 - FLEXCOMM11_TX"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm11_tx(&mut self) -> FLEXCOMM11_TX_W<1> {
        FLEXCOMM11_TX_W::new(self)
    }
    #[doc = "Bit 2 - FLEXCOMM12_RX"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm12_rx(&mut self) -> FLEXCOMM12_RX_W<2> {
        FLEXCOMM12_RX_W::new(self)
    }
    #[doc = "Bit 3 - FLEXCOMM12_TX"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm12_tx(&mut self) -> FLEXCOMM12_TX_W<3> {
        FLEXCOMM12_TX_W::new(self)
    }
    #[doc = "Bit 4 - HASHCRYPT_IN"]
    #[inline(always)]
    #[must_use]
    pub fn hashcrypt_in(&mut self) -> HASHCRYPT_IN_W<4> {
        HASHCRYPT_IN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC1 request enable 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac1_req_ena1](index.html) module"]
pub struct DMAC1_REQ_ENA1_SPEC;
impl crate::RegisterSpec for DMAC1_REQ_ENA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac1_req_ena1::R](R) reader structure"]
impl crate::Readable for DMAC1_REQ_ENA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac1_req_ena1::W](W) writer structure"]
impl crate::Writable for DMAC1_REQ_ENA1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAC1_REQ_ENA1 to value 0xffff_ffff"]
impl crate::Resettable for DMAC1_REQ_ENA1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
