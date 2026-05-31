//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1282/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1282<F: Float>(t41912: F, t41925: F, t894: F, t901: F, t41646: F, t41651: F, t41680: F, t41695: F, t41707: F, t41713: F, t41717: F, t41882: F, t41885: F, t41887: F, t41889: F, t41892: F) -> (F, F, F) {
    let t41926 = t41912 + t41925;
    let t41927 = t894 * t41926;
    let t41929 = t901 * t41926;
    let t41931 = -F::cast_from(0.8585111111111111111e-1_f64) * t41882 - F::cast_from(0.82785e-1_f64) * t41885 - F::cast_from(0.132456e1_f64) * t41887 + F::cast_from(0.22076e0_f64) * t41889 + F::cast_from(0.99342e0_f64) * t41892 + F::cast_from(0.24154e1_f64) * t41646 + F::cast_from(0.72462e1_f64) * t41651 + F::cast_from(0.80513333333333333333e0_f64) * t41680 - F::cast_from(0.20128333333333333334e1_f64) * t41695 - F::cast_from(0.80513333333333333332e0_f64) * t41707 - F::cast_from(0.24154e1_f64) * t41713 - F::cast_from(0.108693e2_f64) * t41717 + F::cast_from(0.258925e1_f64) * t41927 + F::cast_from(0.16504875e0_f64) * t41929;
    (t41927, t41929, t41931)
}
