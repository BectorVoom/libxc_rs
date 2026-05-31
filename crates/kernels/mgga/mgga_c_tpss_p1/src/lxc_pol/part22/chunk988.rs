//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 988/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk988<F: Float>(t10552: F, t774: F, t801: F, t2143: F, t3622: F, t124: F, t762: F, t1369: F, t8176: F, t1368: F, t2116: F, t3618: F, t8167: F) -> (F, F, F, F, F, F) {
    let t10623 = t801 * t774 * t10552;
    let t10630 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t2143 * t3622;
    let t10631 = t124 * t10552;
    let t10632 = t762 * t10631;
    let t10635 = t8176 * t1369;
    let t10638 = t762 * t1368 * t2116;
    let t10642 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t8167 * t3618;
    (t10623, t10630, t10632, t10635, t10638, t10642)
}
