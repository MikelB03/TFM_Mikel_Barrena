#[doc = "Register `WEEK` reader"]
pub type R = crate::R<WeekSpec>;
#[doc = "Register `WEEK` writer"]
pub type W = crate::W<WeekSpec>;
#[doc = "Day-of-Week Counting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Week {
    #[doc = "0: Sunday"]
    _000 = 0,
    #[doc = "1: Monday"]
    _001 = 1,
    #[doc = "2: Tuesday"]
    _010 = 2,
    #[doc = "3: Wednesday"]
    _011 = 3,
    #[doc = "4: Thursday"]
    _100 = 4,
    #[doc = "5: Friday"]
    _101 = 5,
    #[doc = "6: Saturday"]
    _110 = 6,
    #[doc = "7: Setting prohibited"]
    Others = 7,
}
impl From<Week> for u8 {
    #[inline(always)]
    fn from(variant: Week) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Week {
    type Ux = u8;
}
impl crate::IsEnum for Week {}
#[doc = "Field `WEEK` reader - Day-of-Week Counting"]
pub type WeekR = crate::FieldReader<Week>;
impl WeekR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Week {
        match self.bits {
            0 => Week::_000,
            1 => Week::_001,
            2 => Week::_010,
            3 => Week::_011,
            4 => Week::_100,
            5 => Week::_101,
            6 => Week::_110,
            7 => Week::Others,
            _ => unreachable!(),
        }
    }
    #[doc = "Sunday"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Week::_000
    }
    #[doc = "Monday"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Week::_001
    }
    #[doc = "Tuesday"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Week::_010
    }
    #[doc = "Wednesday"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Week::_011
    }
    #[doc = "Thursday"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Week::_100
    }
    #[doc = "Friday"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Week::_101
    }
    #[doc = "Saturday"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Week::_110
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == Week::Others
    }
}
#[doc = "Field `WEEK` writer - Day-of-Week Counting"]
pub type WeekW<'a, REG> = crate::FieldWriter<'a, REG, 3, Week, crate::Safe>;
impl<'a, REG> WeekW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sunday"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Week::_000)
    }
    #[doc = "Monday"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Week::_001)
    }
    #[doc = "Tuesday"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Week::_010)
    }
    #[doc = "Wednesday"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Week::_011)
    }
    #[doc = "Thursday"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Week::_100)
    }
    #[doc = "Friday"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Week::_101)
    }
    #[doc = "Saturday"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Week::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Week::Others)
    }
}
impl R {
    #[doc = "Bits 0:2 - Day-of-Week Counting"]
    #[inline(always)]
    pub fn week(&self) -> WeekR {
        WeekR::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Day-of-Week Counting"]
    #[inline(always)]
    pub fn week(&mut self) -> WeekW<WeekSpec> {
        WeekW::new(self, 0)
    }
}
#[doc = "Day-of-Week Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`week::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`week::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WeekSpec;
impl crate::RegisterSpec for WeekSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`week::R`](R) reader structure"]
impl crate::Readable for WeekSpec {}
#[doc = "`write(|w| ..)` method takes [`week::W`](W) writer structure"]
impl crate::Writable for WeekSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WEEK to value 0"]
impl crate::Resettable for WeekSpec {
    const RESET_VALUE: u8 = 0;
}
