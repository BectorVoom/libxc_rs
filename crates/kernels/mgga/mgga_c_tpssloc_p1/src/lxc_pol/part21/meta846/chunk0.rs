//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3061/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3061<F: Float>(t11424: F, t18680: F, t14913: F, t1671: F, t3264: F, t18683: F, t44162: F, t11190: F, t3307: F, t6024: F, t18265: F, t3265: F, t43969: F) -> (F, F, F, F, F) {
    let t63563 = F::new(8.0) * t11424 * t18680;
    let t63566 = F::new(4.0) * t3264 * t1671 * t14913;
    let t63568 = F::cast_from(0.19298375398431042081e3_f64) * t44162 * t18683;
    let t63571 = F::cast_from(0.96491876992155210402e2_f64) * t11190 * t6024 * t3307;
    let t63574 = F::cast_from(0.62071215503128080361e4_f64) * t43969 * t18265 * t3265;
    (t63563, t63566, t63568, t63571, t63574)
}
