//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1832/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1832<F: Float>(t1827: F, t80991: F, t22765: F, t5289: F, t22764: F, t5234: F, t1354: F, t26298: F, t80958: F, t22779: F, t26319: F, t1358: F, t26248: F) -> (F, F, F, F, F, F, F) {
    let t91281 = t80991 * t1827;
    let t91283 = t22765 * t5289;
    let t91285 = t5234 * t22764;
    let t91286 = t91285 * t1354;
    let t91290 = t80958 * t26298;
    let t91300 = t22779 * t26319;
    let t91303 = t26248 * t1358;
    (t91281, t91283, t91285, t91286, t91290, t91300, t91303)
}
