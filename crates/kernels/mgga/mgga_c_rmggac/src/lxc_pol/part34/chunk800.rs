//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 800/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk800<F: Float>(t14124: F, t201: F, t21714: F, t236: F, t457: F, t551: F, t14125: F, t515: F, t570: F, t14131: F, t9164: F, t15411: F, t68552: F) -> (F, F, F, F) {
    let t74396 = t14124 * t21714 * t236 * t551 * t457 * t201;
    let t74403 = t14124 * t14125 * t515 * t570 * t457 * t201;
    let t74406 = t14131 * t14125 * t9164;
    let t74408 = t68552 * t15411;
    (t74396, t74403, t74406, t74408)
}
