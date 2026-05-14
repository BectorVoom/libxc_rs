//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1170/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1170<F: Float>(t19103: F, t219: F, t6017: F, t1705: F, t3110: F, t935: F, t5570: F, t6021: F) -> (F, F, F, F, F) {
    let t19104 = param_beta * t19103;
    let t19106 = t6017 * t219;
    let t19112 = t1705 * t3110;
    let t19113 = t19112 * t935;
    let t19115 = t6021 * t5570;
    (t19104, t19106, t19112, t19113, t19115)
}
