//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 751/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk751<F: Float>(t4875: F, t866: F, t846: F, t2533: F, t4843: F, t2531: F, t2537: F, t3746: F, t4828: F, t4832: F, t4836: F, t1436: F, t885: F, t2557: F, t2564: F, t3795: F, t4848: F, t4855: F, t4861: F, t4863: F, t4867: F, t4870: F, t4873: F) -> (F, F, F, F, F, F, F, F) {
    let t4876 = t4875 * t866;
    let t4878 = 1.0 * t846 * t4876;
    let t4879 = t4843 * t2533;
    let t4881 = 0.16081979498692535067e2 * t2531 * t4879;
    let t4886 = t2537 + 0.11415555555555555555e-1 * t3746 - 0.11415555555555555555e-1 * t4828 + 0.34246666666666666666e-1 * t4832 - 0.17123333333333333333e-1 * t4836;
    let t4891 = t1436 * t1436;
    let t4892 = t4891 * t885;
    let t4907 = -0.17648625e1 * t4848 + 0.3529725e1 * t4855 + t2557 + 0.34431666666666666666e0 * t3746 - 0.34431666666666666667e0 * t4828 + 0.103295e1 * t4832 - 0.516475e0 * t4836 + 0.31558125e0 * t4861 + 0.6311625e0 * t4863 + t2564 + 0.13892666666666666667e0 * t3795 - 0.34731666666666666667e-1 * t4867 + 0.20839e0 * t4870 - 0.104195e0 * t4873;
    (t4876, t4878, t4879, t4881, t4886, t4891, t4892, t4907)
}
