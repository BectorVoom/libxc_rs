//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1330/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1330<F: Float>(t3537: F, t547: F, t71038: F, t1668: F, t20125: F, t1279: F, t21563: F, t21566: F, t5470: F, t5773: F, t16052: F, t1786: F, t5776: F, t1338: F, t6112: F, t645: F) -> (F, F, F, F, F, F, F, F) {
    let t71041 = 12.0 * t547 * t71038 * t3537;
    let t71043 = 12.0 * t1668 * t20125;
    let t71045 = 12.0 * t1279 * t21563;
    let t71049 = 6.0 * t1279 * t21566;
    let t71057 = 6.0 * t5470 * t5773;
    let t71059 = 3.0 * t16052 * t1786;
    let t71063 = 3.0 * t5470 * t5776;
    let t71067 = 12.0 * t547 * t645 * t6112 * t1338;
    (t71041, t71043, t71045, t71049, t71057, t71059, t71063, t71067)
}
