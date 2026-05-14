//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1267/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1267<F: Float>(t19308: F, t6106: F, t19327: F, t6103: F, t1760: F, t19571: F, t4525: F, t6273: F, t9895: F, t19579: F, t19581: F, t19305: F, t6113: F, t19441: F, t19614: F, t6243: F) -> (F, F, F, F, F, F, F, F) {
    let t68848 = 4.0 * t19308 * t6106;
    let t68850 = 4.0 * t6103 * t19327;
    let t68853 = 2.0 * t1760 * t19571 * t4525;
    let t68854 = t6273 * t9895;
    let t68857 = 4.0 * t19579 * t68854 * t19581;
    let t68859 = 4.0 * t19305 * t6113;
    let t68861 = 4.0 * t19308 * t6113;
    let t68863 = 4.0 * t6103 * t19441;
    let t68865 = 6.0 * t6243 * t19614;
    (t68848, t68850, t68853, t68857, t68859, t68861, t68863, t68865)
}
