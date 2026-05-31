//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1378/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1378<F: Float>(t23637: F, t82822: F, t1920: F, t23620: F, t968: F, t23617: F, t6680: F, t11034: F, t11046: F, t11048: F, t11051: F, t1950: F, t23654: F, t23701: F, t23704: F, t3180: F, t3186: F, t3200: F, t4673: F, t4684: F, t6790: F, t6811: F, t82382: F, t82730: F, t82799: F, t82803: F, t82806: F, t82809: F) -> F {
    let t82823 = t82822 * t23637;
    let t82828 = t1920 * t968 * t23620;
    let t82830 = t6680 * t23617;
    let t82834 = t82799 - F::cast_from(0.24125699647107321069e0_f64) * t82382 * t6790 - F::cast_from(0.3752886611772249944e0_f64) * t82803 * t1950 + F::cast_from(0.80418998823691070229e-1_f64) * t82806 - F::cast_from(0.54831135561607547884e-2_f64) * t82809 + F::cast_from(3.0_f64) * t11051 * t6811 + t11046 * t82730 * t11048 - F::cast_from(3.0_f64) * t3200 * t23704 * t4684 + F::cast_from(6.0_f64) * t3186 * t23704 * t4673 + F::cast_from(0.54831135561607547883e-2_f64) * t82823 + F::cast_from(6.0_f64) * t11034 * t23701 + F::cast_from(0.82246703342411321826e-2_f64) * t82828 + F::cast_from(0.14621636149762012769e-1_f64) * t82830 + F::cast_from(6.0_f64) * t3180 * t23654;
    t82834
}
