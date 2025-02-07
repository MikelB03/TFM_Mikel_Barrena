#[doc = "Register `ITLFDIV00` reader"]
pub type R = crate::R<Itlfdiv00Spec>;
#[doc = "Register `ITLFDIV00` writer"]
pub type W = crate::W<Itlfdiv00Spec>;
#[doc = "8-bit counter mode: Counter clock for ITL000\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fdiv0 {
    #[doc = "0: fITL0"]
    _000 = 0,
    #[doc = "1: fITL0/2"]
    _001 = 1,
    #[doc = "2: fITL0/4"]
    _010 = 2,
    #[doc = "3: fITL0/8"]
    _011 = 3,
    #[doc = "4: fITL0/16"]
    _100 = 4,
    #[doc = "5: fITL0/32"]
    _101 = 5,
    #[doc = "6: fITL0/64"]
    _110 = 6,
    #[doc = "7: fITL0/128"]
    _111 = 7,
}
impl From<Fdiv0> for u8 {
    #[inline(always)]
    fn from(variant: Fdiv0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fdiv0 {
    type Ux = u8;
}
impl crate::IsEnum for Fdiv0 {}
#[doc = "Field `FDIV0` reader - 8-bit counter mode: Counter clock for ITL000"]
pub type Fdiv0R = crate::FieldReader<Fdiv0>;
impl Fdiv0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fdiv0 {
        match self.bits {
            0 => Fdiv0::_000,
            1 => Fdiv0::_001,
            2 => Fdiv0::_010,
            3 => Fdiv0::_011,
            4 => Fdiv0::_100,
            5 => Fdiv0::_101,
            6 => Fdiv0::_110,
            7 => Fdiv0::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "fITL0"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Fdiv0::_000
    }
    #[doc = "fITL0/2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Fdiv0::_001
    }
    #[doc = "fITL0/4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Fdiv0::_010
    }
    #[doc = "fITL0/8"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Fdiv0::_011
    }
    #[doc = "fITL0/16"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Fdiv0::_100
    }
    #[doc = "fITL0/32"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Fdiv0::_101
    }
    #[doc = "fITL0/64"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Fdiv0::_110
    }
    #[doc = "fITL0/128"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Fdiv0::_111
    }
}
#[doc = "Field `FDIV0` writer - 8-bit counter mode: Counter clock for ITL000"]
pub type Fdiv0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fdiv0, crate::Safe>;
impl<'a, REG> Fdiv0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fITL0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv0::_000)
    }
    #[doc = "fITL0/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv0::_001)
    }
    #[doc = "fITL0/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv0::_010)
    }
    #[doc = "fITL0/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv0::_011)
    }
    #[doc = "fITL0/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv0::_100)
    }
    #[doc = "fITL0/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv0::_101)
    }
    #[doc = "fITL0/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv0::_110)
    }
    #[doc = "fITL0/128"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv0::_111)
    }
}
#[doc = "8-bit counter mode: Counter clock for ITL001\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fdiv1 {
    #[doc = "0: fITL0"]
    _000 = 0,
    #[doc = "1: fITL0/2"]
    _001 = 1,
    #[doc = "2: fITL0/4"]
    _010 = 2,
    #[doc = "3: fITL0/8"]
    _011 = 3,
    #[doc = "4: fITL0/16"]
    _100 = 4,
    #[doc = "5: fITL0/32"]
    _101 = 5,
    #[doc = "6: fITL0/64"]
    _110 = 6,
    #[doc = "7: fITL0/128"]
    _111 = 7,
}
impl From<Fdiv1> for u8 {
    #[inline(always)]
    fn from(variant: Fdiv1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fdiv1 {
    type Ux = u8;
}
impl crate::IsEnum for Fdiv1 {}
#[doc = "Field `FDIV1` reader - 8-bit counter mode: Counter clock for ITL001"]
pub type Fdiv1R = crate::FieldReader<Fdiv1>;
impl Fdiv1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fdiv1 {
        match self.bits {
            0 => Fdiv1::_000,
            1 => Fdiv1::_001,
            2 => Fdiv1::_010,
            3 => Fdiv1::_011,
            4 => Fdiv1::_100,
            5 => Fdiv1::_101,
            6 => Fdiv1::_110,
            7 => Fdiv1::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "fITL0"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Fdiv1::_000
    }
    #[doc = "fITL0/2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Fdiv1::_001
    }
    #[doc = "fITL0/4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Fdiv1::_010
    }
    #[doc = "fITL0/8"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Fdiv1::_011
    }
    #[doc = "fITL0/16"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Fdiv1::_100
    }
    #[doc = "fITL0/32"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Fdiv1::_101
    }
    #[doc = "fITL0/64"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Fdiv1::_110
    }
    #[doc = "fITL0/128"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Fdiv1::_111
    }
}
#[doc = "Field `FDIV1` writer - 8-bit counter mode: Counter clock for ITL001"]
pub type Fdiv1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fdiv1, crate::Safe>;
impl<'a, REG> Fdiv1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fITL0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv1::_000)
    }
    #[doc = "fITL0/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv1::_001)
    }
    #[doc = "fITL0/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv1::_010)
    }
    #[doc = "fITL0/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv1::_011)
    }
    #[doc = "fITL0/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv1::_100)
    }
    #[doc = "fITL0/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv1::_101)
    }
    #[doc = "fITL0/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv1::_110)
    }
    #[doc = "fITL0/128"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Fdiv1::_111)
    }
}
impl R {
    #[doc = "Bits 0:2 - 8-bit counter mode: Counter clock for ITL000"]
    #[inline(always)]
    pub fn fdiv0(&self) -> Fdiv0R {
        Fdiv0R::new(self.bits & 7)
    }
    #[doc = "Bits 4:6 - 8-bit counter mode: Counter clock for ITL001"]
    #[inline(always)]
    pub fn fdiv1(&self) -> Fdiv1R {
        Fdiv1R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - 8-bit counter mode: Counter clock for ITL000"]
    #[inline(always)]
    pub fn fdiv0(&mut self) -> Fdiv0W<Itlfdiv00Spec> {
        Fdiv0W::new(self, 0)
    }
    #[doc = "Bits 4:6 - 8-bit counter mode: Counter clock for ITL001"]
    #[inline(always)]
    pub fn fdiv1(&mut self) -> Fdiv1W<Itlfdiv00Spec> {
        Fdiv1W::new(self, 4)
    }
}
#[doc = "Interval Timer Frequency Division Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`itlfdiv00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itlfdiv00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itlfdiv00Spec;
impl crate::RegisterSpec for Itlfdiv00Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`itlfdiv00::R`](R) reader structure"]
impl crate::Readable for Itlfdiv00Spec {}
#[doc = "`write(|w| ..)` method takes [`itlfdiv00::W`](W) writer structure"]
impl crate::Writable for Itlfdiv00Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ITLFDIV00 to value 0"]
impl crate::Resettable for Itlfdiv00Spec {
    const RESET_VALUE: u8 = 0;
}
