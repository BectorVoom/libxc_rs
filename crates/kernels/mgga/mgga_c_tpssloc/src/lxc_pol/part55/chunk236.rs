//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 236/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk236<F: Float>(t882: F, t884: F, t123: F, t881: F, t291: F, t287: F, t275: F, t276: F, t880: F, t273: F, t241: F, t697: F, t281: F, t283: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t885 = t882 * t884;
    let t886 = t123 * t885;
    let t888 = -t881 - 0.17808333333333333333e-1 * t886;
    let t890 = 0.621814e-1 * t888 * t291;
    let t891 = t287 * t287;
    let t892 = 1.0 / t891;
    let t893 = t275 * t892;
    let t894 = 1.0 / t276;
    let t896 = -t880 / 3.0 - t886 / 3.0;
    let t897 = t894 * t896;
    let t899 = 0.29896666666666666667e0 * t880;
    let t901 = f64::sqrt(t273);
    let t902 = t901 * t896;
    let t904 = t697 * t241;
    let t906 = t281 * t904 * t283;
    (t885, t886, t888, t890, t891, t892, t893, t894, t896, t897, t899, t901, t902, t904, t906)
}
