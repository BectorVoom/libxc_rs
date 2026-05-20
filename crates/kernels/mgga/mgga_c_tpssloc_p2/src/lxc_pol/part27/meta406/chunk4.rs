//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1689/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1689<F: Float>(t16080: F, t16121: F, t225: F, t3856: F, t5335: F, t3851: F, t5348: F, t1332: F, t1336: F, t1381: F, t16033: F, t16037: F, t16041: F, t16044: F, t16047: F, t16049: F, t16052: F, t16055: F, t16060: F, t16065: F, t16068: F, t3777: F, t3902: F, t5234: F, t5334: F, t5336: F, t5344: F, t5345: F, t5349: F, t5351: F, t564: F) -> (F, F, F, F) {
    let t16122 = t16080 + t16121;
    let t16123 = t16122 * t225;
    let t16125 = t5335 * t3856;
    let t16127 = t5348 * t3851;
    let t16131 = F::new(2.0) * t1332 * t5351 - t1336 * t16127 - F::new(2.0) * t1381 * t16060 - F::new(2.0) * t16033 * t5345 + F::new(4.0) * t16037 * t5334 + F::new(4.0) * t16041 * t5334 - t16044 * t5344 - F::new(6.0) * t16047 * t16049 + F::new(6.0) * t16052 * t5334 + F::new(4.0) * t16055 * t5336 + F::new(2.0) * t16065 * t5334 - F::new(2.0) * t16068 * t5344 + t16123 * t564 - t16125 * t5344 - F::new(2.0) * t3777 * t5349 - F::new(2.0) * t3902 * t5234;
    (t16122, t16123, t16125, t16131)
}
