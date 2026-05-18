//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1448/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1448<F: Float>(t22892: F, t22893: F, t33276: F, t22751: F, t33277: F, t552: F, t7918: F, t1307: F, t6637: F, t6888: F, t114104: F, t114119: F, t120505: F, t120506: F, t120507: F, t120513: F, t120515: F, t120522: F, t120525: F, t120526: F, t122518: F, t122522: F, t122526: F, t122530: F) -> F {
    let t122533 = t22892 * t22893 * t33276;
    let t122535 = t22751 * t33277;
    let t122537 = t552 * t7918;
    let t122540 = t6888 * t6637 * t122537 * t1307;
    let t122542 = F::new(0.16449340668482264365e-1) * t122518 + t120505 - t120506 + t114104 + t120507 + t120513 - t120515 - t120522 + F::new(0.16449340668482264365e-1) * t122522 - F::new(0.16449340668482264365e-1) * t122526 - F::new(0.16449340668482264365e-1) * t122530 + F::new(0.82246703342411321825e-2) * t122533 + F::new(0.38381794893125283518e-1) * t122535 - F::new(0.16449340668482264365e-1) * t122540 - t120525 + t114119 + t120526;
    t122542
}
