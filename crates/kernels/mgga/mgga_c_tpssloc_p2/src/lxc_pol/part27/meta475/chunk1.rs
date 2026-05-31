//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1845/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1845<F: Float>(t23476: F, t343: F, t23562: F, t1046: F, t1935: F, t23533: F, t23537: F, t23541: F, t23544: F, t23548: F, t23551: F, t23554: F, t23557: F, t23560: F, t3043: F, t3134: F, t3153: F, t378: F, t6717: F, t6747: F) -> (F, F, F) {
    let t23563 = t23476 * t343;
    let t23564 = t23562 * t23563;
    let t23569 = t23533 / F::cast_from(1728.0_f64) + t23537 * t3134 / F::cast_from(768.0_f64) - t23541 * t3043 / F::cast_from(1536.0_f64) + t23544 * t1046 / F::cast_from(1152.0_f64) - F::cast_from(0.10093189023535097714e-3_f64) * t1935 * t23548 - t23551 * t378 / F::cast_from(144.0_f64) + t23554 / F::cast_from(1152.0_f64) + F::cast_from(19.0_f64) / F::cast_from(864.0_f64) * t23557 * t378 - t23560 / F::cast_from(216.0_f64) - F::cast_from(0.20186378047070195428e-3_f64) * t23564 * t6747 - t6717 * t3153 / F::cast_from(144.0_f64);
    (t23563, t23564, t23569)
}
