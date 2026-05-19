//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 449/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk449<F: Float>(t1997: F, t1999: F, t553: F, t59: F, t544: F, t559: F, t1992: F) -> (F, F, F) {
    let t2000 = t1997 * t1999;
    let t2002 = t553 * t59;
    let t2003 = t544 * t2002;
    let t2004 = t2003 * t559;
    let t2006 = t1992 / F::new(96.0) + F::cast_from(0.20186378047070195427e-3_f64) * t2000 + t2004 / F::new(1536.0);
    (t2002, t2003, t2006)
}
