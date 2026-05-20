//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3070/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3070<F: Float>(t4745: F, t51246: F, t14838: F, t15051: F, t15054: F, t15057: F, t51249: F, t4786: F, t51402: F, t14850: F, t15061: F, t15064: F) -> (F, F, F, F, F, F, F) {
    let t63731 = F::new(8.0) * t51246 * t4745;
    let t63733 = F::new(8.0) * t14838 * t15051;
    let t63735 = F::new(4.0) * t14838 * t15054;
    let t63737 = F::cast_from(0.19298375398431042081e3_f64) * t51249 * t15057;
    let t63739 = F::cast_from(0.64327917994770140268e2_f64) * t51402 * t4786;
    let t63741 = F::cast_from(0.64327917994770140268e2_f64) * t14850 * t15061;
    let t63743 = F::cast_from(0.32163958997385070134e2_f64) * t14850 * t15064;
    (t63731, t63733, t63735, t63737, t63739, t63741, t63743)
}
