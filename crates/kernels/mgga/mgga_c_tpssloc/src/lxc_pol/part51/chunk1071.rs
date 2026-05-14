//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1071/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1071<F: Float>(t31584: F, t539: F, t225: F, t567: F, t7191: F, t214: F, t1985: F, t22674: F, t8621: F, t6897: F, t2092: F, t22656: F, t31106: F, t31111: F, t31113: F, t31115: F, t31122: F, t31126: F, t3882: F, t568: F, t8637: F) -> (F, F, F, F, F, F) {
    let t31585 = t539 * t31584;
    let t31589 = t7191 * t225 * t567;
    let t31590 = t214 * t31589;
    let t31591 = t1985 * t31590;
    let t31594 = t22674 * t8621;
    let t31595 = t6897 * t31594;
    let t31596 = 0.41123351671205660912e-2 * t31595;
    let t31597 = t31585 * t568 - t31106 + t31111 - t22656 * t2092 - t31113 + 0.82246703342411321825e-2 * t31591 - t3882 * t8637 + t31115 + t31596 - t31122 - t31126;
    (t31585, t31589, t31590, t31594, t31596, t31597)
}
