//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 898/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk898<F: Float>(t1266: F, t1393: F, t1983: F, t2036: F, t2040: F, t2075: F, t2314: F, t23938: F, t26977: F, t31057: F, t31060: F, t32213: F, t32220: F, t32235: F, t32263: F, t32278: F, t4034: F, t510: F, t574: F, t652: F, t672: F, t7040: F, t7042: F, t7050: F, t7057: F, t7061: F, t7156: F, t7220: F, t8607: F, t8711: F, t8721: F, t8780: F) -> F {
    let t32280 = -t31057 - t31060 - F::new(3.0) * t1983 * t32213 - F::new(4.0) * t2314 * t8721 - F::new(4.0) * t4034 * t8721 - F::new(4.0) * t652 * t32220 - F::new(4.0) * t7042 * t7057 - F::new(4.0) * t7042 * t7061 - F::new(2.0) * t8607 * t7220 - F::new(4.0) * t23938 * t2040 - F::new(4.0) * t26977 * t2040 - F::new(4.0) * t7042 * t7050 - F::new(2.0) * t32235 * t672 - F::new(2.0) * t7040 * t2075 - F::new(2.0) * t2036 * t7156 - t32263 * t510 - t8711 * t1266 + t8780 * t1393 + t32278 * t574;
    t32280
}
