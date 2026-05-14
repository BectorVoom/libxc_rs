//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 579/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk579<F: Float>(t325: F, t5058: F, t128: F, t25640: F, t305: F, t4616: F, t326: F, t793: F, t874: F) -> (F, F, F, F, F) {
    let t26857 = t5058 * t325;
    let t27041 = t25640 * t128;
    let t27048 = t305 * t4616;
    let t27055 = t326 * t4616;
    let t27101 = t793 * t874;
    (t26857, t27041, t27048, t27055, t27101)
}
