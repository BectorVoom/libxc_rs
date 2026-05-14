//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1257/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1257<F: Float>(t6185: F, t9133: F, t31814: F, t33: F, t1497: F, t2436: F, t19570: F, t508: F, t1317: F, t5506: F, t1678: F, t1679: F, t3486: F, t1290: F, t7682: F, t1981: F, t3426: F) -> (F, F, F, F, F, F, F, F) {
    let t64735 = t6185 * t9133;
    let t64879 = t31814 * t33;
    let t64975 = t2436 * t1497;
    let t65135 = t508 * t19570;
    let t65157 = t5506 * t1317;
    let t65158 = t1678 * t65157;
    let t65165 = t1679 * t3486;
    let t65166 = t1678 * t65165;
    let t65169 = t7682 * t1290;
    let t65172 = t1981 * t3426;
    (t64735, t64879, t64975, t65135, t65158, t65166, t65169, t65172)
}
