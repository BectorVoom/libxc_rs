//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 641/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk641<F: Float>(t120: F, t1824: F, t1351: F, t3792: F, t5248: F, t1827: F, t3799: F, t1315: F, t1354: F, t1369: F, t3733: F, t3762: F, t3763: F, t3778: F, t5220: F, t5223: F, t5227: F, t5231: F, t5235: F, t5238: F, t5240: F, t5246: F, t559: F) -> (F, F, F, F) {
    let t5249 = t120 * t1824;
    let t5250 = t3792 * t1351;
    let t5252 = t5248 * t5249 * t5250;
    let t5255 = t3799 * t1827;
    let t5257 = t3762 + 7.0 / 144.0 * t3763 + 7.0 / 144.0 * t5220 + t3733 * t5223 / 16.0 - t1315 * t5227 / 48.0 + t5231 * t559 / 3072.0 - t5235 * t1354 / 3072.0 - 7.0 / 4608.0 * t5238 - t5240 * t1369 / 768.0 - t3778 * t1827 / 3072.0 + t5246 * t5252 / 1536.0 + 7.0 / 4608.0 * t5255;
    (t5249, t5250, t5252, t5257)
}
