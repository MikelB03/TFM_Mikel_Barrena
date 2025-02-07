#[doc = "Register `clicinfo` reader"]
pub type R = crate::R<ClicinfoSpec>;
#[doc = "Field `NUM_INTERRUPT` reader - Number of total interrupts supported by CLIC"]
pub type NumInterruptR = crate::FieldReader<u16>;
#[doc = "Field `VERSION` reader - Version of CLIC"]
pub type VersionR = crate::FieldReader;
#[doc = "Field `CLICINTCTLBITS` reader - The value of the CLICINTCTLBITS parameter"]
pub type ClicintctlbitsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:12 - Number of total interrupts supported by CLIC"]
    #[inline(always)]
    pub fn num_interrupt(&self) -> NumInterruptR {
        NumInterruptR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:20 - Version of CLIC"]
    #[inline(always)]
    pub fn version(&self) -> VersionR {
        VersionR::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bits 21:24 - The value of the CLICINTCTLBITS parameter"]
    #[inline(always)]
    pub fn clicintctlbits(&self) -> ClicintctlbitsR {
        ClicintctlbitsR::new(((self.bits >> 21) & 0x0f) as u8)
    }
}
#[doc = "CLIC Information Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clicinfo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClicinfoSpec;
impl crate::RegisterSpec for ClicinfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clicinfo::R`](R) reader structure"]
impl crate::Readable for ClicinfoSpec {}
#[doc = "`reset()` method sets clicinfo to value 0x0080_2033"]
impl crate::Resettable for ClicinfoSpec {
    const RESET_VALUE: u32 = 0x0080_2033;
}
