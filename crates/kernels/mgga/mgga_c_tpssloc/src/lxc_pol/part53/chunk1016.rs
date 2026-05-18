//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1016/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1016<F: Float>(t118573: F, t112784: F, t114714: F, t114720: F, t118533: F, t118535: F, t118539: F, t118546: F, t118549: F, t118552: F, t118556: F, t118559: F, t118562: F, t118566: F, t118569: F, t118576: F, t118578: F, t118580: F) -> F {
    let t123566 = F::new(0.32298204875312312682e-2) * t118573;
    let t123570 = -t118533 / F::new(384.0) - t118535 / F::new(384.0) - t118539 / F::new(384.0) + F::new(5.0) / F::new(96.0) * t118546 - F::new(0.32298204875312312682e-2) * t118549 + F::new(0.13565246047631171326e0) * t118552 + t114714 + F::new(0.64596409750624625364e-2) * t118556 + F::new(0.19378922925187387609e-1) * t118559 + F::new(0.13565246047631171326e0) * t112784 + t118562 / F::new(192.0) + t114720 + F::new(0.19378922925187387609e-1) * t118566 - F::new(0.32298204875312312682e-2) * t118569 + t123566 + t118576 / F::new(384.0) + F::new(0.22608743412718618877e-1) * t118578 + F::new(0.13565246047631171326e0) * t118580;
    t123570
}
