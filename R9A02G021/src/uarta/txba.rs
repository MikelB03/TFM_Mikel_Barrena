#[doc = "Register `TXBA%s` reader"]
pub type R = crate::R<TxbaSpec>;
#[doc = "Register `TXBA%s` writer"]
pub type W = crate::W<TxbaSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Transmit Buffer Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`txba::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txba::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbaSpec;
impl crate::RegisterSpec for TxbaSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`txba::R`](R) reader structure"]
impl crate::Readable for TxbaSpec {}
#[doc = "`write(|w| ..)` method takes [`txba::W`](W) writer structure"]
impl crate::Writable for TxbaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TXBA%s to value 0xff"]
impl crate::Resettable for TxbaSpec {
    const RESET_VALUE: u8 = 0xff;
}
