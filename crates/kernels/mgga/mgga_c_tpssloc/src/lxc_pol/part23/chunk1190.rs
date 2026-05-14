//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1190/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1190<F: Float>(t40: F, t52: F, t5499: F, t57973: F, t46369: F, t46371: F, t16637: F, t20217: F, t2291: F, t4104: F, t5398: F, t75: F, t75836: F, t75847: F, t75912: F, t767: F, t16649: F, t2298: F, t4111: F, t771: F, t78: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t75950 = 72.0 * t57973 * t5499;
    let t75951 = 16.0 * t46369;
    let t75952 = 0.86748650402413918736e-1 * t46371;
    let t75964 = piecewise3(t146, 0.0, -56.0 / 81.0 * t2291 * t75836 + 16.0 / 9.0 * t16637 * t5398 - 2.0 / 3.0 * t75 * t75847 - 8.0 / 9.0 * t4104 * t20217 + 2.0 / 3.0 * t767 * t75912);
    let t75976 = piecewise3(t150, 0.0, -56.0 / 81.0 * t2298 * t75836 - 16.0 / 9.0 * t16649 * t5398 - 2.0 / 3.0 * t78 * t75847 - 8.0 / 9.0 * t4111 * t20217 - 2.0 / 3.0 * t771 * t75912);
    (t75950, t75951, t75952, t75964, t75976)
}
