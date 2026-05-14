//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1185/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1185<F: Float>(t2186: F, t2319: F, t112: F, t30217: F, t1268: F, t12725: F, t12734: F, t15857: F, t19456: F, t2180: F, t2181: F, t2183: F, t2314: F, t26114: F, t26117: F, t26179: F, t29890: F, t29935: F, t29944: F, t29947: F, t29956: F, t29963: F, t30186: F, t4028: F, t5361: F, t652: F, t7458: F, t7676: F, t8124: F, t8143: F, t8144: F, t8148: F, t8150: F, t8235: F, t90375: F, t90381: F) -> (F, F, F) {
    let t110671 = t2186 * t2319;
    let t110684 = t30217 * t112;
    let t110736 = 4.0 * t26117 * t8150 + 2.0 * t7676 * t29956 - 2.0 * t652 * t15857 * t2180 + 2.0 * t4028 * t29944 + 4.0 * t2314 * t30186 - 2.0 * t4028 * t29963 - 4.0 * t26114 * t8144 - 4.0 * t26179 * t8144 - 4.0 * t7458 * t29890 - 4.0 * t19456 * t8144 - 4.0 * t12725 * t8124 - 2.0 * t7458 * t29963 + 2.0 * t90375 * t2183 + 4.0 * t26117 * t8148 + 4.0 * t4028 * t29947 - 2.0 * t4028 * t29935 - 2.0 * t90381 * t2181 + 2.0 * t4028 * t29956 + 4.0 * t1268 * t8143 * t5361 + 4.0 * t12734 * t8235;
    (t110671, t110684, t110736)
}
