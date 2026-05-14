//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1132/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1132<F: Float>(t26739: F, t2752: F, t193: F, t201: F, t7844: F, t225: F, t26722: F, t2053: F, t40889: F, t26708: F, t1509: F, t7084: F, t2047: F, t4233: F, t26732: F, t26734: F) -> (F, F, F, F, F, F, F, F, F) {
    let t92276 = t26739 * t2752;
    let t92319 = t193 * t201 * t7844;
    let t92386 = t26722 * t225;
    let t92394 = t40889 * t2053;
    let t92439 = t26708 * t225;
    let t92552 = t7084 * t1509;
    let t92745 = t2047 * t4233;
    let t92847 = t26732 * t225;
    let t92939 = t26734 * t225;
    (t92276, t92319, t92386, t92394, t92439, t92552, t92745, t92847, t92939)
}
