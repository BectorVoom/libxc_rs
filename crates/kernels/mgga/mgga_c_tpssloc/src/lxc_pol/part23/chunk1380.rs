//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1380/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1380<F: Float>(t1268: F, t1458: F, t1774: F, t1849: F, t19451: F, t20293: F, t20296: F, t20347: F, t20350: F, t20720: F, t22425: F, t28002: F, t4028: F, t510: F, t513: F, t5460: F, t5493: F, t574: F, t6287: F, t6295: F, t6468: F, t652: F, t67001: F, t7458: F, t7676: F, t79713: F, t79817: F, t79825: F, t79829: F, t79855: F, t79891: F, t79903: F, t79915: F, t79926: F, t79939: F, t79988: F, t80534: F, t88: F, t89: F) -> (F,) {
    let t80558 = -8.0 * t652 * t22425 * t1458 - 12.0 * t652 * t6287 * t5493 - 8.0 * t7458 * t20720 - 8.0 * t4028 * t20720 + 6.0 * t6295 * t6468 - 2.0 * t652 * t510 * t79817 - 24.0 * t19451 * t5460 - 4.0 * t20293 * t1774 - 6.0 * t89 * t79825 * t510 - 12.0 * t79829 * t510 - 24.0 * t20296 * t1774 + t513 * (t79855 + t79891 + t79903 + t79915 + t79926 + t79939 + t79988 + t80534) + 4.0 * t20350 * t1849 + (2.0 * t1268 * t79817 + 8.0 * t1458 * t67001 + 12.0 * t19451 * t5493 + 8.0 * t20347 * t4028 + 8.0 * t20347 * t7676 + 24.0 * t28002 * t5493 + 6.0 * t79825 * t88 + t79713 + 12.0 * t79829) * t574;
    (t80558,)
}
