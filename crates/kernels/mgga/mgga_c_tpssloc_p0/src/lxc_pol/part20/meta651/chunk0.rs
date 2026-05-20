//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2394/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2394<F: Float>(t13654: F, t2842: F, t2844: F, t912: F, t10727: F, t13727: F, t10731: F, t13520: F, t41811: F, t4359: F, t41623: F, t4400: F) -> (F, F, F, F, F) {
    let t49080 = F::cast_from(0.48245938496077605201e2_f64) * t2842 * t13654 * t2844 * t912;
    let t49082 = F::new(6.0) * t13727 * t10727;
    let t49084 = F::cast_from(0.48245938496077605201e2_f64) * t13520 * t10731;
    let t49086 = F::new(6.0) * t41811 * t4359;
    let t49088 = F::cast_from(0.48245938496077605201e2_f64) * t41623 * t4400;
    (t49080, t49082, t49084, t49086, t49088)
}
