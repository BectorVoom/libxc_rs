//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 820/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk820<F: Float>(t1356: F, t77416: F, t2010: F, t72162: F, t8465: F, t2415: F, t72171: F, t7349: F, t75016: F, t75020: F, t75022: F, t75024: F, t75033: F, t75037: F, t1986: F, t2464: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t77418 = 0.39914139006212695214e-1 * t1356 * t77416;
    let t77420 = t2010 * t8465 * t72162;
    let t77421 = 0.36021158228745895953e-3 * t77420;
    let t77423 = t7349 * t2415 * t72171;
    let t77424 = 0.5124043883133942371e-4 * t77423;
    let t77425 = 0.2553875993597870364e-4 * t75016;
    let t77426 = 0.2553875993597870364e-4 * t75020;
    let t77427 = 0.3830813990396805546e-4 * t75022;
    let t77428 = 0.638468998399467591e-4 * t75024;
    let t77430 = 0.23268647941669485538e-4 * t75033;
    let t77431 = 0.23268647941669485538e-4 * t75037;
    let t77435 = t1986 * t2464;
    (t77418, t77421, t77424, t77425, t77426, t77427, t77428, t77430, t77431, t77435)
}
