#[doc = "Register `EOSR` reader"]
pub type R = crate::R<EosrSpec>;
#[doc = "Register `EOSR` writer"]
pub type W = crate::W<EosrSpec>;
#[doc = "Pin and Pjn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr00 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Eosr00> for bool {
    #[inline(always)]
    fn from(variant: Eosr00) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSR00` reader - Pin and Pjn Event Output Set"]
pub type Eosr00R = crate::BitReader<Eosr00>;
impl Eosr00R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eosr00 {
        match self.bits {
            false => Eosr00::_0,
            true => Eosr00::_1,
        }
    }
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr00::_0
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr00::_1
    }
}
#[doc = "Field `EOSR00` writer - Pin and Pjn Event Output Set"]
pub type Eosr00W<'a, REG> = crate::BitWriter<'a, REG, Eosr00>;
impl<'a, REG> Eosr00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr00::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr00::_1)
    }
}
#[doc = "Pin and Pjn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr01 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Eosr01> for bool {
    #[inline(always)]
    fn from(variant: Eosr01) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSR01` reader - Pin and Pjn Event Output Set"]
pub type Eosr01R = crate::BitReader<Eosr01>;
impl Eosr01R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eosr01 {
        match self.bits {
            false => Eosr01::_0,
            true => Eosr01::_1,
        }
    }
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr01::_0
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr01::_1
    }
}
#[doc = "Field `EOSR01` writer - Pin and Pjn Event Output Set"]
pub type Eosr01W<'a, REG> = crate::BitWriter<'a, REG, Eosr01>;
impl<'a, REG> Eosr01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr01::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr01::_1)
    }
}
#[doc = "Pin and Pjn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr02 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Eosr02> for bool {
    #[inline(always)]
    fn from(variant: Eosr02) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSR02` reader - Pin and Pjn Event Output Set"]
pub type Eosr02R = crate::BitReader<Eosr02>;
impl Eosr02R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eosr02 {
        match self.bits {
            false => Eosr02::_0,
            true => Eosr02::_1,
        }
    }
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr02::_0
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr02::_1
    }
}
#[doc = "Field `EOSR02` writer - Pin and Pjn Event Output Set"]
pub type Eosr02W<'a, REG> = crate::BitWriter<'a, REG, Eosr02>;
impl<'a, REG> Eosr02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr02::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr02::_1)
    }
}
#[doc = "Pin and Pjn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr03 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Eosr03> for bool {
    #[inline(always)]
    fn from(variant: Eosr03) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSR03` reader - Pin and Pjn Event Output Set"]
pub type Eosr03R = crate::BitReader<Eosr03>;
impl Eosr03R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eosr03 {
        match self.bits {
            false => Eosr03::_0,
            true => Eosr03::_1,
        }
    }
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr03::_0
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr03::_1
    }
}
#[doc = "Field `EOSR03` writer - Pin and Pjn Event Output Set"]
pub type Eosr03W<'a, REG> = crate::BitWriter<'a, REG, Eosr03>;
impl<'a, REG> Eosr03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr03::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr03::_1)
    }
}
#[doc = "Pin and Pjn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr04 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Eosr04> for bool {
    #[inline(always)]
    fn from(variant: Eosr04) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSR04` reader - Pin and Pjn Event Output Set"]
pub type Eosr04R = crate::BitReader<Eosr04>;
impl Eosr04R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eosr04 {
        match self.bits {
            false => Eosr04::_0,
            true => Eosr04::_1,
        }
    }
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr04::_0
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr04::_1
    }
}
#[doc = "Field `EOSR04` writer - Pin and Pjn Event Output Set"]
pub type Eosr04W<'a, REG> = crate::BitWriter<'a, REG, Eosr04>;
impl<'a, REG> Eosr04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr04::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr04::_1)
    }
}
#[doc = "Pin and Pjn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr05 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Eosr05> for bool {
    #[inline(always)]
    fn from(variant: Eosr05) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSR05` reader - Pin and Pjn Event Output Set"]
pub type Eosr05R = crate::BitReader<Eosr05>;
impl Eosr05R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eosr05 {
        match self.bits {
            false => Eosr05::_0,
            true => Eosr05::_1,
        }
    }
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr05::_0
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr05::_1
    }
}
#[doc = "Field `EOSR05` writer - Pin and Pjn Event Output Set"]
pub type Eosr05W<'a, REG> = crate::BitWriter<'a, REG, Eosr05>;
impl<'a, REG> Eosr05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr05::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr05::_1)
    }
}
#[doc = "Pin and Pjn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr06 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Eosr06> for bool {
    #[inline(always)]
    fn from(variant: Eosr06) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSR06` reader - Pin and Pjn Event Output Set"]
pub type Eosr06R = crate::BitReader<Eosr06>;
impl Eosr06R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eosr06 {
        match self.bits {
            false => Eosr06::_0,
            true => Eosr06::_1,
        }
    }
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr06::_0
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr06::_1
    }
}
#[doc = "Field `EOSR06` writer - Pin and Pjn Event Output Set"]
pub type Eosr06W<'a, REG> = crate::BitWriter<'a, REG, Eosr06>;
impl<'a, REG> Eosr06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr06::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr06::_1)
    }
}
#[doc = "Pin and Pjn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr07 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Eosr07> for bool {
    #[inline(always)]
    fn from(variant: Eosr07) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSR07` reader - Pin and Pjn Event Output Set"]
pub type Eosr07R = crate::BitReader<Eosr07>;
impl Eosr07R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eosr07 {
        match self.bits {
            false => Eosr07::_0,
            true => Eosr07::_1,
        }
    }
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr07::_0
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr07::_1
    }
}
#[doc = "Field `EOSR07` writer - Pin and Pjn Event Output Set"]
pub type Eosr07W<'a, REG> = crate::BitWriter<'a, REG, Eosr07>;
impl<'a, REG> Eosr07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr07::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr07::_1)
    }
}
#[doc = "Pin and Pjn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr08 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Eosr08> for bool {
    #[inline(always)]
    fn from(variant: Eosr08) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSR08` reader - Pin and Pjn Event Output Set"]
pub type Eosr08R = crate::BitReader<Eosr08>;
impl Eosr08R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eosr08 {
        match self.bits {
            false => Eosr08::_0,
            true => Eosr08::_1,
        }
    }
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr08::_0
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr08::_1
    }
}
#[doc = "Field `EOSR08` writer - Pin and Pjn Event Output Set"]
pub type Eosr08W<'a, REG> = crate::BitWriter<'a, REG, Eosr08>;
impl<'a, REG> Eosr08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr08::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr08::_1)
    }
}
#[doc = "Pin and Pjn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr09 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Eosr09> for bool {
    #[inline(always)]
    fn from(variant: Eosr09) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSR09` reader - Pin and Pjn Event Output Set"]
pub type Eosr09R = crate::BitReader<Eosr09>;
impl Eosr09R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eosr09 {
        match self.bits {
            false => Eosr09::_0,
            true => Eosr09::_1,
        }
    }
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr09::_0
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr09::_1
    }
}
#[doc = "Field `EOSR09` writer - Pin and Pjn Event Output Set"]
pub type Eosr09W<'a, REG> = crate::BitWriter<'a, REG, Eosr09>;
impl<'a, REG> Eosr09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr09::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr09::_1)
    }
}
#[doc = "Pin and Pjn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr10 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Eosr10> for bool {
    #[inline(always)]
    fn from(variant: Eosr10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSR10` reader - Pin and Pjn Event Output Set"]
pub type Eosr10R = crate::BitReader<Eosr10>;
impl Eosr10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eosr10 {
        match self.bits {
            false => Eosr10::_0,
            true => Eosr10::_1,
        }
    }
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr10::_0
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr10::_1
    }
}
#[doc = "Field `EOSR10` writer - Pin and Pjn Event Output Set"]
pub type Eosr10W<'a, REG> = crate::BitWriter<'a, REG, Eosr10>;
impl<'a, REG> Eosr10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr10::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr10::_1)
    }
}
#[doc = "Pin and Pjn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr11 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Eosr11> for bool {
    #[inline(always)]
    fn from(variant: Eosr11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSR11` reader - Pin and Pjn Event Output Set"]
pub type Eosr11R = crate::BitReader<Eosr11>;
impl Eosr11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eosr11 {
        match self.bits {
            false => Eosr11::_0,
            true => Eosr11::_1,
        }
    }
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr11::_0
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr11::_1
    }
}
#[doc = "Field `EOSR11` writer - Pin and Pjn Event Output Set"]
pub type Eosr11W<'a, REG> = crate::BitWriter<'a, REG, Eosr11>;
impl<'a, REG> Eosr11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr11::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr11::_1)
    }
}
#[doc = "Pin and Pjn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr12 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Eosr12> for bool {
    #[inline(always)]
    fn from(variant: Eosr12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSR12` reader - Pin and Pjn Event Output Set"]
pub type Eosr12R = crate::BitReader<Eosr12>;
impl Eosr12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eosr12 {
        match self.bits {
            false => Eosr12::_0,
            true => Eosr12::_1,
        }
    }
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr12::_0
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr12::_1
    }
}
#[doc = "Field `EOSR12` writer - Pin and Pjn Event Output Set"]
pub type Eosr12W<'a, REG> = crate::BitWriter<'a, REG, Eosr12>;
impl<'a, REG> Eosr12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr12::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr12::_1)
    }
}
#[doc = "Pin and Pjn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr13 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Eosr13> for bool {
    #[inline(always)]
    fn from(variant: Eosr13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSR13` reader - Pin and Pjn Event Output Set"]
pub type Eosr13R = crate::BitReader<Eosr13>;
impl Eosr13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eosr13 {
        match self.bits {
            false => Eosr13::_0,
            true => Eosr13::_1,
        }
    }
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr13::_0
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr13::_1
    }
}
#[doc = "Field `EOSR13` writer - Pin and Pjn Event Output Set"]
pub type Eosr13W<'a, REG> = crate::BitWriter<'a, REG, Eosr13>;
impl<'a, REG> Eosr13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr13::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr13::_1)
    }
}
#[doc = "Pin and Pjn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr14 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Eosr14> for bool {
    #[inline(always)]
    fn from(variant: Eosr14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSR14` reader - Pin and Pjn Event Output Set"]
pub type Eosr14R = crate::BitReader<Eosr14>;
impl Eosr14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eosr14 {
        match self.bits {
            false => Eosr14::_0,
            true => Eosr14::_1,
        }
    }
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr14::_0
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr14::_1
    }
}
#[doc = "Field `EOSR14` writer - Pin and Pjn Event Output Set"]
pub type Eosr14W<'a, REG> = crate::BitWriter<'a, REG, Eosr14>;
impl<'a, REG> Eosr14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr14::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr14::_1)
    }
}
#[doc = "Pin and Pjn Event Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eosr15 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Eosr15> for bool {
    #[inline(always)]
    fn from(variant: Eosr15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSR15` reader - Pin and Pjn Event Output Set"]
pub type Eosr15R = crate::BitReader<Eosr15>;
impl Eosr15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eosr15 {
        match self.bits {
            false => Eosr15::_0,
            true => Eosr15::_1,
        }
    }
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eosr15::_0
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eosr15::_1
    }
}
#[doc = "Field `EOSR15` writer - Pin and Pjn Event Output Set"]
pub type Eosr15W<'a, REG> = crate::BitWriter<'a, REG, Eosr15>;
impl<'a, REG> Eosr15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr15::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eosr15::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr00(&self) -> Eosr00R {
        Eosr00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr01(&self) -> Eosr01R {
        Eosr01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr02(&self) -> Eosr02R {
        Eosr02R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr03(&self) -> Eosr03R {
        Eosr03R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr04(&self) -> Eosr04R {
        Eosr04R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr05(&self) -> Eosr05R {
        Eosr05R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr06(&self) -> Eosr06R {
        Eosr06R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr07(&self) -> Eosr07R {
        Eosr07R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr08(&self) -> Eosr08R {
        Eosr08R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr09(&self) -> Eosr09R {
        Eosr09R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr10(&self) -> Eosr10R {
        Eosr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr11(&self) -> Eosr11R {
        Eosr11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr12(&self) -> Eosr12R {
        Eosr12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr13(&self) -> Eosr13R {
        Eosr13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr14(&self) -> Eosr14R {
        Eosr14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr15(&self) -> Eosr15R {
        Eosr15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr00(&mut self) -> Eosr00W<EosrSpec> {
        Eosr00W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr01(&mut self) -> Eosr01W<EosrSpec> {
        Eosr01W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr02(&mut self) -> Eosr02W<EosrSpec> {
        Eosr02W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr03(&mut self) -> Eosr03W<EosrSpec> {
        Eosr03W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr04(&mut self) -> Eosr04W<EosrSpec> {
        Eosr04W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr05(&mut self) -> Eosr05W<EosrSpec> {
        Eosr05W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr06(&mut self) -> Eosr06W<EosrSpec> {
        Eosr06W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr07(&mut self) -> Eosr07W<EosrSpec> {
        Eosr07W::new(self, 7)
    }
    #[doc = "Bit 8 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr08(&mut self) -> Eosr08W<EosrSpec> {
        Eosr08W::new(self, 8)
    }
    #[doc = "Bit 9 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr09(&mut self) -> Eosr09W<EosrSpec> {
        Eosr09W::new(self, 9)
    }
    #[doc = "Bit 10 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr10(&mut self) -> Eosr10W<EosrSpec> {
        Eosr10W::new(self, 10)
    }
    #[doc = "Bit 11 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr11(&mut self) -> Eosr11W<EosrSpec> {
        Eosr11W::new(self, 11)
    }
    #[doc = "Bit 12 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr12(&mut self) -> Eosr12W<EosrSpec> {
        Eosr12W::new(self, 12)
    }
    #[doc = "Bit 13 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr13(&mut self) -> Eosr13W<EosrSpec> {
        Eosr13W::new(self, 13)
    }
    #[doc = "Bit 14 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr14(&mut self) -> Eosr14W<EosrSpec> {
        Eosr14W::new(self, 14)
    }
    #[doc = "Bit 15 - Pin and Pjn Event Output Set"]
    #[inline(always)]
    pub fn eosr15(&mut self) -> Eosr15W<EosrSpec> {
        Eosr15W::new(self, 15)
    }
}
#[doc = "Port Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`eosr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eosr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EosrSpec;
impl crate::RegisterSpec for EosrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`eosr::R`](R) reader structure"]
impl crate::Readable for EosrSpec {}
#[doc = "`write(|w| ..)` method takes [`eosr::W`](W) writer structure"]
impl crate::Writable for EosrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EOSR to value 0"]
impl crate::Resettable for EosrSpec {
    const RESET_VALUE: u16 = 0;
}
