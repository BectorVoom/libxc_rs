//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 686/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk686<F: Float>(t27: F, t8435: F, t16074: F, t69609: F, t15411: F, t68761: F, t8420: F, t16058: F, t8425: F, t16064: F, t15061: F, t7487: F, t34975: F, t34976: F, t665: F, t8455: F) -> (F, F, F, F, F, F) {
    let t74205 = t27 * t8435;
    let t74207 = t69609 * t16074 * t74205;
    let t74209 = t68761 * t15411;
    let t74211 = t27 * t8420;
    let t74213 = t69609 * t16058 * t74211;
    let t74215 = t27 * t8425;
    let t74217 = t69609 * t16064 * t74215;
    let t74219 = t7487 * t15061;
    let t74225 = 0.1064114997332445985e-4 * t34975 * t34976 * t665 * t8455;
    (t74207, t74209, t74213, t74217, t74219, t74225)
}
