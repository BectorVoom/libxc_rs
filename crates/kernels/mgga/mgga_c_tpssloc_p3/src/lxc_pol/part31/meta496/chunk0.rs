//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1691/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1691<F: Float>(t5493: F, t88: F, t22473: F, t5464: F, t5488: F, t6530: F, t89: F, t3788: F, t6388: F, t6936: F, t1339: F, t6420: F) -> (F, F, F, F, F, F, F) {
    let t28007 = t88 * t5493;
    let t28012 = t22473 * t5464;
    let t28014 = t6530 * t5488;
    let t28030 = t89 * t5493;
    let t28057 = t3788 * t6388;
    let t28058 = t6936 * t28057;
    let t28060 = t1339 * t6420;
    (t28007, t28012, t28014, t28030, t28057, t28058, t28060)
}
