//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta487 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1832;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta487<F: Float>(t3243: F, t7363: F, t24776: F, t2148: F, t3471: F, t3616: F, t7376: F, t7375: F, t225: F, t7319: F, t7364: F, t24757: F, t493: F) -> (F, F, F, F, F, F, F, F) {
        let (t24777, t24778, t24781, t24784, t24785, t24788, t24789, t24792) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1832::<F>(t3243, t7363, t24776, t2148, t3471, t3616, t7376, t7375, t225, t7319, t7364, t24757, t493);
    (t24777, t24778, t24781, t24784, t24785, t24788, t24789, t24792)
}
