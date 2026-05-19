//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 741/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk741<F: Float>(t4875: F, t866: F, t846: F, t2533: F, t4843: F, t2531: F, t2537: F, t3746: F, t4828: F, t4832: F, t4836: F, t1436: F) -> (F, F, F, F, F, F) {
    let t4876 = t4875 * t866;
    let t4878 = F::new(1.0) * t846 * t4876;
    let t4879 = t4843 * t2533;
    let t4881 = F::cast_from(0.16081979498692535067e2_f64) * t2531 * t4879;
    let t4886 = t2537 + F::cast_from(0.11415555555555555555e-1_f64) * t3746 - F::cast_from(0.11415555555555555555e-1_f64) * t4828 + F::cast_from(0.34246666666666666666e-1_f64) * t4832 - F::cast_from(0.17123333333333333333e-1_f64) * t4836;
    let t4891 = t1436 * t1436;
    (t4876, t4878, t4879, t4881, t4886, t4891)
}
