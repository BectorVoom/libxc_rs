//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 982/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk982<F: Float>(t3684: F, t9885: F, t12098: F, t12101: F, t12103: F, t12105: F, t12107: F, t12109: F, t12112: F, t12114: F, t12116: F, t9820: F, t9824: F, t3824: F, t588: F, t1287: F, t2225: F) -> (F, F, F, F) {
    let t12118 = 0.16265371950452609763e-1 * t3684 * t9885;
    let t12119 = -t9820 - t9824 + t12098 - t12101 + t12103 - t12105 + t12107 - t12109 + t12112 - t12114 + t12116 + t12118;
    let t12120 = t588 * t3824;
    let t12121 = 12.0 * t12120;
    let t12123 = 60.0 * t2225 * t1287;
    (t12118, t12119, t12121, t12123)
}
