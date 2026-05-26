use bitflags::bitflags;
use crate::model::DerivativeOrder;

bitflags! {
    /// Bitmask selecting which derivative orders to compute.
    ///
    /// [`OutputMask::from_order`] returns a cumulative mask: requesting
    /// `Vxc` includes `EXC | VXC` (energy + first derivatives).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct OutputMask: u8 {
        /// Energy density (order 0).
        const EXC = 1 << 0;
        /// First derivatives (order 1).
        const VXC = 1 << 1;
        /// Second derivatives (order 2).
        const FXC = 1 << 2;
        /// Third derivatives (order 3).
        const KXC = 1 << 3;
        /// Fourth derivatives (order 4).
        const LXC = 1 << 4;
    }
}

impl OutputMask {
    /// Build a cumulative mask for all orders up to and including `order`.
    ///
    /// This matches libxc semantics where requesting Vxc also computes Exc.
    pub fn from_order(order: DerivativeOrder) -> Self {
        match order {
            DerivativeOrder::Exc => Self::EXC,
            DerivativeOrder::Vxc => Self::EXC | Self::VXC,
            DerivativeOrder::Fxc => Self::EXC | Self::VXC | Self::FXC,
            DerivativeOrder::Kxc => Self::EXC | Self::VXC | Self::FXC | Self::KXC,
            DerivativeOrder::Lxc => Self::EXC | Self::VXC | Self::FXC | Self::KXC | Self::LXC,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exc_value() {
        assert_eq!(OutputMask::EXC.bits(), 1);
    }

    #[test]
    fn vxc_value() {
        assert_eq!(OutputMask::VXC.bits(), 2);
    }

    #[test]
    fn fxc_value() {
        assert_eq!(OutputMask::FXC.bits(), 4);
    }

    #[test]
    fn kxc_value() {
        assert_eq!(OutputMask::KXC.bits(), 8);
    }

    #[test]
    fn lxc_value() {
        assert_eq!(OutputMask::LXC.bits(), 16);
    }

    #[test]
    fn from_order_exc() {
        assert_eq!(OutputMask::from_order(DerivativeOrder::Exc), OutputMask::EXC);
    }

    #[test]
    fn from_order_vxc_cumulative() {
        let mask = OutputMask::from_order(DerivativeOrder::Vxc);
        assert_eq!(mask, OutputMask::EXC | OutputMask::VXC);
        assert!(mask.contains(OutputMask::EXC));
        assert!(mask.contains(OutputMask::VXC));
    }

    #[test]
    fn from_order_fxc_cumulative() {
        let mask = OutputMask::from_order(DerivativeOrder::Fxc);
        assert!(mask.contains(OutputMask::EXC));
        assert!(mask.contains(OutputMask::VXC));
        assert!(mask.contains(OutputMask::FXC));
        assert!(!mask.contains(OutputMask::KXC));
    }

    #[test]
    fn from_order_lxc_is_all() {
        assert_eq!(OutputMask::from_order(DerivativeOrder::Lxc), OutputMask::all());
    }
}
