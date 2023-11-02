#[derive(Clone, Debug)]
pub enum PrimarySubtype {
    Rifle,
    Shotgun,
}

#[derive(Clone, Debug)]
pub enum MeleeSubtype {
    Nikana,
    Staff,
}

#[derive(Clone, Debug)]
pub enum WeaponType {
    Primary {
        subtype: Option<PrimarySubtype>,
        kitgun: bool,
    },
    Secondary {
        // subtype: Option<SecondarySubtype>,
        kitgun: bool,
    },
    Melee {
        subtype: Option<MeleeSubtype>,
        zaw: bool,
    },
}

#[derive(Clone, Debug)]
pub enum CompanionType {
    Kavat,
    Kubrow,
    Moa,
}

#[derive(Clone, Debug)]
pub enum VehicleType {
    Archwing,
    KDrive,
    Kaithe,
}

#[derive(Clone, Debug)]
pub enum EqType {
    Warframe,
    Weapon(WeaponType),
    Companion,
    Vehicle(VehicleType),
}
