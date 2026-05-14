//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 884/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk884<F: Float>(t74166: F, t74168: F, t68526: F, t70877: F, t74183: F, t76878: F, t76879: F, t76880: F, t76884: F, t76885: F, t76886: F, t76887: F, t76888: F, t76892: F, t76893: F, t76894: F, t76896: F) -> (F,) {
    let t80053 = 0.29085809927086856922e-4 * t74166;
    let t80054 = 0.29085809927086856922e-4 * t74168;
    let t80056 = -t76878 + t76879 - t76880 - t80053 + t80054 + t76884 - t76885 + t76886 + t76887 - t76888 - 0.72714524817717142305e-5 * t74183 + t76892 - t76893 - t76894 + t70877 + t68526 - t76896;
    (t80056,)
}
