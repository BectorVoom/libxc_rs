//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2485/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2485<F: Float>(t17659: F, t4644: F, t10422: F, t21573: F, t3070: F, t10390: F, t10408: F, t10937: F, t14080: F, t21516: F, t21520: F, t21574: F, t3117: F, t4337: F, t49994: F, t50048: F, t5857: F, t62441: F, t62445: F, t70442: F) -> F {
    let t70711 = t4644 * t17659;
    let t70724 = t3070 * t10422 * t21573;
    let t70728 = -t49994 - t14080 * t5857 / F::cast_from(288.0_f64) + t70711 / F::cast_from(2304.0_f64) + t62441 / F::cast_from(216.0_f64) - F::cast_from(5.0_f64) / F::cast_from(20736.0_f64) * t62445 + F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t3070 * t10408 * t4337 * t70442 - t10390 * t21520 / F::cast_from(768.0_f64) - t10937 * t21574 / F::cast_from(288.0_f64) + t70724 / F::cast_from(2304.0_f64) + F::cast_from(5.0_f64) / F::cast_from(5184.0_f64) * t3117 * t21516 + t50048;
    t70728
}
