#[doc = "Register `KRM` reader"]
pub type R = crate::R<KrmSpec>;
#[doc = "Register `KRM` writer"]
pub type W = crate::W<KrmSpec>;
#[doc = "Key Interrupt Mode Control n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kimc0 {
    #[doc = "0: Do not detect key interrupt signals"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signals"]
    _1 = 1,
}
impl From<Kimc0> for bool {
    #[inline(always)]
    fn from(variant: Kimc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KIMC0` reader - Key Interrupt Mode Control n"]
pub type Kimc0R = crate::BitReader<Kimc0>;
impl Kimc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Kimc0 {
        match self.bits {
            false => Kimc0::_0,
            true => Kimc0::_1,
        }
    }
    #[doc = "Do not detect key interrupt signals"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Kimc0::_0
    }
    #[doc = "Detect key interrupt signals"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Kimc0::_1
    }
}
#[doc = "Field `KIMC0` writer - Key Interrupt Mode Control n"]
pub type Kimc0W<'a, REG> = crate::BitWriter<'a, REG, Kimc0>;
impl<'a, REG> Kimc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not detect key interrupt signals"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Kimc0::_0)
    }
    #[doc = "Detect key interrupt signals"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Kimc0::_1)
    }
}
#[doc = "Key Interrupt Mode Control n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kimc1 {
    #[doc = "0: Do not detect key interrupt signals"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signals"]
    _1 = 1,
}
impl From<Kimc1> for bool {
    #[inline(always)]
    fn from(variant: Kimc1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KIMC1` reader - Key Interrupt Mode Control n"]
pub type Kimc1R = crate::BitReader<Kimc1>;
impl Kimc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Kimc1 {
        match self.bits {
            false => Kimc1::_0,
            true => Kimc1::_1,
        }
    }
    #[doc = "Do not detect key interrupt signals"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Kimc1::_0
    }
    #[doc = "Detect key interrupt signals"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Kimc1::_1
    }
}
#[doc = "Field `KIMC1` writer - Key Interrupt Mode Control n"]
pub type Kimc1W<'a, REG> = crate::BitWriter<'a, REG, Kimc1>;
impl<'a, REG> Kimc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not detect key interrupt signals"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Kimc1::_0)
    }
    #[doc = "Detect key interrupt signals"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Kimc1::_1)
    }
}
#[doc = "Key Interrupt Mode Control n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kimc2 {
    #[doc = "0: Do not detect key interrupt signals"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signals"]
    _1 = 1,
}
impl From<Kimc2> for bool {
    #[inline(always)]
    fn from(variant: Kimc2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KIMC2` reader - Key Interrupt Mode Control n"]
pub type Kimc2R = crate::BitReader<Kimc2>;
impl Kimc2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Kimc2 {
        match self.bits {
            false => Kimc2::_0,
            true => Kimc2::_1,
        }
    }
    #[doc = "Do not detect key interrupt signals"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Kimc2::_0
    }
    #[doc = "Detect key interrupt signals"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Kimc2::_1
    }
}
#[doc = "Field `KIMC2` writer - Key Interrupt Mode Control n"]
pub type Kimc2W<'a, REG> = crate::BitWriter<'a, REG, Kimc2>;
impl<'a, REG> Kimc2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not detect key interrupt signals"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Kimc2::_0)
    }
    #[doc = "Detect key interrupt signals"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Kimc2::_1)
    }
}
#[doc = "Key Interrupt Mode Control n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kimc3 {
    #[doc = "0: Do not detect key interrupt signals"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signals"]
    _1 = 1,
}
impl From<Kimc3> for bool {
    #[inline(always)]
    fn from(variant: Kimc3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KIMC3` reader - Key Interrupt Mode Control n"]
pub type Kimc3R = crate::BitReader<Kimc3>;
impl Kimc3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Kimc3 {
        match self.bits {
            false => Kimc3::_0,
            true => Kimc3::_1,
        }
    }
    #[doc = "Do not detect key interrupt signals"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Kimc3::_0
    }
    #[doc = "Detect key interrupt signals"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Kimc3::_1
    }
}
#[doc = "Field `KIMC3` writer - Key Interrupt Mode Control n"]
pub type Kimc3W<'a, REG> = crate::BitWriter<'a, REG, Kimc3>;
impl<'a, REG> Kimc3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not detect key interrupt signals"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Kimc3::_0)
    }
    #[doc = "Detect key interrupt signals"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Kimc3::_1)
    }
}
#[doc = "Key Interrupt Mode Control n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kimc4 {
    #[doc = "0: Do not detect key interrupt signals"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signals"]
    _1 = 1,
}
impl From<Kimc4> for bool {
    #[inline(always)]
    fn from(variant: Kimc4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KIMC4` reader - Key Interrupt Mode Control n"]
pub type Kimc4R = crate::BitReader<Kimc4>;
impl Kimc4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Kimc4 {
        match self.bits {
            false => Kimc4::_0,
            true => Kimc4::_1,
        }
    }
    #[doc = "Do not detect key interrupt signals"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Kimc4::_0
    }
    #[doc = "Detect key interrupt signals"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Kimc4::_1
    }
}
#[doc = "Field `KIMC4` writer - Key Interrupt Mode Control n"]
pub type Kimc4W<'a, REG> = crate::BitWriter<'a, REG, Kimc4>;
impl<'a, REG> Kimc4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not detect key interrupt signals"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Kimc4::_0)
    }
    #[doc = "Detect key interrupt signals"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Kimc4::_1)
    }
}
#[doc = "Key Interrupt Mode Control n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kimc5 {
    #[doc = "0: Do not detect key interrupt signals"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signals"]
    _1 = 1,
}
impl From<Kimc5> for bool {
    #[inline(always)]
    fn from(variant: Kimc5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KIMC5` reader - Key Interrupt Mode Control n"]
pub type Kimc5R = crate::BitReader<Kimc5>;
impl Kimc5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Kimc5 {
        match self.bits {
            false => Kimc5::_0,
            true => Kimc5::_1,
        }
    }
    #[doc = "Do not detect key interrupt signals"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Kimc5::_0
    }
    #[doc = "Detect key interrupt signals"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Kimc5::_1
    }
}
#[doc = "Field `KIMC5` writer - Key Interrupt Mode Control n"]
pub type Kimc5W<'a, REG> = crate::BitWriter<'a, REG, Kimc5>;
impl<'a, REG> Kimc5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not detect key interrupt signals"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Kimc5::_0)
    }
    #[doc = "Detect key interrupt signals"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Kimc5::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Key Interrupt Mode Control n"]
    #[inline(always)]
    pub fn kimc0(&self) -> Kimc0R {
        Kimc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Key Interrupt Mode Control n"]
    #[inline(always)]
    pub fn kimc1(&self) -> Kimc1R {
        Kimc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Key Interrupt Mode Control n"]
    #[inline(always)]
    pub fn kimc2(&self) -> Kimc2R {
        Kimc2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Key Interrupt Mode Control n"]
    #[inline(always)]
    pub fn kimc3(&self) -> Kimc3R {
        Kimc3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Key Interrupt Mode Control n"]
    #[inline(always)]
    pub fn kimc4(&self) -> Kimc4R {
        Kimc4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Key Interrupt Mode Control n"]
    #[inline(always)]
    pub fn kimc5(&self) -> Kimc5R {
        Kimc5R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Key Interrupt Mode Control n"]
    #[inline(always)]
    pub fn kimc0(&mut self) -> Kimc0W<KrmSpec> {
        Kimc0W::new(self, 0)
    }
    #[doc = "Bit 1 - Key Interrupt Mode Control n"]
    #[inline(always)]
    pub fn kimc1(&mut self) -> Kimc1W<KrmSpec> {
        Kimc1W::new(self, 1)
    }
    #[doc = "Bit 2 - Key Interrupt Mode Control n"]
    #[inline(always)]
    pub fn kimc2(&mut self) -> Kimc2W<KrmSpec> {
        Kimc2W::new(self, 2)
    }
    #[doc = "Bit 3 - Key Interrupt Mode Control n"]
    #[inline(always)]
    pub fn kimc3(&mut self) -> Kimc3W<KrmSpec> {
        Kimc3W::new(self, 3)
    }
    #[doc = "Bit 4 - Key Interrupt Mode Control n"]
    #[inline(always)]
    pub fn kimc4(&mut self) -> Kimc4W<KrmSpec> {
        Kimc4W::new(self, 4)
    }
    #[doc = "Bit 5 - Key Interrupt Mode Control n"]
    #[inline(always)]
    pub fn kimc5(&mut self) -> Kimc5W<KrmSpec> {
        Kimc5W::new(self, 5)
    }
}
#[doc = "Key Return Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`krm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`krm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KrmSpec;
impl crate::RegisterSpec for KrmSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`krm::R`](R) reader structure"]
impl crate::Readable for KrmSpec {}
#[doc = "`write(|w| ..)` method takes [`krm::W`](W) writer structure"]
impl crate::Writable for KrmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets KRM to value 0"]
impl crate::Resettable for KrmSpec {
    const RESET_VALUE: u8 = 0;
}
