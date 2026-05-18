//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1172/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1172<F: Float>(t3252: F, t7363: F, t7362: F, t3248: F, t1201: F, t2152: F, t24589: F, t24760: F, t24762: F, t24765: F, t24773: F, t24778: F, t24781: F, t24785: F, t24789: F, t24792: F, t3565: F, t3604: F, t470: F, t7283: F, t7373: F, t7387: F, t7389: F) -> (F, F, F, F, F) {
    let t24794 = t7363 * t3252;
    let t24795 = t7362 * t24794;
    let t24798 = t7363 * t3248;
    let t24799 = t7362 * t24798;
    let t24802 = -F::new(0.54831135561607547884e-2) * t24760 - F::new(0.82246703342411321825e-2) * t7283 * t24762 - F::new(0.16449340668482264365e-1) * t7283 * t24765 + t3565 * t2152 + F::new(2.0) * t1201 * t7389 - t24773 + F::new(2.0) * t3604 * t7387 + F::new(0.36554090374405031923e-2) * t7283 * t24778 - F::new(0.82246703342411321825e-2) * t7283 * t24781 + F::new(0.16449340668482264365e-1) * t7373 * t24785 + F::new(0.54831135561607547884e-2) * t24589 * t24789 + t470 * t24792 - F::new(0.27415567780803773942e-2) * t7283 * t24795 - F::new(0.54831135561607547884e-2) * t7283 * t24799;
    (t24794, t24795, t24798, t24799, t24802)
}
