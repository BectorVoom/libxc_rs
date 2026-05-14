//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1302/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1302<F: Float>(t104589: F, t104609: F, t1238: F, t2121: F, t2123: F, t2154: F, t2155: F, t21762: F, t21776: F, t22007: F, t22040: F, t22327: F, t225: F, t24595: F, t27406: F, t27751: F, t29545: F, t29674: F, t29691: F, t29795: F, t29817: F, t3598: F, t45350: F, t462: F, t497: F, t5055: F, t6140: F, t6267: F, t7283: F, t7285: F, t7286: F, t73856: F, t8010: F, t8087: F) -> (F,) {
    let t109888 = 0.82246703342411321825e-2 * t2121 * t462 * t22327 * t225 * t497 - 3.0 * t5055 * t29795 - 0.82246703342411321825e-2 * t7283 * t22040 * t2123 + 0.13159472534785811492e0 * t27406 * t29674 - 0.29243272299524025538e-1 * t27406 * t29691 + 0.21932454224643019154e-1 * t7283 * t24595 * t7286 * t21762 - 0.43864908449286038307e-1 * t104589 - 0.27415567780803773942e-2 * t7283 * t7285 * t7286 * t21776 + 6.0 * t1238 * t3598 * t8087 * t6267 - 0.24674011002723396548e-1 * t7283 * t6140 * t8010 + 0.43864908449286038307e-1 * t27406 * t29817 - 0.24674011002723396548e-1 * t7283 * t27751 * t29545 - 3.0 * t73856 * t2155 - 0.27415567780803773942e-2 * t104609 + 24.0 * t1238 * t45350 * t2154 * t22007;
    (t109888,)
}
