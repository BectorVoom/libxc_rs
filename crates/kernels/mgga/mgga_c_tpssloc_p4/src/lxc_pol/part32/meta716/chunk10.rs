//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2269/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2269<F: Float>(t28299: F, t81979: F, t28273: F, t6547: F, t13042: F, t17052: F, t17090: F, t218: F, t25170: F, t25330: F, t259: F, t4147: F, t6632: F, t7517: F, t82259: F, t98876: F, t98975: F, t98983: F, t98986: F) -> F {
    let t98993 = t81979 * t28299;
    let t98995 = t6547 * t28273;
    let t98999 = -F::new(12.0) * t98975 * t25170 + F::new(4.0) * t13042 * t7517 + F::new(2.0) * t17052 * t6632 + F::cast_from(0.41123351671205660912e-2_f64) * t98983 - F::cast_from(0.82246703342411321825e-2_f64) * t98986 - F::new(2.0) * t4147 * t25330 + F::cast_from(0.63969658155208805863e-1_f64) * t82259 + F::new(2.0) * t17090 * t6632 - F::cast_from(0.11514538467937585055e0_f64) * t98993 - F::cast_from(0.19190897446562641759e-1_f64) * t98995 + t218 * t98876 * t259;
    t98999
}
