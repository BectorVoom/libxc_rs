//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 747/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk747<F: Float>(t109: F, t4067: F, t656: F, t2327: F, t2328: F, t4041: F, t4044: F, t64: F) -> (F, F) {
    let t110 = F::cast_from(1.0_f64) < t109;
    let t4068 = t656 * t4067;
    let t4072 = piecewise3::<F>(t110, F::cast_from(0.0_f64), t2327 + t2328 / F::cast_from(3.0_f64) + t4041 / F::cast_from(3.0_f64) + t64 * t4044 / F::cast_from(4.0_f64) - t64 * t4068 / F::cast_from(8.0_f64));
    (t4068, t4072)
}
