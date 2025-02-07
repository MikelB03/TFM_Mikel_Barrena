#[doc = "Register `PCNTR3` writer"]
pub type W = crate::W<Pcntr3Spec>;
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr00 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr00> for bool {
    #[inline(always)]
    fn from(variant: Posr00) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR00` writer - Pin and Pjn Output Set"]
pub type Posr00W<'a, REG> = crate::BitWriter<'a, REG, Posr00>;
impl<'a, REG> Posr00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr00::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr00::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr01 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr01> for bool {
    #[inline(always)]
    fn from(variant: Posr01) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR01` writer - Pin and Pjn Output Set"]
pub type Posr01W<'a, REG> = crate::BitWriter<'a, REG, Posr01>;
impl<'a, REG> Posr01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr01::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr01::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr02 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr02> for bool {
    #[inline(always)]
    fn from(variant: Posr02) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR02` writer - Pin and Pjn Output Set"]
pub type Posr02W<'a, REG> = crate::BitWriter<'a, REG, Posr02>;
impl<'a, REG> Posr02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr02::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr02::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr03 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr03> for bool {
    #[inline(always)]
    fn from(variant: Posr03) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR03` writer - Pin and Pjn Output Set"]
pub type Posr03W<'a, REG> = crate::BitWriter<'a, REG, Posr03>;
impl<'a, REG> Posr03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr03::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr03::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr04 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr04> for bool {
    #[inline(always)]
    fn from(variant: Posr04) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR04` writer - Pin and Pjn Output Set"]
pub type Posr04W<'a, REG> = crate::BitWriter<'a, REG, Posr04>;
impl<'a, REG> Posr04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr04::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr04::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr05 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr05> for bool {
    #[inline(always)]
    fn from(variant: Posr05) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR05` writer - Pin and Pjn Output Set"]
pub type Posr05W<'a, REG> = crate::BitWriter<'a, REG, Posr05>;
impl<'a, REG> Posr05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr05::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr05::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr06 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr06> for bool {
    #[inline(always)]
    fn from(variant: Posr06) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR06` writer - Pin and Pjn Output Set"]
pub type Posr06W<'a, REG> = crate::BitWriter<'a, REG, Posr06>;
impl<'a, REG> Posr06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr06::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr06::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr07 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr07> for bool {
    #[inline(always)]
    fn from(variant: Posr07) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR07` writer - Pin and Pjn Output Set"]
pub type Posr07W<'a, REG> = crate::BitWriter<'a, REG, Posr07>;
impl<'a, REG> Posr07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr07::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr07::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr08 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr08> for bool {
    #[inline(always)]
    fn from(variant: Posr08) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR08` writer - Pin and Pjn Output Set"]
pub type Posr08W<'a, REG> = crate::BitWriter<'a, REG, Posr08>;
impl<'a, REG> Posr08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr08::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr08::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr09 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr09> for bool {
    #[inline(always)]
    fn from(variant: Posr09) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR09` writer - Pin and Pjn Output Set"]
pub type Posr09W<'a, REG> = crate::BitWriter<'a, REG, Posr09>;
impl<'a, REG> Posr09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr09::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr09::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr10 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr10> for bool {
    #[inline(always)]
    fn from(variant: Posr10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR10` writer - Pin and Pjn Output Set"]
pub type Posr10W<'a, REG> = crate::BitWriter<'a, REG, Posr10>;
impl<'a, REG> Posr10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr10::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr10::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr11 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr11> for bool {
    #[inline(always)]
    fn from(variant: Posr11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR11` writer - Pin and Pjn Output Set"]
pub type Posr11W<'a, REG> = crate::BitWriter<'a, REG, Posr11>;
impl<'a, REG> Posr11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr11::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr11::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr12 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr12> for bool {
    #[inline(always)]
    fn from(variant: Posr12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR12` writer - Pin and Pjn Output Set"]
pub type Posr12W<'a, REG> = crate::BitWriter<'a, REG, Posr12>;
impl<'a, REG> Posr12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr12::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr12::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr13 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr13> for bool {
    #[inline(always)]
    fn from(variant: Posr13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR13` writer - Pin and Pjn Output Set"]
pub type Posr13W<'a, REG> = crate::BitWriter<'a, REG, Posr13>;
impl<'a, REG> Posr13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr13::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr13::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr14 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr14> for bool {
    #[inline(always)]
    fn from(variant: Posr14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR14` writer - Pin and Pjn Output Set"]
pub type Posr14W<'a, REG> = crate::BitWriter<'a, REG, Posr14>;
impl<'a, REG> Posr14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr14::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr14::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr15 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr15> for bool {
    #[inline(always)]
    fn from(variant: Posr15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR15` writer - Pin and Pjn Output Set"]
pub type Posr15W<'a, REG> = crate::BitWriter<'a, REG, Posr15>;
impl<'a, REG> Posr15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr15::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr15::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr00 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr00> for bool {
    #[inline(always)]
    fn from(variant: Porr00) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR00` writer - Pin and Pjn Output Reset"]
pub type Porr00W<'a, REG> = crate::BitWriter<'a, REG, Porr00>;
impl<'a, REG> Porr00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr00::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr00::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr01 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr01> for bool {
    #[inline(always)]
    fn from(variant: Porr01) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR01` writer - Pin and Pjn Output Reset"]
pub type Porr01W<'a, REG> = crate::BitWriter<'a, REG, Porr01>;
impl<'a, REG> Porr01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr01::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr01::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr02 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr02> for bool {
    #[inline(always)]
    fn from(variant: Porr02) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR02` writer - Pin and Pjn Output Reset"]
pub type Porr02W<'a, REG> = crate::BitWriter<'a, REG, Porr02>;
impl<'a, REG> Porr02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr02::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr02::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr03 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr03> for bool {
    #[inline(always)]
    fn from(variant: Porr03) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR03` writer - Pin and Pjn Output Reset"]
pub type Porr03W<'a, REG> = crate::BitWriter<'a, REG, Porr03>;
impl<'a, REG> Porr03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr03::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr03::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr04 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr04> for bool {
    #[inline(always)]
    fn from(variant: Porr04) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR04` writer - Pin and Pjn Output Reset"]
pub type Porr04W<'a, REG> = crate::BitWriter<'a, REG, Porr04>;
impl<'a, REG> Porr04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr04::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr04::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr05 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr05> for bool {
    #[inline(always)]
    fn from(variant: Porr05) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR05` writer - Pin and Pjn Output Reset"]
pub type Porr05W<'a, REG> = crate::BitWriter<'a, REG, Porr05>;
impl<'a, REG> Porr05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr05::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr05::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr06 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr06> for bool {
    #[inline(always)]
    fn from(variant: Porr06) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR06` writer - Pin and Pjn Output Reset"]
pub type Porr06W<'a, REG> = crate::BitWriter<'a, REG, Porr06>;
impl<'a, REG> Porr06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr06::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr06::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr07 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr07> for bool {
    #[inline(always)]
    fn from(variant: Porr07) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR07` writer - Pin and Pjn Output Reset"]
pub type Porr07W<'a, REG> = crate::BitWriter<'a, REG, Porr07>;
impl<'a, REG> Porr07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr07::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr07::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr08 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr08> for bool {
    #[inline(always)]
    fn from(variant: Porr08) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR08` writer - Pin and Pjn Output Reset"]
pub type Porr08W<'a, REG> = crate::BitWriter<'a, REG, Porr08>;
impl<'a, REG> Porr08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr08::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr08::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr09 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr09> for bool {
    #[inline(always)]
    fn from(variant: Porr09) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR09` writer - Pin and Pjn Output Reset"]
pub type Porr09W<'a, REG> = crate::BitWriter<'a, REG, Porr09>;
impl<'a, REG> Porr09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr09::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr09::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr10 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr10> for bool {
    #[inline(always)]
    fn from(variant: Porr10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR10` writer - Pin and Pjn Output Reset"]
pub type Porr10W<'a, REG> = crate::BitWriter<'a, REG, Porr10>;
impl<'a, REG> Porr10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr10::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr10::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr11 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr11> for bool {
    #[inline(always)]
    fn from(variant: Porr11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR11` writer - Pin and Pjn Output Reset"]
pub type Porr11W<'a, REG> = crate::BitWriter<'a, REG, Porr11>;
impl<'a, REG> Porr11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr11::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr11::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr12 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr12> for bool {
    #[inline(always)]
    fn from(variant: Porr12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR12` writer - Pin and Pjn Output Reset"]
pub type Porr12W<'a, REG> = crate::BitWriter<'a, REG, Porr12>;
impl<'a, REG> Porr12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr12::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr12::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr13 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr13> for bool {
    #[inline(always)]
    fn from(variant: Porr13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR13` writer - Pin and Pjn Output Reset"]
pub type Porr13W<'a, REG> = crate::BitWriter<'a, REG, Porr13>;
impl<'a, REG> Porr13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr13::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr13::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr14 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr14> for bool {
    #[inline(always)]
    fn from(variant: Porr14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR14` writer - Pin and Pjn Output Reset"]
pub type Porr14W<'a, REG> = crate::BitWriter<'a, REG, Porr14>;
impl<'a, REG> Porr14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr14::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr14::_1)
    }
}
#[doc = "Pin and Pjn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porr15 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Porr15> for bool {
    #[inline(always)]
    fn from(variant: Porr15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORR15` writer - Pin and Pjn Output Reset"]
pub type Porr15W<'a, REG> = crate::BitWriter<'a, REG, Porr15>;
impl<'a, REG> Porr15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr15::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr15::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr00(&mut self) -> Posr00W<Pcntr3Spec> {
        Posr00W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr01(&mut self) -> Posr01W<Pcntr3Spec> {
        Posr01W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr02(&mut self) -> Posr02W<Pcntr3Spec> {
        Posr02W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr03(&mut self) -> Posr03W<Pcntr3Spec> {
        Posr03W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr04(&mut self) -> Posr04W<Pcntr3Spec> {
        Posr04W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr05(&mut self) -> Posr05W<Pcntr3Spec> {
        Posr05W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr06(&mut self) -> Posr06W<Pcntr3Spec> {
        Posr06W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr07(&mut self) -> Posr07W<Pcntr3Spec> {
        Posr07W::new(self, 7)
    }
    #[doc = "Bit 8 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr08(&mut self) -> Posr08W<Pcntr3Spec> {
        Posr08W::new(self, 8)
    }
    #[doc = "Bit 9 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr09(&mut self) -> Posr09W<Pcntr3Spec> {
        Posr09W::new(self, 9)
    }
    #[doc = "Bit 10 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr10(&mut self) -> Posr10W<Pcntr3Spec> {
        Posr10W::new(self, 10)
    }
    #[doc = "Bit 11 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr11(&mut self) -> Posr11W<Pcntr3Spec> {
        Posr11W::new(self, 11)
    }
    #[doc = "Bit 12 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr12(&mut self) -> Posr12W<Pcntr3Spec> {
        Posr12W::new(self, 12)
    }
    #[doc = "Bit 13 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr13(&mut self) -> Posr13W<Pcntr3Spec> {
        Posr13W::new(self, 13)
    }
    #[doc = "Bit 14 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr14(&mut self) -> Posr14W<Pcntr3Spec> {
        Posr14W::new(self, 14)
    }
    #[doc = "Bit 15 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr15(&mut self) -> Posr15W<Pcntr3Spec> {
        Posr15W::new(self, 15)
    }
    #[doc = "Bit 16 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr00(&mut self) -> Porr00W<Pcntr3Spec> {
        Porr00W::new(self, 16)
    }
    #[doc = "Bit 17 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr01(&mut self) -> Porr01W<Pcntr3Spec> {
        Porr01W::new(self, 17)
    }
    #[doc = "Bit 18 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr02(&mut self) -> Porr02W<Pcntr3Spec> {
        Porr02W::new(self, 18)
    }
    #[doc = "Bit 19 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr03(&mut self) -> Porr03W<Pcntr3Spec> {
        Porr03W::new(self, 19)
    }
    #[doc = "Bit 20 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr04(&mut self) -> Porr04W<Pcntr3Spec> {
        Porr04W::new(self, 20)
    }
    #[doc = "Bit 21 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr05(&mut self) -> Porr05W<Pcntr3Spec> {
        Porr05W::new(self, 21)
    }
    #[doc = "Bit 22 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr06(&mut self) -> Porr06W<Pcntr3Spec> {
        Porr06W::new(self, 22)
    }
    #[doc = "Bit 23 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr07(&mut self) -> Porr07W<Pcntr3Spec> {
        Porr07W::new(self, 23)
    }
    #[doc = "Bit 24 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr08(&mut self) -> Porr08W<Pcntr3Spec> {
        Porr08W::new(self, 24)
    }
    #[doc = "Bit 25 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr09(&mut self) -> Porr09W<Pcntr3Spec> {
        Porr09W::new(self, 25)
    }
    #[doc = "Bit 26 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr10(&mut self) -> Porr10W<Pcntr3Spec> {
        Porr10W::new(self, 26)
    }
    #[doc = "Bit 27 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr11(&mut self) -> Porr11W<Pcntr3Spec> {
        Porr11W::new(self, 27)
    }
    #[doc = "Bit 28 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr12(&mut self) -> Porr12W<Pcntr3Spec> {
        Porr12W::new(self, 28)
    }
    #[doc = "Bit 29 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr13(&mut self) -> Porr13W<Pcntr3Spec> {
        Porr13W::new(self, 29)
    }
    #[doc = "Bit 30 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr14(&mut self) -> Porr14W<Pcntr3Spec> {
        Porr14W::new(self, 30)
    }
    #[doc = "Bit 31 - Pin and Pjn Output Reset"]
    #[inline(always)]
    pub fn porr15(&mut self) -> Porr15W<Pcntr3Spec> {
        Porr15W::new(self, 31)
    }
}
#[doc = "Port Control Register 3\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcntr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcntr3Spec;
impl crate::RegisterSpec for Pcntr3Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pcntr3::W`](W) writer structure"]
impl crate::Writable for Pcntr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCNTR3 to value 0"]
impl crate::Resettable for Pcntr3Spec {
    const RESET_VALUE: u32 = 0;
}
