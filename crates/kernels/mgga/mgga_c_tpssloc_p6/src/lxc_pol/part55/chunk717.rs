//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 717/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk717<F: Float>(t5: F, t67: F, t7254: F, t1864: F, t2109: F, t6509: F, t1860: F, t2110: F, t6486: F, t6492: F, t6495: F, t7246: F, t112: F) -> (F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t7255 = t7254 * t67;
    let t7256 = t7255 * t1864;
    let t7259 = t2109 * t6509;
    let t7263 = piecewise3::<F>(t8, F::cast_from(0.0_f64), -t6486 * t2110 / F::cast_from(6.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7246 * t6492 + t6495 * t2110 / F::cast_from(3.0_f64) - t1860 * t7256 / F::cast_from(6.0_f64) - t1860 * t7259 / F::cast_from(6.0_f64));
    let t7264 = t7263 * t112;
    (t7255, t7256, t7259, t7263, t7264)
}
