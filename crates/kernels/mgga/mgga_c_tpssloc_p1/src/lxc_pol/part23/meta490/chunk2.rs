//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1500/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1500<F: Float>(t6460: F, t40343: F, t40347: F, t40350: F, t54633: F, t54639: F, t56465: F, t56469: F, t56484: F, t56491: F, t74702: F, t74724: F, t74741: F, t74745: F) -> (F, F) {
    let t79993 = t6460 * t6460;
    let t80019 = -t40343 + t40347 + t40350 + F::cast_from(0.13148148148148148148e0_f64) * t54633 + F::cast_from(0.22469135802469135801e0_f64) * t54639 - F::cast_from(0.29999999999999999998e-1_f64) * t56465 + F::cast_from(0.99999999999999999996e-2_f64) * t56469 + F::cast_from(0.33333333333333333332e-2_f64) * t74702 - F::cast_from(0.29999999999999999998e-1_f64) * t74724 + F::cast_from(0.23333333333333333332e0_f64) * t56484 - F::cast_from(0.77777777777777777775e-1_f64) * t56491 + F::cast_from(0.18666666666666666665e0_f64) * t74741 + F::cast_from(0.39999999999999999998e-1_f64) * t74745;
    (t79993, t80019)
}
