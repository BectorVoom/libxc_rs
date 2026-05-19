//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 981/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk981<F: Float>(t45: F, t57: F, t3431: F, t80: F, t10353: F, t1310: F, t1985: F, t1992: F, t3595: F, t581: F, t741: F, t83: F, t1311: F, t3602: F, t745: F, zeta_threshold: F) -> (F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t10531 = t80 * t3431;
    let t10539 = piecewise3::<F>(t151, F::new(0.0), F::new(8.0) / F::new(27.0) * t1310 * t1985 - F::new(4.0) / F::new(9.0) * t10531 * t581 - F::new(2.0) / F::new(9.0) * t3595 * t1992 + F::new(2.0) / F::new(3.0) * t741 * t10353);
    let t10542 = t83 * t3431;
    let t10550 = piecewise3::<F>(t155, F::new(0.0), -F::new(8.0) / F::new(27.0) * t1311 * t1985 - F::new(4.0) / F::new(9.0) * t10542 * t581 - F::new(2.0) / F::new(9.0) * t3602 * t1992 - F::new(2.0) / F::new(3.0) * t745 * t10353);
    (t10539, t10550)
}
