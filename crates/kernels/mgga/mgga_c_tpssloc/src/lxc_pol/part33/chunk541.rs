//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 541/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk541<F: Float>(t1340: F, t5234: F, t1358: F, t1815: F, t1362: F, t242: F, t3788: F, t1336: F, t557: F, t67: F, t246: F, t120: F, t1824: F, t1827: F, t3799: F, t1788: F, t588: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5235 = t5234 * t1340;
    let t5238 = t1815 * t1358;
    let t5240 = t5234 * t1362;
    let t5245 = t3788 * t242;
    let t5246 = t1336 * t5245;
    let t5247 = t557 * t67;
    let t5248 = t5247 * t246;
    let t5249 = t120 * t1824;
    let t5255 = t3799 * t1827;
    let t5264 = t588 * t1788;
    (t5235, t5238, t5240, t5246, t5247, t5248, t5249, t5255, t5264)
}
