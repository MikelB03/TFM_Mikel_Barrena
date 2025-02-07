#[doc = "Register `HOCOCR2` reader"]
pub type R = crate::R<Hococr2Spec>;
#[doc = "Register `HOCOCR2` writer"]
pub type W = crate::W<Hococr2Spec>;
#[doc = "HOCO Frequency Setting 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hcfrq1 {
    #[doc = "0: 24 MHz"]
    _000 = 0,
    #[doc = "2: 32 MHz"]
    _010 = 2,
    #[doc = "4: 48 MHz"]
    _100 = 4,
    #[doc = "1: Setting prohibited"]
    Others = 1,
}
impl From<Hcfrq1> for u8 {
    #[inline(always)]
    fn from(variant: Hcfrq1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hcfrq1 {
    type Ux = u8;
}
impl crate::IsEnum for Hcfrq1 {}
#[doc = "Field `HCFRQ1` reader - HOCO Frequency Setting 1"]
pub type Hcfrq1R = crate::FieldReader<Hcfrq1>;
impl Hcfrq1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hcfrq1 {
        match self.bits {
            0 => Hcfrq1::_000,
            2 => Hcfrq1::_010,
            4 => Hcfrq1::_100,
            _ => Hcfrq1::Others,
        }
    }
    #[doc = "24 MHz"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Hcfrq1::_000
    }
    #[doc = "32 MHz"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Hcfrq1::_010
    }
    #[doc = "48 MHz"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Hcfrq1::_100
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Hcfrq1::Others)
    }
}
#[doc = "Field `HCFRQ1` writer - HOCO Frequency Setting 1"]
pub type Hcfrq1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Hcfrq1, crate::Safe>;
impl<'a, REG> Hcfrq1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "24 MHz"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Hcfrq1::_000)
    }
    #[doc = "32 MHz"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Hcfrq1::_010)
    }
    #[doc = "48 MHz"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Hcfrq1::_100)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Hcfrq1::Others)
    }
}
impl R {
    #[doc = "Bits 3:5 - HOCO Frequency Setting 1"]
    #[inline(always)]
    pub fn hcfrq1(&self) -> Hcfrq1R {
        Hcfrq1R::new((self.bits >> 3) & 7)
    }
}
impl W {
    #[doc = "Bits 3:5 - HOCO Frequency Setting 1"]
    #[inline(always)]
    pub fn hcfrq1(&mut self) -> Hcfrq1W<Hococr2Spec> {
        Hcfrq1W::new(self, 3)
    }
}
#[doc = "High-Speed On-Chip Oscillator Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`hococr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hococr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hococr2Spec;
impl crate::RegisterSpec for Hococr2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hococr2::R`](R) reader structure"]
impl crate::Readable for Hococr2Spec {}
#[doc = "`write(|w| ..)` method takes [`hococr2::W`](W) writer structure"]
impl crate::Writable for Hococr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HOCOCR2 to value 0"]
impl crate::Resettable for Hococr2Spec {
    const RESET_VALUE: u8 = 0;
}
