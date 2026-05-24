//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 518/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk518<F: Float>(t5679: F, t5734: F, t1163: F, t1166: F, t1168: F, t1174: F, t1452: F, t1454: F, t1455: F, t1459: F, t228: F, t4435: F, t4438: F, t4444: F, t458: F, t462: F, t5531: F, t5533: F, t5538: F, t5540: F, t5543: F, t5555: F, t5558: F, t5561: F, t5564: F, t5567: F, t598: F) -> (F, F) {
    let t5735 = t5679 + t5734;
    let t5738 = t5531 * t228 + F::new(2.0) * t5533 * t1455 + t1452 * t1168 / F::new(2.0) + t5538 * t1455 + t5540 * t1455 + t1454 * t5543 / F::new(2.0) - F::new(5.0) / F::new(16.0) * t598 * t4435 + t598 * t4438 / F::new(4.0) + t1163 * t1459 / F::new(4.0) + t1166 * t1459 / F::new(4.0) - F::new(5.0) / F::new(8.0) * t458 * t5555 + t458 * t5558 / F::new(2.0) + F::new(45.0) / F::new(64.0) * t4444 * t5561 - F::new(5.0) / F::new(8.0) * t1174 * t5564 - F::new(5.0) / F::new(16.0) * t1174 * t5567 + t462 * t5735 / F::new(4.0);
    (t5735, t5738)
}
