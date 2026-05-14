//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 859/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk859<F: Float>(t1356: F, t77980: F, t2392: F, t739: F, t8264: F, t2211: F, t8924: F, t76027: F, t76029: F, t76031: F, t76033: F, t77831: F, t11905: F, t3188: F, t1971: F, t2144: F, t495: F, t7230: F, t9540: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t78427 = 0.39914139006212695214e-1 * t1356 * t77980;
    let t78430 = t739 * t8264 * t2392;
    let t78431 = 0.2993560425465952141e-1 * t78430;
    let t78433 = t739 * t2211 * t8924;
    let t78434 = 0.2993560425465952141e-1 * t78433;
    let t78436 = 0.38430329123504567781e-4 * t76027;
    let t78438 = 0.1276937996798935182e-4 * t76029;
    let t78439 = 0.2553875993597870364e-4 * t76031;
    let t78440 = 0.3830813990396805546e-4 * t76033;
    let t78444 = 0.39914139006212695214e-1 * t1356 * t77831;
    let t78445 = t11905 * t3188;
    let t78446 = 0.14967802127329760705e-1 * t78445;
    let t78450 = t7230 * t1971 * t2144 * t9540 * t495;
    (t78427, t78431, t78434, t78436, t78438, t78439, t78440, t78444, t78446, t78450)
}
