//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1109/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1109<F: Float>(t19703: F, t3632: F, t1381: F, t17960: F, t17964: F, t3638: F, t3667: F, t5552: F, t3671: F, t3678: F, t1385: F, t17974: F, t3685: F, t5559: F, t3689: F, t1705: F, t3692: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19704 = t19703 * t3632;
    let t19706 = t17960 * t1381;
    let t19708 = t17964 * t3638;
    let t19710 = t5552 * t3667;
    let t19712 = t17964 * t3671;
    let t19716 = t17964 * t3678;
    let t19718 = t17974 * t1385;
    let t19720 = t5559 * t3685;
    let t19722 = t5559 * t3689;
    let t19733 = t1705 * t3692;
    (t19704, t19706, t19708, t19710, t19712, t19716, t19718, t19720, t19722, t19733)
}
