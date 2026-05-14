//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 517/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk517<F: Float>(t1993: F, t70: F, t582: F, t602: F, t350: F, t41: F, t47: F, t1985: F, t1992: F, t48: F, t59: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t1994 = t1993 * t70;
    let t1997 = t582 * t602;
    let t2003 = 1.0 / t41 / t350;
    let t2004 = sigma0 * t2003;
    let t2009 = 1.0 / t47;
    let t2010 = t2009 * t1985;
    let t2013 = t48 * t1992;
    let t2016 = 1.0 / t59;
    (t1994, t1997, t2004, t2009, t2010, t2013, t2016)
}
