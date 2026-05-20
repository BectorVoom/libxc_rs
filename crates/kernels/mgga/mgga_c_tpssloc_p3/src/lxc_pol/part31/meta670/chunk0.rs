//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1989/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1989<F: Float>(t101832: F, t870: F, t193: F, t7859: F, t16557: F, t1877: F, t2057: F, t24191: F, t24339: F, t25: F, t25024: F, t2522: F, t25375: F, t25377: F, t25385: F, t26563: F, t26744: F, t28256: F, t28459: F, t29106: F, t4314: F, t606: F, t7110: F, t7114: F, t7845: F, t97950: F, t97953: F, t97985: F, t98015: F, t98034: F, t98075: F) -> (F, F, F) {
    let t101833 = t101832 * t870;
    let t101840 = t193 * t7859;
    let t101843 = F::new(6.0) * t26563 * t97950 - F::new(3.0) * t24191 * t97953 - F::new(3.0) * t24191 * t98015 + t1877 * t29106 * t606 / F::new(2.0) + t1877 * t2057 * t16557 / F::new(2.0) + F::new(3.0) * t2522 * t7845 * t25385 - t1877 * t24339 * t28459 + F::new(3.0) / F::new(2.0) * t2522 * t7110 * t28256 - F::new(3.0) / F::new(2.0) * t24191 * t98034 + F::new(3.0) * t2522 * t7845 * t25024 - t1877 * t26744 * t25377 - t1877 * t7114 * t98075 / F::new(2.0) + t1877 * t101833 * t25 / F::new(2.0) + F::new(3.0) * t4314 * t2057 * t97985 + F::new(2.0) * t101840 * t25375;
    (t101833, t101840, t101843)
}
