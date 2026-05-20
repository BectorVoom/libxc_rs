//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2402/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2402<F: Float>(t48155: F, t41680: F, t41713: F, t47777: F, t48153: F, t48157: F, t48159: F, t48161: F, t48163: F, t48165: F, t48167: F, t49040: F) -> F {
    let t49200 = F::new(0.5519e0) * t48155;
    let t49208 = F::cast_from(0.20128333333333333334e0_f64) * t41680 - F::cast_from(0.60385000000000000002e0_f64) * t41713 + F::new(0.36231e1) * t47777 + F::new(0.198684e1) * t48153 + t49200 - F::cast_from(0.91983333333333333334e-1_f64) * t48157 - F::new(0.66228e0) * t48159 - F::new(0.33114e0) * t48161 - F::cast_from(0.33114000000000000001e0_f64) * t48163 + F::new(0.11038e0) * t48165 + F::cast_from(0.55190000000000000001e-1_f64) * t48167 - F::cast_from(0.1237865625e0_f64) * t49040;
    t49208
}
