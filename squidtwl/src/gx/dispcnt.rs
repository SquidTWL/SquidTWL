use bitfield_struct::bitfield;

// GBATEK makes this pointleessly confusing, so note to self:
// "Text" = Static tilemap background
// "Affine" = controlled by BG rotation and scaling registers
// "Extended" = Affine, but with more palettes.


/** Enumeration of the possible display modes for DISPCNT. */
#[derive(Debug)]
#[repr(u8)]
pub enum DisplayMode {
    /** Turns the display off. */
    Off = 0, 
    /** Regular graphics, controlled by BG + OBJ. */
    GraphicsDisplay = 1,
    /** Engine A only: Framebuffer (LCDC) mode. */
    Framebuffer = 2,
    /** Display A only: Framebuffer (LCDC) via DMA mode. */
    DmaFramebuffer = 3
}

impl DisplayMode {
    const fn into_bits(self) -> u8 {
        return self as u8;
    }

    const fn from_bits(value: u8) -> Self {
        return match value {
            0b00 => DisplayMode::Off,
            0b01 => DisplayMode::GraphicsDisplay,
            0b10 => DisplayMode::Framebuffer,
            0b11 => DisplayMode::DmaFramebuffer,
            _ => unreachable!()
        };
    }
}

/** 
 * Enumeration of the possible background modes for an engine. The name refers to the abilities
 * of background layers 1 through 3. Layer 0 is always either 3D or static.
 * 
 * Note that all modes allow rendering 3D to BG0, not just mode 6.
 */
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BackgroundMode {
    /** Three static, untransformed backgrounds */
    TilesTilesTiles = 0,
    /** Two static backgrounds, one scaled/rotated background */
    TilesTilesAffine = 1,
    /** One static background, two scaled/rotated backgrounds */
    TilesAffineAffine = 2,
    /** Two static backgrounds, one scaled/rotated with extra abilities */
    TilesTilesExtended = 3,
    /** One static background, one scaled/rotated, one scaled/rotated with extra abilities */
    TilesAffineExtended = 4,
    /** One static background, two scaled/rotated with extra abilities */
    TilesExtendedExtendeed = 5,
    /** Engine A only: 3D-only + bitmap background on layer 2 */
    Only3D = 6
}

impl BackgroundMode {
    const fn into_bits(self) -> u8 {
        return self as u8;
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => BackgroundMode::TilesTilesTiles,
            1 => BackgroundMode::TilesTilesAffine,
            2 => BackgroundMode::TilesAffineAffine,
            3 => BackgroundMode::TilesTilesExtended,
            4 => BackgroundMode::TilesAffineExtended,
            5 => BackgroundMode::TilesExtendedExtendeed,
            6 => BackgroundMode::Only3D,
            _ => unreachable!()
        }
    }
}

/** Enumeration of the four VRAM banks for framebuffer mode. */
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Disp2VramBank {
    BankA,
    BankB,
    BankC,
    BankD,
}

impl Disp2VramBank {
    pub const fn into_bits(self) -> u8 {
        return self as u8;
    }

    pub const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::BankA,
            1 => Self::BankB,
            2 => Self::BankC,
            3 => Self::BankD,
            _ => unreachable!()
        }
    }
}

/**
 * Wraps the bits for the Display Control registers.
 */
#[bitfield(u32)]
#[derive(PartialEq)]
pub struct DisplayControl {
    #[bits(3)]
    pub bg_mode: BackgroundMode,

    // Engine A only
    #[bits(default = true)]
    pub enable_3d: bool,

    #[bits(default = true)]
    pub tile_obj_mapping: bool,  // ??
    #[bits(default = true)]
    pub bitmap_obj_2d_dimension: bool,  // ??
    #[bits(default = true)]
    pub bitmap_obj_mapping: bool,  // ??

    #[bits(9)]
    _pad1: u32,

    #[bits(2)]
    pub display_mode: DisplayMode,

    #[bits(2)]
    pub framebuffer_vram_block: Disp2VramBank,

    #[bits(12)]
    _pad2: u32,
}
