//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2327/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2327<F: Float>(t18356: F, t24729: F, t27614: F, t4997: F, t1730: F, t27603: F, t27598: F, t5001: F, t1218: F, t1232: F, t1737: F, t18523: F, t19101: F, t2134: F, t24736: F, t460: F, t5014: F, t6211: F, t6227: F, t7320: F, t7345: F, t86140: F, t95238: F, t95507: F, t95511: F, t95512: F) -> F {
    let t104294 = t24729 * t18356;
    let t104296 = t27614 * t4997;
    let t104300 = t1730 * t27603;
    let t104303 = t5001 * t27598;
    let t104319 = t104294 / F::new(1152.0) + t104296 / F::new(1152.0) + t86140 * t6227 / F::new(768.0) + t104300 * t1232 / F::new(216.0) - t104303 * t1218 / F::new(144.0) + t95238 * t1737 / F::new(768.0) + t27614 * t5014 / F::new(768.0) - t7345 * t19101 / F::new(2304.0) - t24736 * t6211 / F::new(1152.0) - F::cast_from(0.10093189023535097714e-3_f64) * t2134 * t18523 * t460 * t7320 - t95507 + t95511 + t95512 / F::new(648.0);
    t104319
}
