//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1264/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1264<F: Float>(t30: F, t31814: F, t19797: F, t2436: F, t198: F, t206: F, t6148: F, t2: F, t8096: F, t33: F, t1497: F, t19570: F, t508: F, t1317: F, t5506: F, t1679: F, t3486: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t64247 = t31814 * t30;
    let t64277 = t19797 * t2436;
    let t64284 = t198 * t206 * t6148;
    let t64300 = t2436 * t2;
    let t64305 = t6148 * t8096;
    let t64879 = t31814 * t33;
    let t64975 = t2436 * t1497;
    let t65135 = t508 * t19570;
    let t65157 = t5506 * t1317;
    let t65165 = t1679 * t3486;
    (t64247, t64277, t64284, t64300, t64305, t64879, t64975, t65135, t65157, t65165)
}
