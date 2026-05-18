//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 243/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk243<F: Float>(t287: F, t275: F, t276: F, t880: F, t273: F, t241: F, t697: F, t281: F, t283: F, t340: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t891 = t287 * t287;
    let t892 = F::new(1.0) / t891;
    let t893 = t275 * t892;
    let t894 = F::new(1.0) / t276;
    let t899 = F::new(0.29896666666666666667e0) * t880;
    let t901 = f64::sqrt(t273);
    let t904 = t697 * t241;
    let t906 = t281 * t904 * t283;
    let t907 = F::new(0.82156666666666666667e-1) * t906;
    let t908 = t241 * t340;
    (t891, t892, t893, t894, t899, t901, t904, t906, t907, t908)
}
