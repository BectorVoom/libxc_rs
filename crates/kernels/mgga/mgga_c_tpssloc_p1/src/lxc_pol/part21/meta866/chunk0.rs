//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3160/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3160<F: Float>(t65278: F, t65279: F, t65281: F, t65282: F, t65285: F, t65286: F, t65297: F, t65327: F, t11881: F, t11907: F, t1235: F, t1244: F, t1246: F, t14986: F, t15000: F, t15009: F, t15027: F, t15239: F, t1755: F, t18940: F, t19128: F, t19160: F, t19179: F, t3610: F, t3612: F, t3613: F, t3624: F, t3626: F, t491: F, t5064: F, t5079: F, t6260: F, t65221: F, t65254: F, t65262: F, t65265: F) -> (F, F) {
    let t65330 = t65278 + t65279 + t65281 + t65282 + t65285 + t65286 + t65297 + t65327;
    let t65343 = F::cast_from(2.0_f64) * t1235 * t1244 * t1246 * t18940 + t1244 * t1246 * t491 * t65330 + F::cast_from(4.0_f64) * t15239 * t1755 * t3610 * t3612 + F::cast_from(6.0_f64) * t11881 * t15000 * t6260 - F::cast_from(2.0_f64) * t19128 * t3624 * t5079 - F::cast_from(4.0_f64) * t19179 * t3624 * t5079 + F::cast_from(2.0_f64) * t3610 * t3612 * t65221 + F::cast_from(4.0_f64) * t3610 * t3612 * t65265 - F::cast_from(2.0_f64) * t11907 * t19160 + F::cast_from(2.0_f64) * t14986 * t5064 + F::cast_from(4.0_f64) * t15009 * t15027 + F::cast_from(2.0_f64) * t3613 * t65254 - t3626 * t65262;
    (t65330, t65343)
}
