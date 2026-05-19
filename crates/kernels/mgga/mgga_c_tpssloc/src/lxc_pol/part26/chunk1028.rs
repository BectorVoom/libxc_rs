//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1028/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1028<F: Float>(t118: F, t1284: F, t2375: F, t3684: F, t9882: F, t9888: F, t9885: F, t12098: F, t12101: F, t12103: F, t12105: F, t12107: F, t12109: F, t9820: F, t9824: F) -> (F, F, F, F, F) {
    let t12110 = t1284 * t118;
    let t12111 = t12110 * t2375;
    let t12112 = F::cast_from(0.32530743900905219526e-1_f64) * t12111;
    let t12114 = F::cast_from(0.32530743900905219526e-1_f64) * t3684 * t9882;
    let t12116 = F::cast_from(0.48159733137676571078e0_f64) * t3684 * t9888;
    let t12118 = F::cast_from(0.16265371950452609763e-1_f64) * t3684 * t9885;
    let t12119 = -t9820 - t9824 + t12098 - t12101 + t12103 - t12105 + t12107 - t12109 + t12112 - t12114 + t12116 + t12118;
    (t12112, t12114, t12116, t12118, t12119)
}
