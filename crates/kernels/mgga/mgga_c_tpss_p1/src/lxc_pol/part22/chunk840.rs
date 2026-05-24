//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 840/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk840<F: Float>(t114: F, t5525: F, t5528: F) -> (F, F) {
    let t115 = F::new(1.0) < t114;
    let t5812 = F::new(2.0) / F::new(3.0) * t5525;
    let t5815 = piecewise3::<F>(t115, F::new(0.0), -t5812 - t5528 / F::new(4.0));
    (t5812, t5815)
}
