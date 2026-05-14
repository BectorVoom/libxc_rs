//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 842/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk842<F: Float>(t234: F, t7084: F, t6552: F, t6637: F, t776: F, t2047: F, t22986: F, t6646: F, t829: F, t1880: F, t1894: F, t214: F, t24234: F, t112778: F, t112803: F, t112818: F) -> (F, F, F, F, F, F) {
    let t114696 = t234 * t7084;
    let t114699 = t6552 * t6637 * t114696 * t776;
    let t114704 = t22986 * t6646 * t2047 * t776 * t829;
    let t114708 = t1880 * t214 * t1894 * t24234;
    let t114714 = 0.5383034145885385447e-3 * t112778;
    let t114720 = 7.0 / 576.0 * t112803;
    let t114724 = 0.32298204875312312682e-2 * t112818;
    (t114699, t114704, t114708, t114714, t114720, t114724)
}
