//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1074/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1074<F: Float>(t76029: F, t76031: F, t76033: F, t1356: F, t77831: F, t11905: F, t3188: F, t1971: F, t2144: F, t495: F, t7230: F, t9540: F) -> (F, F, F, F, F, F) {
    let t78438 = F::new(0.1276937996798935182e-4) * t76029;
    let t78439 = F::new(0.2553875993597870364e-4) * t76031;
    let t78440 = F::new(0.3830813990396805546e-4) * t76033;
    let t78444 = F::new(0.39914139006212695214e-1) * t1356 * t77831;
    let t78445 = t11905 * t3188;
    let t78446 = F::new(0.14967802127329760705e-1) * t78445;
    let t78450 = t7230 * t1971 * t2144 * t9540 * t495;
    (t78438, t78439, t78440, t78444, t78446, t78450)
}
