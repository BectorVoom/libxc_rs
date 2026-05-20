//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2269/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2269<F: Float>(t11588: F, t6144: F, t3447: F, t3451: F, t15402: F, t18237: F, t1887: F, t337: F, t5416: F, t4904: F, t51968: F, t1174: F, t135: F, t18525: F) -> (F, F, F, F, F, F) {
    let t64779 = t11588 * t6144;
    let t64781 = t3447 * t64779 * t3451;
    let t64784 = t3447 * t15402 * t18237;
    let t64811 = t5416 * t337 * t1887;
    let t64821 = t3447 * t51968 * t4904;
    let t64858 = t1174 * t135 * t18525;
    (t64779, t64781, t64784, t64811, t64821, t64858)
}
