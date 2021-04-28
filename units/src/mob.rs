// Copyright 2021 MobileCoin Foundation

#[repr(transparent)]
pub struct PicoMob(u64);

#[repr(transparent)]
pub struct NanoMob(PicoMob);

#[repr(transparent)]
pub struct MicroMob(PicoMob);

#[repr(transparent)]
pub struct MilliMob(PicoMob);

#[repr(transparent)]
pub struct Mob(PicoMob);

#[repr(transparent)]
pub struct KiloMob(PicoMob);

#[repr(transparent)]
pub struct MegaMob(PicoMob);

#[repr(transparent)]
pub struct GigaMob(PicoMob);
