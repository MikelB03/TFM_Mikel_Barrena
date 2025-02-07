#[doc = "Register `SDR0%s` reader"]
pub type R = crate::R<Sdr0Spec>;
#[doc = "Register `SDR0%s` writer"]
pub type W = crate::W<Sdr0Spec>;
#[doc = "Field `DAT` reader - Data buffer for transmit and receive"]
pub type DatR = crate::FieldReader<u16>;
#[doc = "Field `DAT` writer - Data buffer for transmit and receive"]
pub type DatW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `STCLK` reader - Transfer clock setting by dividing the operation clock"]
pub type StclkR = crate::FieldReader;
#[doc = "Field `STCLK` writer - Transfer clock setting by dividing the operation clock"]
pub type StclkW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:8 - Data buffer for transmit and receive"]
    #[inline(always)]
    pub fn dat(&self) -> DatR {
        DatR::new(self.bits & 0x01ff)
    }
    #[doc = "Bits 9:15 - Transfer clock setting by dividing the operation clock"]
    #[inline(always)]
    pub fn stclk(&self) -> StclkR {
        StclkR::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Data buffer for transmit and receive"]
    #[inline(always)]
    pub fn dat(&mut self) -> DatW<Sdr0Spec> {
        DatW::new(self, 0)
    }
    #[doc = "Bits 9:15 - Transfer clock setting by dividing the operation clock"]
    #[inline(always)]
    pub fn stclk(&mut self) -> StclkW<Sdr0Spec> {
        StclkW::new(self, 9)
    }
}
#[doc = "Serial Data Register 0%s\n\nYou can [`read`](crate::Reg::read) this register and get [`sdr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sdr0Spec;
impl crate::RegisterSpec for Sdr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdr0::R`](R) reader structure"]
impl crate::Readable for Sdr0Spec {}
#[doc = "`write(|w| ..)` method takes [`sdr0::W`](W) writer structure"]
impl crate::Writable for Sdr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDR0%s to value 0"]
impl crate::Resettable for Sdr0Spec {
    const RESET_VALUE: u16 = 0;
}
