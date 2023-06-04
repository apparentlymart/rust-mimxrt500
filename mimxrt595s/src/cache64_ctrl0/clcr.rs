#[doc = "Register `CLCR` reader"]
pub struct R(crate::R<CLCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLCR` writer"]
pub struct W(crate::W<CLCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLCR_SPEC>;
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
impl From<crate::W<CLCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LGO` reader - Initiate Cache Line Command"]
pub type LGO_R = crate::BitReader<LGO_A>;
#[doc = "Initiate Cache Line Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LGO_A {
    #[doc = "0: Write: no effect; Read: no line command active"]
    NO_EFFECT = 0,
    #[doc = "1: Write: initiate line command; Read: line command active"]
    INIT_CMD = 1,
}
impl From<LGO_A> for bool {
    #[inline(always)]
    fn from(variant: LGO_A) -> Self {
        variant as u8 != 0
    }
}
impl LGO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LGO_A {
        match self.bits {
            false => LGO_A::NO_EFFECT,
            true => LGO_A::INIT_CMD,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == LGO_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `INIT_CMD`"]
    #[inline(always)]
    pub fn is_init_cmd(&self) -> bool {
        *self == LGO_A::INIT_CMD
    }
}
#[doc = "Field `LGO` writer - Initiate Cache Line Command"]
pub type LGO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLCR_SPEC, LGO_A, O>;
impl<'a, const O: u8> LGO_W<'a, O> {
    #[doc = "Write: no effect; Read: no line command active"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(LGO_A::NO_EFFECT)
    }
    #[doc = "Write: initiate line command; Read: line command active"]
    #[inline(always)]
    pub fn init_cmd(self) -> &'a mut W {
        self.variant(LGO_A::INIT_CMD)
    }
}
#[doc = "Field `CACHEADDR` reader - Cache Address"]
pub type CACHEADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CACHEADDR` writer - Cache Address"]
pub type CACHEADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLCR_SPEC, u16, u16, 12, O>;
#[doc = "Field `WSEL` reader - Way Select"]
pub type WSEL_R = crate::BitReader<WSEL_A>;
#[doc = "Way Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WSEL_A {
    #[doc = "0: Way 0"]
    WAY0 = 0,
    #[doc = "1: Way 1"]
    WAY1 = 1,
}
impl From<WSEL_A> for bool {
    #[inline(always)]
    fn from(variant: WSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl WSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WSEL_A {
        match self.bits {
            false => WSEL_A::WAY0,
            true => WSEL_A::WAY1,
        }
    }
    #[doc = "Checks if the value of the field is `WAY0`"]
    #[inline(always)]
    pub fn is_way0(&self) -> bool {
        *self == WSEL_A::WAY0
    }
    #[doc = "Checks if the value of the field is `WAY1`"]
    #[inline(always)]
    pub fn is_way1(&self) -> bool {
        *self == WSEL_A::WAY1
    }
}
#[doc = "Field `WSEL` writer - Way Select"]
pub type WSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLCR_SPEC, WSEL_A, O>;
impl<'a, const O: u8> WSEL_W<'a, O> {
    #[doc = "Way 0"]
    #[inline(always)]
    pub fn way0(self) -> &'a mut W {
        self.variant(WSEL_A::WAY0)
    }
    #[doc = "Way 1"]
    #[inline(always)]
    pub fn way1(self) -> &'a mut W {
        self.variant(WSEL_A::WAY1)
    }
}
#[doc = "Field `TDSEL` reader - Tag Or Data Select"]
pub type TDSEL_R = crate::BitReader<TDSEL_A>;
#[doc = "Tag Or Data Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDSEL_A {
    #[doc = "0: Data"]
    DATA = 0,
    #[doc = "1: Tag"]
    TAG = 1,
}
impl From<TDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TDSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl TDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDSEL_A {
        match self.bits {
            false => TDSEL_A::DATA,
            true => TDSEL_A::TAG,
        }
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == TDSEL_A::DATA
    }
    #[doc = "Checks if the value of the field is `TAG`"]
    #[inline(always)]
    pub fn is_tag(&self) -> bool {
        *self == TDSEL_A::TAG
    }
}
#[doc = "Field `TDSEL` writer - Tag Or Data Select"]
pub type TDSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLCR_SPEC, TDSEL_A, O>;
impl<'a, const O: u8> TDSEL_W<'a, O> {
    #[doc = "Data"]
    #[inline(always)]
    pub fn data(self) -> &'a mut W {
        self.variant(TDSEL_A::DATA)
    }
    #[doc = "Tag"]
    #[inline(always)]
    pub fn tag(self) -> &'a mut W {
        self.variant(TDSEL_A::TAG)
    }
}
#[doc = "Field `LCIVB` reader - Line Command Initial Valid Bit"]
pub type LCIVB_R = crate::BitReader<LCIVB_A>;
#[doc = "Line Command Initial Valid Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCIVB_A {
    #[doc = "0: Initial state 0"]
    LCIVB_0 = 0,
    #[doc = "1: Initial state 1"]
    LCIVB_1 = 1,
}
impl From<LCIVB_A> for bool {
    #[inline(always)]
    fn from(variant: LCIVB_A) -> Self {
        variant as u8 != 0
    }
}
impl LCIVB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCIVB_A {
        match self.bits {
            false => LCIVB_A::LCIVB_0,
            true => LCIVB_A::LCIVB_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCIVB_0`"]
    #[inline(always)]
    pub fn is_lcivb_0(&self) -> bool {
        *self == LCIVB_A::LCIVB_0
    }
    #[doc = "Checks if the value of the field is `LCIVB_1`"]
    #[inline(always)]
    pub fn is_lcivb_1(&self) -> bool {
        *self == LCIVB_A::LCIVB_1
    }
}
#[doc = "Field `LCIVB` writer - Line Command Initial Valid Bit"]
pub type LCIVB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLCR_SPEC, LCIVB_A, O>;
impl<'a, const O: u8> LCIVB_W<'a, O> {
    #[doc = "Initial state 0"]
    #[inline(always)]
    pub fn lcivb_0(self) -> &'a mut W {
        self.variant(LCIVB_A::LCIVB_0)
    }
    #[doc = "Initial state 1"]
    #[inline(always)]
    pub fn lcivb_1(self) -> &'a mut W {
        self.variant(LCIVB_A::LCIVB_1)
    }
}
#[doc = "Field `LCIMB` reader - Line Command Initial Modified Bit"]
pub type LCIMB_R = crate::BitReader<LCIMB_A>;
#[doc = "Line Command Initial Modified Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCIMB_A {
    #[doc = "0: Initial state 0"]
    LCIMB_0 = 0,
    #[doc = "1: Initial state 1"]
    LCIMB_1 = 1,
}
impl From<LCIMB_A> for bool {
    #[inline(always)]
    fn from(variant: LCIMB_A) -> Self {
        variant as u8 != 0
    }
}
impl LCIMB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCIMB_A {
        match self.bits {
            false => LCIMB_A::LCIMB_0,
            true => LCIMB_A::LCIMB_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCIMB_0`"]
    #[inline(always)]
    pub fn is_lcimb_0(&self) -> bool {
        *self == LCIMB_A::LCIMB_0
    }
    #[doc = "Checks if the value of the field is `LCIMB_1`"]
    #[inline(always)]
    pub fn is_lcimb_1(&self) -> bool {
        *self == LCIMB_A::LCIMB_1
    }
}
#[doc = "Field `LCIMB` writer - Line Command Initial Modified Bit"]
pub type LCIMB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLCR_SPEC, LCIMB_A, O>;
impl<'a, const O: u8> LCIMB_W<'a, O> {
    #[doc = "Initial state 0"]
    #[inline(always)]
    pub fn lcimb_0(self) -> &'a mut W {
        self.variant(LCIMB_A::LCIMB_0)
    }
    #[doc = "Initial state 1"]
    #[inline(always)]
    pub fn lcimb_1(self) -> &'a mut W {
        self.variant(LCIMB_A::LCIMB_1)
    }
}
#[doc = "Field `LCWAY` reader - Line Command Way"]
pub type LCWAY_R = crate::BitReader<LCWAY_A>;
#[doc = "Line Command Way\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCWAY_A {
    #[doc = "0: Way 0"]
    WAY0 = 0,
    #[doc = "1: Way 1"]
    WAY1 = 1,
}
impl From<LCWAY_A> for bool {
    #[inline(always)]
    fn from(variant: LCWAY_A) -> Self {
        variant as u8 != 0
    }
}
impl LCWAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCWAY_A {
        match self.bits {
            false => LCWAY_A::WAY0,
            true => LCWAY_A::WAY1,
        }
    }
    #[doc = "Checks if the value of the field is `WAY0`"]
    #[inline(always)]
    pub fn is_way0(&self) -> bool {
        *self == LCWAY_A::WAY0
    }
    #[doc = "Checks if the value of the field is `WAY1`"]
    #[inline(always)]
    pub fn is_way1(&self) -> bool {
        *self == LCWAY_A::WAY1
    }
}
#[doc = "Field `LCWAY` writer - Line Command Way"]
pub type LCWAY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLCR_SPEC, LCWAY_A, O>;
impl<'a, const O: u8> LCWAY_W<'a, O> {
    #[doc = "Way 0"]
    #[inline(always)]
    pub fn way0(self) -> &'a mut W {
        self.variant(LCWAY_A::WAY0)
    }
    #[doc = "Way 1"]
    #[inline(always)]
    pub fn way1(self) -> &'a mut W {
        self.variant(LCWAY_A::WAY1)
    }
}
#[doc = "Field `LCMD` reader - Line Command"]
pub type LCMD_R = crate::FieldReader<u8, LCMD_A>;
#[doc = "Line Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LCMD_A {
    #[doc = "0: Search and read or write"]
    SEARCH_RW = 0,
    #[doc = "1: Invalidate"]
    INVALIDATE = 1,
    #[doc = "2: Push"]
    PUSH = 2,
    #[doc = "3: Clear"]
    CLEAR = 3,
}
impl From<LCMD_A> for u8 {
    #[inline(always)]
    fn from(variant: LCMD_A) -> Self {
        variant as _
    }
}
impl LCMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCMD_A {
        match self.bits {
            0 => LCMD_A::SEARCH_RW,
            1 => LCMD_A::INVALIDATE,
            2 => LCMD_A::PUSH,
            3 => LCMD_A::CLEAR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SEARCH_RW`"]
    #[inline(always)]
    pub fn is_search_rw(&self) -> bool {
        *self == LCMD_A::SEARCH_RW
    }
    #[doc = "Checks if the value of the field is `INVALIDATE`"]
    #[inline(always)]
    pub fn is_invalidate(&self) -> bool {
        *self == LCMD_A::INVALIDATE
    }
    #[doc = "Checks if the value of the field is `PUSH`"]
    #[inline(always)]
    pub fn is_push(&self) -> bool {
        *self == LCMD_A::PUSH
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == LCMD_A::CLEAR
    }
}
#[doc = "Field `LCMD` writer - Line Command"]
pub type LCMD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CLCR_SPEC, u8, LCMD_A, 2, O>;
impl<'a, const O: u8> LCMD_W<'a, O> {
    #[doc = "Search and read or write"]
    #[inline(always)]
    pub fn search_rw(self) -> &'a mut W {
        self.variant(LCMD_A::SEARCH_RW)
    }
    #[doc = "Invalidate"]
    #[inline(always)]
    pub fn invalidate(self) -> &'a mut W {
        self.variant(LCMD_A::INVALIDATE)
    }
    #[doc = "Push"]
    #[inline(always)]
    pub fn push(self) -> &'a mut W {
        self.variant(LCMD_A::PUSH)
    }
    #[doc = "Clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LCMD_A::CLEAR)
    }
}
#[doc = "Field `LADSEL` reader - Line Address Select"]
pub type LADSEL_R = crate::BitReader<LADSEL_A>;
#[doc = "Line Address Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LADSEL_A {
    #[doc = "0: Cache"]
    CACHE_ADDR = 0,
    #[doc = "1: Physical"]
    PHYS_ADDR = 1,
}
impl From<LADSEL_A> for bool {
    #[inline(always)]
    fn from(variant: LADSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl LADSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LADSEL_A {
        match self.bits {
            false => LADSEL_A::CACHE_ADDR,
            true => LADSEL_A::PHYS_ADDR,
        }
    }
    #[doc = "Checks if the value of the field is `CACHE_ADDR`"]
    #[inline(always)]
    pub fn is_cache_addr(&self) -> bool {
        *self == LADSEL_A::CACHE_ADDR
    }
    #[doc = "Checks if the value of the field is `PHYS_ADDR`"]
    #[inline(always)]
    pub fn is_phys_addr(&self) -> bool {
        *self == LADSEL_A::PHYS_ADDR
    }
}
#[doc = "Field `LADSEL` writer - Line Address Select"]
pub type LADSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLCR_SPEC, LADSEL_A, O>;
impl<'a, const O: u8> LADSEL_W<'a, O> {
    #[doc = "Cache"]
    #[inline(always)]
    pub fn cache_addr(self) -> &'a mut W {
        self.variant(LADSEL_A::CACHE_ADDR)
    }
    #[doc = "Physical"]
    #[inline(always)]
    pub fn phys_addr(self) -> &'a mut W {
        self.variant(LADSEL_A::PHYS_ADDR)
    }
}
#[doc = "Field `LACC` reader - Line Access Type"]
pub type LACC_R = crate::BitReader<LACC_A>;
#[doc = "Line Access Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LACC_A {
    #[doc = "0: Read"]
    READ = 0,
    #[doc = "1: Write"]
    WRITE = 1,
}
impl From<LACC_A> for bool {
    #[inline(always)]
    fn from(variant: LACC_A) -> Self {
        variant as u8 != 0
    }
}
impl LACC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LACC_A {
        match self.bits {
            false => LACC_A::READ,
            true => LACC_A::WRITE,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == LACC_A::READ
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == LACC_A::WRITE
    }
}
#[doc = "Field `LACC` writer - Line Access Type"]
pub type LACC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLCR_SPEC, LACC_A, O>;
impl<'a, const O: u8> LACC_W<'a, O> {
    #[doc = "Read"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(LACC_A::READ)
    }
    #[doc = "Write"]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(LACC_A::WRITE)
    }
}
impl R {
    #[doc = "Bit 0 - Initiate Cache Line Command"]
    #[inline(always)]
    pub fn lgo(&self) -> LGO_R {
        LGO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:13 - Cache Address"]
    #[inline(always)]
    pub fn cacheaddr(&self) -> CACHEADDR_R {
        CACHEADDR_R::new(((self.bits >> 2) & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - Way Select"]
    #[inline(always)]
    pub fn wsel(&self) -> WSEL_R {
        WSEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Tag Or Data Select"]
    #[inline(always)]
    pub fn tdsel(&self) -> TDSEL_R {
        TDSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Line Command Initial Valid Bit"]
    #[inline(always)]
    pub fn lcivb(&self) -> LCIVB_R {
        LCIVB_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Line Command Initial Modified Bit"]
    #[inline(always)]
    pub fn lcimb(&self) -> LCIMB_R {
        LCIMB_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Line Command Way"]
    #[inline(always)]
    pub fn lcway(&self) -> LCWAY_R {
        LCWAY_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Line Command"]
    #[inline(always)]
    pub fn lcmd(&self) -> LCMD_R {
        LCMD_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Line Address Select"]
    #[inline(always)]
    pub fn ladsel(&self) -> LADSEL_R {
        LADSEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Line Access Type"]
    #[inline(always)]
    pub fn lacc(&self) -> LACC_R {
        LACC_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initiate Cache Line Command"]
    #[inline(always)]
    #[must_use]
    pub fn lgo(&mut self) -> LGO_W<0> {
        LGO_W::new(self)
    }
    #[doc = "Bits 2:13 - Cache Address"]
    #[inline(always)]
    #[must_use]
    pub fn cacheaddr(&mut self) -> CACHEADDR_W<2> {
        CACHEADDR_W::new(self)
    }
    #[doc = "Bit 14 - Way Select"]
    #[inline(always)]
    #[must_use]
    pub fn wsel(&mut self) -> WSEL_W<14> {
        WSEL_W::new(self)
    }
    #[doc = "Bit 16 - Tag Or Data Select"]
    #[inline(always)]
    #[must_use]
    pub fn tdsel(&mut self) -> TDSEL_W<16> {
        TDSEL_W::new(self)
    }
    #[doc = "Bit 20 - Line Command Initial Valid Bit"]
    #[inline(always)]
    #[must_use]
    pub fn lcivb(&mut self) -> LCIVB_W<20> {
        LCIVB_W::new(self)
    }
    #[doc = "Bit 21 - Line Command Initial Modified Bit"]
    #[inline(always)]
    #[must_use]
    pub fn lcimb(&mut self) -> LCIMB_W<21> {
        LCIMB_W::new(self)
    }
    #[doc = "Bit 22 - Line Command Way"]
    #[inline(always)]
    #[must_use]
    pub fn lcway(&mut self) -> LCWAY_W<22> {
        LCWAY_W::new(self)
    }
    #[doc = "Bits 24:25 - Line Command"]
    #[inline(always)]
    #[must_use]
    pub fn lcmd(&mut self) -> LCMD_W<24> {
        LCMD_W::new(self)
    }
    #[doc = "Bit 26 - Line Address Select"]
    #[inline(always)]
    #[must_use]
    pub fn ladsel(&mut self) -> LADSEL_W<26> {
        LADSEL_W::new(self)
    }
    #[doc = "Bit 27 - Line Access Type"]
    #[inline(always)]
    #[must_use]
    pub fn lacc(&mut self) -> LACC_W<27> {
        LACC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Line Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clcr](index.html) module"]
pub struct CLCR_SPEC;
impl crate::RegisterSpec for CLCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clcr::R](R) reader structure"]
impl crate::Readable for CLCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clcr::W](W) writer structure"]
impl crate::Writable for CLCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLCR to value 0"]
impl crate::Resettable for CLCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
