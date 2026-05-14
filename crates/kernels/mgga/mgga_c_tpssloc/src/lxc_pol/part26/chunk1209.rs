//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1209/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1209<F: Float>(t11720: F, t2144: F, t1193: F, t24811: F, t24817: F, t24660: F, t7319: F, t24667: F, t24823: F, t10913: F, t11639: F, t11868: F, t11871: F, t11877: F, t11896: F, t2121: F, t2147: F, t24589: F, t24777: F, t24788: F, t24812: F, t24816: F, t24822: F, t27549: F, t27550: F, t27551: F, t27561: F, t3610: F, t462: F, t7373: F, t7375: F, t7376: F, t7386: F, t7387: F) -> (F, F, F) {
    let t85836 = t2144 * t11720;
    let t85853 = t24811 * t1193;
    let t85854 = t85853 * t24817;
    let t85859 = t7319 * t24660;
    let t85863 = t7319 * t24667;
    let t85883 = t85853 * t24823;
    let t85895 = 3.0 * t11877 * t7387 - 0.49348022005446793095e-1 * t24812 * t85859 * t24816 + 0.24674011002723396548e-1 * t24812 * t85863 * t24822 + 0.82246703342411321825e-2 * t2121 * t462 * t2147 * t11868 + 0.24674011002723396548e-1 * t7373 * t7375 * t11896 * t7376 - 0.16449340668482264365e-1 * t24589 * t27550 * t27561 * t10913 + 0.82246703342411321825e-2 * t7373 * t7375 * t11639 * t7376 - 0.82246703342411321826e-2 * t85883 + 6.0 * t3610 * t7386 * t11871 + 0.10966227112321509577e-1 * t27549 * t27550 * t27551 * t10913 - 0.10966227112321509577e-1 * t27549 * t24788 * t24777;
    (t85836, t85854, t85895)
}
