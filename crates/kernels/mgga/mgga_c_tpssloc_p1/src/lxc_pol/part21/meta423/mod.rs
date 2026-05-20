//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta423 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1945;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta423<F: Float>(t1117: F, t4782: F, t3264: F, t1671: F, t3307: F, t3265: F, t4785: F, t11190: F, t3315: F, t4781: F, t3313: F, t11277: F, t1670: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t15051, t15053, t15054, t15056, t15057, t15059, t15061, t15063, t15064, t15066, t15067) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1945::<F>(t1117, t4782, t3264, t1671, t3307, t3265, t4785, t11190, t3315, t4781, t3313, t11277, t1670);
    (t15051, t15053, t15054, t15056, t15057, t15059, t15061, t15063, t15064, t15066, t15067)
}
