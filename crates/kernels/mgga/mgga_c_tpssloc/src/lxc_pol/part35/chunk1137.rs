//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1137/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1137<F: Float>(t20684: F, t40611: F, t20602: F, t225: F, t20420: F, t20672: F, t20670: F, t1834: F, t6414: F, t6387: F, t20553: F, t562: F, t20489: F, t1824: F, t6434: F, t20193: F, t604: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t74064 = t20684 * t40611;
    let t74849 = t20602 * t225;
    let t74860 = t20420 * t225;
    let t74908 = t20672 * t225;
    let t74930 = t20670 * t225;
    let t74937 = t1834 * t6414;
    let t74941 = t1834 * t6387;
    let t74949 = t562 * t20553;
    let t74967 = t562 * t20489;
    let t75026 = t6434 * t1824;
    let t75284 = t20193 * t604;
    (t74064, t74849, t74860, t74908, t74930, t74937, t74941, t74949, t74967, t75026, t75284)
}
