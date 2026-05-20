//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 777/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk777<F: Float>(t2020: F, t7685: F, t1390: F, t1799: F, t6878: F, t1983: F, t6890: F) -> (F, F, F, F, F) {
    let t7686 = t7685 * t2020;
    let t7687 = t1390 * t1799;
    let t7688 = t6878 * t7687;
    let t7690 = F::new(3.0) * t1983 * t7688;
    let t7691 = t6890 * t1799;
    (t7686, t7687, t7688, t7690, t7691)
}
