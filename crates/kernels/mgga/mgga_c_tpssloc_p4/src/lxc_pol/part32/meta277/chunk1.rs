//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1254/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1254<F: Float>(t6915: F, t6922: F, t6935: F, t6949: F, t7706: F, t7710: F, t7713: F, t7716: F, t7718: F, t7720: F) -> F {
    let t7722 = -t6915 - t7706 / F::new(48.0) - t6922 - F::cast_from(0.12111826828242117256e-2_f64) * t7710 - t6935 - F::cast_from(0.20186378047070195427e-3_f64) * t7713 + t7716 / F::new(1536.0) - t7718 / F::new(1536.0) - t6949 - t7720 / F::new(384.0);
    t7722
}
