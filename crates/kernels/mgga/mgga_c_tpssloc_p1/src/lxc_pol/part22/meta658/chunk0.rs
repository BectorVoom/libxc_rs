//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2200/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2200<F: Float>(t13360: F, t4261: F, t5584: F, t9975: F, t5619: F, t9674: F, t4250: F, t46657: F, t16907: F, t9638: F, t17013: F, t13258: F, t16845: F) -> (F, F, F, F, F, F, F) {
    let t58847 = t13360 * t4261;
    let t58853 = t5584 * t9975;
    let t58859 = t9674 * t5619;
    let t58873 = t46657 * t4250;
    let t58885 = t9638 * t16907;
    let t58890 = t9638 * t17013;
    let t58900 = t13258 * t16845;
    (t58847, t58853, t58859, t58873, t58885, t58890, t58900)
}
