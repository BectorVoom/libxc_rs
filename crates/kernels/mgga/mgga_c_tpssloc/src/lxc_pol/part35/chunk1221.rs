//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1221/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1221<F: Float>(t5: F, t29484: F, t112: F, t2113: F, t5456: F, t1458: F, t27863: F, t28001: F, t28004: F, t28006: F, t28009: F, t28011: F, t28019: F, t5493: F, t7266: F) -> (F, F, F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t29485 = piecewise3::<f64>(t8, F::new(0.0), t29484);
    let t29486 = t29485 * t112;
    let t29493 = t2113 * t5456;
    let t29497 = F::new(4.0) * t1458 * t27863 + F::new(2.0) * t5493 * t7266 + t28001 + t28004 + t28006 + t28009 + t28011 + t28019 + t29486 + F::new(2.0) * t29493;
    (t29485, t29486, t29493, t29497)
}
