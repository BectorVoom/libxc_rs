//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 719/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk719<F: Float>(t45: F, t4674: F, t485: F, t190: F, t4579: F, t681: F, t1342: F, t3572: F, t4573: F, t2337: F, t3558: F, t3561: F, t741: F, t80: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t4675 = t485 * t4674;
    let t4678 = t190 * t4579;
    let t4680 = F::new(4.0) * t681 * t4678;
    let t4682 = F::new(8.0) * t3572 * t1342;
    let t4683 = t190 * t4573;
    let t4685 = F::new(12.0) * t2337 * t4683;
    let t4686 = F::new(0.11696447245269292414e1) * t3558;
    let t4687 = F::new(0.36622894612013090108e-3) * t3561;
    let t4693 = piecewise3::<f64>(t151, F::new(0.0), -F::new(2.0) / F::new(9.0) * t80 * t4573 + F::new(2.0) / F::new(3.0) * t741 * t4579);
    (t4675, t4678, t4680, t4682, t4683, t4685, t4686, t4687, t4693)
}
