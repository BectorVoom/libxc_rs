//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2197/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2197<F: Float>(t2020: F, t97804: F, t15868: F, t1983: F, t7753: F, t22574: F, t74032: F, t8643: F, t28237: F, t532: F, t6879: F, t510: F, t652: F, t96729: F) -> (F, F, F, F, F) {
    let t97805 = t97804 * t2020;
    let t97808 = F::new(2.0) * t1983 * t7753 * t15868;
    let t97811 = F::new(3.0) * t22574 * t8643 * t74032;
    let t97817 = t532 * t28237;
    let t97820 = F::new(3.0) * t1983 * t97817 * t6879;
    let t97829 = F::new(2.0) * t652 * t510 * t96729;
    (t97805, t97808, t97811, t97820, t97829)
}
