//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 798/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk798<F: Float>(t73938: F, t73940: F, t73944: F, t1970: F, t1971: F, t209: F, t2447: F, t476: F, t515: F, t2211: F, t40983: F, t739: F, t15672: F, t4041: F, t14391: F, t17859: F) -> (F, F, F, F, F, F, F) {
    let t76779 = 0.20455996240684006296e-1 * t73938;
    let t76780 = 0.40911992481368012592e-1 * t73940;
    let t76781 = 0.20455996240684006296e-1 * t73944;
    let t76786 = t1970 * t1971 * t515 * t2447 * t476 * t209;
    let t76787 = 0.42564599893297839398e-5 * t76786;
    let t76790 = 0.11974241701863808564e0 * t739 * t2211 * t40983;
    let t76792 = 0.11974241701863808564e0 * t4041 * t15672;
    let t76793 = t17859 * t14391;
    (t76779, t76780, t76781, t76787, t76790, t76792, t76793)
}
