#[doc = "Register `clicintip%s` reader"]
pub type R = crate::R<ClicintipSpec>;
#[doc = "Register `clicintip%s` writer"]
pub type W = crate::W<ClicintipSpec>;
#[doc = "Field `IP` reader - Pending bit for the associated interrupt"]
pub type IpR = crate::BitReader;
#[doc = "Field `IP` writer - Pending bit for the associated interrupt"]
pub type IpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pending bit for the associated interrupt"]
    #[inline(always)]
    pub fn ip(&self) -> IpR {
        IpR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pending bit for the associated interrupt"]
    #[inline(always)]
    pub fn ip(&mut self) -> IpW<ClicintipSpec> {
        IpW::new(self, 0)
    }
}
#[doc = "CLIC Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clicintip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clicintip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClicintipSpec;
impl crate::RegisterSpec for ClicintipSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`clicintip::R`](R) reader structure"]
impl crate::Readable for ClicintipSpec {}
#[doc = "`write(|w| ..)` method takes [`clicintip::W`](W) writer structure"]
impl crate::Writable for ClicintipSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets clicintip%s to value 0"]
impl crate::Resettable for ClicintipSpec {
    const RESET_VALUE: u8 = 0;
}
