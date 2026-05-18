//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1335/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1335<F: Float>(t1873: F, t45557: F, t45560: F, t7015: F, t20173: F, t23896: F, t22479: F, t3941: F, t671: F, t2363: F, t6534: F, t55344: F) -> (F, F, F, F, F, F) {
    let t83999 = F::new(0.135e2) * t45557 * t1873;
    let t84001 = F::new(81.0) * t45560 * t7015;
    let t84003 = F::new(81.0) * t20173 * t23896;
    let t84009 = F::new(81.0) * t3941 * t22479 * t671;
    let t84012 = F::new(81.0) * t3941 * t6534 * t2363;
    let t84014 = F::new(81.0) * t55344 * t1873;
    (t83999, t84001, t84003, t84009, t84012, t84014)
}
