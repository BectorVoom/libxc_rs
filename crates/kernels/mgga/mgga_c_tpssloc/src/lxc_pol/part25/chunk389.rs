//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 389/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk389<F: Float>(t1997: F, t1999: F, t553: F, t59: F, t544: F, t559: F, t1998: F, t562: F, t214: F, t1985: F, t63: F, t67: F) -> (F, F, F, F, F, F, F, F) {
    let t2000 = t1997 * t1999;
    let t2002 = t553 * t59;
    let t2003 = t544 * t2002;
    let t2004 = t2003 * t559;
    let t2009 = t1998 * t562;
    let t2010 = t214 * t2009;
    let t2011 = t1985 * t2010;
    let t2031 = t63 * t67;
    (t2000, t2002, t2003, t2004, t2009, t2010, t2011, t2031)
}
