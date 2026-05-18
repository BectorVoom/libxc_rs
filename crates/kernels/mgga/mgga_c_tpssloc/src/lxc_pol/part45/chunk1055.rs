//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1055/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1055<F: Float>(t2105: F, t7002: F, t2098: F, t7020: F, t115972: F, t115981: F, t116011: F, t116014: F, t116021: F, t116026: F, t116028: F, t116032: F, t116036: F, t1396: F, t1398: F, t1404: F, t2023: F, t2029: F, t2099: F, t23863: F, t23901: F, t24448: F, t24486: F, t3: F, t31782: F, t31820: F, t3932: F, t3946: F, t580: F, t7003: F, t7223: F, t7240: F, t8647: F, t8660: F) -> F {
    let t116038 = t7002 * t2105;
    let t116044 = t2098 * t7020;
    let tv4rho2sigma21 = t1398 * (t115981 + t116011) + F::new(2.0) * t116014 + F::new(2.0) * t1396 * t31820 + t3 * t115972 * t580 + t2023 * t24486 + F::new(2.0) * t116021 + t23863 * t2105 + t8647 * t3946 + t2099 * t23901 + F::new(2.0) * t116026 + F::new(2.0) * t116028 + F::new(2.0) * t31782 * t1404 + F::new(2.0) * t116032 + t24448 * t2029 + t3932 * t8660 + F::new(2.0) * t116036 + F::new(2.0) * t116038 + F::new(2.0) * t7223 * t7020 + F::new(2.0) * t7003 * t7240 + F::new(2.0) * t116044;
    tv4rho2sigma21
}
