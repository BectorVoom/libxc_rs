//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 911/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk911<F: Float>(t2135: F, t477: F, t3242: F, t491: F, t3961: F, t24826: F, t8074: F, t24788: F, t8066: F, t3247: F, t24589: F, t24845: F, t24849: F, t27533: F, t27537: F, t27540: F, t27543: F, t27546: F, t27549: F, t3604: F, t3610: F, t3624: F, t7373: F, t8083: F) -> (F, F) {
    let t27550 = t2135 * t477;
    let t27551 = t491 * t3242;
    let t27552 = t27551 * t3961;
    let t27553 = t27550 * t27552;
    let t27556 = t24826 * t8074;
    let t27558 = t24788 * t8066;
    let t27561 = t491 * t3247;
    let t27562 = t27561 * t3961;
    let t27563 = t27550 * t27562;
    let t27568 = -0.27415567780803773942e-2 * t24849 * t27533 - 0.82246703342411321825e-2 * t7373 * t27537 - 0.82246703342411321825e-2 * t7373 * t27540 + 2.0 * t3610 * t27543 - t3624 * t27546 + 0.36554090374405031923e-2 * t27549 * t27553 + 0.27415567780803773942e-2 * t27556 + 0.27415567780803773942e-2 * t24589 * t27558 - 0.54831135561607547884e-2 * t24589 * t27563 + 0.27415567780803773942e-2 * t24845 + t3604 * t8083;
    (t27550, t27568)
}
