//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 924/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk924<F: Float>(t117014: F, t121004: F, t121007: F, t124018: F, t124040: F, t124069: F, t124093: F, t124122: F, t124176: F, t124205: F, t124281: F, t1390: F, t1459: F, t1774: F, t1983: F, t2040: F, t26878: F, t26977: F, t27145: F, t27163: F, t27188: F, t32263: F, t32674: F, t32676: F, t32679: F, t33234: F, t33899: F, t5107: F, t533: F, t7042: F, t7050: F, t7057: F, t7061: F, t7217: F, t7796: F, t8329: F, t8607: F, t8711: F) -> (F,) {
    let t124292 = -2.0 * t8607 * t26878 - 4.0 * t26977 * t7796 - 4.0 * t7042 * t27163 - t32674 - t32676 - t32679 - t32263 * t1774 - t8711 * t5107 - 4.0 * t27188 * t7057 - 4.0 * t121004 * t2040 - 4.0 * t121007 * t2040 - 4.0 * t33234 * t7050 + 2.0 * t8607 * t27145 - 2.0 * t1983 * t7217 * t33899 - t8329 + t1983 * t533 * (t124018 + t124040 + t124069 + t124093 + t124122 + t124176 + t124205 + t124281) * t1390 - 4.0 * t27188 * t7061 - 2.0 * t117014 * t1459;
    (t124292,)
}
