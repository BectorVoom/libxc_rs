//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1181/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1181<F: Float>(t109: F, t1873: F, t28002: F, t4028: F, t7467: F, t5493: F, t88: F, t7676: F, t22473: F, t5464: F, t5488: F, t6530: F, t22469: F, t27166: F) -> (F, F, F, F, F, F) {
    let t110 = F::new(1.0) < t109;
    let t28004 = F::new(4.0) * t28002 * t1873;
    let t28006 = F::new(4.0) * t4028 * t7467;
    let t28007 = t88 * t5493;
    let t28009 = F::new(2.0) * t28007 * t1873;
    let t28011 = F::new(4.0) * t7676 * t7467;
    let t28012 = t22473 * t5464;
    let t28014 = t6530 * t5488;
    let t28017 = piecewise3::<f64>(t110, F::new(0.0), t22469 + t27166 + t28012 / F::new(4.0) - t28014 / F::new(8.0));
    (t28004, t28006, t28007, t28009, t28011, t28017)
}
