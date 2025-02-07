#[doc = "Register `REMCPD` reader"]
pub type R = crate::R<RemcpdSpec>;
#[doc = "Register `REMCPD` writer"]
pub type W = crate::W<RemcpdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Compare Value Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`remcpd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remcpd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RemcpdSpec;
impl crate::RegisterSpec for RemcpdSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`remcpd::R`](R) reader structure"]
impl crate::Readable for RemcpdSpec {}
#[doc = "`write(|w| ..)` method takes [`remcpd::W`](W) writer structure"]
impl crate::Writable for RemcpdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets REMCPD to value 0"]
impl crate::Resettable for RemcpdSpec {
    const RESET_VALUE: u16 = 0;
}
