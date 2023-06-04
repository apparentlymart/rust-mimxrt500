#[doc = "Register `DBG_FEATURES` reader"]
pub struct R(crate::R<DBG_FEATURES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_FEATURES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG_FEATURES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG_FEATURES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBG_FEATURES` writer"]
pub struct W(crate::W<DBG_FEATURES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBG_FEATURES_SPEC>;
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
impl From<crate::W<DBG_FEATURES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBG_FEATURES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBGEN1` reader - CM33 Debug Enable Control"]
pub type DBGEN1_R = crate::FieldReader<u8, DBGEN1_A>;
#[doc = "CM33 Debug Enable Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DBGEN1_A {
    #[doc = "0: Disabled"]
    DBGEN1_00 = 0,
    #[doc = "1: Disabled"]
    DBGEN1_01 = 1,
    #[doc = "2: Enabled"]
    DBGEN1_10 = 2,
    #[doc = "3: Disabled"]
    DBGEN1_11 = 3,
}
impl From<DBGEN1_A> for u8 {
    #[inline(always)]
    fn from(variant: DBGEN1_A) -> Self {
        variant as _
    }
}
impl DBGEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGEN1_A {
        match self.bits {
            0 => DBGEN1_A::DBGEN1_00,
            1 => DBGEN1_A::DBGEN1_01,
            2 => DBGEN1_A::DBGEN1_10,
            3 => DBGEN1_A::DBGEN1_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DBGEN1_00`"]
    #[inline(always)]
    pub fn is_dbgen1_00(&self) -> bool {
        *self == DBGEN1_A::DBGEN1_00
    }
    #[doc = "Checks if the value of the field is `DBGEN1_01`"]
    #[inline(always)]
    pub fn is_dbgen1_01(&self) -> bool {
        *self == DBGEN1_A::DBGEN1_01
    }
    #[doc = "Checks if the value of the field is `DBGEN1_10`"]
    #[inline(always)]
    pub fn is_dbgen1_10(&self) -> bool {
        *self == DBGEN1_A::DBGEN1_10
    }
    #[doc = "Checks if the value of the field is `DBGEN1_11`"]
    #[inline(always)]
    pub fn is_dbgen1_11(&self) -> bool {
        *self == DBGEN1_A::DBGEN1_11
    }
}
#[doc = "Field `DBGEN1` writer - CM33 Debug Enable Control"]
pub type DBGEN1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DBG_FEATURES_SPEC, u8, DBGEN1_A, 2, O>;
impl<'a, const O: u8> DBGEN1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn dbgen1_00(self) -> &'a mut W {
        self.variant(DBGEN1_A::DBGEN1_00)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn dbgen1_01(self) -> &'a mut W {
        self.variant(DBGEN1_A::DBGEN1_01)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dbgen1_10(self) -> &'a mut W {
        self.variant(DBGEN1_A::DBGEN1_10)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn dbgen1_11(self) -> &'a mut W {
        self.variant(DBGEN1_A::DBGEN1_11)
    }
}
#[doc = "Field `NIDEN1` reader - CM33 NID Enable Control"]
pub type NIDEN1_R = crate::FieldReader<u8, NIDEN1_A>;
#[doc = "CM33 NID Enable Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NIDEN1_A {
    #[doc = "0: Disabled"]
    NIDEN1_00 = 0,
    #[doc = "1: Disabled"]
    NIDEN1_01 = 1,
    #[doc = "2: Enabled"]
    NIDEN1_10 = 2,
    #[doc = "3: Disabled"]
    NIDEN1_11 = 3,
}
impl From<NIDEN1_A> for u8 {
    #[inline(always)]
    fn from(variant: NIDEN1_A) -> Self {
        variant as _
    }
}
impl NIDEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NIDEN1_A {
        match self.bits {
            0 => NIDEN1_A::NIDEN1_00,
            1 => NIDEN1_A::NIDEN1_01,
            2 => NIDEN1_A::NIDEN1_10,
            3 => NIDEN1_A::NIDEN1_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NIDEN1_00`"]
    #[inline(always)]
    pub fn is_niden1_00(&self) -> bool {
        *self == NIDEN1_A::NIDEN1_00
    }
    #[doc = "Checks if the value of the field is `NIDEN1_01`"]
    #[inline(always)]
    pub fn is_niden1_01(&self) -> bool {
        *self == NIDEN1_A::NIDEN1_01
    }
    #[doc = "Checks if the value of the field is `NIDEN1_10`"]
    #[inline(always)]
    pub fn is_niden1_10(&self) -> bool {
        *self == NIDEN1_A::NIDEN1_10
    }
    #[doc = "Checks if the value of the field is `NIDEN1_11`"]
    #[inline(always)]
    pub fn is_niden1_11(&self) -> bool {
        *self == NIDEN1_A::NIDEN1_11
    }
}
#[doc = "Field `NIDEN1` writer - CM33 NID Enable Control"]
pub type NIDEN1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DBG_FEATURES_SPEC, u8, NIDEN1_A, 2, O>;
impl<'a, const O: u8> NIDEN1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn niden1_00(self) -> &'a mut W {
        self.variant(NIDEN1_A::NIDEN1_00)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn niden1_01(self) -> &'a mut W {
        self.variant(NIDEN1_A::NIDEN1_01)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn niden1_10(self) -> &'a mut W {
        self.variant(NIDEN1_A::NIDEN1_10)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn niden1_11(self) -> &'a mut W {
        self.variant(NIDEN1_A::NIDEN1_11)
    }
}
#[doc = "Field `SPIDEN1` reader - CM33 SPID Enable Control"]
pub type SPIDEN1_R = crate::FieldReader<u8, SPIDEN1_A>;
#[doc = "CM33 SPID Enable Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPIDEN1_A {
    #[doc = "0: Disabled"]
    SPIDEN1_00 = 0,
    #[doc = "1: Disabled"]
    SPIDEN1_01 = 1,
    #[doc = "2: Enabled"]
    SPIDEN1_10 = 2,
    #[doc = "3: Disabled"]
    SPIDEN1_11 = 3,
}
impl From<SPIDEN1_A> for u8 {
    #[inline(always)]
    fn from(variant: SPIDEN1_A) -> Self {
        variant as _
    }
}
impl SPIDEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIDEN1_A {
        match self.bits {
            0 => SPIDEN1_A::SPIDEN1_00,
            1 => SPIDEN1_A::SPIDEN1_01,
            2 => SPIDEN1_A::SPIDEN1_10,
            3 => SPIDEN1_A::SPIDEN1_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SPIDEN1_00`"]
    #[inline(always)]
    pub fn is_spiden1_00(&self) -> bool {
        *self == SPIDEN1_A::SPIDEN1_00
    }
    #[doc = "Checks if the value of the field is `SPIDEN1_01`"]
    #[inline(always)]
    pub fn is_spiden1_01(&self) -> bool {
        *self == SPIDEN1_A::SPIDEN1_01
    }
    #[doc = "Checks if the value of the field is `SPIDEN1_10`"]
    #[inline(always)]
    pub fn is_spiden1_10(&self) -> bool {
        *self == SPIDEN1_A::SPIDEN1_10
    }
    #[doc = "Checks if the value of the field is `SPIDEN1_11`"]
    #[inline(always)]
    pub fn is_spiden1_11(&self) -> bool {
        *self == SPIDEN1_A::SPIDEN1_11
    }
}
#[doc = "Field `SPIDEN1` writer - CM33 SPID Enable Control"]
pub type SPIDEN1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DBG_FEATURES_SPEC, u8, SPIDEN1_A, 2, O>;
impl<'a, const O: u8> SPIDEN1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn spiden1_00(self) -> &'a mut W {
        self.variant(SPIDEN1_A::SPIDEN1_00)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn spiden1_01(self) -> &'a mut W {
        self.variant(SPIDEN1_A::SPIDEN1_01)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn spiden1_10(self) -> &'a mut W {
        self.variant(SPIDEN1_A::SPIDEN1_10)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn spiden1_11(self) -> &'a mut W {
        self.variant(SPIDEN1_A::SPIDEN1_11)
    }
}
#[doc = "Field `SPNIDEN1` reader - CM33 SPNIDEN Enable Control"]
pub type SPNIDEN1_R = crate::FieldReader<u8, SPNIDEN1_A>;
#[doc = "CM33 SPNIDEN Enable Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPNIDEN1_A {
    #[doc = "0: Disabled"]
    SPNIDEN_00 = 0,
    #[doc = "1: Disabled"]
    SPNIDEN_01 = 1,
    #[doc = "2: Enabled"]
    SPNIDEN_10 = 2,
    #[doc = "3: Disabled"]
    SPNIDEN_11 = 3,
}
impl From<SPNIDEN1_A> for u8 {
    #[inline(always)]
    fn from(variant: SPNIDEN1_A) -> Self {
        variant as _
    }
}
impl SPNIDEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPNIDEN1_A {
        match self.bits {
            0 => SPNIDEN1_A::SPNIDEN_00,
            1 => SPNIDEN1_A::SPNIDEN_01,
            2 => SPNIDEN1_A::SPNIDEN_10,
            3 => SPNIDEN1_A::SPNIDEN_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SPNIDEN_00`"]
    #[inline(always)]
    pub fn is_spniden_00(&self) -> bool {
        *self == SPNIDEN1_A::SPNIDEN_00
    }
    #[doc = "Checks if the value of the field is `SPNIDEN_01`"]
    #[inline(always)]
    pub fn is_spniden_01(&self) -> bool {
        *self == SPNIDEN1_A::SPNIDEN_01
    }
    #[doc = "Checks if the value of the field is `SPNIDEN_10`"]
    #[inline(always)]
    pub fn is_spniden_10(&self) -> bool {
        *self == SPNIDEN1_A::SPNIDEN_10
    }
    #[doc = "Checks if the value of the field is `SPNIDEN_11`"]
    #[inline(always)]
    pub fn is_spniden_11(&self) -> bool {
        *self == SPNIDEN1_A::SPNIDEN_11
    }
}
#[doc = "Field `SPNIDEN1` writer - CM33 SPNIDEN Enable Control"]
pub type SPNIDEN1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DBG_FEATURES_SPEC, u8, SPNIDEN1_A, 2, O>;
impl<'a, const O: u8> SPNIDEN1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn spniden_00(self) -> &'a mut W {
        self.variant(SPNIDEN1_A::SPNIDEN_00)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn spniden_01(self) -> &'a mut W {
        self.variant(SPNIDEN1_A::SPNIDEN_01)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn spniden_10(self) -> &'a mut W {
        self.variant(SPNIDEN1_A::SPNIDEN_10)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn spniden_11(self) -> &'a mut W {
        self.variant(SPNIDEN1_A::SPNIDEN_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - CM33 Debug Enable Control"]
    #[inline(always)]
    pub fn dbgen1(&self) -> DBGEN1_R {
        DBGEN1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - CM33 NID Enable Control"]
    #[inline(always)]
    pub fn niden1(&self) -> NIDEN1_R {
        NIDEN1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - CM33 SPID Enable Control"]
    #[inline(always)]
    pub fn spiden1(&self) -> SPIDEN1_R {
        SPIDEN1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - CM33 SPNIDEN Enable Control"]
    #[inline(always)]
    pub fn spniden1(&self) -> SPNIDEN1_R {
        SPNIDEN1_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CM33 Debug Enable Control"]
    #[inline(always)]
    #[must_use]
    pub fn dbgen1(&mut self) -> DBGEN1_W<0> {
        DBGEN1_W::new(self)
    }
    #[doc = "Bits 2:3 - CM33 NID Enable Control"]
    #[inline(always)]
    #[must_use]
    pub fn niden1(&mut self) -> NIDEN1_W<2> {
        NIDEN1_W::new(self)
    }
    #[doc = "Bits 4:5 - CM33 SPID Enable Control"]
    #[inline(always)]
    #[must_use]
    pub fn spiden1(&mut self) -> SPIDEN1_W<4> {
        SPIDEN1_W::new(self)
    }
    #[doc = "Bits 6:7 - CM33 SPNIDEN Enable Control"]
    #[inline(always)]
    #[must_use]
    pub fn spniden1(&mut self) -> SPNIDEN1_W<6> {
        SPNIDEN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Features\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_features](index.html) module"]
pub struct DBG_FEATURES_SPEC;
impl crate::RegisterSpec for DBG_FEATURES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg_features::R](R) reader structure"]
impl crate::Readable for DBG_FEATURES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbg_features::W](W) writer structure"]
impl crate::Writable for DBG_FEATURES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBG_FEATURES to value 0x55"]
impl crate::Resettable for DBG_FEATURES_SPEC {
    const RESET_VALUE: Self::Ux = 0x55;
}
