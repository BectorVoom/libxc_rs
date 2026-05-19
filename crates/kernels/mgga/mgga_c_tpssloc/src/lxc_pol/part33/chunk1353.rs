//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1353/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1353<F: Float>(t20217: F, t3: F, t1933: F, t1937: F, t21526: F, t23604: F, t23678: F, t25652: F, t25653: F, t25658: F, t28582: F, t5866: F, t7578: F, t83016: F, t88372: F, t99692: F, t99796: F, t99799: F, t99802: F, t99813: F, t99834: F) -> F {
    let t106348 = t3 * t20217;
    let t106352 = -F::cast_from(0.30279567070605293142e-3_f64) * t99692 * t7578 - F::cast_from(0.60559134141210586284e-3_f64) * t99796 - F::cast_from(0.30279567070605293142e-3_f64) * t99799 + F::cast_from(0.60559134141210586284e-3_f64) * t99802 + F::cast_from(0.60559134141210586284e-3_f64) * t99813 - F::cast_from(0.60559134141210586284e-3_f64) * t99834 + t83016 * t21526 / F::new(384.0) + F::cast_from(0.30279567070605293142e-3_f64) * t88372 * t28582 - F::cast_from(0.30279567070605293142e-3_f64) * t25652 * t25658 * t23604 * t5866 + F::cast_from(0.60559134141210586284e-3_f64) * t25652 * t25653 * t23678 * t5866 + F::cast_from(0.10093189023535097714e-3_f64) * t1933 * t106348 * t1937;
    t106352
}
