//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1092/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1092<F: Float>(t39621: F, t39629: F, t39631: F, t39633: F, t39635: F, t39637: F, t39640: F, t39643: F, t39645: F, t39655: F, t39658: F, t39660: F, t12126: F, t588: F, t39037: F, t522: F) -> (F, F, F) {
    let t40220 = t39621 + t39629 + t39631 - t39633 + t39635 + t39637 + t39640 + t39643 - t39645 - t39655 - t39658 - t39660;
    let t40221 = t588 * t12126;
    let t40222 = 48.0 * t40221;
    let t40224 = 840.0 * t39037 * t522;
    (t40220, t40222, t40224)
}
