//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 863/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk863<F: Float>(t75479: F, t7720: F, t2046: F, t2050: F, t2406: F, t31: F, t15382: F, t1971: F, t2144: F, t333: F, t7230: F, t352: F, t875: F) -> (F, F, F, F) {
    let t75480 = t7720 * t75479;
    let t75484 = t2046 * t2050 * t2406 * t31;
    let t75490 = F::new(0.3192344991997337955e-4) * t7230 * t1971 * t2144 * t15382 * t333;
    let t75495 = F::new(0.212822999466489197e-4) * t7230 * t1971 * t875 * t15382 * t352;
    (t75480, t75484, t75490, t75495)
}
