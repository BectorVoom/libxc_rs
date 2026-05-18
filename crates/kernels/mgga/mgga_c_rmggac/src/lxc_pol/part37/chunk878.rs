//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 878/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk878<F: Float>(t14077: F, t15290: F, t7282: F, t12200: F, t15313: F, t15227: F, t70207: F, t1971: F, t495: F, t7230: F, t875: F, t8936: F) -> (F, F, F, F) {
    let t75736 = t7282 * t14077 * t15290;
    let t75739 = t12200 * t14077 * t15313;
    let t75748 = t70207 * t15227;
    let t75756 = t7230 * t1971 * t875 * t8936 * t495;
    (t75736, t75739, t75748, t75756)
}
