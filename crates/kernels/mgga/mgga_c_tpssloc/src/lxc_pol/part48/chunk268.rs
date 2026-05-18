//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 268/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk268<F: Float>(t25: F, t28: F, t514: F, t606: F, t517: F, t1081: F, zeta_threshold: F) -> (F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t1298 = F::new(1.0) / t514;
    let t1301 = piecewise3::<f64>(t26, F::new(0.0), F::new(2.0) / F::new(3.0) * t1298 * t606);
    let t1302 = F::new(1.0) / t517;
    let t1305 = piecewise3::<f64>(t29, F::new(0.0), F::new(2.0) / F::new(3.0) * t1302 * t1081);
    let t1307 = t1301 / F::new(2.0) + t1305 / F::new(2.0);
    (t1298, t1302, t1307)
}
