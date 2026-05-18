//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 930/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk930<F: Float>(t242: F, t837: F, t8951: F, t967: F, t2655: F, t943: F, t938: F, t941: F, t357: F, t339: F, t349: F, t2677: F, t2682: F) -> (F, F, F, F) {
    let t8953 = t242 * t8951 * t837;
    let t8954 = t967 * t8953;
    let t8956 = t2655 * t943;
    let t8958 = t938 * t941 * t8956;
    let t8961 = t2655 * t357;
    let t8963 = t339 * t349 * t8961;
    let t8966 = t2682 * t2677;
    (t8954, t8958, t8963, t8966)
}
