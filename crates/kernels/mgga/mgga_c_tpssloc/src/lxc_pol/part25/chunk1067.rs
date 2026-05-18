//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1067/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1067<F: Float>(t28: F, t265: F, t504: F, t24379: F, t2071: F, t2250: F, t24419: F, t52: F, t607: F, t7150: F, t24387: F, t2094: F, t3701: F, t15904: F, t2075: F, t2363: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t24420 = piecewise3::<f64>(t505, F::new(0.0), t24379);
    let t24427 = piecewise3::<f64>(t401, t24419, t24420 * t52 / F::new(2.0) - t7150 * t607 - t2071 * t2250 / F::new(2.0));
    let t24428 = t24387 + t24427;
    let t24432 = t2094 * t3701;
    let t24433 = t24432 * t15904;
    let t24442 = t2075 * t2363;
    (t24420, t24428, t24432, t24433, t24442)
}
