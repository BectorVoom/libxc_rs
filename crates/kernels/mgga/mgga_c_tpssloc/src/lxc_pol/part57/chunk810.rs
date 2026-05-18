//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 810/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk810<F: Float>(t28816: F, t28867: F, t3: F, t1873: F, t20162: F, t16524: F, t7769: F, t5371: F, t7467: F, t5456: F, t576: F, t1458: F) -> (F, F, F, F, F, F, F, F) {
    let t28868 = t28816 + t28867;
    let t28869 = t3 * t28868;
    let t28888 = F::new(0.135e2) * t20162 * t1873;
    let t28890 = F::new(54.0) * t16524 * t7769;
    let t28892 = F::new(27.0) * t5371 * t7467;
    let t28893 = t576 * t5456;
    let t28895 = F::new(27.0) * t28893 * t1873;
    let t28896 = t7467 * t1458;
    (t28868, t28869, t28888, t28890, t28892, t28893, t28895, t28896)
}
