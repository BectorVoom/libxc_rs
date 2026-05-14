//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 861/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk861<F: Float>(t10889: F, t3128: F, t3033: F, t248: F, t3101: F, t3121: F, t1020: F, t2250: F, t607: F) -> (F, F, F) {
    let t10903 = t3128 * t10889;
    let t10904 = t3033 * t10903;
    let t10908 = t248 * t3101 * t3121;
    let t10909 = t1020 * t10908;
    let t10913 = t607 * t2250;
    (t10904, t10909, t10913)
}
