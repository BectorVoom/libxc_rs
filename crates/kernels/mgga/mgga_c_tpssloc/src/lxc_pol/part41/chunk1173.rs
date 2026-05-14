//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1173/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1173<F: Float>(t29895: F, t30514: F, t30521: F, t626: F, t111125: F, t111127: F, t111129: F, t1449: F, t26129: F, t29903: F, t30063: F, t30284: F, t30293: F, t30297: F, t4067: F, t5464: F, t5484: F, t662: F, t666: F, t8128: F, t8137: F, t8180: F, t8184: F, t96718: F) -> (F,) {
    let t111775 = t29895 * t30514;
    let t111803 = t626 * t30521;
    let t111805 = -110.0 / 27.0 * t111125 - 10.0 / 9.0 * t111127 - 20.0 / 9.0 * t111775 + 5.0 / 12.0 * t8128 * t8184 * t5484 * t666 - 5.0 / 36.0 * t8137 * t30063 * t5484 * t662 - 5.0 / 4.0 * t29903 * t8184 * t5464 * t662 - 3.0 / 2.0 * t29903 * t8180 * t96718 + 5.0 / 2.0 * t29903 * t30293 * t26129 - 25.0 / 18.0 * t8128 * t30297 * t30284 + 5.0 / 6.0 * t8128 * t8184 * t4067 * t1449 + 110.0 / 27.0 * t111129 + 40.0 / 27.0 * t111803;
    (t111805,)
}
