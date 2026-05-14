//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 906/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk906<F: Float>(t10375: F, t38: F, t10314: F, t10317: F, t10320: F, t1291: F, t1307: F, t1314: F, t1986: F, t1994: F, t1997: F, t2046: F, t3441: F, t3463: F, t3483: F, t583: F, t616: F, t85: F) -> (F,) {
    let t10376 = t38 * t10375;
    let t10383 = -t1291 * t2046 / 12.0 - t10314 * t85 / 12.0 - t10317 * t85 / 12.0 - t10320 * t85 / 6.0 - t3441 * t616 / 6.0 - t1986 * t1314 / 12.0 - t1994 * t1314 / 12.0 - t1997 * t1314 / 6.0 - t583 * t3483 / 6.0 + t10376 * t85 / 24.0 + t3463 * t616 / 12.0 + t1307 * t2046 / 24.0;
    (t10383,)
}
