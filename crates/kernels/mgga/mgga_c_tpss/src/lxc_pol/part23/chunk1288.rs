//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1288/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1288<F: Float>(t18547: F, t44070: F, t7029: F, t13220: F, t94: F, t1689: F, t19308: F, t5522: F, t5753: F, t9895: F, t19579: F, t19581: F, t1206: F, t19580: F, t13119: F, t1760: F, t5754: F) -> (F, F, F, F, F, F) {
    let t65066 = 6.0 * t18547 * t7029 * t44070;
    let t65067 = t94 * t13220;
    let t65069 = 2.0 * t65067 * t1689;
    let t65071 = 4.0 * t19308 * t5522;
    let t65076 = t5753 * t9895;
    let t65079 = 4.0 * t19579 * t65076 * t19581;
    let t65085 = t19581 * t1206;
    let t65088 = 12.0 * t18547 * t19580 * t65085;
    let t65091 = 2.0 * t1760 * t5754 * t13119;
    (t65066, t65069, t65071, t65079, t65088, t65091)
}
