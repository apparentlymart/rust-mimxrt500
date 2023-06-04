#[doc = "Register `STROBE_DLL_CTRL` reader"]
pub struct R(crate::R<STROBE_DLL_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STROBE_DLL_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STROBE_DLL_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STROBE_DLL_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STROBE_DLL_CTRL` writer"]
pub struct W(crate::W<STROBE_DLL_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STROBE_DLL_CTRL_SPEC>;
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
impl From<crate::W<STROBE_DLL_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STROBE_DLL_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STROBE_DLL_CTRL_ENABLE` reader - Strobe DLL control enable"]
pub type STROBE_DLL_CTRL_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `STROBE_DLL_CTRL_ENABLE` writer - Strobe DLL control enable"]
pub type STROBE_DLL_CTRL_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STROBE_DLL_CTRL_SPEC, bool, O>;
#[doc = "Field `STROBE_DLL_CTRL_RESET` reader - Strobe DLL control reset"]
pub type STROBE_DLL_CTRL_RESET_R = crate::BitReader<bool>;
#[doc = "Field `STROBE_DLL_CTRL_RESET` writer - Strobe DLL control reset"]
pub type STROBE_DLL_CTRL_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STROBE_DLL_CTRL_SPEC, bool, O>;
#[doc = "Field `STROBE_DLL_CTRL_SLV_FORCE_UPD` reader - Strobe DLL control slave force updated"]
pub type STROBE_DLL_CTRL_SLV_FORCE_UPD_R = crate::BitReader<bool>;
#[doc = "Field `STROBE_DLL_CTRL_SLV_FORCE_UPD` writer - Strobe DLL control slave force updated"]
pub type STROBE_DLL_CTRL_SLV_FORCE_UPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STROBE_DLL_CTRL_SPEC, bool, O>;
#[doc = "Field `STROBE_DLL_CTRL_SLV_DLY_TARGET` reader - Strobe DLL Control Slave Delay Target"]
pub type STROBE_DLL_CTRL_SLV_DLY_TARGET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STROBE_DLL_CTRL_SLV_DLY_TARGET` writer - Strobe DLL Control Slave Delay Target"]
pub type STROBE_DLL_CTRL_SLV_DLY_TARGET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STROBE_DLL_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `STROBE_DLL_CTRL_GATE_UPDATE_0` reader - Strobe DLL control gate update"]
pub type STROBE_DLL_CTRL_GATE_UPDATE_0_R = crate::BitReader<bool>;
#[doc = "Field `STROBE_DLL_CTRL_GATE_UPDATE_0` writer - Strobe DLL control gate update"]
pub type STROBE_DLL_CTRL_GATE_UPDATE_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STROBE_DLL_CTRL_SPEC, bool, O>;
#[doc = "Field `STROBE_DLL_CTRL_GATE_UPDATE_1` reader - Strobe DLL control gate update"]
pub type STROBE_DLL_CTRL_GATE_UPDATE_1_R = crate::BitReader<bool>;
#[doc = "Field `STROBE_DLL_CTRL_GATE_UPDATE_1` writer - Strobe DLL control gate update"]
pub type STROBE_DLL_CTRL_GATE_UPDATE_1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STROBE_DLL_CTRL_SPEC, bool, O>;
#[doc = "Field `STROBE_DLL_CTRL_SLV_OVERRIDE` reader - Strobe DLL control slave override"]
pub type STROBE_DLL_CTRL_SLV_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `STROBE_DLL_CTRL_SLV_OVERRIDE` writer - Strobe DLL control slave override"]
pub type STROBE_DLL_CTRL_SLV_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STROBE_DLL_CTRL_SPEC, bool, O>;
#[doc = "Field `STROBE_DLL_CTRL_SLV_OVERRIDE_VAL` reader - Strobe DLL control slave Override value"]
pub type STROBE_DLL_CTRL_SLV_OVERRIDE_VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STROBE_DLL_CTRL_SLV_OVERRIDE_VAL` writer - Strobe DLL control slave Override value"]
pub type STROBE_DLL_CTRL_SLV_OVERRIDE_VAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STROBE_DLL_CTRL_SPEC, u8, u8, 7, O>;
#[doc = "Field `STROBE_DLL_CTRL_SLV_UPDATE_INT` reader - Strobe DLL control slave update interval"]
pub type STROBE_DLL_CTRL_SLV_UPDATE_INT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STROBE_DLL_CTRL_SLV_UPDATE_INT` writer - Strobe DLL control slave update interval"]
pub type STROBE_DLL_CTRL_SLV_UPDATE_INT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STROBE_DLL_CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `STROBE_DLL_CTRL_REF_UPDATE_INT` reader - Strobe DLL control reference update interval"]
pub type STROBE_DLL_CTRL_REF_UPDATE_INT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STROBE_DLL_CTRL_REF_UPDATE_INT` writer - Strobe DLL control reference update interval"]
pub type STROBE_DLL_CTRL_REF_UPDATE_INT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STROBE_DLL_CTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Strobe DLL control enable"]
    #[inline(always)]
    pub fn strobe_dll_ctrl_enable(&self) -> STROBE_DLL_CTRL_ENABLE_R {
        STROBE_DLL_CTRL_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Strobe DLL control reset"]
    #[inline(always)]
    pub fn strobe_dll_ctrl_reset(&self) -> STROBE_DLL_CTRL_RESET_R {
        STROBE_DLL_CTRL_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Strobe DLL control slave force updated"]
    #[inline(always)]
    pub fn strobe_dll_ctrl_slv_force_upd(&self) -> STROBE_DLL_CTRL_SLV_FORCE_UPD_R {
        STROBE_DLL_CTRL_SLV_FORCE_UPD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Strobe DLL Control Slave Delay Target"]
    #[inline(always)]
    pub fn strobe_dll_ctrl_slv_dly_target(&self) -> STROBE_DLL_CTRL_SLV_DLY_TARGET_R {
        STROBE_DLL_CTRL_SLV_DLY_TARGET_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Strobe DLL control gate update"]
    #[inline(always)]
    pub fn strobe_dll_ctrl_gate_update_0(&self) -> STROBE_DLL_CTRL_GATE_UPDATE_0_R {
        STROBE_DLL_CTRL_GATE_UPDATE_0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Strobe DLL control gate update"]
    #[inline(always)]
    pub fn strobe_dll_ctrl_gate_update_1(&self) -> STROBE_DLL_CTRL_GATE_UPDATE_1_R {
        STROBE_DLL_CTRL_GATE_UPDATE_1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Strobe DLL control slave override"]
    #[inline(always)]
    pub fn strobe_dll_ctrl_slv_override(&self) -> STROBE_DLL_CTRL_SLV_OVERRIDE_R {
        STROBE_DLL_CTRL_SLV_OVERRIDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Strobe DLL control slave Override value"]
    #[inline(always)]
    pub fn strobe_dll_ctrl_slv_override_val(&self) -> STROBE_DLL_CTRL_SLV_OVERRIDE_VAL_R {
        STROBE_DLL_CTRL_SLV_OVERRIDE_VAL_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 20:27 - Strobe DLL control slave update interval"]
    #[inline(always)]
    pub fn strobe_dll_ctrl_slv_update_int(&self) -> STROBE_DLL_CTRL_SLV_UPDATE_INT_R {
        STROBE_DLL_CTRL_SLV_UPDATE_INT_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:31 - Strobe DLL control reference update interval"]
    #[inline(always)]
    pub fn strobe_dll_ctrl_ref_update_int(&self) -> STROBE_DLL_CTRL_REF_UPDATE_INT_R {
        STROBE_DLL_CTRL_REF_UPDATE_INT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Strobe DLL control enable"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_dll_ctrl_enable(&mut self) -> STROBE_DLL_CTRL_ENABLE_W<0> {
        STROBE_DLL_CTRL_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Strobe DLL control reset"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_dll_ctrl_reset(&mut self) -> STROBE_DLL_CTRL_RESET_W<1> {
        STROBE_DLL_CTRL_RESET_W::new(self)
    }
    #[doc = "Bit 2 - Strobe DLL control slave force updated"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_dll_ctrl_slv_force_upd(&mut self) -> STROBE_DLL_CTRL_SLV_FORCE_UPD_W<2> {
        STROBE_DLL_CTRL_SLV_FORCE_UPD_W::new(self)
    }
    #[doc = "Bits 3:5 - Strobe DLL Control Slave Delay Target"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_dll_ctrl_slv_dly_target(&mut self) -> STROBE_DLL_CTRL_SLV_DLY_TARGET_W<3> {
        STROBE_DLL_CTRL_SLV_DLY_TARGET_W::new(self)
    }
    #[doc = "Bit 6 - Strobe DLL control gate update"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_dll_ctrl_gate_update_0(&mut self) -> STROBE_DLL_CTRL_GATE_UPDATE_0_W<6> {
        STROBE_DLL_CTRL_GATE_UPDATE_0_W::new(self)
    }
    #[doc = "Bit 7 - Strobe DLL control gate update"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_dll_ctrl_gate_update_1(&mut self) -> STROBE_DLL_CTRL_GATE_UPDATE_1_W<7> {
        STROBE_DLL_CTRL_GATE_UPDATE_1_W::new(self)
    }
    #[doc = "Bit 8 - Strobe DLL control slave override"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_dll_ctrl_slv_override(&mut self) -> STROBE_DLL_CTRL_SLV_OVERRIDE_W<8> {
        STROBE_DLL_CTRL_SLV_OVERRIDE_W::new(self)
    }
    #[doc = "Bits 9:15 - Strobe DLL control slave Override value"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_dll_ctrl_slv_override_val(&mut self) -> STROBE_DLL_CTRL_SLV_OVERRIDE_VAL_W<9> {
        STROBE_DLL_CTRL_SLV_OVERRIDE_VAL_W::new(self)
    }
    #[doc = "Bits 20:27 - Strobe DLL control slave update interval"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_dll_ctrl_slv_update_int(&mut self) -> STROBE_DLL_CTRL_SLV_UPDATE_INT_W<20> {
        STROBE_DLL_CTRL_SLV_UPDATE_INT_W::new(self)
    }
    #[doc = "Bits 28:31 - Strobe DLL control reference update interval"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_dll_ctrl_ref_update_int(&mut self) -> STROBE_DLL_CTRL_REF_UPDATE_INT_W<28> {
        STROBE_DLL_CTRL_REF_UPDATE_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Strobe DLL control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [strobe_dll_ctrl](index.html) module"]
pub struct STROBE_DLL_CTRL_SPEC;
impl crate::RegisterSpec for STROBE_DLL_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [strobe_dll_ctrl::R](R) reader structure"]
impl crate::Readable for STROBE_DLL_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [strobe_dll_ctrl::W](W) writer structure"]
impl crate::Writable for STROBE_DLL_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STROBE_DLL_CTRL to value 0"]
impl crate::Resettable for STROBE_DLL_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
