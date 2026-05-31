//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1227/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1227<F: Float>(t19693: F, t19706: F, t19718: F, t17948: F, t17962: F, t17976: F, t18737: F, t18746: F, t19698: F, t19700: F, t19704: F, t19708: F, t19710: F, t19712: F, t19716: F, t19720: F, t19722: F) -> F {
    let t20434 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t19693;
    let t20438 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t19706;
    let t20443 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t19718;
    let t20446 = t18737 + t17948 + t20434 + t19698 / F::cast_from(8.0_f64) - t19700 / F::cast_from(24.0_f64) + t19704 / F::cast_from(384.0_f64) + t20438 + t19708 / F::cast_from(192.0_f64) - t19710 / F::cast_from(768.0_f64) - t19712 / F::cast_from(768.0_f64) + t17962 + t18746 + t17976 + t19716 / F::cast_from(192.0_f64) + t20443 + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t19720 - t19722 / F::cast_from(192.0_f64);
    t20446
}
