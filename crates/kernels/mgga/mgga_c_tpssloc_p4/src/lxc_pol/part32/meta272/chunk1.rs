//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1239/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1239<F: Float>(t6580: F, t6587: F, t6603: F, t6618: F, t7494: F, t7498: F, t7501: F, t7504: F, t7506: F, t7508: F) -> F {
    let t7510 = -t6580 - t7494 / F::cast_from(48.0_f64) - t6587 - F::cast_from(0.12111826828242117256e-2_f64) * t7498 - t6603 - F::cast_from(0.20186378047070195427e-3_f64) * t7501 + t7504 / F::cast_from(1536.0_f64) - t7506 / F::cast_from(1536.0_f64) - t6618 - t7508 / F::cast_from(384.0_f64);
    t7510
}
