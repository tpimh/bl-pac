#[doc = "Register `reg_key_slot_10_w2` reader"]
pub struct R(crate::R<REG_KEY_SLOT_10_W2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_KEY_SLOT_10_W2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_KEY_SLOT_10_W2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_KEY_SLOT_10_W2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `reg_key_slot_10_w2` writer"]
pub struct W(crate::W<REG_KEY_SLOT_10_W2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_KEY_SLOT_10_W2_SPEC>;
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
impl From<crate::W<REG_KEY_SLOT_10_W2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_KEY_SLOT_10_W2_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "reg_key_slot_10_w2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_10_w2](index.html) module"]
pub struct REG_KEY_SLOT_10_W2_SPEC;
impl crate::RegisterSpec for REG_KEY_SLOT_10_W2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_key_slot_10_w2::R](R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_10_W2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_key_slot_10_w2::W](W) writer structure"]
impl crate::Writable for REG_KEY_SLOT_10_W2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets reg_key_slot_10_w2 to value 0"]
impl crate::Resettable for REG_KEY_SLOT_10_W2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}