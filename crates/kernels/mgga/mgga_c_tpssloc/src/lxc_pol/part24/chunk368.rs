//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 368/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk368<F: Float>(t25: F, t28: F, t1294: F, t763: F, t532: F, t571: F, t514: F, t606: F, t517: F, t1081: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t1296 = F::cast_from(0.5848223622634646207e0_f64) * t1294 * t763;
    let t1297 = t532 * t571;
    let t1298 = F::new(1.0) / t514;
    let t1301 = piecewise3::<F>(t26, F::new(0.0), F::new(2.0) / F::new(3.0) * t1298 * t606);
    let t1302 = F::new(1.0) / t517;
    let t1305 = piecewise3::<F>(t29, F::new(0.0), F::new(2.0) / F::new(3.0) * t1302 * t1081);
    let t1307 = t1301 / F::new(2.0) + t1305 / F::new(2.0);
    (t1296, t1297, t1298, t1302, t1307)
}
