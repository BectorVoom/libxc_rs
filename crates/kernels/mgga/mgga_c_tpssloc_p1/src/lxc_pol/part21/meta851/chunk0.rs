//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3079/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3079<F: Float>(t3271: F, t43889: F, t5992: F, t11243: F, t5999: F, t43880: F, t11265: F, t63323: F, t63327: F, t63330: F, t63848: F, t63853: F, t63856: F, t63858: F, t63860: F, t63862: F, t63865: F, t63867: F) -> (F, F, F, F, F) {
    let t63870 = t43889 * t5992 * t3271;
    let t63873 = t11243 * t5999 * t3271;
    let t63876 = t43880 * t5992 * t3271;
    let t63879 = t11265 * t5999 * t3271;
    let t63881 = -F::cast_from(0.76790625e-1_f64) * t63848 + F::cast_from(0.13287407407407407407e1_f64) * t63323 + F::cast_from(0.71752000000000000001e1_f64) * t63327 - F::cast_from(0.47834666666666666668e1_f64) * t63330 + F::cast_from(0.3071625e0_f64) * t63853 + F::cast_from(0.3071625e0_f64) * t63856 + F::cast_from(0.15358125e0_f64) * t63858 + F::cast_from(0.142419375e1_f64) * t63860 - F::cast_from(0.1898925e1_f64) * t63862 - F::cast_from(0.1898925e1_f64) * t63865 - F::cast_from(0.9494625e0_f64) * t63867 + F::cast_from(0.1151859375e0_f64) * t63870 - F::cast_from(0.76790625e-1_f64) * t63873 - F::cast_from(0.3560484375e1_f64) * t63876 + F::cast_from(0.142419375e1_f64) * t63879;
    (t63870, t63873, t63876, t63879, t63881)
}
