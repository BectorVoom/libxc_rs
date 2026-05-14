//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 927/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk927<F: Float>(t5: F, t124330: F, t124364: F, t112: F, t120067: F, t124293: F, t1442: F, t1459: F, t1774: F, t1983: F, t24987: F, t24990: F, t26902: F, t26906: F, t26969: F, t27147: F, t27171: F, t31055: F, t31057: F, t31060: F, t32108: F, t32110: F, t32197: F, t32206: F, t33878: F, t35233: F, t4028: F, t510: F, t5107: F, t6876: F, t7042: F, t7057: F, t8607: F, t8718: F, t8809: F, t9003: F) -> (F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t124366 = piecewise3(t8, 0.0, t124330 + t124364);
    let t124367 = t124366 * t112;
    let t124383 = -2.0 * t124293 * t1459 - 4.0 * t9003 * t27171 - t24987 * t8809 - 2.0 * t8607 * t26902 - t31055 - t31057 - t31060 - t120067 - 4.0 * t7042 * t27147 - 4.0 * t35233 * t7057 - 2.0 * t4028 * t32197 - t124367 * t510 - t1442 * t32108 + 3.0 * t1983 * t32110 * t24990 + 6.0 * t8607 * t26969 + 3.0 * t6876 * t33878 + 6.0 * t8607 * t26906 - 2.0 * t32206 * t1774 - 2.0 * t8718 * t5107;
    (t124367, t124383)
}
