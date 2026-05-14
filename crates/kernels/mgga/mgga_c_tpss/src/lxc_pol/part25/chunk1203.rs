//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1203/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1203<F: Float>(t13771: F, t5728: F, t13793: F, t215: F, t65595: F, t13798: F, t19469: F, t19539: F, t6259: F, t1232: F, t43710: F, t5381: F, t1656: F, t4459: F, t520: F, t5432: F) -> (F, F, F, F, F, F, F, F) {
    let t69558 = t5728 * t13771;
    let t69561 = t65595 * t215 * t13793;
    let t69564 = t19469 * t215 * t13798;
    let t69654 = t6259 * t19539;
    let t69663 = t43710 * t1232;
    let t69667 = t5381 * t1232;
    let t69676 = t1656 * t4459 * t520;
    let t69681 = t5432 * t1232 * t520;
    (t69558, t69561, t69564, t69654, t69663, t69667, t69676, t69681)
}
