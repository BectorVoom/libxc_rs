//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 702/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk702<F: Float>(t5: F, t1860: F, t7032: F, t2031: F, t6509: F, t2032: F, t6486: F, t6492: F, t6495: F, t7026: F, t112: F) -> (F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t7034 = 8.0 / 9.0 * t1860 * t7032;
    let t7035 = t2031 * t6509;
    let t7039 = piecewise3(t8, 0.0, t6486 * t2032 / 3.0 - 5.0 / 3.0 * t7026 * t6492 - 2.0 / 3.0 * t6495 * t2032 - t7034 + t1860 * t7035 / 3.0);
    let t7040 = t7039 * t112;
    (t7034, t7035, t7039, t7040)
}
