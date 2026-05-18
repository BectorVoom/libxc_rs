//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1078/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1078<F: Float>(t4844: F, t865: F, t2531: F, t1425: F, t3806: F, t2481: F, t4879: F, t8600: F, t4876: F, t2533: F, t4875: F, t3810: F) -> (F, F, F, F, F, F) {
    let t14863 = t4844 * t865;
    let t14865 = F::new(6.0) * t2531 * t14863;
    let t14866 = t1425 * t3806;
    let t14868 = F::new(4.0) * t2481 * t14866;
    let t14869 = t4879 * t865;
    let t14871 = F::new(0.96491876992155210402e2) * t8600 * t14869;
    let t14872 = t4876 * t865;
    let t14874 = F::new(2.0) * t2481 * t14872;
    let t14875 = t4875 * t2533;
    let t14876 = t14875 * t865;
    let t14878 = F::new(0.16081979498692535067e2) * t2531 * t14876;
    let t14879 = t3810 * t3806;
    (t14865, t14868, t14871, t14874, t14878, t14879)
}
