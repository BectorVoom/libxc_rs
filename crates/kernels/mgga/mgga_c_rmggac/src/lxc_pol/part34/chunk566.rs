//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 566/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk566<F: Float>(t14567: F, t289: F, t13970: F, t13976: F, t2012: F, t2265: F, t2010: F, t3194: F, t4965: F, t1356: F, t14498: F, t2144: F, t698: F) -> (F, F, F, F, F, F, F, F) {
    let t14568 = t289 * t14567;
    let t14569 = F::new(0.2363e1) * t14568;
    let t14570 = F::cast_from(0.68186654135613354325e-2_f64) * t13970;
    let t14571 = F::cast_from(0.85129199786595678799e-5_f64) * t13976;
    let t14572 = t2012 * t2265;
    let t14573 = t2010 * t14572;
    let t14574 = F::cast_from(0.36021158228745895953e-3_f64) * t14573;
    let t14576 = t4965 * t3194;
    let t14577 = F::cast_from(0.39914139006212695214e-1_f64) * t14576;
    let t14578 = t1356 * t14498;
    let t14579 = F::cast_from(0.39914139006212695214e-1_f64) * t14578;
    let t14580 = t2144 * t698;
    (t14569, t14570, t14571, t14572, t14574, t14577, t14579, t14580)
}
