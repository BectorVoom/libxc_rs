//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1452/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1452<F: Float>(t14933: F, t449: F, t300: F, t1671: F, t3265: F, t3313: F, t14722: F, t14704: F, t11137: F, t11139: F, t11141: F, t11143: F, t11459: F, t14702: F, t14708: F, t14720: F, t14728: F, t14733: F, t14738: F, t14742: F, t14746: F, t14751: F, t14755: F) -> (F, F, F, F) {
    let t14934 = t14933 * t449;
    let t14936 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t14934;
    let t14937 = t1671 * t3265;
    let t14939 = F::new(6.0) * t3313 * t14937;
    let t14946 = F::cast_from(0.23744444444444444444e-1_f64) * t14722;
    let t14947 = F::cast_from(0.11872222222222222222e-1_f64) * t14704;
    let t14956 = -t11459 + F::cast_from(0.15829629629629629629e-1_f64) * t11137 + F::cast_from(0.39574074074074074073e-2_f64) * t11139 - F::cast_from(0.11872222222222222222e-1_f64) * t11141 - F::cast_from(0.5936111111111111111e-2_f64) * t11143 + F::cast_from(0.79148148148148148146e-2_f64) * t14702 + F::cast_from(0.79148148148148148146e-2_f64) * t14720 - t14946 - t14947 + F::cast_from(0.19787037037037037037e-1_f64) * t14728 - F::cast_from(0.71233333333333333332e-1_f64) * t14733 - F::cast_from(0.23744444444444444444e-1_f64) * t14738 - F::cast_from(0.11872222222222222222e-1_f64) * t14742 + F::new(0.10685e0) * t14746 + F::cast_from(0.71233333333333333332e-1_f64) * t14751 + F::cast_from(0.35616666666666666666e-1_f64) * t14755 + F::cast_from(0.17808333333333333333e-1_f64) * t14708;
    (t14934, t14936, t14939, t14956)
}
