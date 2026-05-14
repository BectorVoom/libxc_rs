//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 924/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk924<F: Float>(t75806: F, t75808: F, t75828: F, t75831: F, t75834: F, t75838: F, t75841: F, t75844: F, t75847: F, t75850: F, t78304: F, t78308: F, t78309: F, t78310: F, t78311: F, t78312: F, t78313: F) -> (F,) {
    let t80333 = -t78304 - 0.87596530464506835932e-6 * t75806 + 0.87596530464506835932e-6 * t75808 + t78308 + t78309 + t78310 + t78311 + t78312 - t78313 - 0.10511583655740820312e-5 * t75828 + 0.15767375483611230468e-5 * t75831 - 0.21023167311481640624e-5 * t75834 - t75838 + t75841 + t75844 - t75847 + t75850;
    (t80333,)
}
