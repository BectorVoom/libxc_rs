//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1836/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1836<F: Float>(t1888: F, t23270: F, t25044: F, t2742: F, t23168: F, t25342: F, t25345: F, t82038: F, t1519: F, t213: F, t225: F, t22986: F, t23272: F) -> (F, F, F, F) {
    let t86866 = t1888 * t23270 * t25044 * t2742;
    let t86868 = t23168 * t25342;
    let t86870 = t82038 * t25345;
    let t86873 = t213 * t1519 * t225;
    let t86875 = t22986 * t86873 * t23272;
    (t86866, t86868, t86870, t86875)
}
