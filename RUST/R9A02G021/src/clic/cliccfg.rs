#[doc = "Register `cliccfg` reader"]
pub type R = crate::R<CliccfgSpec>;
#[doc = "Register `cliccfg` writer"]
pub type W = crate::W<CliccfgSpec>;
#[doc = "Field `NVBITS` reader - Selective interrupt hardware vectoring feature is implemented. This bit is read as 1."]
pub type NvbitsR = crate::BitReader;
#[doc = "Field `NLBITS` reader - Number of bits in clicintctl\\[i\\]
allocated for specifying interrupt levels."]
pub type NlbitsR = crate::FieldReader;
#[doc = "Field `NLBITS` writer - Number of bits in clicintctl\\[i\\]
allocated for specifying interrupt levels."]
pub type NlbitsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NMBITS` reader - Number of bits in clicintattr\\[i\\].MODE register"]
pub type NmbitsR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Selective interrupt hardware vectoring feature is implemented. This bit is read as 1."]
    #[inline(always)]
    pub fn nvbits(&self) -> NvbitsR {
        NvbitsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Number of bits in clicintctl\\[i\\]
allocated for specifying interrupt levels."]
    #[inline(always)]
    pub fn nlbits(&self) -> NlbitsR {
        NlbitsR::new((self.bits >> 1) & 0x0f)
    }
    #[doc = "Bits 5:6 - Number of bits in clicintattr\\[i\\].MODE register"]
    #[inline(always)]
    pub fn nmbits(&self) -> NmbitsR {
        NmbitsR::new((self.bits >> 5) & 3)
    }
}
impl W {
    #[doc = "Bits 1:4 - Number of bits in clicintctl\\[i\\]
allocated for specifying interrupt levels."]
    #[inline(always)]
    pub fn nlbits(&mut self) -> NlbitsW<CliccfgSpec> {
        NlbitsW::new(self, 1)
    }
}
#[doc = "CLIC Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cliccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cliccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CliccfgSpec;
impl crate::RegisterSpec for CliccfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cliccfg::R`](R) reader structure"]
impl crate::Readable for CliccfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cliccfg::W`](W) writer structure"]
impl crate::Writable for CliccfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets cliccfg to value 0x01"]
impl crate::Resettable for CliccfgSpec {
    const RESET_VALUE: u8 = 0x01;
}
