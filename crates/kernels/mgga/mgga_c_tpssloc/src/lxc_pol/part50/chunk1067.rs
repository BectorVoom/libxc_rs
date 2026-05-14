//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1067/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1067<F: Float>(t23204: F, t32866: F, t6562: F, t1880: F, t214: F, t225: F, t25160: F, t258: F, t32809: F, t6547: F, t32880: F, t8335: F, t87782: F, t86893: F, t7510: F, t6572: F) -> (F, F, F, F, F, F, F, F) {
    let t118885 = t6562 * t23204 * t32866;
    let t118886 = 0.82246703342411321825e-2 * t118885;
    let t118892 = 0.16449340668482264365e-1 * t1880 * t214 * t25160 * t225 * t258;
    let t118893 = t6547 * t32809;
    let t118894 = 0.38381794893125283518e-1 * t118893;
    let t118895 = t32880 * t225;
    let t118901 = 0.16449340668482264365e-1 * t1880 * t87782 * t8335;
    let t118903 = t6562 * t86893 * t8335;
    let t118904 = 0.82246703342411321825e-2 * t118903;
    let t118910 = t214 * t7510;
    let t118913 = 0.16449340668482264365e-1 * t1880 * t118910 * t6572;
    (t118886, t118892, t118894, t118895, t118901, t118904, t118910, t118913)
}
