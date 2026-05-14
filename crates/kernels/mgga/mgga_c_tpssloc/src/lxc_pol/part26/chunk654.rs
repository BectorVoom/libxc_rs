//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 654/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk654<F: Float>(t1043: F, t2775: F, t2770: F, t3061: F, t1022: F, t3131: F, t1932: F, t360: F, t193: F, t336: F) -> (F, F, F, F, F) {
    let t4583 = t1043 * t2775;
    let t4588 = t3061 * t2770;
    let t4594 = t3131 * t1022;
    let t4684 = t1932 * t1022 * t360;
    let t4700 = t193 * t336;
    (t4583, t4588, t4594, t4684, t4700)
}
