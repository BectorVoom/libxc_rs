//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 481/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk481<F: Float>(t60: F, t1383: F, t284: F, t441: F, t5873: F, t6054: F, t6059: F, t815: F, t6053: F, zeta_threshold: F) -> F {
    let t61 = t60 <= zeta_threshold;
    let t6065 = piecewise3::<F>(t61, F::new(0.0), F::new(8.0) / F::new(27.0) * t6054 * t284 + F::new(8.0) / F::new(9.0) * t1383 * t815 - F::new(2.0) / F::new(9.0) * t6059 * t284 + F::new(2.0) / F::new(3.0) * t441 * t5873);
    let t6067 = t6053 / F::new(2.0) + t6065 / F::new(2.0);
    t6067
}
