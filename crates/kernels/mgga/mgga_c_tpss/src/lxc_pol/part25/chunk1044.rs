//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1044/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1044<F: Float>(t10706: F, t10719: F, t10923: F, t1364: F, t14147: F, t14151: F, t14156: F, t14157: F, t14160: F, t14162: F, t14163: F, t14165: F, t14168: F, t14426: F, t198: F, t207: F, t2439: F, t3552: F, t750: F, t7979: F, t7988: F, t7992: F, t8222: F, t8225: F, t823: F, t8231: F, t8234: F) -> F {
    let t14430 = t14426 * t198 * t207 * t823 + F::new(6.0) * t10923 * t1364 * t2439 + F::new(6.0) * t14151 * t3552 * t750 + t10706 - t10719 + t14147 + t14156 + t14157 + t14160 + t14162 + t14163 + t14165 + t14168 + t7979 + t7988 + t7992 + t8222 + t8225 - t8231 - t8234;
    t14430
}
