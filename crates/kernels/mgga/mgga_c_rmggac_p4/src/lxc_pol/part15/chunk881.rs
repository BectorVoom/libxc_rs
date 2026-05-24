//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 881/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk881<F: Float>(t10024: F, t34761: F, t38415: F, t38460: F, t42600: F, t44755: F, t44759: F, t44763: F, t44767: F, t44771: F, t44773: F, t44777: F, t44781: F, t44784: F, t44786: F, t44789: F, t44793: F, t44795: F, t44799: F) -> F {
    let t44801 = t34761 * t10024;
    let t44803 = -F::cast_from(0.25538759935978703638e-4_f64) * t44755 + t42600 - F::cast_from(0.1064114997332445985e-4_f64) * t44759 - F::cast_from(0.25538759935978703638e-4_f64) * t44763 - F::cast_from(0.85129199786595678796e-5_f64) * t44767 + F::cast_from(0.25538759935978703638e-4_f64) * t44771 - F::cast_from(0.85129199786595678796e-5_f64) * t44773 - F::cast_from(0.23942587439980034662e-4_f64) * t44777 + t38415 - F::cast_from(0.31923449919973379548e-4_f64) * t44781 - F::cast_from(0.5586603735995341421e-4_f64) * t38460 + F::cast_from(0.59590439850616975155e-4_f64) * t44784 + F::cast_from(0.27274661654245341729e-1_f64) * t44786 + F::cast_from(0.13637330827122670864e-1_f64) * t44789 + F::cast_from(0.31923449919973379548e-4_f64) * t44793 + F::cast_from(0.20455996240684006296e-1_f64) * t44795 - F::cast_from(0.25538759935978703638e-4_f64) * t44799 + F::cast_from(0.25538759935978703638e-4_f64) * t44801;
    t44803
}
