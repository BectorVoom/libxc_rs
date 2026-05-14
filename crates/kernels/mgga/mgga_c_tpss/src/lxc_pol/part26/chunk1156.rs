//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1156/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1156<F: Float>(t116: F, t5531: F, t1872: F, t3025: F, t3028: F, t6002: F, t3048: F, t2713: F, t3050: F, sigma2: F) -> (F, F, F, F, F) {
    let t18592 = t116 * t5531;
    let t19066 = t1872 * t3025 / 432.0;
    let t19067 = t6002 * t3028;
    let t19075 = t3048 * sigma2;
    let t19077 = t2713 * t19075 * t3050;
    (t18592, t19066, t19067, t19075, t19077)
}
