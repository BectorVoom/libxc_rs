//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 251/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk251<F: Float>(t974: F, t976: F, t344: F, t883: F, t607: F, t906: F, t910: F, t340: F, t343: F, t346: F, t964: F, t971: F, t973: F) -> (F, F, F, F, F, F) {
    let t977 = t974 * t976;
    let t978 = t344 * t883;
    let t979 = t978 * t607;
    let t980 = t977 * t979;
    let t984 = t906 / F::cast_from(6.0_f64) + t910 / F::cast_from(6.0_f64);
    let t985 = t340 * t984;
    let t986 = t985 * t343;
    let t987 = t974 * t986;
    let t990 = -F::cast_from(0.22222222222222222222e-2_f64) * t964 * t346 + t971 + F::cast_from(0.27777777777777777777e-3_f64) * t973 * t980 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t987;
    (t977, t978, t979, t984, t986, t990)
}
