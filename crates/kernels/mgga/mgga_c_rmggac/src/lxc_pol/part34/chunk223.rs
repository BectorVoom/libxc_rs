//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 223/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk223<F: Float>(t1196: F, t236: F, t194: F, t457: F, t201: F, t211: F, t214: F, t1965: F, t1968: F, t490: F, t500: F, t6: F) -> (F, F, F, F, F, F, F) {
    let t1972 = t236 * t1196;
    let t1976 = t194 * t457;
    let t1977 = t1976 * t201;
    let t1978 = t211 * t214;
    let t1979 = t1965 * t1978;
    let t1981 = t1968 * t490;
    let t1985 = t6 * t500;
    (t1972, t1976, t1977, t1978, t1979, t1981, t1985)
}
