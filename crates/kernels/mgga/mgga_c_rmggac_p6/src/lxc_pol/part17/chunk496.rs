//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 496/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk496<F: Float>(t196: F, t6261: F, t231: F, t446: F, t1839: F, t500: F, t1910: F, t195: F, t1023: F, t1143: F, t1535: F, t4155: F, t4163: F, t4187: F, t4585: F, t5385: F, t5388: F, t5402: F, t5452: F, t5981: F, t5985: F, t5988: F, t5989: F, t5990: F, t5992: F, t5994: F, t6034: F, t6039: F) -> F {
    let t6262 = t196 * t6261;
    let t6265 = t446 * t231;
    let t6268 = t500 * t1839;
    let t6275 = t195 * t1910;
    let t6280 = t5981 - t5385 + F::cast_from(0.31091e-1_f64) * t6262 * t500 + F::cast_from(0.186546e0_f64) * t6265 * t1839 + t5388 - t5985 + F::cast_from(0.186546e0_f64) * t4585 * t6268 + t5988 + F::cast_from(0.186546e0_f64) * t1143 * t6039 - t4155 - t4163 - t5989 - t5990 + F::cast_from(0.186546e0_f64) * t5452 * t1535 + F::cast_from(0.93273e-1_f64) * t6275 * t1023 + F::cast_from(0.373092e0_f64) * t1143 * t6034 - t5402 + t5992 + t5994 + t4187;
    t6280
}
