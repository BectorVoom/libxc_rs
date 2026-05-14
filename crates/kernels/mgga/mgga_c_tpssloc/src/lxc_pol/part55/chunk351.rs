//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 351/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk351<F: Float>(t1554: F, t906: F, t340: F, t343: F, t974: F, t1593: F, t971: F, t973: F, t381: F, t1409: F, t998: F, t225: F, t68: F, t369: F, t1545: F, t1559: F, t1585: F, t1587: F, t1591: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t1597 = t906 / 6.0 + t1554 / 6.0;
    let t1598 = t340 * t1597;
    let t1599 = t1598 * t343;
    let t1600 = t974 * t1599;
    let t1603 = t971 + 0.27777777777777777777e-3 * t973 * t1593 - 0.83333333333333333332e-3 * t973 * t1600;
    let t1604 = t1603 * t381;
    let t1606 = t998 * t1409;
    let t1607 = t974 * t1606;
    let t1610 = t1603 * t225;
    let t1611 = t1610 * t68;
    let t1612 = t1611 * t369;
    let t1615 = -t1545 + t1559 + t1585 + t1587 - t1591;
    (t1597, t1599, t1600, t1603, t1604, t1606, t1607, t1610, t1611, t1612, t1615)
}
