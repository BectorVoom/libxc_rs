//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 956/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk956<F: Float>(t12475: F, t3067: F, t11453: F, t4284: F, t1125: F, t3096: F, t9199: F, t9187: F, t9684: F, t3028: F, t4212: F, t140: F, t4227: F) -> (F, F, F, F, F, F) {
    let t12477 = t3067 * t12475 / F::new(3456.0);
    let t12478 = t11453 * t4284;
    let t12480 = t1125 * t12478 / F::new(1728.0);
    let t12490 = t3096 * t9199;
    let t12510 = t9684 * t9187;
    let t12530 = t4212 * t3028 / F::new(162.0);
    let t12535 = t140 * t4227;
    (t12477, t12480, t12490, t12510, t12530, t12535)
}
