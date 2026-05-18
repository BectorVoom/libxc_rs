//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 891/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk891<F: Float>(t12021: F, t1385: F, t8793: F, t32147: F, t539: F, t1323: F, t8788: F, t31648: F, t8800: F, t3887: F, t31662: F, t1375: F, t31609: F, t31613: F, t31646: F, t31651: F, t3758: F, t3882: F, t568: F, t7194: F, t7199: F, t8794: F) -> (F, F, F, F, F, F, F) {
    let t32161 = t12021 * t8793 * t1385;
    let t32164 = t539 * t32147;
    let t32168 = t1323 * t8788;
    let t32173 = F::new(0.76763589786250567037e-1) * t31648;
    let t32175 = t8800 * t1385;
    let t32176 = t3887 * t32175;
    let t32183 = F::new(0.76763589786250567037e-1) * t31662;
    let t32184 = -F::new(6.0) * t1375 * t32161 + t32164 * t568 - F::new(0.3289868133696452873e-1) * t31609 - F::new(0.3289868133696452873e-1) * t31613 + t32168 * t568 + F::new(4.0) * t7194 * t7199 - F::new(0.6579736267392905746e-1) * t31646 + t32173 - F::new(0.3289868133696452873e-1) * t31651 + F::new(2.0) * t1375 * t32176 + F::new(2.0) * t3758 * t8794 + F::new(2.0) * t3882 * t8794 - t32183;
    (t32161, t32164, t32168, t32173, t32176, t32183, t32184)
}
