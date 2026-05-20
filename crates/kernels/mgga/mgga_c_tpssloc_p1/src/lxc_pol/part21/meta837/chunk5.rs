//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2983/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2983<F: Float>(t10413: F, t10422: F, t17700: F, t1023: F, t10403: F, t10408: F, t13611: F, t1616: F, t2771: F, t2780: F, t3039: F, t3070: F, t3071: F, t42397: F, t42735: F, t42752: F, t4582: F, t4600: F, t48607: F, t49743: F, t49852: F, t49871: F, t49873: F, t49877: F, t49884: F, t49887: F, t5873: F, t61524: F, t62091: F) -> F {
    let t62306 = t10413 * t10422 * t17700;
    let t62333 = -F::new(5.0) / F::new(15552.0) * t49852 + t42735 / F::new(13824.0) + t42752 / F::new(7776.0) - t49871 / F::new(5184.0) - t62306 / F::new(3456.0) + F::new(5.0) / F::new(1296.0) * t48607 * t42397 * t61524 + t10403 * t3071 * t5873 * t2780 / F::new(2304.0) + F::new(5.0) / F::new(6912.0) * t10403 * t10408 * t5873 * t2771 + t3070 * t3071 * t1616 * t13611 / F::new(2304.0) + t49743 * t4600 / F::new(144.0) - t49873 / F::new(864.0) - t49877 / F::new(324.0) - t3039 * t4582 * t62091 * t1023 / F::new(1536.0) - t49884 / F::new(2304.0) - t49887 / F::new(384.0);
    t62333
}
