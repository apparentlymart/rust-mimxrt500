#[doc = "Register `IDXBLK_H` reader"]
pub struct R(crate::R<IDXBLK_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDXBLK_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDXBLK_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDXBLK_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDXBLK_H` writer"]
pub struct W(crate::W<IDXBLK_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDXBLK_H_SPEC>;
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
impl From<crate::W<IDXBLK_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDXBLK_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDX8` reader - Index n"]
pub type IDX8_R = crate::FieldReader<u8, IDX8_A>;
#[doc = "Index n\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDX8_A {
    #[doc = "1: PUF index is blocked."]
    BLOCKED = 1,
    #[doc = "2: PUF index is accessible."]
    ACCESSIBLE = 2,
}
impl From<IDX8_A> for u8 {
    #[inline(always)]
    fn from(variant: IDX8_A) -> Self {
        variant as _
    }
}
impl IDX8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDX8_A> {
        match self.bits {
            1 => Some(IDX8_A::BLOCKED),
            2 => Some(IDX8_A::ACCESSIBLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == IDX8_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `ACCESSIBLE`"]
    #[inline(always)]
    pub fn is_accessible(&self) -> bool {
        *self == IDX8_A::ACCESSIBLE
    }
}
#[doc = "Field `IDX8` writer - Index n"]
pub type IDX8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_H_SPEC, u8, IDX8_A, 2, O>;
impl<'a, const O: u8> IDX8_W<'a, O> {
    #[doc = "PUF index is blocked."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(IDX8_A::BLOCKED)
    }
    #[doc = "PUF index is accessible."]
    #[inline(always)]
    pub fn accessible(self) -> &'a mut W {
        self.variant(IDX8_A::ACCESSIBLE)
    }
}
#[doc = "Field `IDX9` reader - Index n"]
pub type IDX9_R = crate::FieldReader<u8, IDX9_A>;
#[doc = "Index n\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDX9_A {
    #[doc = "1: PUF index is blocked."]
    BLOCKED = 1,
    #[doc = "2: PUF index is accessible."]
    ACCESSIBLE = 2,
}
impl From<IDX9_A> for u8 {
    #[inline(always)]
    fn from(variant: IDX9_A) -> Self {
        variant as _
    }
}
impl IDX9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDX9_A> {
        match self.bits {
            1 => Some(IDX9_A::BLOCKED),
            2 => Some(IDX9_A::ACCESSIBLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == IDX9_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `ACCESSIBLE`"]
    #[inline(always)]
    pub fn is_accessible(&self) -> bool {
        *self == IDX9_A::ACCESSIBLE
    }
}
#[doc = "Field `IDX9` writer - Index n"]
pub type IDX9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_H_SPEC, u8, IDX9_A, 2, O>;
impl<'a, const O: u8> IDX9_W<'a, O> {
    #[doc = "PUF index is blocked."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(IDX9_A::BLOCKED)
    }
    #[doc = "PUF index is accessible."]
    #[inline(always)]
    pub fn accessible(self) -> &'a mut W {
        self.variant(IDX9_A::ACCESSIBLE)
    }
}
#[doc = "Field `IDX10` reader - Index n"]
pub type IDX10_R = crate::FieldReader<u8, IDX10_A>;
#[doc = "Index n\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDX10_A {
    #[doc = "1: PUF index is blocked."]
    BLOCKED = 1,
    #[doc = "2: PUF index is accessible."]
    ACCESSIBLE = 2,
}
impl From<IDX10_A> for u8 {
    #[inline(always)]
    fn from(variant: IDX10_A) -> Self {
        variant as _
    }
}
impl IDX10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDX10_A> {
        match self.bits {
            1 => Some(IDX10_A::BLOCKED),
            2 => Some(IDX10_A::ACCESSIBLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == IDX10_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `ACCESSIBLE`"]
    #[inline(always)]
    pub fn is_accessible(&self) -> bool {
        *self == IDX10_A::ACCESSIBLE
    }
}
#[doc = "Field `IDX10` writer - Index n"]
pub type IDX10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_H_SPEC, u8, IDX10_A, 2, O>;
impl<'a, const O: u8> IDX10_W<'a, O> {
    #[doc = "PUF index is blocked."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(IDX10_A::BLOCKED)
    }
    #[doc = "PUF index is accessible."]
    #[inline(always)]
    pub fn accessible(self) -> &'a mut W {
        self.variant(IDX10_A::ACCESSIBLE)
    }
}
#[doc = "Field `IDX11` reader - Index n"]
pub type IDX11_R = crate::FieldReader<u8, IDX11_A>;
#[doc = "Index n\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDX11_A {
    #[doc = "1: PUF index is blocked."]
    BLOCKED = 1,
    #[doc = "2: PUF index is accessible."]
    ACCESSIBLE = 2,
}
impl From<IDX11_A> for u8 {
    #[inline(always)]
    fn from(variant: IDX11_A) -> Self {
        variant as _
    }
}
impl IDX11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDX11_A> {
        match self.bits {
            1 => Some(IDX11_A::BLOCKED),
            2 => Some(IDX11_A::ACCESSIBLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == IDX11_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `ACCESSIBLE`"]
    #[inline(always)]
    pub fn is_accessible(&self) -> bool {
        *self == IDX11_A::ACCESSIBLE
    }
}
#[doc = "Field `IDX11` writer - Index n"]
pub type IDX11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_H_SPEC, u8, IDX11_A, 2, O>;
impl<'a, const O: u8> IDX11_W<'a, O> {
    #[doc = "PUF index is blocked."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(IDX11_A::BLOCKED)
    }
    #[doc = "PUF index is accessible."]
    #[inline(always)]
    pub fn accessible(self) -> &'a mut W {
        self.variant(IDX11_A::ACCESSIBLE)
    }
}
#[doc = "Field `IDX12` reader - Index n"]
pub type IDX12_R = crate::FieldReader<u8, IDX12_A>;
#[doc = "Index n\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDX12_A {
    #[doc = "1: PUF index is blocked."]
    BLOCKED = 1,
    #[doc = "2: PUF index is accessible."]
    ACCESSIBLE = 2,
}
impl From<IDX12_A> for u8 {
    #[inline(always)]
    fn from(variant: IDX12_A) -> Self {
        variant as _
    }
}
impl IDX12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDX12_A> {
        match self.bits {
            1 => Some(IDX12_A::BLOCKED),
            2 => Some(IDX12_A::ACCESSIBLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == IDX12_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `ACCESSIBLE`"]
    #[inline(always)]
    pub fn is_accessible(&self) -> bool {
        *self == IDX12_A::ACCESSIBLE
    }
}
#[doc = "Field `IDX12` writer - Index n"]
pub type IDX12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_H_SPEC, u8, IDX12_A, 2, O>;
impl<'a, const O: u8> IDX12_W<'a, O> {
    #[doc = "PUF index is blocked."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(IDX12_A::BLOCKED)
    }
    #[doc = "PUF index is accessible."]
    #[inline(always)]
    pub fn accessible(self) -> &'a mut W {
        self.variant(IDX12_A::ACCESSIBLE)
    }
}
#[doc = "Field `IDX13` reader - Index n"]
pub type IDX13_R = crate::FieldReader<u8, IDX13_A>;
#[doc = "Index n\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDX13_A {
    #[doc = "1: PUF index is blocked."]
    BLOCKED = 1,
    #[doc = "2: PUF index is accessible."]
    ACCESSIBLE = 2,
}
impl From<IDX13_A> for u8 {
    #[inline(always)]
    fn from(variant: IDX13_A) -> Self {
        variant as _
    }
}
impl IDX13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDX13_A> {
        match self.bits {
            1 => Some(IDX13_A::BLOCKED),
            2 => Some(IDX13_A::ACCESSIBLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == IDX13_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `ACCESSIBLE`"]
    #[inline(always)]
    pub fn is_accessible(&self) -> bool {
        *self == IDX13_A::ACCESSIBLE
    }
}
#[doc = "Field `IDX13` writer - Index n"]
pub type IDX13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_H_SPEC, u8, IDX13_A, 2, O>;
impl<'a, const O: u8> IDX13_W<'a, O> {
    #[doc = "PUF index is blocked."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(IDX13_A::BLOCKED)
    }
    #[doc = "PUF index is accessible."]
    #[inline(always)]
    pub fn accessible(self) -> &'a mut W {
        self.variant(IDX13_A::ACCESSIBLE)
    }
}
#[doc = "Field `IDX14` reader - Index n"]
pub type IDX14_R = crate::FieldReader<u8, IDX14_A>;
#[doc = "Index n\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDX14_A {
    #[doc = "1: PUF index is blocked."]
    BLOCKED = 1,
    #[doc = "2: PUF index is accessible."]
    ACCESSIBLE = 2,
}
impl From<IDX14_A> for u8 {
    #[inline(always)]
    fn from(variant: IDX14_A) -> Self {
        variant as _
    }
}
impl IDX14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDX14_A> {
        match self.bits {
            1 => Some(IDX14_A::BLOCKED),
            2 => Some(IDX14_A::ACCESSIBLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == IDX14_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `ACCESSIBLE`"]
    #[inline(always)]
    pub fn is_accessible(&self) -> bool {
        *self == IDX14_A::ACCESSIBLE
    }
}
#[doc = "Field `IDX14` writer - Index n"]
pub type IDX14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_H_SPEC, u8, IDX14_A, 2, O>;
impl<'a, const O: u8> IDX14_W<'a, O> {
    #[doc = "PUF index is blocked."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(IDX14_A::BLOCKED)
    }
    #[doc = "PUF index is accessible."]
    #[inline(always)]
    pub fn accessible(self) -> &'a mut W {
        self.variant(IDX14_A::ACCESSIBLE)
    }
}
#[doc = "Field `IDX15` reader - Index n"]
pub type IDX15_R = crate::FieldReader<u8, IDX15_A>;
#[doc = "Index n\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDX15_A {
    #[doc = "1: PUF index is blocked."]
    BLOCKED = 1,
    #[doc = "2: PUF index is accessible."]
    ACCESSIBLE = 2,
}
impl From<IDX15_A> for u8 {
    #[inline(always)]
    fn from(variant: IDX15_A) -> Self {
        variant as _
    }
}
impl IDX15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDX15_A> {
        match self.bits {
            1 => Some(IDX15_A::BLOCKED),
            2 => Some(IDX15_A::ACCESSIBLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == IDX15_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `ACCESSIBLE`"]
    #[inline(always)]
    pub fn is_accessible(&self) -> bool {
        *self == IDX15_A::ACCESSIBLE
    }
}
#[doc = "Field `IDX15` writer - Index n"]
pub type IDX15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_H_SPEC, u8, IDX15_A, 2, O>;
impl<'a, const O: u8> IDX15_W<'a, O> {
    #[doc = "PUF index is blocked."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(IDX15_A::BLOCKED)
    }
    #[doc = "PUF index is accessible."]
    #[inline(always)]
    pub fn accessible(self) -> &'a mut W {
        self.variant(IDX15_A::ACCESSIBLE)
    }
}
#[doc = "Field `LOCK_IDX` reader - Lock Index"]
pub type LOCK_IDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOCK_IDX` writer - Lock Index"]
pub type LOCK_IDX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_H_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Index n"]
    #[inline(always)]
    pub fn idx8(&self) -> IDX8_R {
        IDX8_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Index n"]
    #[inline(always)]
    pub fn idx9(&self) -> IDX9_R {
        IDX9_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Index n"]
    #[inline(always)]
    pub fn idx10(&self) -> IDX10_R {
        IDX10_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Index n"]
    #[inline(always)]
    pub fn idx11(&self) -> IDX11_R {
        IDX11_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Index n"]
    #[inline(always)]
    pub fn idx12(&self) -> IDX12_R {
        IDX12_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Index n"]
    #[inline(always)]
    pub fn idx13(&self) -> IDX13_R {
        IDX13_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Index n"]
    #[inline(always)]
    pub fn idx14(&self) -> IDX14_R {
        IDX14_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Index n"]
    #[inline(always)]
    pub fn idx15(&self) -> IDX15_R {
        IDX15_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Lock Index"]
    #[inline(always)]
    pub fn lock_idx(&self) -> LOCK_IDX_R {
        LOCK_IDX_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Index n"]
    #[inline(always)]
    #[must_use]
    pub fn idx8(&mut self) -> IDX8_W<0> {
        IDX8_W::new(self)
    }
    #[doc = "Bits 2:3 - Index n"]
    #[inline(always)]
    #[must_use]
    pub fn idx9(&mut self) -> IDX9_W<2> {
        IDX9_W::new(self)
    }
    #[doc = "Bits 4:5 - Index n"]
    #[inline(always)]
    #[must_use]
    pub fn idx10(&mut self) -> IDX10_W<4> {
        IDX10_W::new(self)
    }
    #[doc = "Bits 6:7 - Index n"]
    #[inline(always)]
    #[must_use]
    pub fn idx11(&mut self) -> IDX11_W<6> {
        IDX11_W::new(self)
    }
    #[doc = "Bits 8:9 - Index n"]
    #[inline(always)]
    #[must_use]
    pub fn idx12(&mut self) -> IDX12_W<8> {
        IDX12_W::new(self)
    }
    #[doc = "Bits 10:11 - Index n"]
    #[inline(always)]
    #[must_use]
    pub fn idx13(&mut self) -> IDX13_W<10> {
        IDX13_W::new(self)
    }
    #[doc = "Bits 12:13 - Index n"]
    #[inline(always)]
    #[must_use]
    pub fn idx14(&mut self) -> IDX14_W<12> {
        IDX14_W::new(self)
    }
    #[doc = "Bits 14:15 - Index n"]
    #[inline(always)]
    #[must_use]
    pub fn idx15(&mut self) -> IDX15_W<14> {
        IDX15_W::new(self)
    }
    #[doc = "Bits 30:31 - Lock Index"]
    #[inline(always)]
    #[must_use]
    pub fn lock_idx(&mut self) -> LOCK_IDX_W<30> {
        LOCK_IDX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Index Block High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idxblk_h](index.html) module"]
pub struct IDXBLK_H_SPEC;
impl crate::RegisterSpec for IDXBLK_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idxblk_h::R](R) reader structure"]
impl crate::Readable for IDXBLK_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idxblk_h::W](W) writer structure"]
impl crate::Writable for IDXBLK_H_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDXBLK_H to value 0x8000_aaaa"]
impl crate::Resettable for IDXBLK_H_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_aaaa;
}
