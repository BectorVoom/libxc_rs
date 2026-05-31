//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1192/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1192<F: Float>(t1307: F, t1390: F, t6878: F, t1983: F, t1984: F, t6546: F) -> (F, F, F, F) {
    let t6879 = t1390 * t1307;
    let t6880 = t6878 * t6879;
    let t6882 = F::cast_from(3.0_f64) * t1983 * t6880;
    let t6883 = t6546 * t1984;
    (t6879, t6880, t6882, t6883)
}
