//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 919/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk919<F: Float>(t242: F, t837: F, t8951: F, t967: F, t2719: F, t956: F, t2713: F, t2716: F, t941: F, t2751: F, t774: F, t348: F, t2738: F, t983: F, t2650: F, t958: F) -> (F, F, F, F, F, F) {
    let t8953 = t242 * t8951 * t837;
    let t8954 = t967 * t8953;
    let t8970 = t956 * t2719;
    let t8972 = t2713 * t2716 * t8970;
    let t8976 = t2713 * t941 * t8970;
    let t8983 = t774 * t2751;
    let t8987 = t348 * t956;
    let t8989 = t983 * t8987 * t2738;
    let t9033 = t958 * t2650;
    (t8954, t8972, t8976, t8983, t8989, t9033)
}
