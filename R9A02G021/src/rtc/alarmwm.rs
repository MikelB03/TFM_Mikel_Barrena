#[doc = "Register `ALARMWM` reader"]
pub type R = crate::R<AlarmwmSpec>;
#[doc = "Register `ALARMWM` writer"]
pub type W = crate::W<AlarmwmSpec>;
#[doc = "Field `WM1` reader - 1-digit minute setting"]
pub type Wm1R = crate::FieldReader;
#[doc = "Field `WM1` writer - 1-digit minute setting"]
pub type Wm1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WM10` reader - 10-digit minute setting"]
pub type Wm10R = crate::FieldReader;
#[doc = "Field `WM10` writer - 10-digit minute setting"]
pub type Wm10W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - 1-digit minute setting"]
    #[inline(always)]
    pub fn wm1(&self) -> Wm1R {
        Wm1R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - 10-digit minute setting"]
    #[inline(always)]
    pub fn wm10(&self) -> Wm10R {
        Wm10R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-digit minute setting"]
    #[inline(always)]
    pub fn wm1(&mut self) -> Wm1W<AlarmwmSpec> {
        Wm1W::new(self, 0)
    }
    #[doc = "Bits 4:6 - 10-digit minute setting"]
    #[inline(always)]
    pub fn wm10(&mut self) -> Wm10W<AlarmwmSpec> {
        Wm10W::new(self, 4)
    }
}
#[doc = "Alarm Minute Register\n\nYou can [`read`](crate::Reg::read) this register and get [`alarmwm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarmwm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlarmwmSpec;
impl crate::RegisterSpec for AlarmwmSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`alarmwm::R`](R) reader structure"]
impl crate::Readable for AlarmwmSpec {}
#[doc = "`write(|w| ..)` method takes [`alarmwm::W`](W) writer structure"]
impl crate::Writable for AlarmwmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ALARMWM to value 0"]
impl crate::Resettable for AlarmwmSpec {
    const RESET_VALUE: u8 = 0;
}
