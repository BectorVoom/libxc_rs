//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1161/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1161<F: Float>(t19046: F, t5018: F, t5023: F, t6169: F, t18321: F, t5040: F, t1009: F, t22113: F, t1011: F, t1212: F, t18375: F, t5002: F, t1730: F, t19032: F, t1017: F, t1207: F, t1210: F, t22173: F) -> (F, F, F, F, F, F, F, F) {
    let t72304 = t19046 * t5018;
    let t72307 = t6169 * t5023;
    let t72352 = t18321 * t5040;
    let t72361 = t22113 * t1009;
    let t72363 = t72361 * t1011 * t1212;
    let t72366 = t5002 * t18375;
    let t72384 = t1730 * t19032;
    let t72389 = t1207 * t1210 * t22173 * t1017;
    (t72304, t72307, t72352, t72361, t72363, t72366, t72384, t72389)
}
