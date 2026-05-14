//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1279/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1279<F: Float>(t6273: F, t9895: F, t19579: F, t19581: F, t19305: F, t6113: F, t19308: F, t19441: F, t6103: F, t19614: F, t6243: F, t21012: F, t61801: F, t19619: F, t6242: F, t19621: F) -> (F, F, F, F, F, F, F) {
    let t68854 = t6273 * t9895;
    let t68857 = 4.0 * t19579 * t68854 * t19581;
    let t68859 = 4.0 * t19305 * t6113;
    let t68861 = 4.0 * t19308 * t6113;
    let t68863 = 4.0 * t6103 * t19441;
    let t68865 = 6.0 * t6243 * t19614;
    let t68867 = 6.0 * t61801 * t21012;
    let t68868 = t6242 * t19619;
    let t68870 = 12.0 * t68868 * t19621;
    (t68857, t68859, t68861, t68863, t68865, t68867, t68870)
}
