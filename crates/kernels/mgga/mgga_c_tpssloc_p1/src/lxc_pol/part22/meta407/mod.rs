//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta407 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1707;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta407<F: Float>(t18523: F, t457: F, t460: F, t974: F, t135: F, t6146: F, t1174: F, t6140: F, t11558: F, t15341: F, t15364: F, t15366: F, t15374: F, t15376: F, t18475: F, t18484: F, t18489: F, t3447: F, t4905: F, t4909: F, t4920: F) -> (F, F, F, F, F, F, F) {
        let (t18525, t18526, t18529, t18530, t18532, t18533, t18535) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1707::<F>(t18523, t457, t460, t974, t135, t6146, t1174, t6140, t11558, t15341, t15364, t15366, t15374, t15376, t18475, t18484, t18489, t3447, t4905, t4909, t4920);
    (t18525, t18526, t18529, t18530, t18532, t18533, t18535)
}
