#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Operational Mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Operational Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: SHA1 is enabled"]
    SHA1 = 1,
    #[doc = "2: SHA2-256 is enabled"]
    SHA2_256 = 2,
    #[doc = "4: AES is enabled (see also CRYPTCFG register for more controls)"]
    AES = 4,
    #[doc = "5: ICB-AES is enabled (see also CRYPTCFG register for more controls)"]
    ICB_AES = 5,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::DISABLED),
            1 => Some(MODE_A::SHA1),
            2 => Some(MODE_A::SHA2_256),
            4 => Some(MODE_A::AES),
            5 => Some(MODE_A::ICB_AES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `SHA1`"]
    #[inline(always)]
    pub fn is_sha1(&self) -> bool {
        *self == MODE_A::SHA1
    }
    #[doc = "Checks if the value of the field is `SHA2_256`"]
    #[inline(always)]
    pub fn is_sha2_256(&self) -> bool {
        *self == MODE_A::SHA2_256
    }
    #[doc = "Checks if the value of the field is `AES`"]
    #[inline(always)]
    pub fn is_aes(&self) -> bool {
        *self == MODE_A::AES
    }
    #[doc = "Checks if the value of the field is `ICB_AES`"]
    #[inline(always)]
    pub fn is_icb_aes(&self) -> bool {
        *self == MODE_A::ICB_AES
    }
}
#[doc = "Field `MODE` writer - Operational Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, MODE_A, 3, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE_A::DISABLED)
    }
    #[doc = "SHA1 is enabled"]
    #[inline(always)]
    pub fn sha1(self) -> &'a mut W {
        self.variant(MODE_A::SHA1)
    }
    #[doc = "SHA2-256 is enabled"]
    #[inline(always)]
    pub fn sha2_256(self) -> &'a mut W {
        self.variant(MODE_A::SHA2_256)
    }
    #[doc = "AES is enabled (see also CRYPTCFG register for more controls)"]
    #[inline(always)]
    pub fn aes(self) -> &'a mut W {
        self.variant(MODE_A::AES)
    }
    #[doc = "ICB-AES is enabled (see also CRYPTCFG register for more controls)"]
    #[inline(always)]
    pub fn icb_aes(self) -> &'a mut W {
        self.variant(MODE_A::ICB_AES)
    }
}
#[doc = "Field `NEW_HASH` reader - New Hash Operation"]
pub type NEW_HASH_R = crate::BitReader<NEW_HASH_A>;
#[doc = "New Hash Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NEW_HASH_A {
    #[doc = "1: Starts a new Hash/Crypto and initializes the Digest/Result."]
    START = 1,
}
impl From<NEW_HASH_A> for bool {
    #[inline(always)]
    fn from(variant: NEW_HASH_A) -> Self {
        variant as u8 != 0
    }
}
impl NEW_HASH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NEW_HASH_A> {
        match self.bits {
            true => Some(NEW_HASH_A::START),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == NEW_HASH_A::START
    }
}
#[doc = "Field `NEW_HASH` writer - New Hash Operation"]
pub type NEW_HASH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, NEW_HASH_A, O>;
impl<'a, const O: u8> NEW_HASH_W<'a, O> {
    #[doc = "Starts a new Hash/Crypto and initializes the Digest/Result."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(NEW_HASH_A::START)
    }
}
#[doc = "Field `RELOAD` reader - Reload"]
pub type RELOAD_R = crate::BitReader<RELOAD_A>;
#[doc = "Reload\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RELOAD_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Allows the SHA RELOAD registers to be used."]
    ENABLED = 1,
}
impl From<RELOAD_A> for bool {
    #[inline(always)]
    fn from(variant: RELOAD_A) -> Self {
        variant as u8 != 0
    }
}
impl RELOAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RELOAD_A {
        match self.bits {
            false => RELOAD_A::DISABLED,
            true => RELOAD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RELOAD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RELOAD_A::ENABLED
    }
}
#[doc = "Field `RELOAD` writer - Reload"]
pub type RELOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, RELOAD_A, O>;
impl<'a, const O: u8> RELOAD_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RELOAD_A::DISABLED)
    }
    #[doc = "Allows the SHA RELOAD registers to be used."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RELOAD_A::ENABLED)
    }
}
#[doc = "Field `DMA_I` reader - DMA to Fill INDATA."]
pub type DMA_I_R = crate::BitReader<DMA_I_A>;
#[doc = "DMA to Fill INDATA.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_I_A {
    #[doc = "0: DMA is not used. Processor writes the necessary words when WAITING is set (interrupts), unless AHB Master is used."]
    NOT_USED = 0,
    #[doc = "1: DMA will push in the data."]
    PUSH = 1,
}
impl From<DMA_I_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_I_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_I_A {
        match self.bits {
            false => DMA_I_A::NOT_USED,
            true => DMA_I_A::PUSH,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_USED`"]
    #[inline(always)]
    pub fn is_not_used(&self) -> bool {
        *self == DMA_I_A::NOT_USED
    }
    #[doc = "Checks if the value of the field is `PUSH`"]
    #[inline(always)]
    pub fn is_push(&self) -> bool {
        *self == DMA_I_A::PUSH
    }
}
#[doc = "Field `DMA_I` writer - DMA to Fill INDATA."]
pub type DMA_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, DMA_I_A, O>;
impl<'a, const O: u8> DMA_I_W<'a, O> {
    #[doc = "DMA is not used. Processor writes the necessary words when WAITING is set (interrupts), unless AHB Master is used."]
    #[inline(always)]
    pub fn not_used(self) -> &'a mut W {
        self.variant(DMA_I_A::NOT_USED)
    }
    #[doc = "DMA will push in the data."]
    #[inline(always)]
    pub fn push(self) -> &'a mut W {
        self.variant(DMA_I_A::PUSH)
    }
}
#[doc = "Field `DMA_O` reader - DMA to Drain the Digest/Output"]
pub type DMA_O_R = crate::BitReader<DMA_O_A>;
#[doc = "DMA to Drain the Digest/Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_O_A {
    #[doc = "0: DMA is not used. Processor reads the digest/output in response to DIGEST interrupt."]
    NOTUSED = 0,
    #[doc = "1: DMA will drain the data."]
    PUSH = 1,
}
impl From<DMA_O_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_O_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_O_A {
        match self.bits {
            false => DMA_O_A::NOTUSED,
            true => DMA_O_A::PUSH,
        }
    }
    #[doc = "Checks if the value of the field is `NOTUSED`"]
    #[inline(always)]
    pub fn is_notused(&self) -> bool {
        *self == DMA_O_A::NOTUSED
    }
    #[doc = "Checks if the value of the field is `PUSH`"]
    #[inline(always)]
    pub fn is_push(&self) -> bool {
        *self == DMA_O_A::PUSH
    }
}
#[doc = "Field `DMA_O` writer - DMA to Drain the Digest/Output"]
pub type DMA_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, DMA_O_A, O>;
impl<'a, const O: u8> DMA_O_W<'a, O> {
    #[doc = "DMA is not used. Processor reads the digest/output in response to DIGEST interrupt."]
    #[inline(always)]
    pub fn notused(self) -> &'a mut W {
        self.variant(DMA_O_A::NOTUSED)
    }
    #[doc = "DMA will drain the data."]
    #[inline(always)]
    pub fn push(self) -> &'a mut W {
        self.variant(DMA_O_A::PUSH)
    }
}
#[doc = "Field `HASHSWPB` reader - Hash Swap Bytes"]
pub type HASHSWPB_R = crate::BitReader<bool>;
#[doc = "Field `HASHSWPB` writer - Hash Swap Bytes"]
pub type HASHSWPB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Operational Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - New Hash Operation"]
    #[inline(always)]
    pub fn new_hash(&self) -> NEW_HASH_R {
        NEW_HASH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reload"]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA to Fill INDATA."]
    #[inline(always)]
    pub fn dma_i(&self) -> DMA_I_R {
        DMA_I_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA to Drain the Digest/Output"]
    #[inline(always)]
    pub fn dma_o(&self) -> DMA_O_R {
        DMA_O_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Hash Swap Bytes"]
    #[inline(always)]
    pub fn hashswpb(&self) -> HASHSWPB_R {
        HASHSWPB_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Operational Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 4 - New Hash Operation"]
    #[inline(always)]
    #[must_use]
    pub fn new_hash(&mut self) -> NEW_HASH_W<4> {
        NEW_HASH_W::new(self)
    }
    #[doc = "Bit 5 - Reload"]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> RELOAD_W<5> {
        RELOAD_W::new(self)
    }
    #[doc = "Bit 8 - DMA to Fill INDATA."]
    #[inline(always)]
    #[must_use]
    pub fn dma_i(&mut self) -> DMA_I_W<8> {
        DMA_I_W::new(self)
    }
    #[doc = "Bit 9 - DMA to Drain the Digest/Output"]
    #[inline(always)]
    #[must_use]
    pub fn dma_o(&mut self) -> DMA_O_W<9> {
        DMA_O_W::new(self)
    }
    #[doc = "Bit 12 - Hash Swap Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn hashswpb(&mut self) -> HASHSWPB_W<12> {
        HASHSWPB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
