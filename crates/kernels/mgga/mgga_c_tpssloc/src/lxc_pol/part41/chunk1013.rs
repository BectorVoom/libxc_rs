//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1013/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1013<F: Float>(t15567: F, t3068: F, t1244: F, t11697: F, t4949: F, t3577: F, t3431: F, t4729: F, t1174: F, t1011: F, t15031: F, t1212: F) -> (F, F, F, F) {
    let t15568 = t15567 * t3068;
    let t15569 = t1244 * t15568;
    let t15572 = t11697 * t4949;
    let t15574 = t3577 * t15572 / F::new(3456.0);
    let t15578 = t3431 * t4729;
    let t15580 = t1174 * t15578 / F::new(216.0);
    let t15590 = t15031 * t1011;
    let t15591 = t15590 * t1212;
    (t15569, t15574, t15580, t15591)
}
