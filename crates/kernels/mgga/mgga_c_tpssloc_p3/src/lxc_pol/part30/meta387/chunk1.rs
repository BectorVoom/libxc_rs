//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1469/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1469<F: Float>(t16935: F, t4282: F, t13433: F, t1510: F, t17030: F, t829: F, t13397: F, t16817: F, t16820: F, t16823: F, t16825: F, t16828: F, t16830: F, t17023: F, t17028: F, t17031: F, t17034: F, t226: F, t2617: F, t4166: F, t4281: F, t4283: F, t4288: F, t4291: F, t4292: F, t5575: F, t5651: F, t5655: F, t808: F, t812: F, t863: F) -> (F, F, F) {
    let t17037 = t4282 * t16935;
    let t17041 = t13433 * t1510;
    let t17046 = t17030 * t829;
    let t17048 = -F::new(6.0) * t13397 * t16817 + F::new(4.0) * t16820 * t4281 - t16823 * t812 + F::new(6.0) * t16825 * t4281 - t16828 * t4291 - F::new(2.0) * t16830 * t4292 + t17023 * t226 - t17028 * t812 + F::new(2.0) * t17031 * t4281 + F::new(4.0) * t17034 * t4283 + F::new(4.0) * t17037 * t4281 - F::new(2.0) * t17041 * t812 - t17046 * t4291 - t2617 * t5651 - F::new(2.0) * t4166 * t4288 + t5575 * t863 + t5655 * t808;
    (t17037, t17046, t17048)
}
