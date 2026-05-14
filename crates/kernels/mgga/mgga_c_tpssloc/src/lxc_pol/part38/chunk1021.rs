//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1021/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1021<F: Float>(t3108: F, t4640: F, t1611: F, t3047: F, t3103: F, t4641: F, t1040: F, t4616: F, t1044: F, t13611: F, t248: F, t1023: F, t13975: F, t4582: F, t3121: F, t4593: F) -> (F, F, F, F, F, F, F) {
    let t14077 = t4640 * t3108;
    let t14080 = t1611 * t3047;
    let t14084 = t4641 * t3103 / 2304.0;
    let t14085 = t4616 * t1040;
    let t14093 = t248 * t1044 * t13611;
    let t14098 = t13975 * t1023;
    let t14099 = t4582 * t14098;
    let t14102 = t4593 * t3121;
    (t14077, t14080, t14084, t14085, t14093, t14099, t14102)
}
