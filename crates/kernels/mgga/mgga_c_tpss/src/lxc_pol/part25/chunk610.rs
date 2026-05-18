//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 610/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk610<F: Float>(t45: F, t57: F, t2219: F, t1289: F, t80: F, t3431: F, t581: F, t741: F, t83: F, t745: F, zeta_threshold: F) -> (F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t3594 = F::new(0.5848223622634646207e0) * t2219;
    let t3595 = t80 * t1289;
    let t3601 = piecewise3::<f64>(t151, F::new(0.0), -F::new(2.0) / F::new(9.0) * t3595 * t581 + F::new(2.0) / F::new(3.0) * t741 * t3431);
    let t3602 = t83 * t1289;
    let t3608 = piecewise3::<f64>(t155, F::new(0.0), -F::new(2.0) / F::new(9.0) * t3602 * t581 - F::new(2.0) / F::new(3.0) * t745 * t3431);
    let t3610 = t3601 / F::new(2.0) + t3608 / F::new(2.0);
    (t3594, t3595, t3602, t3610)
}
