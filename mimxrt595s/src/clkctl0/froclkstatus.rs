#[doc = "Register `FROCLKSTATUS` reader"]
pub struct R(crate::R<FROCLKSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FROCLKSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FROCLKSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FROCLKSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CLK_OK` reader - FRO Clock OK"]
pub type CLK_OK_R = crate::BitReader<CLK_OK_A>;
#[doc = "FRO Clock OK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK_OK_A {
    #[doc = "0: FRO clock has not yet reached 10% frequency accuracy"]
    CLK_NOT_OK = 0,
    #[doc = "1: FRO clock has reached 10% frequency accuracy"]
    CLK_OK = 1,
}
impl From<CLK_OK_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_OK_A) -> Self {
        variant as u8 != 0
    }
}
impl CLK_OK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_OK_A {
        match self.bits {
            false => CLK_OK_A::CLK_NOT_OK,
            true => CLK_OK_A::CLK_OK,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_NOT_OK`"]
    #[inline(always)]
    pub fn is_clk_not_ok(&self) -> bool {
        *self == CLK_OK_A::CLK_NOT_OK
    }
    #[doc = "Checks if the value of the field is `CLK_OK`"]
    #[inline(always)]
    pub fn is_clk_ok(&self) -> bool {
        *self == CLK_OK_A::CLK_OK
    }
}
impl R {
    #[doc = "Bit 0 - FRO Clock OK"]
    #[inline(always)]
    pub fn clk_ok(&self) -> CLK_OK_R {
        CLK_OK_R::new((self.bits & 1) != 0)
    }
}
#[doc = "FRO Clock Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [froclkstatus](index.html) module"]
pub struct FROCLKSTATUS_SPEC;
impl crate::RegisterSpec for FROCLKSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [froclkstatus::R](R) reader structure"]
impl crate::Readable for FROCLKSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FROCLKSTATUS to value 0"]
impl crate::Resettable for FROCLKSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
