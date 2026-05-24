//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1054/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1054<F: Float>(t74381: F, t74387: F, t74390: F, t74396: F, t74403: F, t74406: F, t74408: F, t74414: F, t74436: F, t76986: F, t76998: F, t76999: F, t77004: F, t77005: F, t77006: F, t77007: F) -> F {
    let t80098 = -F::cast_from(0.39418438709028076168e-5_f64) * t74381 + t76986 + F::cast_from(0.70077224371605468748e-6_f64) * t74387 - F::cast_from(0.70077224371605468748e-6_f64) * t74390 - F::cast_from(0.10511583655740820312e-5_f64) * t74396 + t76998 - t76999 + F::cast_from(0.35038612185802734374e-6_f64) * t74403 - F::cast_from(0.35038612185802734374e-6_f64) * t74406 - F::cast_from(0.58171619854173713844e-5_f64) * t74408 - F::cast_from(0.58171619854173713844e-5_f64) * t74414 + t77004 + t77005 + t77006 + t77007 - F::cast_from(0.35038612185802734374e-6_f64) * t74436;
    t80098
}
