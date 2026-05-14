//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 926/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk926<F: Float>(t10277: F, t20234: F, t2826: F, t136: F, t4337: F, t5398: F, t2768: F, t123: F) -> (F, F, F, F) {
    let t21118 = t10277 * t20234;
    let t21119 = t2826 * t21118;
    let t21120 = t136 * t21119;
    let t21122 = t4337 * t5398;
    let t21123 = t2768 * t21122;
    let t21124 = t123 * t21123;
    (t21118, t21120, t21122, t21124)
}
