//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 925/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk925<F: Float>(t1212: F, t1970: F, t209: F, t236: F, t618: F, t7231: F, t495: F, t511: F, t7230: F, t8502: F, t1540: F, t2144: F) -> (F, F, F) {
    let t39946 = t1970 * t7231 * t236 * t618 * t1212 * t209;
    let t39951 = t7230 * t7231 * t511 * t8502 * t495;
    let t39953 = t1540 * t2144;
    (t39946, t39951, t39953)
}
