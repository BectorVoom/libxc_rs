//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2728/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2728<F: Float>(t1834: F, t5286: F, t12240: F, t1352: F, t16036: F, t16037: F, t16041: F, t16047: F, t16048: F, t16055: F, t16419: F, t19654: F, t19661: F, t19735: F, t19736: F, t19739: F, t19743: F, t19810: F, t3793: F, t3851: F, t40335: F, t5334: F, t5344: F) -> (F, F) {
    let t57499 = t1834 * t5286;
    let t57526 = F::cast_from(6.0_f64) * t12240 * t19743 * t5334 - F::cast_from(4.0_f64) * t1352 * t5344 * t57499 + F::cast_from(8.0_f64) * t16036 * t19735 * t5334 - F::cast_from(12.0_f64) * t16047 * t16048 * t19739 - F::cast_from(6.0_f64) * t16047 * t19743 * t40335 + F::cast_from(12.0_f64) * t19739 * t3793 * t5334 - F::cast_from(2.0_f64) * t19739 * t3851 * t5344 - t19743 * t3851 * t5344 + F::cast_from(8.0_f64) * t16037 * t19654 + F::cast_from(8.0_f64) * t16041 * t19654 + F::cast_from(4.0_f64) * t16055 * t19661 + F::cast_from(8.0_f64) * t16055 * t19736 - F::cast_from(4.0_f64) * t16419 * t19810;
    (t57499, t57526)
}
