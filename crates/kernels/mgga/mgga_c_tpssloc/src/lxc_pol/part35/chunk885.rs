//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 885/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk885<F: Float>(t1512: F, t9671: F, t1500: F, t2693: F, t2642: F, t4166: F, t2638: F, t2629: F, t2696: F, t1516: F, t9601: F, t1519: F, t2627: F) -> (F, F, F, F, F, F, F, F) {
    let t13182 = t9671 * t1512;
    let t13234 = t1500 * t2693;
    let t13251 = t4166 * t2642;
    let t13278 = t4166 * t2638;
    let t13283 = t4166 * t2629;
    let t13360 = t4166 * t2696;
    let t13368 = t9601 * t1516;
    let t13416 = t2627 * t1519;
    (t13182, t13234, t13251, t13278, t13283, t13360, t13368, t13416)
}
