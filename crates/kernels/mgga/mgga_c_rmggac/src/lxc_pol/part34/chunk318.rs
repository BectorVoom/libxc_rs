//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 318/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk318<F: Float>(t3095: F, t3097: F, t3091: F, t3100: F, t3103: F, t3197: F) -> (F,) {
    let t3199 = 0.10354269702074620472e-2 * t3095;
    let t3200 = 0.16595192631325726674e-3 * t3097;
    let t3203 = t3197 - 0.34093327067806677161e-2 * t3091 + t3199 + t3200 - 0.90720386384580639149e-4 * t3100 + 0.24108102678124669848e-4 * t3103;
    (t3203,)
}
