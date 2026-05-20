//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2199/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2199<F: Float>(t28002: F, t6535: F, t12725: F, t7461: F, t19456: F, t25980: F, t4028: F, t7468: F, t2314: F, t28045: F, t4034: F, t5107: F, t652: F, t7467: F) -> (F, F, F, F, F, F, F, F) {
    let t97844 = F::new(4.0) * t28002 * t6535;
    let t97846 = F::new(4.0) * t12725 * t7461;
    let t97848 = F::new(4.0) * t19456 * t7461;
    let t97850 = F::new(4.0) * t4028 * t25980;
    let t97854 = F::new(4.0) * t12725 * t7468;
    let t97856 = F::new(4.0) * t2314 * t28045;
    let t97858 = F::new(4.0) * t4034 * t28045;
    let t97862 = F::new(4.0) * t652 * t5107 * t7467;
    (t97844, t97846, t97848, t97850, t97854, t97856, t97858, t97862)
}
