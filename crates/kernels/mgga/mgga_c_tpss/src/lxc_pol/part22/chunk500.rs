//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 500/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk500<F: Float>(t2025: F, t38: F, t45: F, t606: F, t78: F, t57: F, t610: F, t81: F, t1985: F, t1992: F, t608: F, t612: F, t77: F, t1986: F, t1994: F, t1997: F, t583: F, t603: F, t616: F, t71: F, t85: F) -> (F, F, F, F, F, F, F, F) {
    let t2026 = t38 * t2025;
    let t2031 = t606 * t45;
    let t2033 = 1.0 / t78 / t2031;
    let t2038 = t610 * t57;
    let t2040 = 1.0 / t81 / t2038;
    let t2045 = 28.0 / 9.0 * t2033 * t1985 - 4.0 / 3.0 * t608 * t1992 + 28.0 / 9.0 * t2040 * t1985 + 4.0 / 3.0 * t612 * t1992;
    let t2046 = t77 * t2045;
    let t2049 = -t1986 * t85 / 12.0 - t1994 * t85 / 12.0 - t1997 * t85 / 6.0 - t583 * t616 / 6.0 + t2026 * t85 / 24.0 + t603 * t616 / 12.0 + t71 * t2046 / 24.0;
    (t2026, t2031, t2033, t2038, t2040, t2045, t2046, t2049)
}
