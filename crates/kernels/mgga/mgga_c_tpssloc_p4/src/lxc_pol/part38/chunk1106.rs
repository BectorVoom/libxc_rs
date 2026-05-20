//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1106/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1106<F: Float>(t13716: F, t942: F, t951: F, t959: F, t2940: F, t4489: F, t10523: F, t1580: F, t2933: F, t1543: F, t2791: F, t2794: F) -> (F, F, F, F) {
    let t13718 = t942 * t13716 * t951;
    let t13720 = F::cast_from(0.5848223622634646207e0_f64) * t959 * t13718;
    let t13722 = F::cast_from(0.23392894490538584828e1_f64) * t2940 * t4489;
    let t13723 = t10523 * t1580;
    let t13724 = t13723 * t2933;
    let t13726 = F::cast_from(0.10389515463408878255e3_f64) * t959 * t13724;
    let t13727 = t1543 * t2791;
    let t13729 = F::new(2.0) * t13727 * t2794;
    (t13720, t13722, t13726, t13729)
}
