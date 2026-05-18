//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1268/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1268<F: Float>(t7015: F, t96334: F, t7769: F, t85416: F, t24972: F, t26550: F, t1873: F, t96311: F, t34385: F, t580: F, t117773: F, t119795: F, t119796: F, t119810: F, t122875: F, t122897: F, t122910: F, t122914: F, t122918: F, t122921: F, t122923: F, t122925: F, t1266: F, t1459: F, t1774: F, t27290: F, t32595: F, t34229: F, t34372: F, t4072: F, t5107: F, t650: F, t652: F, t7266: F, t8860: F, t8913: F) -> (F, F, F, F, F, F) {
    let t123294 = t96334 * t7015;
    let t123296 = t85416 * t7769;
    let t123298 = t24972 * t26550;
    let t123306 = t96311 * t1873;
    let t125074 = t34385 * t580;
    let t125094 = -F::new(2.0) * t4072 * t652 * t8913 - F::new(2.0) * t117773 * t1459 - t1266 * t34229 - t1774 * t32595 - F::new(4.0) * t27290 * t7266 - t34372 * t650 - t5107 * t8860 + t119795 - t119796 - t119810 - F::new(4.0) * t122875 - F::new(4.0) * t122897 + F::new(6.0) * t122910 + F::new(6.0) * t122914 - F::new(4.0) * t122918 - F::new(4.0) * t122921 - F::new(4.0) * t122923 - F::new(2.0) * t122925;
    (t123294, t123296, t123298, t123306, t125074, t125094)
}
