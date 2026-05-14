//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1147/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1147<F: Float>(t12211: F, t5223: F, t1307: F, t210: F, t5226: F, t1810: F, t3719: F, t3804: F, t820: F, t1351: F, t1824: F, t3807: F, t3792: F, t12345: F, t1831: F, t12429: F, t1354: F, t16257: F, t16261: F, t16265: F, t16269: F, t16271: F, t16275: F, t16278: F, t16285: F, t16290: F, t3733: F, t3783: F, t3795: F, t3803: F, t3853: F, t3858: F, t3872: F, t5235: F, t5240: F, t5246: F, t5293: F, t5310: F) -> (F,) {
    let t16294 = 7.0 / 24.0 * t12211 * t5223;
    let t16296 = t210 * t5226 * t1307;
    let t16300 = t210 * t1810 * t3719;
    let t16305 = t3804 * t820;
    let t16306 = t1824 * t1351;
    let t16307 = t16306 * t3807;
    let t16308 = t16305 * t16307;
    let t16311 = t1824 * t3792;
    let t16312 = t1351 * t1307;
    let t16313 = t16311 * t16312;
    let t16314 = t16305 * t16313;
    let t16317 = t12345 * t1831;
    let t16319 = -t12429 * t5293 / 1536.0 + t5246 * t16257 / 768.0 + t5246 * t16261 / 1536.0 - t3803 * t16265 / 3072.0 - t16269 - t3803 * t16271 / 1536.0 - t3803 * t16275 / 3072.0 - t16278 * t1354 / 1536.0 - t5235 * t3853 / 3072.0 + 5.0 / 384.0 * t3783 * t5310 + t16285 * t3795 / 1536.0 + t16290 - t5235 * t3858 / 3072.0 - t16294 + t3733 * t16296 / 8.0 + t3733 * t16300 / 16.0 + 5.0 / 768.0 * t5240 * t3872 + t3803 * t16308 / 384.0 - t5246 * t16314 / 192.0 - 119.0 / 3456.0 * t16317;
    (t16319,)
}
