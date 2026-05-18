//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 874/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk874<F: Float>(t3: F, t6441: F, t1338: F, t5953: F, t117: F, t6323: F, t1668: F, t1670: F, t1851: F, t1853: F, t547: F, t548: F) -> (F, F, F, F, F) {
    let t6442 = t3 * t6441;
    let t6446 = param_d * t6441;
    let t6452 = t5953 * t1338;
    let t6455 = t117 * t6323;
    let t6458 = F::new(3.0) * t1668 * t1853 + F::new(3.0) * t1670 * t1851 + F::new(6.0) * t547 * t6452 + F::new(3.0) * t547 * t6455 + t548 * t6446;
    (t6442, t6446, t6452, t6455, t6458)
}
