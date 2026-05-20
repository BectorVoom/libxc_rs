//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta694 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2274;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta694<F: Float>(t15338: F, t18542: F, t3447: F, t15293: F, t19256: F, t225: F, t19211: F, t3030: F, t6150: F, t3609: F, t3623: F, t18710: F, t300: F) -> (F, F, F, F, F, F, F, F) {
        let (t65139, t65142, t65203, t65208, t65253, t65254, t65262, t65288) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2274::<F>(t15338, t18542, t3447, t15293, t19256, t225, t19211, t3030, t6150, t3609, t3623, t18710, t300);
    (t65139, t65142, t65203, t65208, t65253, t65254, t65262, t65288)
}
