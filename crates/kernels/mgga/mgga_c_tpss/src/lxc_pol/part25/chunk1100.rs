//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1100/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1100<F: Float>(t116: F, t5815: F, t1338: F, t623: F, t3537: F, t94: F, t6076: F, t619: F, t77: F, t1317: F, t1679: F, t1290: F, t1981: F) -> (F, F, F, F, F, F) {
    let t19040 = t116 * t5815;
    let t19305 = t623 * t1338;
    let t19308 = t94 * t3537;
    let t19342 = t77 * t6076 * t619;
    let t19345 = t1679 * t1317;
    let t19349 = t1981 * t1290;
    (t19040, t19305, t19308, t19342, t19345, t19349)
}
