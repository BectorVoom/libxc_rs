//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1998/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1998<F: Float>(t1215: F, t1409: F, t254: F, t492: F, t1254: F, t1763: F, t1441: F, t1458: F, t343: F, t5842: F, t5456: F, t576: F) -> (F, F, F, F, F, F) {
    let t27524 = t1409 * t1215;
    let t27784 = t492 * t254;
    let t27843 = t1763 * t1254;
    let t28002 = t1441 * t1458;
    let t28565 = t5842 * t343;
    let t28893 = t576 * t5456;
    (t27524, t27784, t27843, t28002, t28565, t28893)
}
