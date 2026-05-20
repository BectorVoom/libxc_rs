//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1614/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1614<F: Float>(t23121: F, t281: F, t22690: F, t776: F, t841: F, t2617: F, t6620: F, t849: F, t2703: F, t6621: F, t6619: F, t835: F) -> (F, F, F, F, F, F, F) {
    let t23122 = t23121 * t281;
    let t23124 = t22690 * t841 * t776;
    let t23125 = t23122 * t23124;
    let t23127 = t2617 * t6620;
    let t23128 = t23127 * t849;
    let t23130 = t6621 * t2703;
    let t23132 = t6619 * t835;
    (t23122, t23124, t23125, t23127, t23128, t23130, t23132)
}
