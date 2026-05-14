//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1265/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1265<F: Float>(t3683: F, t821: F, t17930: F, t1398: F, t2116: F, t1364: F, t1991: F, t3610: F, t580: F, t44470: F, t19809: F, t19817: F, t2162: F, t782: F, t818: F, t10592: F, t17964: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t63859 = t3683 * t821;
    let t63860 = t17930 * t63859;
    let t63863 = t1398 * t2116;
    let t63864 = t17930 * t63863;
    let t63873 = t1991 * t1364;
    let t63877 = t580 * t3610;
    let t63881 = t17930 * t44470;
    let t63884 = t19809 * t821;
    let t63885 = t19817 * t63884;
    let t63893 = t2162 * t818 * t782;
    let t63899 = t17964 * t10592;
    (t63859, t63860, t63863, t63864, t63873, t63877, t63881, t63884, t63885, t63893, t63899)
}
