//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 281/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk281<F: Float>(t894: F, t896: F, t880: F, t273: F, t241: F, t697: F, t281: F, t283: F, t340: F) -> (F, F, F, F, F, F, F, F) {
    let t897 = t894 * t896;
    let t899 = F::new(0.29896666666666666667e0) * t880;
    let t901 = f64::sqrt(t273);
    let t902 = t901 * t896;
    let t904 = t697 * t241;
    let t906 = t281 * t904 * t283;
    let t907 = F::new(0.82156666666666666667e-1) * t906;
    let t908 = t241 * t340;
    (t897, t899, t901, t902, t904, t906, t907, t908)
}
