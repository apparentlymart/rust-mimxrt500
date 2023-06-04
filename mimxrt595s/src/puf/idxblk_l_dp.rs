#[doc = "Register `IDXBLK_L_DP` reader"]
pub struct R(crate::R<IDXBLK_L_DP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDXBLK_L_DP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDXBLK_L_DP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDXBLK_L_DP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDXBLK_L_DP` writer"]
pub struct W(crate::W<IDXBLK_L_DP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDXBLK_L_DP_SPEC>;
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
impl From<crate::W<IDXBLK_L_DP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDXBLK_L_DP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDX0` reader - Index n"]
pub type IDX0_R = crate::FieldReader<u8, IDX0_A>;
#[doc = "Index n\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDX0_A {
    #[doc = "1: PUF index is blocked."]
    BLOCKED = 1,
    #[doc = "2: PUF index is accessible."]
    ACCESSIBLE = 2,
}
impl From<IDX0_A> for u8 {
    #[inline(always)]
    fn from(variant: IDX0_A) -> Self {
        variant as _
    }
}
impl IDX0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDX0_A> {
        match self.bits {
            1 => Some(IDX0_A::BLOCKED),
            2 => Some(IDX0_A::ACCESSIBLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == IDX0_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `ACCESSIBLE`"]
    #[inline(always)]
    pub fn is_accessible(&self) -> bool {
        *self == IDX0_A::ACCESSIBLE
    }
}
#[doc = "Field `IDX0` writer - Index n"]
pub type IDX0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_L_DP_SPEC, u8, IDX0_A, 2, O>;
impl<'a, const O: u8> IDX0_W<'a, O> {
    #[doc = "PUF index is blocked."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(IDX0_A::BLOCKED)
    }
    #[doc = "PUF index is accessible."]
    #[inline(always)]
    pub fn accessible(self) -> &'a mut W {
        self.variant(IDX0_A::ACCESSIBLE)
    }
}
#[doc = "Field `IDX1` reader - Index n"]
pub type IDX1_R = crate::FieldReader<u8, IDX1_A>;
#[doc = "Index n\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDX1_A {
    #[doc = "1: PUF index is blocked."]
    BLOCKED = 1,
    #[doc = "2: PUF index is accessible."]
    ACCESSIBLE = 2,
}
impl From<IDX1_A> for u8 {
    #[inline(always)]
    fn from(variant: IDX1_A) -> Self {
        variant as _
    }
}
impl IDX1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDX1_A> {
        match self.bits {
            1 => Some(IDX1_A::BLOCKED),
            2 => Some(IDX1_A::ACCESSIBLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == IDX1_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `ACCESSIBLE`"]
    #[inline(always)]
    pub fn is_accessible(&self) -> bool {
        *self == IDX1_A::ACCESSIBLE
    }
}
#[doc = "Field `IDX1` writer - Index n"]
pub type IDX1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_L_DP_SPEC, u8, IDX1_A, 2, O>;
impl<'a, const O: u8> IDX1_W<'a, O> {
    #[doc = "PUF index is blocked."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(IDX1_A::BLOCKED)
    }
    #[doc = "PUF index is accessible."]
    #[inline(always)]
    pub fn accessible(self) -> &'a mut W {
        self.variant(IDX1_A::ACCESSIBLE)
    }
}
#[doc = "Field `IDX2` reader - Index n"]
pub type IDX2_R = crate::FieldReader<u8, IDX2_A>;
#[doc = "Index n\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDX2_A {
    #[doc = "1: PUF index is blocked."]
    BLOCKED = 1,
    #[doc = "2: PUF index is accessible."]
    ACCESSIBLE = 2,
}
impl From<IDX2_A> for u8 {
    #[inline(always)]
    fn from(variant: IDX2_A) -> Self {
        variant as _
    }
}
impl IDX2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDX2_A> {
        match self.bits {
            1 => Some(IDX2_A::BLOCKED),
            2 => Some(IDX2_A::ACCESSIBLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == IDX2_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `ACCESSIBLE`"]
    #[inline(always)]
    pub fn is_accessible(&self) -> bool {
        *self == IDX2_A::ACCESSIBLE
    }
}
#[doc = "Field `IDX2` writer - Index n"]
pub type IDX2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_L_DP_SPEC, u8, IDX2_A, 2, O>;
impl<'a, const O: u8> IDX2_W<'a, O> {
    #[doc = "PUF index is blocked."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(IDX2_A::BLOCKED)
    }
    #[doc = "PUF index is accessible."]
    #[inline(always)]
    pub fn accessible(self) -> &'a mut W {
        self.variant(IDX2_A::ACCESSIBLE)
    }
}
#[doc = "Field `IDX3` reader - Index n"]
pub type IDX3_R = crate::FieldReader<u8, IDX3_A>;
#[doc = "Index n\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDX3_A {
    #[doc = "1: PUF index is blocked."]
    BLOCKED = 1,
    #[doc = "2: PUF index is accessible."]
    ACCESSIBLE = 2,
}
impl From<IDX3_A> for u8 {
    #[inline(always)]
    fn from(variant: IDX3_A) -> Self {
        variant as _
    }
}
impl IDX3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDX3_A> {
        match self.bits {
            1 => Some(IDX3_A::BLOCKED),
            2 => Some(IDX3_A::ACCESSIBLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == IDX3_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `ACCESSIBLE`"]
    #[inline(always)]
    pub fn is_accessible(&self) -> bool {
        *self == IDX3_A::ACCESSIBLE
    }
}
#[doc = "Field `IDX3` writer - Index n"]
pub type IDX3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_L_DP_SPEC, u8, IDX3_A, 2, O>;
impl<'a, const O: u8> IDX3_W<'a, O> {
    #[doc = "PUF index is blocked."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(IDX3_A::BLOCKED)
    }
    #[doc = "PUF index is accessible."]
    #[inline(always)]
    pub fn accessible(self) -> &'a mut W {
        self.variant(IDX3_A::ACCESSIBLE)
    }
}
#[doc = "Field `IDX4` reader - Index n"]
pub type IDX4_R = crate::FieldReader<u8, IDX4_A>;
#[doc = "Index n\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDX4_A {
    #[doc = "1: PUF index is blocked."]
    BLOCKED = 1,
    #[doc = "2: PUF index is accessible."]
    ACCESSIBLE = 2,
}
impl From<IDX4_A> for u8 {
    #[inline(always)]
    fn from(variant: IDX4_A) -> Self {
        variant as _
    }
}
impl IDX4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDX4_A> {
        match self.bits {
            1 => Some(IDX4_A::BLOCKED),
            2 => Some(IDX4_A::ACCESSIBLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == IDX4_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `ACCESSIBLE`"]
    #[inline(always)]
    pub fn is_accessible(&self) -> bool {
        *self == IDX4_A::ACCESSIBLE
    }
}
#[doc = "Field `IDX4` writer - Index n"]
pub type IDX4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_L_DP_SPEC, u8, IDX4_A, 2, O>;
impl<'a, const O: u8> IDX4_W<'a, O> {
    #[doc = "PUF index is blocked."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(IDX4_A::BLOCKED)
    }
    #[doc = "PUF index is accessible."]
    #[inline(always)]
    pub fn accessible(self) -> &'a mut W {
        self.variant(IDX4_A::ACCESSIBLE)
    }
}
#[doc = "Field `IDX5` reader - Index n"]
pub type IDX5_R = crate::FieldReader<u8, IDX5_A>;
#[doc = "Index n\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDX5_A {
    #[doc = "1: PUF index is blocked."]
    BLOCKED = 1,
    #[doc = "2: PUF index is accessible."]
    ACCESSIBLE = 2,
}
impl From<IDX5_A> for u8 {
    #[inline(always)]
    fn from(variant: IDX5_A) -> Self {
        variant as _
    }
}
impl IDX5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDX5_A> {
        match self.bits {
            1 => Some(IDX5_A::BLOCKED),
            2 => Some(IDX5_A::ACCESSIBLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == IDX5_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `ACCESSIBLE`"]
    #[inline(always)]
    pub fn is_accessible(&self) -> bool {
        *self == IDX5_A::ACCESSIBLE
    }
}
#[doc = "Field `IDX5` writer - Index n"]
pub type IDX5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_L_DP_SPEC, u8, IDX5_A, 2, O>;
impl<'a, const O: u8> IDX5_W<'a, O> {
    #[doc = "PUF index is blocked."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(IDX5_A::BLOCKED)
    }
    #[doc = "PUF index is accessible."]
    #[inline(always)]
    pub fn accessible(self) -> &'a mut W {
        self.variant(IDX5_A::ACCESSIBLE)
    }
}
#[doc = "Field `IDX6` reader - Index n"]
pub type IDX6_R = crate::FieldReader<u8, IDX6_A>;
#[doc = "Index n\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDX6_A {
    #[doc = "1: PUF index is blocked."]
    BLOCKED = 1,
    #[doc = "2: PUF index is accessible."]
    ACCESSIBLE = 2,
}
impl From<IDX6_A> for u8 {
    #[inline(always)]
    fn from(variant: IDX6_A) -> Self {
        variant as _
    }
}
impl IDX6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDX6_A> {
        match self.bits {
            1 => Some(IDX6_A::BLOCKED),
            2 => Some(IDX6_A::ACCESSIBLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == IDX6_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `ACCESSIBLE`"]
    #[inline(always)]
    pub fn is_accessible(&self) -> bool {
        *self == IDX6_A::ACCESSIBLE
    }
}
#[doc = "Field `IDX6` writer - Index n"]
pub type IDX6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_L_DP_SPEC, u8, IDX6_A, 2, O>;
impl<'a, const O: u8> IDX6_W<'a, O> {
    #[doc = "PUF index is blocked."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(IDX6_A::BLOCKED)
    }
    #[doc = "PUF index is accessible."]
    #[inline(always)]
    pub fn accessible(self) -> &'a mut W {
        self.variant(IDX6_A::ACCESSIBLE)
    }
}
#[doc = "Field `IDX7` reader - Index n"]
pub type IDX7_R = crate::FieldReader<u8, IDX7_A>;
#[doc = "Index n\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDX7_A {
    #[doc = "1: PUF index is blocked."]
    BLOCKED = 1,
    #[doc = "2: PUF index is accessible."]
    ACCESSIBLE = 2,
}
impl From<IDX7_A> for u8 {
    #[inline(always)]
    fn from(variant: IDX7_A) -> Self {
        variant as _
    }
}
impl IDX7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDX7_A> {
        match self.bits {
            1 => Some(IDX7_A::BLOCKED),
            2 => Some(IDX7_A::ACCESSIBLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == IDX7_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `ACCESSIBLE`"]
    #[inline(always)]
    pub fn is_accessible(&self) -> bool {
        *self == IDX7_A::ACCESSIBLE
    }
}
#[doc = "Field `IDX7` writer - Index n"]
pub type IDX7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_L_DP_SPEC, u8, IDX7_A, 2, O>;
impl<'a, const O: u8> IDX7_W<'a, O> {
    #[doc = "PUF index is blocked."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(IDX7_A::BLOCKED)
    }
    #[doc = "PUF index is accessible."]
    #[inline(always)]
    pub fn accessible(self) -> &'a mut W {
        self.variant(IDX7_A::ACCESSIBLE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Index n"]
    #[inline(always)]
    pub fn idx0(&self) -> IDX0_R {
        IDX0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Index n"]
    #[inline(always)]
    pub fn idx1(&self) -> IDX1_R {
        IDX1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Index n"]
    #[inline(always)]
    pub fn idx2(&self) -> IDX2_R {
        IDX2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Index n"]
    #[inline(always)]
    pub fn idx3(&self) -> IDX3_R {
        IDX3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Index n"]
    #[inline(always)]
    pub fn idx4(&self) -> IDX4_R {
        IDX4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Index n"]
    #[inline(always)]
    pub fn idx5(&self) -> IDX5_R {
        IDX5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Index n"]
    #[inline(always)]
    pub fn idx6(&self) -> IDX6_R {
        IDX6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Index n"]
    #[inline(always)]
    pub fn idx7(&self) -> IDX7_R {
        IDX7_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Index n"]
    #[inline(always)]
    #[must_use]
    pub fn idx0(&mut self) -> IDX0_W<0> {
        IDX0_W::new(self)
    }
    #[doc = "Bits 2:3 - Index n"]
    #[inline(always)]
    #[must_use]
    pub fn idx1(&mut self) -> IDX1_W<2> {
        IDX1_W::new(self)
    }
    #[doc = "Bits 4:5 - Index n"]
    #[inline(always)]
    #[must_use]
    pub fn idx2(&mut self) -> IDX2_W<4> {
        IDX2_W::new(self)
    }
    #[doc = "Bits 6:7 - Index n"]
    #[inline(always)]
    #[must_use]
    pub fn idx3(&mut self) -> IDX3_W<6> {
        IDX3_W::new(self)
    }
    #[doc = "Bits 8:9 - Index n"]
    #[inline(always)]
    #[must_use]
    pub fn idx4(&mut self) -> IDX4_W<8> {
        IDX4_W::new(self)
    }
    #[doc = "Bits 10:11 - Index n"]
    #[inline(always)]
    #[must_use]
    pub fn idx5(&mut self) -> IDX5_W<10> {
        IDX5_W::new(self)
    }
    #[doc = "Bits 12:13 - Index n"]
    #[inline(always)]
    #[must_use]
    pub fn idx6(&mut self) -> IDX6_W<12> {
        IDX6_W::new(self)
    }
    #[doc = "Bits 14:15 - Index n"]
    #[inline(always)]
    #[must_use]
    pub fn idx7(&mut self) -> IDX7_W<14> {
        IDX7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Index Block Low Duplicate\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idxblk_l_dp](index.html) module"]
pub struct IDXBLK_L_DP_SPEC;
impl crate::RegisterSpec for IDXBLK_L_DP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idxblk_l_dp::R](R) reader structure"]
impl crate::Readable for IDXBLK_L_DP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idxblk_l_dp::W](W) writer structure"]
impl crate::Writable for IDXBLK_L_DP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDXBLK_L_DP to value 0x8000_aaaa"]
impl crate::Resettable for IDXBLK_L_DP_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_aaaa;
}
