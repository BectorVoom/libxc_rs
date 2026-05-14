//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1344/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1344<F: Float>(t10456: F, t1273: F, t13244: F, t1339: F, t19247: F, t2056: F, t20706: F, t20944: F, t20953: F, t20969: F, t20981: F, t3493: F, t3502: F, t4541: F, t5986: F, t6058: F, t626: F, t645: F, t6486: F, t65483: F, t65485: F, t65487: F, t65489: F, t65500: F, t65504: F, t65506: F, t65508: F, t68168: F, t7798: F) -> (F,) {
    let t68681 = -4.0 * t20944 * t626 * t645 - 4.0 * t10456 * t6486 + 2.0 * t1273 * t20981 - 2.0 * t13244 * t5986 - 2.0 * t1339 * t68168 - 4.0 * t19247 * t3493 - 4.0 * t2056 * t20953 - 4.0 * t2056 * t20969 - 4.0 * t20706 * t3502 + 2.0 * t4541 * t6058 - 2.0 * t6486 * t7798 - t65483 - t65485 + t65487 - t65489 + t65500 + t65504 - t65506 - t65508;
    (t68681,)
}
