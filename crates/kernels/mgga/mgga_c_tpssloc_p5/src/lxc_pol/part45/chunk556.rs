//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 556/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk556<F: Float>(t1882: F, t794: F, t6562: F, t225: F, t258: F, t852: F, t214: F, t1880: F, t857: F) -> (F, F, F, F, F, F, F) {
    let t6563 = t794 * t1882;
    let t6564 = t6562 * t6563;
    let t6565 = F::cast_from(0.41123351671205660912e-2_f64) * t6564;
    let t6567 = t852 * t225 * t258;
    let t6568 = t214 * t6567;
    let t6569 = t1880 * t6568;
    let t6571 = t225 * t857;
    (t6563, t6564, t6565, t6567, t6568, t6569, t6571)
}
