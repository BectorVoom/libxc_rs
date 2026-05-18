//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1359/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1359<F: Float>(t33617: F, t4034: F, t652: F, t7156: F, t7467: F, t6534: F, t7890: F, t1458: F, t7039: F, t1874: F, t2035: F, t4072: F) -> (F, F, F, F, F, F) {
    let t120995 = F::new(2.0) * t4034 * t33617;
    let t120998 = F::new(2.0) * t652 * t7156 * t7467;
    let t121003 = F::new(2.0) * t652 * t7890 * t6534;
    let t121004 = t7039 * t1458;
    let t121006 = F::new(2.0) * t121004 * t1874;
    let t121007 = t2035 * t4072;
    (t120995, t120998, t121003, t121004, t121006, t121007)
}
