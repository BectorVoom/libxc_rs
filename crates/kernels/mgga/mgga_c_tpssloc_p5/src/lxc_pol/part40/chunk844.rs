//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 844/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk844<F: Float>(t248: F, t3585: F, t5971: F, t1230: F, t5979: F, t5975: F, t5985: F, t5987: F, t5991: F, t6023: F, t6026: F, t6092: F, t6094: F, t6096: F, t6100: F, t6104: F, t6108: F) -> (F, F, F, F) {
    let t6203 = t248 * t3585 * t5971;
    let t6207 = t248 * t1230 * t5979;
    let t6211 = t248 * t1230 * t5975;
    let t6218 = -t5985 + t5987 - t5991 + t6023 + t6026 + t6092 + t6094 - t6096 + t6100 - t6104 - t6108;
    (t6203, t6207, t6211, t6218)
}
