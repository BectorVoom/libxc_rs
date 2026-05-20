//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2433/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2433<F: Float>(t1020: F, t14489: F, t248: F, t3101: F, t3038: F, t49650: F, t1022: F, t10403: F, t10413: F, t10480: F, t10876: F, t13975: F, t13985: F, t14143: F, t14180: F, t14211: F, t14218: F, t2244: F, t2775: F, t2776: F, t3043: F, t3071: F, t3117: F, t3132: F, t360: F, t42610: F, t42613: F, t42619: F, t42622: F, t42651: F, t4582: F) -> F {
    let t49757 = t1020 * t248 * t3101 * t14489;
    let t49771 = t49650 * t3038;
    let t49786 = -t42610 / F::new(432.0) - t42613 / F::new(324.0) - t42619 / F::new(108.0) - t42622 / F::new(81.0) + t49757 / F::new(1536.0) + F::new(3.0) / F::new(512.0) * t10480 * t4582 * t13975 * t13985 - F::new(3.0) / F::new(512.0) * t10876 * t4582 * t13975 * t3132 - t3117 * t14143 / F::new(384.0) + F::new(5.0) / F::new(2304.0) * t3117 * t14180 - t49771 * t3043 / F::new(1024.0) - t42651 / F::new(216.0) - t10403 * t3071 * t14211 * t2776 * t1022 / F::new(384.0) + t10413 * t3071 * t14218 * t360 * t2775 * t2244 / F::new(768.0);
    t49786
}
