//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1285/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1285<F: Float>(t1734: F, t5392: F, t7376: F, t103830: F, t103867: F, t103877: F, t1244: F, t1246: F, t2144: F, t21510: F, t22243: F, t22348: F, t22364: F, t22389: F, t24589: F, t24812: F, t24820: F, t24821: F, t24849: F, t27406: F, t27516: F, t27550: F, t27561: F, t29735: F, t29741: F, t29758: F, t29759: F, t7373: F, t7375: F, t85963: F, t86015: F, t86022: F, t86023: F, t86076: F, t86077: F, t94837: F) -> (F,) {
    let t109307 = t5392 * t1734 * t7376;
    let t109324 = -0.24674011002723396548e-1 * t24812 * t24820 * t22364 * t24821 + 0.82246703342411321825e-2 * t85963 * t86022 * t22348 * t86023 + 0.43864908449286038307e-1 * t103830 + 0.43864908449286038307e-1 * t27406 * t29741 + 0.24674011002723396548e-1 * t7373 * t7375 * t22389 * t7376 + 0.82246703342411321826e-2 * t24589 * t27516 * t29758 - 0.16449340668482264365e-1 * t24589 * t27550 * t27561 * t21510 + 0.10966227112321509577e-1 * t86076 * t86077 * t109307 - 0.16449340668482264365e-1 * t24849 * t94837 * t29735 - 0.16449340668482264365e-1 * t24849 * t86015 * t109307 - 0.43864908449286038307e-1 * t103867 + 0.82246703342411321826e-2 * t103877 + 0.21932454224643019154e-1 * t27406 * t29759 + t1244 * t2144 * t22243 * t1246;
    (t109324,)
}
