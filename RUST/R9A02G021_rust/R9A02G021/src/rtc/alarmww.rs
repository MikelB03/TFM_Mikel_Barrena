#[doc = "Register `ALARMWW` reader"]
pub type R = crate::R<AlarmwwSpec>;
#[doc = "Register `ALARMWW` writer"]
pub type W = crate::W<AlarmwwSpec>;
#[doc = "Alarm enabled setting \"Sunday\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ww0 {
    #[doc = "0: Disable alarm settings for that day of the week"]
    _0 = 0,
    #[doc = "1: Enable alarm settings for that day of the week"]
    _1 = 1,
}
impl From<Ww0> for bool {
    #[inline(always)]
    fn from(variant: Ww0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WW0` reader - Alarm enabled setting \"Sunday\""]
pub type Ww0R = crate::BitReader<Ww0>;
impl Ww0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ww0 {
        match self.bits {
            false => Ww0::_0,
            true => Ww0::_1,
        }
    }
    #[doc = "Disable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ww0::_0
    }
    #[doc = "Enable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ww0::_1
    }
}
#[doc = "Field `WW0` writer - Alarm enabled setting \"Sunday\""]
pub type Ww0W<'a, REG> = crate::BitWriter<'a, REG, Ww0>;
impl<'a, REG> Ww0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ww0::_0)
    }
    #[doc = "Enable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ww0::_1)
    }
}
#[doc = "Alarm enabled setting \"Monday\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ww1 {
    #[doc = "0: Disable alarm settings for that day of the week"]
    _0 = 0,
    #[doc = "1: Enable alarm settings for that day of the week"]
    _1 = 1,
}
impl From<Ww1> for bool {
    #[inline(always)]
    fn from(variant: Ww1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WW1` reader - Alarm enabled setting \"Monday\""]
pub type Ww1R = crate::BitReader<Ww1>;
impl Ww1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ww1 {
        match self.bits {
            false => Ww1::_0,
            true => Ww1::_1,
        }
    }
    #[doc = "Disable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ww1::_0
    }
    #[doc = "Enable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ww1::_1
    }
}
#[doc = "Field `WW1` writer - Alarm enabled setting \"Monday\""]
pub type Ww1W<'a, REG> = crate::BitWriter<'a, REG, Ww1>;
impl<'a, REG> Ww1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ww1::_0)
    }
    #[doc = "Enable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ww1::_1)
    }
}
#[doc = "Alarm enabled setting \"Tuesday\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ww2 {
    #[doc = "0: Disable alarm settings for that day of the week"]
    _0 = 0,
    #[doc = "1: Enable alarm settings for that day of the week"]
    _1 = 1,
}
impl From<Ww2> for bool {
    #[inline(always)]
    fn from(variant: Ww2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WW2` reader - Alarm enabled setting \"Tuesday\""]
pub type Ww2R = crate::BitReader<Ww2>;
impl Ww2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ww2 {
        match self.bits {
            false => Ww2::_0,
            true => Ww2::_1,
        }
    }
    #[doc = "Disable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ww2::_0
    }
    #[doc = "Enable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ww2::_1
    }
}
#[doc = "Field `WW2` writer - Alarm enabled setting \"Tuesday\""]
pub type Ww2W<'a, REG> = crate::BitWriter<'a, REG, Ww2>;
impl<'a, REG> Ww2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ww2::_0)
    }
    #[doc = "Enable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ww2::_1)
    }
}
#[doc = "Alarm enabled setting \"Wednesday\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ww3 {
    #[doc = "0: Disable alarm settings for that day of the week"]
    _0 = 0,
    #[doc = "1: Enable alarm settings for that day of the week"]
    _1 = 1,
}
impl From<Ww3> for bool {
    #[inline(always)]
    fn from(variant: Ww3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WW3` reader - Alarm enabled setting \"Wednesday\""]
pub type Ww3R = crate::BitReader<Ww3>;
impl Ww3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ww3 {
        match self.bits {
            false => Ww3::_0,
            true => Ww3::_1,
        }
    }
    #[doc = "Disable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ww3::_0
    }
    #[doc = "Enable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ww3::_1
    }
}
#[doc = "Field `WW3` writer - Alarm enabled setting \"Wednesday\""]
pub type Ww3W<'a, REG> = crate::BitWriter<'a, REG, Ww3>;
impl<'a, REG> Ww3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ww3::_0)
    }
    #[doc = "Enable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ww3::_1)
    }
}
#[doc = "Alarm enabled setting \"Thursday\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ww4 {
    #[doc = "0: Disable alarm settings for that day of the week"]
    _0 = 0,
    #[doc = "1: Enable alarm settings for that day of the week"]
    _1 = 1,
}
impl From<Ww4> for bool {
    #[inline(always)]
    fn from(variant: Ww4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WW4` reader - Alarm enabled setting \"Thursday\""]
pub type Ww4R = crate::BitReader<Ww4>;
impl Ww4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ww4 {
        match self.bits {
            false => Ww4::_0,
            true => Ww4::_1,
        }
    }
    #[doc = "Disable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ww4::_0
    }
    #[doc = "Enable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ww4::_1
    }
}
#[doc = "Field `WW4` writer - Alarm enabled setting \"Thursday\""]
pub type Ww4W<'a, REG> = crate::BitWriter<'a, REG, Ww4>;
impl<'a, REG> Ww4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ww4::_0)
    }
    #[doc = "Enable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ww4::_1)
    }
}
#[doc = "Alarm enabled setting \"Friday\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ww5 {
    #[doc = "0: Disable alarm settings for that day of the week"]
    _0 = 0,
    #[doc = "1: Enable alarm settings for that day of the week"]
    _1 = 1,
}
impl From<Ww5> for bool {
    #[inline(always)]
    fn from(variant: Ww5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WW5` reader - Alarm enabled setting \"Friday\""]
pub type Ww5R = crate::BitReader<Ww5>;
impl Ww5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ww5 {
        match self.bits {
            false => Ww5::_0,
            true => Ww5::_1,
        }
    }
    #[doc = "Disable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ww5::_0
    }
    #[doc = "Enable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ww5::_1
    }
}
#[doc = "Field `WW5` writer - Alarm enabled setting \"Friday\""]
pub type Ww5W<'a, REG> = crate::BitWriter<'a, REG, Ww5>;
impl<'a, REG> Ww5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ww5::_0)
    }
    #[doc = "Enable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ww5::_1)
    }
}
#[doc = "Alarm enabled setting \"Saturday\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ww6 {
    #[doc = "0: Disable alarm settings for that day of the week"]
    _0 = 0,
    #[doc = "1: Enable alarm settings for that day of the week"]
    _1 = 1,
}
impl From<Ww6> for bool {
    #[inline(always)]
    fn from(variant: Ww6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WW6` reader - Alarm enabled setting \"Saturday\""]
pub type Ww6R = crate::BitReader<Ww6>;
impl Ww6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ww6 {
        match self.bits {
            false => Ww6::_0,
            true => Ww6::_1,
        }
    }
    #[doc = "Disable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ww6::_0
    }
    #[doc = "Enable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ww6::_1
    }
}
#[doc = "Field `WW6` writer - Alarm enabled setting \"Saturday\""]
pub type Ww6W<'a, REG> = crate::BitWriter<'a, REG, Ww6>;
impl<'a, REG> Ww6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ww6::_0)
    }
    #[doc = "Enable alarm settings for that day of the week"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ww6::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Alarm enabled setting \"Sunday\""]
    #[inline(always)]
    pub fn ww0(&self) -> Ww0R {
        Ww0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm enabled setting \"Monday\""]
    #[inline(always)]
    pub fn ww1(&self) -> Ww1R {
        Ww1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Alarm enabled setting \"Tuesday\""]
    #[inline(always)]
    pub fn ww2(&self) -> Ww2R {
        Ww2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Alarm enabled setting \"Wednesday\""]
    #[inline(always)]
    pub fn ww3(&self) -> Ww3R {
        Ww3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Alarm enabled setting \"Thursday\""]
    #[inline(always)]
    pub fn ww4(&self) -> Ww4R {
        Ww4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Alarm enabled setting \"Friday\""]
    #[inline(always)]
    pub fn ww5(&self) -> Ww5R {
        Ww5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Alarm enabled setting \"Saturday\""]
    #[inline(always)]
    pub fn ww6(&self) -> Ww6R {
        Ww6R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alarm enabled setting \"Sunday\""]
    #[inline(always)]
    pub fn ww0(&mut self) -> Ww0W<AlarmwwSpec> {
        Ww0W::new(self, 0)
    }
    #[doc = "Bit 1 - Alarm enabled setting \"Monday\""]
    #[inline(always)]
    pub fn ww1(&mut self) -> Ww1W<AlarmwwSpec> {
        Ww1W::new(self, 1)
    }
    #[doc = "Bit 2 - Alarm enabled setting \"Tuesday\""]
    #[inline(always)]
    pub fn ww2(&mut self) -> Ww2W<AlarmwwSpec> {
        Ww2W::new(self, 2)
    }
    #[doc = "Bit 3 - Alarm enabled setting \"Wednesday\""]
    #[inline(always)]
    pub fn ww3(&mut self) -> Ww3W<AlarmwwSpec> {
        Ww3W::new(self, 3)
    }
    #[doc = "Bit 4 - Alarm enabled setting \"Thursday\""]
    #[inline(always)]
    pub fn ww4(&mut self) -> Ww4W<AlarmwwSpec> {
        Ww4W::new(self, 4)
    }
    #[doc = "Bit 5 - Alarm enabled setting \"Friday\""]
    #[inline(always)]
    pub fn ww5(&mut self) -> Ww5W<AlarmwwSpec> {
        Ww5W::new(self, 5)
    }
    #[doc = "Bit 6 - Alarm enabled setting \"Saturday\""]
    #[inline(always)]
    pub fn ww6(&mut self) -> Ww6W<AlarmwwSpec> {
        Ww6W::new(self, 6)
    }
}
#[doc = "Alarm Day-of-Week Register\n\nYou can [`read`](crate::Reg::read) this register and get [`alarmww::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarmww::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlarmwwSpec;
impl crate::RegisterSpec for AlarmwwSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`alarmww::R`](R) reader structure"]
impl crate::Readable for AlarmwwSpec {}
#[doc = "`write(|w| ..)` method takes [`alarmww::W`](W) writer structure"]
impl crate::Writable for AlarmwwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ALARMWW to value 0"]
impl crate::Resettable for AlarmwwSpec {
    const RESET_VALUE: u8 = 0;
}
