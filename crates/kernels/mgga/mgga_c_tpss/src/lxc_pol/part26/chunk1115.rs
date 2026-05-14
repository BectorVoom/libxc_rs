//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1115/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1115<F: Float>(t1061: F, t5130: F, t1531: F, t4142: F, t5149: F, t5146: F, t12083: F, t12269: F, t15612: F, t15615: F, t15618: F, t15621: F, t15625: F, t15628: F, t15632: F, t2930: F, t2955: F, t4125: F, t4147: F, t9424: F) -> (F,) {
    let t15723 = t5130 * t1061;
    let t15726 = t1531 * t4142;
    let t15729 = t5149 * t1061;
    let t15732 = t5146 * t1061;
    let t15735 = -t15612 + t15615 + t15618 + t15621 - t15625 - t15628 - t15632 - 4.0 * t12083 * t4125 + 0.64327917994770140268e2 * t12269 * t4147 + 6.0 * t2955 * t15723 - 4.0 * t2930 * t15726 - 0.19298375398431042081e3 * t9424 * t15729 - 2.0 * t2930 * t15732;
    (t15735,)
}
