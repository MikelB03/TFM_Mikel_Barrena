#[doc = "Register `TSCDR` reader"]
pub type R = crate::R<TscdrSpec>;
#[doc = "Field `TSCDR` reader - Temperature Sensor Calibration Data"]
pub type TscdrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Temperature Sensor Calibration Data"]
    #[inline(always)]
    pub fn tscdr(&self) -> TscdrR {
        TscdrR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Temperature Sensor Calibration Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tscdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TscdrSpec;
impl crate::RegisterSpec for TscdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tscdr::R`](R) reader structure"]
impl crate::Readable for TscdrSpec {}
#[doc = "`reset()` method sets TSCDR to value 0"]
impl crate::Resettable for TscdrSpec {
    const RESET_VALUE: u32 = 0;
}
