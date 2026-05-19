//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1029/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1029<F: Float>(t10980: F, t10983: F, t10986: F, t11002: F, t11010: F, t11015: F, t11020: F, t11053: F, t11056: F, t11059: F, t11062: F, t11065: F, t11068: F, t11071: F, t11098: F, t8605: F, t8607: F, t8616: F, t8618: F, t8627: F, t8629: F, t8631: F) -> F {
    let t11100 = F::cast_from(0.99655555555555555557e-1_f64) * t8605 + F::cast_from(0.66437037037037037038e-1_f64) * t8607 - F::cast_from(0.26574814814814814816e0_f64) * t8616 - F::cast_from(0.19931111111111111111e0_f64) * t8618 - F::cast_from(0.18257037037037037037e0_f64) * t8627 + F::cast_from(0.54771111111111111111e-1_f64) * t8629 + F::cast_from(0.18257037037037037037e-1_f64) * t8631 - F::cast_from(0.13287407407407407408e0_f64) * t10980 + t10983 - F::cast_from(0.29896666666666666667e0_f64) * t10986 + t11053 - F::cast_from(0.54771111111111111112e-1_f64) * t11056 - F::cast_from(0.27385555555555555556e-1_f64) * t11059 - F::cast_from(0.36514074074074074075e-1_f64) * t11062 + F::cast_from(0.32862666666666666666e0_f64) * t11065 + F::cast_from(0.16431333333333333333e0_f64) * t11068 + F::cast_from(0.13287407407407407408e0_f64) * t11002 - t11071 - F::cast_from(0.33218518518518518518e0_f64) * t11010 + F::cast_from(0.11958666666666666667e1_f64) * t11015 - F::cast_from(0.39862222222222222222e0_f64) * t11020 + t11098;
    t11100
}
