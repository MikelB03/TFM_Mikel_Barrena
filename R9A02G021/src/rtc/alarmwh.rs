#[doc = "Register `ALARMWH` reader"]
pub type R = crate::R<AlarmwhSpec>;
#[doc = "Register `ALARMWH` writer"]
pub type W = crate::W<AlarmwhSpec>;
#[doc = "Field `WH1` reader - 1-digit hour setting"]
pub type Wh1R = crate::FieldReader;
#[doc = "Field `WH1` writer - 1-digit hour setting"]
pub type Wh1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WH10` reader - 10-digit hour setting"]
pub type Wh10R = crate::FieldReader;
#[doc = "Field `WH10` writer - 10-digit hour setting"]
pub type Wh10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - 1-digit hour setting"]
    #[inline(always)]
    pub fn wh1(&self) -> Wh1R {
        Wh1R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - 10-digit hour setting"]
    #[inline(always)]
    pub fn wh10(&self) -> Wh10R {
        Wh10R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-digit hour setting"]
    #[inline(always)]
    pub fn wh1(&mut self) -> Wh1W<AlarmwhSpec> {
        Wh1W::new(self, 0)
    }
    #[doc = "Bits 4:5 - 10-digit hour setting"]
    #[inline(always)]
    pub fn wh10(&mut self) -> Wh10W<AlarmwhSpec> {
        Wh10W::new(self, 4)
    }
}
#[doc = "Alarm Hour Register\n\nYou can [`read`](crate::Reg::read) this register and get [`alarmwh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarmwh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlarmwhSpec;
impl crate::RegisterSpec for AlarmwhSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`alarmwh::R`](R) reader structure"]
impl crate::Readable for AlarmwhSpec {}
#[doc = "`write(|w| ..)` method takes [`alarmwh::W`](W) writer structure"]
impl crate::Writable for AlarmwhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ALARMWH to value 0"]
impl crate::Resettable for AlarmwhSpec {
    const RESET_VALUE: u8 = 0;
}
