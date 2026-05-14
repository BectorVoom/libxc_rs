//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 728/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk728<F: Float>(t3616: F, t7376: F, t7375: F, t225: F, t7319: F, t7364: F, t24757: F, t493: F, t3252: F, t7363: F, t7362: F, t3248: F, t1201: F, t2152: F, t24589: F, t24760: F, t24762: F, t24765: F, t24773: F, t24778: F, t24781: F, t3565: F, t3604: F, t470: F, t7283: F, t7373: F, t7387: F, t7389: F) -> (F,) {
    let t24784 = t3616 * t7376;
    let t24785 = t7375 * t24784;
    let t24788 = t7319 * t225;
    let t24789 = t24788 * t7364;
    let t24792 = t493 * t24757;
    let t24794 = t7363 * t3252;
    let t24795 = t7362 * t24794;
    let t24798 = t7363 * t3248;
    let t24799 = t7362 * t24798;
    let t24802 = -0.54831135561607547884e-2 * t24760 - 0.82246703342411321825e-2 * t7283 * t24762 - 0.16449340668482264365e-1 * t7283 * t24765 + t3565 * t2152 + 2.0 * t1201 * t7389 - t24773 + 2.0 * t3604 * t7387 + 0.36554090374405031923e-2 * t7283 * t24778 - 0.82246703342411321825e-2 * t7283 * t24781 + 0.16449340668482264365e-1 * t7373 * t24785 + 0.54831135561607547884e-2 * t24589 * t24789 + t470 * t24792 - 0.27415567780803773942e-2 * t7283 * t24795 - 0.54831135561607547884e-2 * t7283 * t24799;
    (t24802,)
}
