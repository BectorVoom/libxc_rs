//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1156/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1156<F: Float>(t1338: F, t5895: F, t18690: F, t19609: F, t1844: F, t9895: F, t19581: F, t5757: F, t6436: F, t13133: F, t13554: F, t1760: F, t1800: F, t18547: F, t19305: F, t19308: F, t19579: F, t20289: F, t2056: F, t3493: F, t3499: F, t5706: F, t5809: F, t5816: F, t6103: F, t626: F, t6328: F, t6439: F, t646: F) -> (F, F, F, F, F, F) {
    let t20343 = t5895 * t1338;
    let t20346 = t18690 * t19609;
    let t20357 = t1844 * t9895;
    let t20358 = t20357 * t19581;
    let t20361 = t6436 * t5757;
    let t20363 = -2.0 * t13133 * t1800 - 2.0 * t13554 * t1800 - t1760 * t20361 - 2.0 * t1800 * t19305 - 2.0 * t1800 * t19308 - 3.0 * t18547 * t20346 + 2.0 * t19579 * t20358 - 2.0 * t20289 * t646 - 2.0 * t20343 * t626 - 2.0 * t2056 * t6328 - 2.0 * t3493 * t5809 - 2.0 * t3493 * t5816 - 2.0 * t3499 * t6328 - t5706 * t6439 - 2.0 * t5809 * t6103;
    (t20343, t20346, t20357, t20358, t20361, t20363)
}
