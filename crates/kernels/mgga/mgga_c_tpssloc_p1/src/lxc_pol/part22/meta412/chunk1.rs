//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1714/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1714<F: Float>(t14838: F, t4745: F, t11350: F, t11420: F, t18257: F, t18261: F, t18264: F, t18268: F, t18631: F, t18634: F, t18637: F, t18640: F, t18644: F, t18647: F, t18651: F, t18668: F, t3332: F, t3357: F, t436: F) -> (F, F) {
    let t18672 = F::new(4.0) * t14838 * t4745;
    let t18673 = F::new(6.0) * t3357 * t18631 - F::new(4.0) * t3332 * t18634 - F::cast_from(0.19298375398431042081e3_f64) * t11420 * t18637 - F::new(2.0) * t3332 * t18640 + F::cast_from(0.32163958997385070134e2_f64) * t3357 * t18644 + F::cast_from(0.64327917994770140268e2_f64) * t3357 * t18647 + F::cast_from(0.2069040516770936012e4_f64) * t11350 * t18651 - F::new(0.310907e-1) * t18668 * t436 + t18257 - t18261 - t18264 - t18268 + t18672;
    (t18672, t18673)
}
