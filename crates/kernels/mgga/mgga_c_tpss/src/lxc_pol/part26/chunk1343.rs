//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1343/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1343<F: Float>(t1859: F, t1981: F, t4573: F, t1289: F, t578: F, t1861: F, t19192: F, t19229: F, t21133: F, t21139: F, t5489: F, t5966: F, t5976: F, t69135: F, t69139: F, t69228: F, t69232: F, t69242: F, t69245: F, t69248: F, t69251: F) -> (F,) {
    let t73011 = t1981 * t4573 * t1859;
    let t73015 = t578 * t1289 * t1859;
    let t73026 = 5.0 / 3.0 * t5966 * t69135 + 5.0 / 3.0 * t5966 * t69139 + 5.0 / 6.0 * t19229 * t21133 + 5.0 / 6.0 * t19192 * t21133 + 5.0 / 6.0 * t5966 * t69228 + 5.0 / 6.0 * t5966 * t69232 - 5.0 / 3.0 * t73011 * t5489 + 2.0 / 3.0 * t73015 * t69242 + t69245 * t1861 / 3.0 + t69248 * t1861 / 3.0 + t69251 * t1861 / 3.0 + t21139 * t5976 / 3.0;
    (t73026,)
}
