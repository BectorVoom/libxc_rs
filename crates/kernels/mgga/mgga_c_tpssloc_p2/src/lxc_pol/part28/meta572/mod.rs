//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta572 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1853;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta572<F: Float>(t13347: F, t6621: F, t131: F, t6598: F, t9537: F, t225: F, t2627: F, t236: F, t25093: F, t25068: F, t2703: F, t23127: F, t4257: F) -> (F, F, F, F, F, F) {
        let (t87226, t87229, t87230, t87233, t87235, t87241) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1853::<F>(t13347, t6621, t131, t6598, t9537, t225, t2627, t236, t25093, t25068, t2703, t23127, t4257);
    (t87226, t87229, t87230, t87233, t87235, t87241)
}
