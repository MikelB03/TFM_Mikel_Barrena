#[doc = "Register `KRF` reader"]
pub type R = crate::R<KrfSpec>;
#[doc = "Register `KRF` writer"]
pub type W = crate::W<KrfSpec>;
#[doc = "Key Interrupt Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kif0 {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected"]
    _1 = 1,
}
impl From<Kif0> for bool {
    #[inline(always)]
    fn from(variant: Kif0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KIF0` reader - Key Interrupt Flag n"]
pub type Kif0R = crate::BitReader<Kif0>;
impl Kif0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Kif0 {
        match self.bits {
            false => Kif0::_0,
            true => Kif0::_1,
        }
    }
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Kif0::_0
    }
    #[doc = "Interrupt detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Kif0::_1
    }
}
#[doc = "Field `KIF0` writer - Key Interrupt Flag n"]
pub type Kif0W<'a, REG> = crate::BitWriter<'a, REG, Kif0>;
impl<'a, REG> Kif0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Kif0::_0)
    }
    #[doc = "Interrupt detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Kif0::_1)
    }
}
#[doc = "Key Interrupt Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kif1 {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected"]
    _1 = 1,
}
impl From<Kif1> for bool {
    #[inline(always)]
    fn from(variant: Kif1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KIF1` reader - Key Interrupt Flag n"]
pub type Kif1R = crate::BitReader<Kif1>;
impl Kif1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Kif1 {
        match self.bits {
            false => Kif1::_0,
            true => Kif1::_1,
        }
    }
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Kif1::_0
    }
    #[doc = "Interrupt detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Kif1::_1
    }
}
#[doc = "Field `KIF1` writer - Key Interrupt Flag n"]
pub type Kif1W<'a, REG> = crate::BitWriter<'a, REG, Kif1>;
impl<'a, REG> Kif1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Kif1::_0)
    }
    #[doc = "Interrupt detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Kif1::_1)
    }
}
#[doc = "Key Interrupt Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kif2 {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected"]
    _1 = 1,
}
impl From<Kif2> for bool {
    #[inline(always)]
    fn from(variant: Kif2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KIF2` reader - Key Interrupt Flag n"]
pub type Kif2R = crate::BitReader<Kif2>;
impl Kif2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Kif2 {
        match self.bits {
            false => Kif2::_0,
            true => Kif2::_1,
        }
    }
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Kif2::_0
    }
    #[doc = "Interrupt detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Kif2::_1
    }
}
#[doc = "Field `KIF2` writer - Key Interrupt Flag n"]
pub type Kif2W<'a, REG> = crate::BitWriter<'a, REG, Kif2>;
impl<'a, REG> Kif2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Kif2::_0)
    }
    #[doc = "Interrupt detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Kif2::_1)
    }
}
#[doc = "Key Interrupt Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kif3 {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected"]
    _1 = 1,
}
impl From<Kif3> for bool {
    #[inline(always)]
    fn from(variant: Kif3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KIF3` reader - Key Interrupt Flag n"]
pub type Kif3R = crate::BitReader<Kif3>;
impl Kif3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Kif3 {
        match self.bits {
            false => Kif3::_0,
            true => Kif3::_1,
        }
    }
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Kif3::_0
    }
    #[doc = "Interrupt detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Kif3::_1
    }
}
#[doc = "Field `KIF3` writer - Key Interrupt Flag n"]
pub type Kif3W<'a, REG> = crate::BitWriter<'a, REG, Kif3>;
impl<'a, REG> Kif3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Kif3::_0)
    }
    #[doc = "Interrupt detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Kif3::_1)
    }
}
#[doc = "Key Interrupt Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kif4 {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected"]
    _1 = 1,
}
impl From<Kif4> for bool {
    #[inline(always)]
    fn from(variant: Kif4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KIF4` reader - Key Interrupt Flag n"]
pub type Kif4R = crate::BitReader<Kif4>;
impl Kif4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Kif4 {
        match self.bits {
            false => Kif4::_0,
            true => Kif4::_1,
        }
    }
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Kif4::_0
    }
    #[doc = "Interrupt detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Kif4::_1
    }
}
#[doc = "Field `KIF4` writer - Key Interrupt Flag n"]
pub type Kif4W<'a, REG> = crate::BitWriter<'a, REG, Kif4>;
impl<'a, REG> Kif4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Kif4::_0)
    }
    #[doc = "Interrupt detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Kif4::_1)
    }
}
#[doc = "Key Interrupt Flag n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kif5 {
    #[doc = "0: No interrupt detected"]
    _0 = 0,
    #[doc = "1: Interrupt detected"]
    _1 = 1,
}
impl From<Kif5> for bool {
    #[inline(always)]
    fn from(variant: Kif5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KIF5` reader - Key Interrupt Flag n"]
pub type Kif5R = crate::BitReader<Kif5>;
impl Kif5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Kif5 {
        match self.bits {
            false => Kif5::_0,
            true => Kif5::_1,
        }
    }
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Kif5::_0
    }
    #[doc = "Interrupt detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Kif5::_1
    }
}
#[doc = "Field `KIF5` writer - Key Interrupt Flag n"]
pub type Kif5W<'a, REG> = crate::BitWriter<'a, REG, Kif5>;
impl<'a, REG> Kif5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Kif5::_0)
    }
    #[doc = "Interrupt detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Kif5::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Key Interrupt Flag n"]
    #[inline(always)]
    pub fn kif0(&self) -> Kif0R {
        Kif0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Key Interrupt Flag n"]
    #[inline(always)]
    pub fn kif1(&self) -> Kif1R {
        Kif1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Key Interrupt Flag n"]
    #[inline(always)]
    pub fn kif2(&self) -> Kif2R {
        Kif2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Key Interrupt Flag n"]
    #[inline(always)]
    pub fn kif3(&self) -> Kif3R {
        Kif3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Key Interrupt Flag n"]
    #[inline(always)]
    pub fn kif4(&self) -> Kif4R {
        Kif4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Key Interrupt Flag n"]
    #[inline(always)]
    pub fn kif5(&self) -> Kif5R {
        Kif5R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Key Interrupt Flag n"]
    #[inline(always)]
    pub fn kif0(&mut self) -> Kif0W<KrfSpec> {
        Kif0W::new(self, 0)
    }
    #[doc = "Bit 1 - Key Interrupt Flag n"]
    #[inline(always)]
    pub fn kif1(&mut self) -> Kif1W<KrfSpec> {
        Kif1W::new(self, 1)
    }
    #[doc = "Bit 2 - Key Interrupt Flag n"]
    #[inline(always)]
    pub fn kif2(&mut self) -> Kif2W<KrfSpec> {
        Kif2W::new(self, 2)
    }
    #[doc = "Bit 3 - Key Interrupt Flag n"]
    #[inline(always)]
    pub fn kif3(&mut self) -> Kif3W<KrfSpec> {
        Kif3W::new(self, 3)
    }
    #[doc = "Bit 4 - Key Interrupt Flag n"]
    #[inline(always)]
    pub fn kif4(&mut self) -> Kif4W<KrfSpec> {
        Kif4W::new(self, 4)
    }
    #[doc = "Bit 5 - Key Interrupt Flag n"]
    #[inline(always)]
    pub fn kif5(&mut self) -> Kif5W<KrfSpec> {
        Kif5W::new(self, 5)
    }
}
#[doc = "Key Return Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`krf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`krf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KrfSpec;
impl crate::RegisterSpec for KrfSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`krf::R`](R) reader structure"]
impl crate::Readable for KrfSpec {}
#[doc = "`write(|w| ..)` method takes [`krf::W`](W) writer structure"]
impl crate::Writable for KrfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets KRF to value 0"]
impl crate::Resettable for KrfSpec {
    const RESET_VALUE: u8 = 0;
}
