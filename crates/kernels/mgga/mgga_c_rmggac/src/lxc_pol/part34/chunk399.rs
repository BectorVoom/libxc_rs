//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 399/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk399<F: Float>(t8747: F, t8761: F, t27: F, t3851: F, t8751: F, t7599: F, t8754: F, t8645: F, t3839: F, t8641: F, t3826: F, t7595: F, t7597: F, t7618: F, t7620: F, t8759: F) -> (F, F, F, F, F, F, F) {
    let t8762 = t8761 * t8747;
    let t8764 = t3851 * t27;
    let t8765 = t8764 * t8751;
    let t8767 = t7599 * t8754;
    let t8769 = t3851 * t8645;
    let t8771 = t3839 * t8641;
    let t8773 = t3826 * t8645;
    let t8778 = -0.63504270469206447408e-3 * t8759 + 0.84672360625608596544e-3 * t8762 + 0.68186654135613354324e-2 * t8765 - 0.13637330827122670865e-1 * t8767 + t7595 + 0.2993560425465952141e-1 * t8769 - 0.13276154105060581339e-2 * t8771 + 0.19914231157590872008e-2 * t8773 + 0.2660942600414179681e-1 * t7597 - 0.39914139006212695215e-1 * t7618 + 0.88507694033737208925e-3 * t7620;
    (t8762, t8765, t8767, t8769, t8771, t8773, t8778)
}
