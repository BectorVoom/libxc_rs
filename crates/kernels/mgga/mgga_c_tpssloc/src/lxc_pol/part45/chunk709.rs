//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 709/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk709<F: Float>(t23402: F, t6689: F, t1945: F, t3020: F, t6768: F, t990: F, t2250: F, t3: F, t1933: F, t368: F, t3068: F, t1058: F, t210: F, t6679: F, t3139: F, t6717: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t23403 = t6689 * t23402;
    let t23408 = t3020 * t1945;
    let t23410 = t990 * t6768;
    let t23413 = t3 * t2250;
    let t23414 = t1933 * t23413;
    let t23417 = sigma0 * t368;
    let t23418 = t23417 * t3068;
    let t23419 = t1058 * t23418;
    let t23422 = t6679 * t210;
    let t23425 = t6717 * t3139;
    (t23403, t23408, t23410, t23414, t23419, t23422, t23425)
}
