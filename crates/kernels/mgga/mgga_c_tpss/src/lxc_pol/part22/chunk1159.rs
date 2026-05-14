//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1159/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1159<F: Float>(t19693: F, t19706: F, t19718: F, t17948: F, t17962: F, t17976: F, t18737: F, t18746: F, t19698: F, t19700: F, t19704: F, t19708: F, t19710: F, t19712: F, t19716: F, t19720: F, t19722: F) -> (F,) {
    let t20434 = 7.0 / 72.0 * t19693;
    let t20438 = 7.0 / 1152.0 * t19706;
    let t20443 = 7.0 / 288.0 * t19718;
    let t20446 = t18737 + t17948 + t20434 + t19698 / 8.0 - t19700 / 24.0 + t19704 / 384.0 + t20438 + t19708 / 192.0 - t19710 / 768.0 - t19712 / 768.0 + t17962 + t18746 + t17976 + t19716 / 192.0 + t20443 + 5.0 / 192.0 * t19720 - t19722 / 192.0;
    (t20446,)
}
