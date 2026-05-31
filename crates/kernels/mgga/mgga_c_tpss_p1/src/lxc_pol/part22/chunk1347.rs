//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1347/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1347<F: Float>(t63973: F, t63977: F, t63990: F, t61081: F, t61087: F, t61089: F, t63971: F, t63975: F, t63979: F, t63981: F, t63984: F, t63987: F, t63995: F) -> F {
    let t66427 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t63973;
    let t66429 = F::cast_from(35.0_f64) / F::cast_from(144.0_f64) * t63977;
    let t66434 = F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t63990;
    let t66439 = -F::cast_from(5.0_f64) / F::cast_from(32.0_f64) * t63971 + t66427 - t63975 / F::cast_from(768.0_f64) - t66429 + F::cast_from(5.0_f64) / F::cast_from(96.0_f64) * t63979 + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t63981 + t63984 / F::cast_from(4.0_f64) + t63987 / F::cast_from(8.0_f64) - t66434 - t63995 / F::cast_from(2.0_f64) + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t61081 - F::cast_from(119.0_f64) / F::cast_from(432.0_f64) * t61087 - F::cast_from(35.0_f64) / F::cast_from(288.0_f64) * t61089;
    t66439
}
