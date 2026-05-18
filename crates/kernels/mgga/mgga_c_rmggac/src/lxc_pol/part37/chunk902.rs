//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 902/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk902<F: Float>(t69171: F, t75957: F, t69428: F, t75963: F, t74960: F, t7788: F, t74964: F, t7782: F, t76078: F, t7835: F, t27041: F, t74973: F) -> (F, F, F, F, F, F) {
    let t76182 = t69171 * t75957;
    let t76184 = t69428 * t75963;
    let t76186 = t7788 * t74960;
    let t76188 = t7782 * t74964;
    let t76190 = t7835 * t76078;
    let t76197 = t27041 * t74973;
    (t76182, t76184, t76186, t76188, t76190, t76197)
}
