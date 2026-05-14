//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 718/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk718<F: Float>(t17: F, t3826: F, t1285: F, t592: F, t1287: F, t588: F, t2423: F, t3686: F, t3697: F, t3819: F, t3821: F, t3823: F, t3825: F, t225: F, t3817: F, t1365: F, t68: F) -> (F, F, F, F, F, F, F) {
    let t3827 = t17 * t3826;
    let t3828 = 2.0 * t3827;
    let t3829 = t592 * t1285;
    let t3830 = 8.0 * t3829;
    let t3832 = 8.0 * t592 * t1287;
    let t3833 = t588 * t1285;
    let t3834 = 8.0 * t3833;
    let t3836 = 8.0 * t588 * t1287;
    let t3837 = t3686 + t3819 + t3821 - t3823 - t2423 + t3825 + t3697 + t3828 - t3830 - t3832 + t3834 + t3836;
    let t3839 = (t3817 + t3837) * t225;
    let t3843 = t68 * t1365;
    (t3828, t3830, t3832, t3834, t3836, t3839, t3843)
}
