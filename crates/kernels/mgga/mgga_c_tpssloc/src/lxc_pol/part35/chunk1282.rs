//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1282/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1282<F: Float>(t103507: F, t103520: F, t103546: F, t103573: F, t103610: F, t21510: F, t22340: F, t22348: F, t22364: F, t22386: F, t24589: F, t24812: F, t24814: F, t24815: F, t27516: F, t27536: F, t27549: F, t27550: F, t27551: F, t29740: F, t29744: F, t29762: F, t7373: F, t7375: F, t7376: F, t8066: F, t85963: F, t85965: F, t85966: F, t94784: F) -> (F,) {
    let t109206 = 0.16449340668482264365e-1 * t24589 * t27516 * t29740 + 0.16449340668482264365e-1 * t103507 - 0.54831135561607547884e-2 * t94784 + 0.10966227112321509577e-1 * t27549 * t27550 * t27551 * t21510 + 0.82246703342411321826e-2 * t24589 * t103520 * t8066 + 0.16449340668482264365e-1 * t103546 - 0.54831135561607547883e-2 * t103573 + 0.82246703342411321825e-2 * t7373 * t7375 * t22386 * t7376 - 0.82246703342411321826e-2 * t103610 + 0.24674011002723396548e-1 * t7373 * t7375 * t22340 * t7376 + 0.49348022005446793095e-1 * t24812 * t24814 * t22364 * t24815 + 0.49348022005446793095e-1 * t85963 * t85965 * t22348 * t85966 - 0.24674011002723396548e-1 * t7373 * t27536 * t29744 + 0.16449340668482264365e-1 * t24589 * t27516 * t29762;
    (t109206,)
}
