//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 922/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk922<F: Float>(t5: F, t115861: F, t115911: F, t112: F, t114387: F, t114388: F, t114405: F, t114413: F, t114415: F, t115813: F, t115815: F, t115817: F, t115819: F, t115821: F, t115824: F, t2039: F, t26103: F, t7056: F, t83935: F) -> (F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t115913 = piecewise3(t8, 0.0, t115861 + t115911);
    let t115914 = t115913 * t112;
    let t115915 = 2.0 * t2039 * t83935 + 4.0 * t26103 * t7056 + t114387 + t114388 + t114405 + t114413 + t114415 + t115813 + t115815 + t115817 + t115819 + t115821 + 2.0 * t115824 + t115914;
    (t115914, t115915)
}
