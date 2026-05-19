//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 713/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk713<F: Float>(t5: F, t1317: F, t1981: F, t3418: F, t4566: F, t4570: F, t4626: F, t578: F, t91: F, t117: F) -> (F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t4630 = piecewise3::<F>(t8, F::new(0.0), -F::new(8.0) * t1317 * t3418 + F::new(20.0) * t1981 * t4570 + t4566 * t91 - F::new(4.0) * t4626 * t578);
    let t4631 = t4630 * t117;
    (t4630, t4631)
}
