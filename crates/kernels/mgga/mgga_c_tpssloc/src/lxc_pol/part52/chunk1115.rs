//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1115/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1115<F: Float>(t2127: F, t27548: F, t2135: F, t477: F, t3242: F, t491: F, t3961: F, t24826: F, t8074: F, t24788: F, t8066: F, t3247: F) -> (F, F, F, F, F, F) {
    let t27549 = t2127 * t27548;
    let t27550 = t2135 * t477;
    let t27551 = t491 * t3242;
    let t27552 = t27551 * t3961;
    let t27553 = t27550 * t27552;
    let t27556 = t24826 * t8074;
    let t27558 = t24788 * t8066;
    let t27561 = t491 * t3247;
    (t27549, t27550, t27553, t27556, t27558, t27561)
}
