//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1959/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1959<F: Float>(t19289: F, t19451: F, t1983: F, t2039: F, t2095: F, t2314: F, t24987: F, t24995: F, t26114: F, t26161: F, t26179: F, t26558: F, t26875: F, t27150: F, t27171: F, t27219: F, t27226: F, t29197: F, t29211: F, t35259: F, t4028: F, t4034: F, t4072: F, t5308: F, t57806: F, t6468: F, t652: F, t671: F, t7057: F, t7166: F, t7458: F, t7802: F, t7890: F, t7941: F, t96830: F, t97890: F) -> F {
    let t101091 = -F::new(4.0) * t7458 * t27219 - F::new(4.0) * t4028 * t27150 - F::new(2.0) * t652 * t29197 * t671 - F::new(2.0) * t19451 * t7057 + F::new(12.0) * t24995 * t35259 * t5308 + t7166 * t6468 - F::new(4.0) * t652 * t7890 * t4072 + F::new(4.0) * t26161 * t26558 * t96830 - F::new(4.0) * t7458 * t27171 - F::new(2.0) * t2314 * t29211 - F::new(2.0) * t4034 * t29211 - F::new(2.0) * t652 * t19289 * t2039 - t1983 * t2095 * t57806 + F::new(2.0) * t24987 * t7941 + F::new(12.0) * t97890 * t26875 - F::new(4.0) * t26114 * t7802 - F::new(4.0) * t26179 * t7802 - F::new(4.0) * t7458 * t27226 - F::new(4.0) * t4028 * t27219;
    t101091
}
