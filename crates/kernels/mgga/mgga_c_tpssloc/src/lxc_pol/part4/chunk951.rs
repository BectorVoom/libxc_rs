//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 951/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk951<F: Float>(t17422: F, t913: F, t893: F, t10655: F, t5730: F, t5737: F, t923: F, t5775: F, t950: F, t1581: F, t4471: F, t10740: F, t14263: F, t14266: F, t14337: F, t1569: F, t17377: F, t17379: F, t2856: F, t2905: F, t2930: F, t4411: F, t4434: F, t4454: F, t4476: F, t5743: F, t5759: F, t933: F) -> (F, F, F) {
    let t17423 = t17422 * t913;
    let t17425 = 1.0 * t893 * t17423;
    let t17427 = 0.16081979498692535067e2 * t10655 * t5730;
    let t17428 = t5737 * t923;
    let t17443 = t5775 * t950;
    let t17446 = t1581 * t4471;
    let t17449 = t17377 - t17379 - t17425 - t17427 + 1.0 * t17428 * t933 + 2.0 * t14266 * t1569 + 2.0 * t4411 * t4434 - 2.0 * t10740 * t5743 + 1.0 * t2856 * t5759 - 0.23392894490538584828e1 * t14263 * t4454 + 0.34631718211362927517e2 * t14337 * t4476 + 0.35089341735807877242e1 * t2930 * t17443 - 0.23392894490538584828e1 * t2905 * t17446;
    (t17425, t17427, t17449)
}
