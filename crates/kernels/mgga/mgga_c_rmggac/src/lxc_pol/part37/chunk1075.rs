//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1075/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1075<F: Float>(t1364: F, t14980: F, t1632: F, t1635: F, t3282: F, t5898: F, t69648: F, t69663: F, t69665: F, t71419: F, t77509: F, t77510: F, t77511: F, t77512: F, t77514: F, t77515: F, t77517: F, t77519: F, t77520: F, t77521: F, t884: F, t903: F) -> F {
    let t80242 = t77509 - t77510 - t77511 + t77512 - t77514 - t77515 - t77517 - t77519 - t77520 - t77521 + t71419 - F::new(0.40878380883436523435e-5) * t69648 + F::new(0.17961362552795712846e0) * t903 * t3282 * t1632 - F::new(0.23948483403727617128e0) * t1364 * t3282 * t1635 - F::new(0.11974241701863808564e0) * t884 * t14980 * t5898 - t69663 + t69665;
    t80242
}
