//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 500/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk500<F: Float>(t4155: F, t4163: F, t4214: F, t4220: F, t4336: F, t4338: F, t5382: F, t5383: F, t5385: F, t5388: F, t5392: F, t5393: F, t5394: F, t5402: F, t5403: F, t5405: F) -> F {
    let t5456 = -t5382 - t5383 + t5385 - t5388 - t4155 - t4163 - t5392 - t5393 - t5394 - t5402 + t5403 + t4336 - t4338 + t4214 - t4220 - t5405;
    t5456
}
