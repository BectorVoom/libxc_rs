//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta427 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1717;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta427<F: Float>(t15904: F, t8643: F, t22574: F, t3701: F, t3914: F, t2019: F, t1983: F, t6996: F, t6999: F, t1390: F, t3719: F, t6878: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t22575, t22577, t22578, t22579, t22580, t22581, t22583, t22584, t22585) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1717::<F>(t15904, t8643, t22574, t3701, t3914, t2019, t1983, t6996, t6999, t1390, t3719, t6878);
    (t22575, t22577, t22578, t22579, t22580, t22581, t22583, t22584, t22585)
}
