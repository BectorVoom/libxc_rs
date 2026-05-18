//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 712/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk712<F: Float>(t13816: F, t35620: F, t13809: F, t7491: F, t34709: F, t34786: F, t14063: F, t3151: F, t7472: F, t118: F, t1986: F, t495: F, t665: F) -> (F, F, F, F, F, F) {
    let t69936 = t35620 * t13816;
    let t69938 = t7491 * t13809;
    let t69940 = t34709 * t13816;
    let t69942 = t34786 * t13816;
    let t69953 = t7472 * t14063 * t3151;
    let t69954 = F::new(0.29085809927086856922e-4) * t69953;
    let t69971 = t1986 * t118 * t665 * t495;
    (t69936, t69938, t69940, t69942, t69954, t69971)
}
