//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2333/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2333<F: Float>(t55388: F, t7015: F, t20173: F, t28896: F, t28893: F, t6534: F, t1401: F, t96729: F, t16524: F, t26542: F, t1458: F, t26135: F, t3941: F) -> (F, F, F, F, F, F) {
    let t100875 = F::new(27.0) * t55388 * t7015;
    let t100879 = F::new(54.0) * t20173 * t28896;
    let t100883 = F::new(27.0) * t28893 * t6534;
    let t100885 = F::new(0.135e2) * t1401 * t96729;
    let t100887 = F::new(54.0) * t16524 * t26542;
    let t100890 = F::new(54.0) * t3941 * t26135 * t1458;
    (t100875, t100879, t100883, t100885, t100887, t100890)
}
