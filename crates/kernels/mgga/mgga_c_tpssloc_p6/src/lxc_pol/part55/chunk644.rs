//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 644/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk644<F: Float>(t120: F, t1824: F, t1351: F, t3792: F, t5248: F, t1827: F, t3799: F, t1315: F, t1354: F, t1369: F, t3733: F, t3762: F, t3763: F, t3778: F, t5220: F, t5223: F, t5227: F, t5231: F, t5235: F, t5238: F, t5240: F, t5246: F, t559: F) -> (F, F, F, F) {
    let t5249 = t120 * t1824;
    let t5250 = t3792 * t1351;
    let t5252 = t5248 * t5249 * t5250;
    let t5255 = t3799 * t1827;
    let t5257 = t3762 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t3763 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t5220 + t3733 * t5223 / F::cast_from(16.0_f64) - t1315 * t5227 / F::cast_from(48.0_f64) + t5231 * t559 / F::cast_from(3072.0_f64) - t5235 * t1354 / F::cast_from(3072.0_f64) - F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t5238 - t5240 * t1369 / F::cast_from(768.0_f64) - t3778 * t1827 / F::cast_from(3072.0_f64) + t5246 * t5252 / F::cast_from(1536.0_f64) + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t5255;
    (t5249, t5250, t5252, t5257)
}
