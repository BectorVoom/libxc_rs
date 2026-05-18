//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1200/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1200<F: Float>(t23204: F, t32866: F, t6562: F, t1880: F, t214: F, t225: F, t25160: F, t258: F, t32809: F, t6547: F, t32880: F, t8335: F, t87782: F) -> (F, F, F, F, F) {
    let t118885 = t6562 * t23204 * t32866;
    let t118886 = F::new(0.82246703342411321825e-2) * t118885;
    let t118892 = F::new(0.16449340668482264365e-1) * t1880 * t214 * t25160 * t225 * t258;
    let t118893 = t6547 * t32809;
    let t118894 = F::new(0.38381794893125283518e-1) * t118893;
    let t118895 = t32880 * t225;
    let t118901 = F::new(0.16449340668482264365e-1) * t1880 * t87782 * t8335;
    (t118886, t118892, t118894, t118895, t118901)
}
