//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 836/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk836<F: Float>(t29580: F, t29610: F, t29636: F, t29662: F, t466: F, t1238: F, t1761: F, t27406: F, t27792: F, t29532: F, t29536: F, t29546: F, t29551: F, t29554: F, t29557: F, t498: F, t5055: F, t6244: F, t7283: F, t7351: F, t8003: F, t8061: F) -> (F, F) {
    let t29664 = t29580 + t29610 + t29636 + t29662;
    let t29665 = t466 * t29664;
    let t29667 = F::new(4.0) * t1238 * t29532 + F::new(2.0) * t1238 * t29536 - F::new(2.0) * t27792 * t1761 + F::new(4.0) * t5055 * t8061 + F::new(0.14621636149762012769e-1) * t27406 * t8003 - F::new(0.82246703342411321825e-2) * t7283 * t29546 + F::new(2.0) * t7351 * t6244 - F::new(0.82246703342411321825e-2) * t7283 * t29551 - F::new(0.16449340668482264365e-1) * t7283 * t29554 + F::new(0.16449340668482264365e-1) * t7283 * t29557 + t29665 * t498;
    (t29664, t29667)
}
