//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2586/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2586<F: Float>(t51993: F, t52047: F, t52094: F, t52150: F, t52197: F, t52257: F, t52303: F, t52374: F, t15814: F, t225: F, t11720: F, t1751: F) -> (F, F, F) {
    let t52377 = t51993 + t52047 + t52094 + t52150 + t52197 + t52257 + t52303 + t52374;
    let t52386 = t15814 * t225;
    let t52424 = t1751 * t11720;
    (t52377, t52386, t52424)
}
