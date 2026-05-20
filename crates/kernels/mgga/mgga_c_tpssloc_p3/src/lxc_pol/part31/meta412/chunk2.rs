//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1513/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1513<F: Float>(t1343: F, t19732: F, t820: F, t120: F, t6387: F, t5248: F, t5250: F, t5234: F, t5245: F, t12283: F, t6396: F, t3805: F, t3807: F) -> (F, F, F, F, F, F) {
    let t19868 = t1343 * t820 * t19732;
    let t19871 = t120 * t6387;
    let t19873 = t5248 * t19871 * t5250;
    let t19876 = t5234 * t5245;
    let t19879 = t12283 * t6396;
    let t19882 = t3805 * t19871 * t3807;
    (t19868, t19871, t19873, t19876, t19879, t19882)
}
