//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1422/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1422<F: Float>(t33250: F, t6914: F, t115614: F, t1842: F, t1992: F, t22635: F, t113934: F, t115292: F, t115294: F, t120180: F, t120184: F, t120196: F, t122102: F, t122107: F, t122110: F, t2092: F, t24082: F, t7750: F, t90732: F) -> F {
    let t122112 = t6914 * t33250;
    let t122117 = t1992 * t22635 * t115614 * t1842;
    let t122119 = t120180 + t120184 - t90732 * t2092 - F::new(0.38381794893125283518e-1) * t122102 - t24082 * t7750 + t113934 + F::new(0.19190897446562641759e-1) * t115292 + F::new(0.16449340668482264365e-1) * t122107 + F::new(0.16449340668482264365e-1) * t122110 - F::new(0.38381794893125283518e-1) * t122112 + F::new(0.19190897446562641759e-1) * t115294 - t120196 + F::new(0.16449340668482264365e-1) * t122117;
    t122119
}
