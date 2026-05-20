//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1028/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1028<F: Float>(t128616: F, t128625: F, t28209: F, t31611: F, t6888: F, t122166: F, t1985: F, t7700: F, t113934: F, t115306: F, t122102: F, t122112: F, t122121: F, t127166: F, t127169: F, t127173: F, t127176: F, t127180: F, t127183: F, t128604: F, t33323: F, t539: F, t568: F, t6361: F, t8617: F, t97626: F) -> (F, F) {
    let t128626 = t128616 + t128625;
    let t128630 = t6888 * t31611 * t28209;
    let t128633 = t1985 * t122166 * t7700;
    let t128639 = t127166 + t6361 * t8617 * t568 + F::cast_from(0.3289868133696452873e-1_f64) * t128604 + t127169 - F::cast_from(0.76763589786250567036e-1_f64) * t122102 + t539 * t128626 * t568 + t127173 + t113934 + t127176 - F::cast_from(0.16449340668482264365e-1_f64) * t128630 - F::cast_from(0.16449340668482264365e-1_f64) * t128633 + t127180 - F::cast_from(0.76763589786250567036e-1_f64) * t122112 - F::new(12.0) * t97626 * t33323 - t115306 + F::cast_from(0.82246703342411321824e-2_f64) * t122121 + t127183;
    (t128626, t128639)
}
