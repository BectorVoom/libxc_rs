//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 633/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk633<F: Float>(t3134: F, t511: F, t27: F, t498: F, t3142: F, t676: F, t880: F, t2144: F, t495: F, t1968: F, t7427: F, t1966: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16058 = t3134 * t511;
    let t16059 = t27 * t498;
    let t16064 = t3142 * t511;
    let t16069 = t676 * t880;
    let t16074 = t676 * t2144;
    let t16129 = t676 * t511;
    let t16130 = t27 * t495;
    let t16155 = t7427 * t1968;
    let t16156 = t1966 * t16155;
    (t16058, t16059, t16064, t16069, t16074, t16129, t16130, t16155, t16156)
}
