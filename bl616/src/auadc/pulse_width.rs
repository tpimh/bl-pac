#[doc = "Register `pulse_width` reader"]
pub struct R(crate::R<PULSE_WIDTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PULSE_WIDTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PULSE_WIDTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PULSE_WIDTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pulse_width` writer"]
pub struct W(crate::W<PULSE_WIDTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PULSE_WIDTH_SPEC>;
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
impl From<crate::W<PULSE_WIDTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PULSE_WIDTH_SPEC>) -> Self {
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
#[doc = "Pulse-Width Modulator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulse_width](index.html) module"]
pub struct PULSE_WIDTH_SPEC;
impl crate::RegisterSpec for PULSE_WIDTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pulse_width::R](R) reader structure"]
impl crate::Readable for PULSE_WIDTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pulse_width::W](W) writer structure"]
impl crate::Writable for PULSE_WIDTH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pulse_width to value 0"]
impl crate::Resettable for PULSE_WIDTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
