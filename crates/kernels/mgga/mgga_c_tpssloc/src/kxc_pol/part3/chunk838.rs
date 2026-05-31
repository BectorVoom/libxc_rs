//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 838/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk838<F: Float>(t25: F, t193: F, t531: F, t1799: F, t571: F, t3919: F, t1408: F, t3664: F, t2: F, t514: F, t584: F, t606: F, t1649: F, t3672: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t5126 = t193 * t531;
    let t5127 = t571 * t1799;
    let t5131 = t3919 * t1799;
    let t5134 = t3664 * t1408;
    let t5137 = t514 * t2;
    let t5141 = piecewise3::<F>(t26, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t5134 * t606 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t5137 * t584);
    let t5142 = t3672 * t1649;
    (t5126, t5127, t5131, t5134, t5137, t5141, t5142)
}
