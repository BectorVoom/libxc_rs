//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 895/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk895<F: Float>(t28: F, t265: F, t504: F, t25882: F, t1409: F, t1972: F, t25949: F, t3966: F, t52: F, t607: F, t6856: F, t7664: F, t25890: F, t113: F, t2314: F, t24980: F, t24983: F, t24988: F, t24989: F, t24993: F, t24998: F, t24999: F, t25005: F, t25007: F, t25011: F, t4073: F, t4077: F, t6517: F, t652: F, t672: F, t7472: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t25950 = piecewise3(t505, 0.0, t25882);
    let t25957 = piecewise3(t401, t25949, -t6856 * t1409 / 2.0 - t1972 * t3966 / 2.0 + t25950 * t52 / 2.0 - t7664 * t607 / 2.0);
    let t25958 = t25890 + t25957;
    let t25962 = -t113 * t25958 - 2.0 * t2314 * t7472 - 2.0 * t24980 * t652 - 2.0 * t24983 * t652 - 2.0 * t24999 * t672 - 2.0 * t4073 * t6517 - 2.0 * t4077 * t6517 + t24988 + t24989 + t24993 + t24998 - t25005 - t25007 - t25011;
    (t25958, t25962)
}
