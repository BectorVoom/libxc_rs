//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2008/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2008<F: Float>(t1307: F, t2094: F, t671: F, t7786: F, t100990: F, t1266: F, t1459: F, t19289: F, t1983: F, t20127: F, t2036: F, t2040: F, t22574: F, t24432: F, t24987: F, t24990: F, t26905: F, t26969: F, t27188: F, t28826: F, t28959: F, t29252: F, t4026: F, t4037: F, t510: F, t5361: F, t5450: F, t6287: F, t652: F, t6876: F, t7040: F, t7042: F, t7156: F, t75214: F, t7685: F, t7890: F, t7900: F, t7943: F, t84733: F, t96356: F, t97789: F) -> (F, F) {
    let t102336 = t1307 * t2094;
    let t102344 = t7786 * t671;
    let t102366 = -F::new(6.0) * t22574 * t24432 * t97789 - t7040 * t6287 - t2036 * t19289 + F::new(6.0) * t1983 * t84733 * t28826 + F::new(6.0) * t1983 * t26905 * t24990 + F::new(6.0) * t6876 * t29252 + F::new(6.0) * t1983 * t102336 * t28826 - t5450 * t7156 - F::new(2.0) * t652 * t510 * t100990 - F::new(4.0) * t102344 * t1459 - F::new(4.0) * t27188 * t4037 - F::new(2.0) * t7042 * t20127 - F::new(2.0) * t28959 * t1266 - F::new(2.0) * t24987 * t7943 - F::new(3.0) * t22574 * t24432 * t75214 + F::new(6.0) * t7685 * t26969 - F::new(2.0) * t4026 * t7890 + F::new(2.0) * t7900 * t5361 - F::new(4.0) * t96356 * t2040;
    (t102344, t102366)
}
