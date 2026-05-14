//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1092/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1092<F: Float>(t1216: F, t18300: F, t4582: F, t5001: F, t5018: F, t1730: F, t5023: F, t1177: F, t18225: F, t1193: F, t6109: F, t248: F, t3570: F, t6230: F, t3515: F, t1230: F, t18241: F) -> (F, F, F, F, F, F, F) {
    let t19076 = t18300 * t1216;
    let t19077 = t4582 * t19076;
    let t19080 = t5001 * t5018;
    let t19083 = t1730 * t5023;
    let t19087 = t1177 * t18225;
    let t19090 = t6109 * t1193;
    let t19095 = t248 * t3570 * t6230;
    let t19096 = t3515 * t19095;
    let t19101 = t248 * t1230 * t18241;
    (t19077, t19080, t19083, t19087, t19090, t19096, t19101)
}
