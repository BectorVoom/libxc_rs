use bitflags::bitflags;

pub mod lda_functional;
pub mod gga_functional;
pub mod mgga_functional;
pub use lda_functional::LdaFunctional;
pub use gga_functional::GgaFunctional;
pub use mgga_functional::MggaFunctional;

/// Functional family classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Family {
    Lda  = 1,
    Gga  = 2,
    Mgga = 4,
}

/// What the functional computes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Kind {
    Exchange            = 0,
    Correlation         = 1,
    ExchangeCorrelation = 2,
    Kinetic             = 3,
}

/// Spin polarization mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Spin {
    Unpolarized = 1,
    Polarized   = 2,
}

/// Derivative order
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum DerivativeOrder {
    Exc = 0,
    Vxc = 1,
    Fxc = 2,
    Kxc = 3,
    Lxc = 4,
}

/// Hybrid functional type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HybridType {
    Semilocal,
    Hybrid,
    Cam,
    CamYukawa,
    CamGaussian,
    DoubleHybrid,
    Mixture,
}

/// Hybrid Fock term type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum HybridTermKind {
    Fock       = 1,
    Pt2        = 2,
    ErfSr      = 4,
    YukawaSr   = 8,
    GaussianSr = 16,
}

/// Dimensionality of the physical system
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dimensionality {
    OneD,
    TwoD,
    ThreeD,
}

/// A validated functional ID. Only constructible from the known set of 649 IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FunctionalId(pub(crate) u16);

impl std::fmt::Display for FunctionalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FunctionalId {
    /// Look up a functional by its integer ID.
    /// Returns Ok(FunctionalId) for valid IDs, Err for unknown or removed.
    pub fn from_raw(id: u16) -> Result<Self, crate::LibxcRsError> {
        crate::registry::lookup_by_id(id).map(|meta| meta.id)
    }

    /// Look up a functional by its canonical name (case-insensitive).
    pub fn from_name(name: &str) -> Result<Self, crate::LibxcRsError> {
        crate::registry::lookup_by_name(name)
    }

    /// Get the raw integer ID.
    pub fn raw(self) -> u16 {
        self.0
    }

    /// Get the canonical name of this functional.
    pub fn name(self) -> &'static str {
        // SAFETY: FunctionalId can only be constructed via from_raw/from_name
        // which validate against the registry, so this lookup always succeeds.
        crate::registry::lookup_by_id(self.0)
            .map(|m| m.name)
            .unwrap_or("UNKNOWN")
    }

    /// Get the family of this functional.
    pub fn family(self) -> Family {
        crate::registry::lookup_by_id(self.0)
            .map(|m| m.family)
            .unwrap_or(Family::Lda)
    }

    /// Get the full metadata for this functional.
    pub fn meta(self) -> &'static crate::meta::FunctionalMeta {
        crate::registry::lookup_by_id(self.0)
            .expect("FunctionalId was constructed from a valid ID")
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct FunctionalFlags: u32 {
        const HAVE_EXC        = 1 << 0;
        const HAVE_VXC        = 1 << 1;
        const HAVE_FXC        = 1 << 2;
        const HAVE_KXC        = 1 << 3;
        const HAVE_LXC        = 1 << 4;
        const DIM_1D          = 1 << 5;
        const DIM_2D          = 1 << 6;
        const DIM_3D          = 1 << 7;
        const VV10            = 1 << 10;
        const STABLE          = 1 << 13;
        const DEVELOPMENT     = 1 << 14;
        const NEEDS_LAPLACIAN = 1 << 15;
        const NEEDS_TAU       = 1 << 16;
        const HAVE_ALL = Self::HAVE_EXC.bits()
                       | Self::HAVE_VXC.bits()
                       | Self::HAVE_FXC.bits()
                       | Self::HAVE_KXC.bits()
                       | Self::HAVE_LXC.bits();
    }
}

/// Numerical thresholds for evaluation stability
#[derive(Debug, Clone, Copy)]
pub struct Thresholds {
    pub density: f64,
    pub zeta: f64,
    pub sigma: f64,
    pub tau: f64,
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            density: 1e-15,
            zeta: 1e-10,
            sigma: 1e-24,
            tau: 1e-20,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_family_repr() {
        assert_eq!(Family::Lda as u8, 1);
        assert_eq!(Family::Gga as u8, 2);
        assert_eq!(Family::Mgga as u8, 4);
    }

    #[test]
    fn test_kind_repr() {
        assert_eq!(Kind::Exchange as u8, 0);
        assert_eq!(Kind::Correlation as u8, 1);
        assert_eq!(Kind::ExchangeCorrelation as u8, 2);
        assert_eq!(Kind::Kinetic as u8, 3);
    }

    #[test]
    fn test_spin_repr() {
        assert_eq!(Spin::Unpolarized as u8, 1);
        assert_eq!(Spin::Polarized as u8, 2);
    }

    #[test]
    fn test_derivative_order_ordering() {
        assert!(DerivativeOrder::Exc < DerivativeOrder::Lxc);
        assert!(DerivativeOrder::Vxc < DerivativeOrder::Fxc);
        assert!(DerivativeOrder::Fxc < DerivativeOrder::Kxc);
        assert!(DerivativeOrder::Kxc < DerivativeOrder::Lxc);
    }

    #[test]
    fn test_flags_values() {
        assert_eq!(FunctionalFlags::HAVE_EXC.bits(), 1);
        assert_eq!(FunctionalFlags::HAVE_VXC.bits(), 2);
        assert_eq!(FunctionalFlags::STABLE.bits(), 8192);
        assert_eq!(FunctionalFlags::NEEDS_TAU.bits(), 65536);
    }

    #[test]
    fn test_flags_have_all() {
        assert!(FunctionalFlags::HAVE_ALL.contains(FunctionalFlags::HAVE_EXC));
        assert!(FunctionalFlags::HAVE_ALL.contains(FunctionalFlags::HAVE_VXC));
        assert!(FunctionalFlags::HAVE_ALL.contains(FunctionalFlags::HAVE_FXC));
        assert!(FunctionalFlags::HAVE_ALL.contains(FunctionalFlags::HAVE_KXC));
        assert!(FunctionalFlags::HAVE_ALL.contains(FunctionalFlags::HAVE_LXC));
    }

    #[test]
    fn test_thresholds_default() {
        let t = Thresholds::default();
        assert_eq!(t.density, 1e-15);
        assert_eq!(t.zeta, 1e-10);
        assert_eq!(t.sigma, 1e-24);
        assert_eq!(t.tau, 1e-20);
    }

    #[test]
    fn test_functional_id_from_raw() {
        let id = FunctionalId::from_raw(1).unwrap();
        assert_eq!(id.raw(), 1);
    }

    #[test]
    fn test_functional_id_from_name() {
        let id = FunctionalId::from_name("XC_LDA_X").unwrap();
        assert_eq!(id.raw(), 1);
    }

    #[test]
    fn test_functional_id_name() {
        let id = FunctionalId::from_raw(1).unwrap();
        assert_eq!(id.name(), "XC_LDA_X");
    }

    #[test]
    fn test_functional_id_family() {
        let id = FunctionalId::from_raw(1).unwrap();
        assert_eq!(id.family(), Family::Lda);
    }

    #[test]
    fn test_functional_id_from_raw_invalid() {
        assert!(FunctionalId::from_raw(9999).is_err());
    }

    #[test]
    fn test_functional_id_from_raw_removed() {
        assert!(FunctionalId::from_raw(104).is_err());
    }

    #[test]
    fn test_functional_id_meta() {
        let id = FunctionalId::from_raw(1).unwrap();
        assert_eq!(id.meta().kind, Kind::Exchange);
    }
}
