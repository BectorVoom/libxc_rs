//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1254/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1254<F: Float>(t13133: F, t13220: F, t13554: F, t1799: F, t18627: F, t19305: F, t19656: F, t3493: F, t41905: F, t42336: F, t42719: F, t5801: F, t5815: F, t6234: F, t6323: F, t65094: F, t65097: F, t65956: F, t7798: F) -> (F,) {
    let t67586 = 4.0 * t13133 * t5815 + 2.0 * t13220 * t5801 + 4.0 * t13554 * t5815 + 2.0 * t1799 * t41905 + 2.0 * t1799 * t42336 + 4.0 * t1799 * t42719 + 2.0 * t1799 * t65094 + 4.0 * t1799 * t65097 + 2.0 * t1799 * t65956 + 2.0 * t18627 * t3493 + 2.0 * t18627 * t6234 + 4.0 * t19305 * t5815 + 4.0 * t19656 * t5815 + 2.0 * t6323 * t7798;
    (t67586,)
}
