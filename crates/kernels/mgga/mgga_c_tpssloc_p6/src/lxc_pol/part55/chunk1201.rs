//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1201/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1201<F: Float>(t6562: F, t8335: F, t86893: F, t214: F, t7510: F, t1880: F, t6572: F, t10109: F, t112908: F, t112936: F, t112942: F, t118886: F, t118892: F, t118894: F, t118895: F, t118901: F, t13065: F, t1528: F, t23278: F, t25168: F, t259: F, t2713: F, t30741: F, t32853: F, t4142: F, t4272: F, t4301: F, t7538: F, t8347: F, t8362: F, t8363: F, t866: F) -> (F, F) {
    let t118903 = t6562 * t86893 * t8335;
    let t118904 = F::cast_from(0.82246703342411321825e-2_f64) * t118903;
    let t118910 = t214 * t7510;
    let t118913 = F::cast_from(0.16449340668482264365e-1_f64) * t1880 * t118910 * t6572;
    let t118914 = -F::cast_from(6.0_f64) * t10109 * t25168 * t4272 * t8362 + t259 * t4142 * t8347 - t112908 * t1528 - t118895 * t866 - t13065 * t8363 - F::cast_from(2.0_f64) * t23278 * t7538 - t2713 * t32853 - t30741 * t4301 + t112936 - t112942 + t118886 + t118892 - t118894 - t118901 + t118904 - t118913;
    (t118910, t118914)
}
