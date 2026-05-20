//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2490/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2490<F: Float>(t10422: F, t21565: F, t3070: F, t10403: F, t1041: F, t10937: F, t13995: F, t14172: F, t17998: F, t21391: F, t21566: F, t3071: F, t42388: F, t43253: F, t4347: F, t4582: F, t5873: F, t62704: F, t62766: F, t62778: F, t62780: F, t70339: F, t884: F) -> F {
    let t70846 = t3070 * t10422 * t21565;
    let t70863 = -t43253 + t62704 / F::new(384.0) - F::new(5.0) / F::new(768.0) * t1041 * t4582 * t14172 * t70339 + t70846 / F::new(2304.0) - t10937 * t21566 / F::new(288.0) + t10403 * t3071 * t5873 * t4347 / F::new(768.0) + F::new(5.0) / F::new(4608.0) * t13995 * t17998 + F::new(7.0) / F::new(648.0) * t62766 - t62778 / F::new(256.0) + t62780 / F::new(1152.0) + t42388 * t3071 * t21391 * t884 / F::new(768.0);
    t70863
}
