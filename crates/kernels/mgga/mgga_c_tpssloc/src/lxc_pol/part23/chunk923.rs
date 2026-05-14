//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 923/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk923<F: Float>(t20936: F, t225: F, t237: F, t119: F, t20756: F, t210: F, t1484: F, t5544: F, t2701: F, t820: F, t20870: F, t819: F, t13283: F, t1512: F, t1516: F, t16872: F, t16976: F, t20904: F, t20908: F, t249: F, t4172: F, t5587: F, t5624: F, t5628: F, t817: F, t843: F, t9559: F, t9974: F) -> (F, F, F, F, F, F, F) {
    let t20937 = t20936 * t225;
    let t20938 = t20937 * t237;
    let t20943 = t119 * t20756;
    let t20944 = t210 * t20943;
    let t20947 = t1484 * t5544;
    let t20949 = t2701 * t820 * t20947;
    let t20953 = t819 * t820 * t20870;
    let t20958 = -t9974 * t20904 / 512.0 - t843 * t20908 / 768.0 + 5.0 / 256.0 * t4172 * t5624 - t16976 * t1516 / 256.0 - t4172 * t5628 / 256.0 + t20938 * t249 / 3072.0 + t13283 * t5587 / 512.0 - t9559 * t20944 / 4.0 + 5.0 / 256.0 * t843 * t20949 - t817 * t20953 / 3072.0 - t16872 * t1512 / 1024.0;
    (t20937, t20938, t20944, t20947, t20949, t20953, t20958)
}
