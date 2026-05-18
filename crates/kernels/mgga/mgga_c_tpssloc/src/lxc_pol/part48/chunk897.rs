//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 897/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk897<F: Float>(t2113: F, t2363: F, t671: F, t7263: F, t2169: F, t2319: F, t12734: F, t8327: F, t2314: F, t31058: F, t3652: F, t652: F, t8326: F) -> (F, F, F, F, F, F) {
    let t94248 = t2113 * t2363;
    let t96222 = t7263 * t671;
    let t96316 = t2169 * t2319;
    let t112521 = F::new(4.0) * t12734 * t8327;
    let t112523 = F::new(4.0) * t2314 * t31058;
    let t112528 = F::new(2.0) * t652 * t3652 * t8326;
    (t94248, t96222, t96316, t112521, t112523, t112528)
}
