//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1012/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1012<F: Float>(t45: F, t8006: F, t4573: F, t608: F, t4579: F, t80: F, t13335: F, t3431: F, t3595: F, t581: F, t741: F, t612: F, t83: F, zeta_threshold: F) -> (F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t14003 = F::cast_from(0.5848223622634646207e0_f64) * t8006;
    let t14004 = t608 * t4573;
    let t14009 = t80 * t4579;
    let t14015 = piecewise3::<F>(t151, F::new(0.0), F::new(8.0) / F::new(27.0) * t14004 * t581 - F::new(4.0) / F::new(9.0) * t3595 * t3431 - F::new(2.0) / F::new(9.0) * t14009 * t581 + F::new(2.0) / F::new(3.0) * t741 * t13335);
    let t14016 = t612 * t4573;
    let t14021 = t83 * t4579;
    (t14003, t14015, t14016, t14021)
}
