//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1077/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1077<F: Float>(t2286: F, t9090: F, t10082: F, t236: F, t3351: F, t35312: F, t498: F, t2186: F, t9935: F, t1970: F, t1971: F, t29439: F) -> (F, F, F, F) {
    let t47646 = t9090 * t2286;
    let t47653 = t3351 * t35312 * t236 * t10082 * t498;
    let t47663 = t2186 * t9935;
    let t47667 = t1970 * t1971 * t236 * t29439;
    (t47646, t47653, t47663, t47667)
}
