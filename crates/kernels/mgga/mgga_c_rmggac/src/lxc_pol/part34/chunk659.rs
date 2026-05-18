//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 659/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk659<F: Float>(t31: F, t574: F, t640: F, t34795: F, t529: F, t1411: F, t7754: F, t1540: F, t880: F, t49: F, t2410: F, t7228: F) -> (F, F, F, F, F, F) {
    let t38843 = t574 * t31;
    let t38844 = t640 * t38843;
    let t38848 = t34795 * t529;
    let t38855 = t7754 * t1411;
    let t38973 = t1540 * t880;
    let t39116 = t49 * t529;
    let t39207 = t2410 * t7228;
    (t38844, t38848, t38855, t38973, t39116, t39207)
}
