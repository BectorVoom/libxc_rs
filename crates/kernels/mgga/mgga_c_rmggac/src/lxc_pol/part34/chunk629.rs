//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 629/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk629<F: Float>(t1986: F, t2088: F, t13806: F, t7508: F, t3154: F, t7939: F, t13809: F, t7335: F, t13816: F, t35620: F, t7491: F, t34709: F, t34786: F, t14063: F, t3151: F, t7472: F) -> (F, F, F, F, F, F, F, F, F) {
    let t69904 = t1986 * t2088;
    let t69907 = t7508 * t13806;
    let t69924 = t7939 * t3154;
    let t69934 = t7335 * t13809;
    let t69936 = t35620 * t13816;
    let t69938 = t7491 * t13809;
    let t69940 = t34709 * t13816;
    let t69942 = t34786 * t13816;
    let t69953 = t7472 * t14063 * t3151;
    (t69904, t69907, t69924, t69934, t69936, t69938, t69940, t69942, t69953)
}
