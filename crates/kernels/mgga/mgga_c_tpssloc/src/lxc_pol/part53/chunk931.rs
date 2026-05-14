//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 931/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk931<F: Float>(t4025: F, t8717: F, t120121: F, t120123: F, t120125: F, t120131: F, t124367: F, t27170: F, t31237: F, t31239: F, t33152: F, t33154: F, t34682: F, t34707: F, t7801: F, t8446: F, t9012: F) -> (F, F) {
    let t124538 = t4025 * t8717;
    let t124540 = 4.0 * t27170 * t9012 + 4.0 * t34682 * t7801 + 4.0 * t34707 * t7801 + t120121 + t120123 + t120125 + t120131 + t124367 + 2.0 * t124538 + t31237 + t31239 + t33152 + t33154 + t8446;
    (t124538, t124540)
}
