//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 695/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk695<F: Float>(t10629: F, t315: F, t2884: F, t307: F, t302: F, t10294: F, t10544: F, t922: F, t2887: F, t310: F, t10523: F, t1043: F, t676: F, t10478: F, t3128: F, t10472: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10756 = t315 * t10629;
    let t10770 = 1.0 / t2884 / t307;
    let t10771 = t302 * t10770;
    let t10784 = 0.46308888888888888888e0 * t10294;
    let t10785 = 0.16068111111111111111e1 * t10544;
    let t10810 = 1.0 / t2884 / t922;
    let t10811 = t302 * t10810;
    let t10813 = 1.0 / t2887 / t310;
    let t10828 = t315 * t10523;
    let t10832 = 0.53272592592592592592e-1 * t10544;
    let t10868 = t676 * t1043;
    let t10875 = t3128 * t10478;
    let t10876 = t10472 * t10875;
    (t10756, t10771, t10784, t10785, t10811, t10813, t10828, t10832, t10868, t10876)
}
