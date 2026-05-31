//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2870/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2870<F: Float>(t4359: F, t49486: F, t4400: F, t49269: F, t13727: F, t14379: F, t10661: F, t2793: F, t5695: F, t13520: F, t14389: F, t10655: F, t17507: F) -> (F, F, F, F, F, F) {
    let t60006 = F::cast_from(8.0_f64) * t49486 * t4359;
    let t60008 = F::cast_from(0.64327917994770140268e2_f64) * t49269 * t4400;
    let t60010 = F::cast_from(8.0_f64) * t13727 * t14379;
    let t60016 = F::cast_from(24.0_f64) * t10661 * t5695 * t2793;
    let t60021 = F::cast_from(0.64327917994770140268e2_f64) * t13520 * t14389;
    let t60023 = F::cast_from(12.0_f64) * t10655 * t17507;
    (t60006, t60008, t60010, t60016, t60021, t60023)
}
