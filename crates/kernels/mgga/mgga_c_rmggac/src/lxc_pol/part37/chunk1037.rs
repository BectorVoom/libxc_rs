//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1037/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1037<F: Float>(t14182: F, t14655: F, t14656: F, t14659: F, t14660: F, t14661: F, t14662: F, t14663: F, t14665: F, t14670: F, t14674: F, t15525: F, t15528: F, t15529: F, t15905: F) -> F {
    let t79961 = -t15525 - t14655 - t14656 + t14182 + t14659 - t14660 + t14661 + t14662 - t14663 - t14665 + t14670 - t14674 - t15905 + t15528 + t15529;
    t79961
}
