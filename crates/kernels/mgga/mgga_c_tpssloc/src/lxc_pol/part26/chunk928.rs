//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 928/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk928<F: Float>(t10277: F, t2978: F, t9288: F, t974: F, t1030: F, t363: F, t3068: F, t1058: F, t10213: F, t10216: F, t3030: F, t990: F) -> (F, F, F, F) {
    let t10930 = t2978 * t10277;
    let t10931 = t10930 * t9288;
    let t10932 = t974 * t10931;
    let t10935 = t363 * t1030;
    let t10936 = t10935 * t3068;
    let t10937 = t1058 * t10936;
    let t10942 = t10213 * t10216;
    let t10943 = t10942 * t9288;
    let t10944 = t974 * t10943;
    let t10947 = t990 * t3030;
    (t10932, t10937, t10944, t10947)
}
