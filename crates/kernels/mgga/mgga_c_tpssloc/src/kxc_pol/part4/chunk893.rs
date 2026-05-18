//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 893/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk893<F: Float>(t10629: F, t315: F, t2885: F, t919: F, t2884: F, t307: F, t302: F, t10294: F, t10544: F, t922: F, t2887: F, t310: F) -> (F, F, F, F, F, F, F) {
    let t10756 = t315 * t10629;
    let t10765 = t919 * t2885;
    let t10770 = F::new(1.0) / t2884 / t307;
    let t10771 = t302 * t10770;
    let t10784 = F::new(0.46308888888888888888e0) * t10294;
    let t10785 = F::new(0.16068111111111111111e1) * t10544;
    let t10810 = F::new(1.0) / t2884 / t922;
    let t10811 = t302 * t10810;
    let t10813 = F::new(1.0) / t2887 / t310;
    (t10756, t10765, t10771, t10784, t10785, t10811, t10813)
}
