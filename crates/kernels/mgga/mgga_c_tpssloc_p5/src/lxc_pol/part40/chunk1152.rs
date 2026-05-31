//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1152/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1152<F: Float>(t11137: F, t11459: F, t14702: F, t14720: F, t14946: F, t14947: F, t18203: F, t18208: F, t18213: F, t18217: F, t18219: F, t18223: F, t18227: F, t18229: F, t18234: F, t18239: F, t18243: F) -> F {
    let t18245 = -t11459 + F::cast_from(0.79148148148148148147e-2_f64) * t11137 + F::cast_from(0.15829629629629629629e-1_f64) * t14702 + F::cast_from(0.79148148148148148147e-2_f64) * t14720 - t14946 - t14947 + F::cast_from(0.39574074074074074073e-2_f64) * t18203 + F::cast_from(0.19787037037037037037e-1_f64) * t18208 - F::cast_from(0.71233333333333333332e-1_f64) * t18213 - F::cast_from(0.23744444444444444444e-1_f64) * t18217 - F::cast_from(0.11872222222222222222e-1_f64) * t18219 + F::cast_from(0.10685e0_f64) * t18223 + F::cast_from(0.71233333333333333332e-1_f64) * t18227 - F::cast_from(0.5936111111111111111e-2_f64) * t18229 - F::cast_from(0.11872222222222222222e-1_f64) * t18234 + F::cast_from(0.35616666666666666666e-1_f64) * t18239 + F::cast_from(0.17808333333333333333e-1_f64) * t18243;
    t18245
}
