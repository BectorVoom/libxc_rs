//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1066/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1066<F: Float>(t75811: F, t75814: F, t75818: F, t75820: F, t75823: F, t75825: F, t75806: F, t75808: F, t75828: F, t75831: F, t75834: F, t75838: F, t75841: F, t75844: F, t75847: F, t75850: F, t75853: F) -> F {
    let t78308 = F::cast_from(0.2627895913935205078e-5_f64) * t75811;
    let t78309 = F::cast_from(0.59127658063542114255e-5_f64) * t75814;
    let t78310 = F::cast_from(0.7661627980793611092e-4_f64) * t75818;
    let t78311 = F::cast_from(0.5959043985061697516e-4_f64) * t75820;
    let t78312 = F::cast_from(0.2553875993597870364e-4_f64) * t75823;
    let t78313 = F::cast_from(0.2553875993597870364e-4_f64) * t75825;
    let t78317 = -F::cast_from(0.87596530464506835935e-6_f64) * t75806 + F::cast_from(0.87596530464506835935e-6_f64) * t75808 + t78308 + t78309 + t78310 + t78311 + t78312 - t78313 - F::cast_from(0.10511583655740820313e-5_f64) * t75828 + F::cast_from(0.15767375483611230469e-5_f64) * t75831 - F::cast_from(0.21023167311481640626e-5_f64) * t75834 - t75838 + t75841 + t75844 - t75847 + t75850 + t75853;
    t78317
}
