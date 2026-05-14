//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 807/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk807<F: Float>(t2039: F, t23938: F, t26977: F, t31237: F, t31239: F, t32206: F, t32235: F, t32263: F, t671: F, t7042: F, t7056: F, t8446: F, t9012: F, t1266: F, t1393: F, t1983: F, t2036: F, t2040: F, t2075: F, t2314: F, t31057: F, t31060: F, t32213: F, t32220: F, t4034: F, t510: F, t574: F, t652: F, t672: F, t7040: F, t7050: F, t7057: F, t7061: F, t7156: F, t7220: F, t8607: F, t8711: F, t8721: F, t8780: F) -> (F, F) {
    let t32278 = 4.0 * t2039 * t23938 + 4.0 * t2039 * t26977 + 2.0 * t32235 * t671 + 4.0 * t7042 * t7056 + 4.0 * t7056 * t9012 + t31237 + t31239 + 2.0 * t32206 + t32263 + t8446;
    let t32280 = -t31057 - t31060 - 3.0 * t1983 * t32213 - 4.0 * t2314 * t8721 - 4.0 * t4034 * t8721 - 4.0 * t652 * t32220 - 4.0 * t7042 * t7057 - 4.0 * t7042 * t7061 - 2.0 * t8607 * t7220 - 4.0 * t23938 * t2040 - 4.0 * t26977 * t2040 - 4.0 * t7042 * t7050 - 2.0 * t32235 * t672 - 2.0 * t7040 * t2075 - 2.0 * t2036 * t7156 - t32263 * t510 - t8711 * t1266 + t8780 * t1393 + t32278 * t574;
    (t32278, t32280)
}
