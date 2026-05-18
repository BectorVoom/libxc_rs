//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 775/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk775<F: Float>(t33: F, t1497: F, t3289: F, t493: F, t5059: F, t162: F, t5334: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t5335 = t1497 * t1497;
    let t5341 = piecewise3::<f64>(t34, F::new(0.0), F::new(4.0) / F::new(9.0) * t3289 * t5335 + F::new(4.0) / F::new(3.0) * t493 * t5059);
    let t5343 = (t5334 + t5341) * t162;
    (t5335, t5343)
}
