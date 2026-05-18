//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1426/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1426<F: Float>(t1983: F, t28826: F, t91675: F, t28030: F, t7468: F, t28002: F, t7461: F, t28864: F, t7458: F, t28045: F, t4028: F, t6287: F, t652: F, t7467: F) -> (F, F, F, F, F, F, F) {
    let t107515 = F::new(18.0) * t1983 * t91675 * t28826;
    let t107519 = F::new(6.0) * t28030 * t7468;
    let t107521 = F::new(12.0) * t28002 * t7461;
    let t107523 = F::new(6.0) * t7458 * t28864;
    let t107525 = F::new(12.0) * t4028 * t28045;
    let t107527 = F::new(12.0) * t7458 * t28045;
    let t107530 = F::new(6.0) * t652 * t6287 * t7467;
    (t107515, t107519, t107521, t107523, t107525, t107527, t107530)
}
