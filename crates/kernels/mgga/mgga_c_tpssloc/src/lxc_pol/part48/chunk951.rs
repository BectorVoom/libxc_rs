//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 951/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk951<F: Float>(t114693: F, t112998: F, t113005: F, t113009: F, t114670: F, t114673: F, t114677: F, t114680: F, t114685: F, t114689: F, t114691: F, t234: F, t7084: F) -> (F, F) {
    let t114694 = F::new(0.63969658155208805863e-1) * t114693;
    let t114695 = -t112998 - F::new(0.38381794893125283518e-1) * t114670 + t114673 + F::new(0.16449340668482264365e-1) * t114677 + F::new(0.82246703342411321824e-2) * t114680 - F::new(0.16449340668482264365e-1) * t114685 - t113005 - t113009 - t114689 - F::new(0.82246703342411321824e-2) * t114691 + t114694;
    let t114696 = t234 * t7084;
    (t114695, t114696)
}
