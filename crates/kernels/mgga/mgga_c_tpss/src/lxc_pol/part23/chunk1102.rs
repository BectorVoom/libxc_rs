//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1102/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1102<F: Float>(t1625: F, t3245: F, t774: F, t9986: F, t1244: F, t12819: F, t12825: F, t12831: F, t12835: F, t12838: F, t12843: F, t12846: F, t3271: F, t4413: F, t9981: F, t9991: F, t9995: F, t9997: F) -> (F, F, F) {
    let t12851 = t1625 * t3245;
    let t12853 = t9986 * t774 * t12851;
    let t12856 = 7.0 / 4608.0 * t9981 - 5.0 / 384.0 * t3271 * t12819 + t3271 * t12825 / 384.0 - t4413 * t12831 / 192.0 - t12835 + 5.0 / 384.0 * t1244 * t12838 + 5.0 / 768.0 * t1244 * t12843 - 119.0 / 13824.0 * t12846 - 35.0 / 1152.0 * t9991 - 119.0 / 1728.0 * t9995 + 7.0 / 1152.0 * t9997 - 5.0 / 128.0 * t1244 * t12853;
    (t12851, t12853, t12856)
}
