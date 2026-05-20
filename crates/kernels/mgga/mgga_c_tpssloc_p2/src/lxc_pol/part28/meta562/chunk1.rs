//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1835/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1835<F: Float>(t25213: F, t6547: F, t22986: F, t23270: F, t25053: F, t2553: F, t4119: F, t857: F, t865: F, t4300: F, t776: F, t1888: F, t2717: F) -> (F, F, F, F, F) {
    let t86843 = t6547 * t25213;
    let t86847 = t22986 * t23270 * t25053 * t2553;
    let t86849 = t857 * t4119;
    let t86852 = t22986 * t23270 * t86849 * t865;
    let t86857 = t22986 * t23270 * t857 * t4300 * t776;
    let t86862 = t1888 * t23270 * t2717 * t4300 * t865;
    (t86843, t86847, t86852, t86857, t86862)
}
