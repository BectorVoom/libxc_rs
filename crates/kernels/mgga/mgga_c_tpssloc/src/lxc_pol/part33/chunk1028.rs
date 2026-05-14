//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1028/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1028<F: Float>(t1914: F, t193: F, t200: F, t25: F, t870: F, t7484: F, t794: F, t6562: F, t1887: F, t23056: F, t1527: F, t2717: F, t6547: F, t7485: F, t1484: F, t22690: F, t841: F) -> (F, F, F, F, F, F, F, F) {
    let t25013 = t193 * t200 * t1914;
    let t25014 = t870 * t25;
    let t25035 = t794 * t7484;
    let t25036 = t6562 * t25035;
    let t25038 = t23056 * t1887;
    let t25044 = t2717 * t1527;
    let t25049 = t6547 * t7485;
    let t25064 = t22690 * t841 * t1484;
    (t25013, t25014, t25035, t25036, t25038, t25044, t25049, t25064)
}
