//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 758/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk758<F: Float>(t2020: F, t6876: F, t2018: F, t532: F, t1307: F, t1390: F, t1983: F, t1984: F, t6546: F) -> (F, F, F, F, F, F) {
    let t6877 = t6876 * t2020;
    let t6878 = t532 * t2018;
    let t6879 = t1390 * t1307;
    let t6880 = t6878 * t6879;
    let t6882 = 3.0 * t1983 * t6880;
    let t6883 = t6546 * t1984;
    (t6877, t6878, t6879, t6880, t6882, t6883)
}
