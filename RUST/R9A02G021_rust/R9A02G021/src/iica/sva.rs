#[doc = "Register `SVA%s` reader"]
pub type R = crate::R<SvaSpec>;
#[doc = "Register `SVA%s` writer"]
pub type W = crate::W<SvaSpec>;
#[doc = "Field `A` reader - 7-bit local address when in slave mode of unit n"]
pub type AR = crate::FieldReader;
#[doc = "Field `A` writer - 7-bit local address when in slave mode of unit n"]
pub type AW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 1:7 - 7-bit local address when in slave mode of unit n"]
    #[inline(always)]
    pub fn a(&self) -> AR {
        AR::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bits 1:7 - 7-bit local address when in slave mode of unit n"]
    #[inline(always)]
    pub fn a(&mut self) -> AW<SvaSpec> {
        AW::new(self, 1)
    }
}
#[doc = "Slave Address Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`sva::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sva::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SvaSpec;
impl crate::RegisterSpec for SvaSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sva::R`](R) reader structure"]
impl crate::Readable for SvaSpec {}
#[doc = "`write(|w| ..)` method takes [`sva::W`](W) writer structure"]
impl crate::Writable for SvaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SVA%s to value 0"]
impl crate::Resettable for SvaSpec {
    const RESET_VALUE: u8 = 0;
}
