//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1072/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1072<F: Float>(t7327: F, t8034: F, t221: F, t4899: F, t2127: F, t2135: F, t477: F, t3242: F, t491: F, t24826: F, t8074: F, t3247: F, t7359: F, t7999: F, t1222: F, t8043: F) -> (F, F, F, F, F, F, F, F) {
    let t27536 = t8034 * t7327;
    let t27548 = t221 * t4899;
    let t27549 = t2127 * t27548;
    let t27550 = t2135 * t477;
    let t27551 = t491 * t3242;
    let t27556 = t24826 * t8074;
    let t27561 = t491 * t3247;
    let t27572 = t7999 * t7359;
    let t27578 = t8043 * t1222;
    (t27536, t27549, t27550, t27551, t27556, t27561, t27572, t27578)
}
