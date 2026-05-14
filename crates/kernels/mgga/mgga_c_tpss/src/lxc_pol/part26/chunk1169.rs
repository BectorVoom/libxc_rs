//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1169/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1169<F: Float>(t19497: F, t219: F, t6256: F, t1705: F, t4487: F, t935: F, t5570: F, t6259: F) -> (F, F, F, F, F) {
    let t19498 = param_beta * t19497;
    let t19500 = t6256 * t219;
    let t19506 = t1705 * t4487;
    let t19507 = t19506 * t935;
    let t19509 = t6259 * t5570;
    (t19498, t19500, t19506, t19507, t19509)
}
