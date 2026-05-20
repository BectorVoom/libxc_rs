//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1199/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1199<F: Float>(t1117: F, t4782: F, t3264: F, t1671: F, t3307: F, t3265: F, t4785: F, t11190: F, t3315: F, t4781: F, t3313: F, t11277: F, t1670: F) -> (F, F, F, F, F, F) {
    let t15051 = t4782 * t1117;
    let t15053 = F::new(4.0) * t3264 * t15051;
    let t15054 = t1671 * t3307;
    let t15056 = F::new(2.0) * t3264 * t15054;
    let t15057 = t4785 * t3265;
    let t15059 = F::cast_from(0.96491876992155210402e2_f64) * t11190 * t15057;
    let t15060 = t4781 * t3315;
    let t15061 = t15060 * t1117;
    let t15063 = F::cast_from(0.32163958997385070134e2_f64) * t3313 * t15061;
    let t15064 = t4785 * t3307;
    let t15066 = F::cast_from(0.16081979498692535067e2_f64) * t3313 * t15064;
    let t15067 = t1670 * t11277;
    (t15053, t15056, t15059, t15063, t15066, t15067)
}
