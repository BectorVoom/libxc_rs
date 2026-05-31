//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1346/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1346<F: Float>(t63957: F, t63960: F, t63964: F, t63966: F, t61063: F, t61065: F, t61073: F, t62711: F, t63951: F, t63953: F, t63955: F, t63962: F, t63968: F) -> F {
    let t66418 = F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t63957;
    let t66420 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t63960;
    let t66422 = F::cast_from(119.0_f64) / F::cast_from(864.0_f64) * t63964;
    let t66423 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t63966;
    let t66425 = -F::cast_from(35.0_f64) / F::cast_from(54.0_f64) * t61063 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t61065 - t63951 / F::cast_from(48.0_f64) + t63953 / F::cast_from(192.0_f64) + t63955 / F::cast_from(384.0_f64) - t66418 - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t61073 + t66420 - t63962 / F::cast_from(192.0_f64) - t66422 - t62711 + t66423 - t63968 / F::cast_from(24.0_f64);
    t66425
}
