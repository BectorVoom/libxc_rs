//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 781/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk781<F: Float>(t1284: F, t750: F, t17: F, t1285: F, t592: F, t1287: F, t588: F, t1365: F, t68: F, t248: F, t2691: F, t557: F) -> (F, F, F, F, F, F, F, F) {
    let t3826 = t1284 * t750;
    let t3827 = t17 * t3826;
    let t3829 = t592 * t1285;
    let t3832 = F::new(8.0) * t592 * t1287;
    let t3833 = t588 * t1285;
    let t3836 = F::new(8.0) * t588 * t1287;
    let t3843 = t68 * t1365;
    let t3862 = t2691 * t557 * t248;
    (t3826, t3827, t3829, t3832, t3833, t3836, t3843, t3862)
}
