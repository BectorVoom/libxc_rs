//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1073/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1073<F: Float>(t32451: F, t466: F, t1170: F, t8891: F, t2121: F, t2144: F, t477: F, t1090: F, t7362: F, t1186: F, t7376: F, t7386: F) -> (F, F, F, F, F, F, F, F) {
    let t32452 = t466 * t32451;
    let t32454 = t1170 * t8891;
    let t32456 = F::cast_from(0.54831135561607547883e-2_f64) * t2121 * t32454;
    let t32457 = t477 * t2144;
    let t32458 = t32457 * t1090;
    let t32459 = t7362 * t32458;
    let t32462 = t1186 * t8891;
    let t32465 = t7386 * t7376;
    (t32452, t32454, t32456, t32457, t32458, t32459, t32462, t32465)
}
