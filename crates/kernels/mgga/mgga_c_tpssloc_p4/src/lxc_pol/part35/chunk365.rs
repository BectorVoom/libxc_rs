//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 365/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk365<F: Float>(t1131: F, t1134: F, t1655: F, t1662: F, t1665: F, t1668: F, t1137: F, t1141: F) -> (F, F, F) {
    let t1682 = F::cast_from(0.3529725e1_f64) * t1662 - t1131 + F::cast_from(0.516475e0_f64) * t1655 + F::cast_from(0.6311625e0_f64) * t1665 - t1134 + F::cast_from(0.104195e0_f64) * t1668;
    let t1683 = t1682 * t1137;
    let t1687 = -t1141 + F::cast_from(0.92708333333333333333e-2_f64) * t1655;
    (t1682, t1683, t1687)
}
