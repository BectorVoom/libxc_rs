//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1097/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1097<F: Float>(t1827: F, t3799: F, t1315: F, t1354: F, t1369: F, t3733: F, t3762: F, t3763: F, t3778: F, t5220: F, t5223: F, t5227: F, t5231: F, t5235: F, t5238: F, t5240: F, t5246: F, t5252: F, t559: F) -> F {
    let t5255 = t3799 * t1827;
    let t5257 = t3762 + F::new(7.0) / F::new(144.0) * t3763 + F::new(7.0) / F::new(144.0) * t5220 + t3733 * t5223 / F::new(16.0) - t1315 * t5227 / F::new(48.0) + t5231 * t559 / F::new(3072.0) - t5235 * t1354 / F::new(3072.0) - F::new(7.0) / F::new(4608.0) * t5238 - t5240 * t1369 / F::new(768.0) - t3778 * t1827 / F::new(3072.0) + t5246 * t5252 / F::new(1536.0) + F::new(7.0) / F::new(4608.0) * t5255;
    t5257
}
