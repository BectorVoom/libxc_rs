//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1007/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1007<F: Float>(t11085: F, t2488: F, t2487: F, t3781: F, t849: F, t2496: F, t3773: F, t2504: F, t3789: F, t11024: F, t11028: F, t11033: F, t11037: F, t11080: F, t11083: F, t10980: F, t10983: F, t10986: F, t11002: F, t11010: F, t11015: F, t11020: F, t11053: F, t11056: F, t11059: F, t11062: F, t11065: F, t11068: F, t11071: F, t8605: F, t8607: F, t8616: F, t8618: F, t8627: F, t8629: F, t8631: F) -> (F, F, F, F, F, F) {
    let t11086 = t11085 * t2488;
    let t11088 = t2487 * t3781;
    let t11089 = t11088 * t849;
    let t11091 = t3773 * t2496;
    let t11093 = t2504 * t3781;
    let t11094 = t11093 * t849;
    let t11096 = t3789 * t2496;
    let t11098 = -0.19931111111111111111e0 * t11024 - 0.17938e1 * t11028 + 0.11958666666666666667e1 * t11033 + 0.59793333333333333334e0 * t11037 + 0.3071625e0 * t11080 + 0.142419375e1 * t11083 - 0.76790625e-1 * t11086 - 0.1898925e1 * t11089 - 0.9494625e0 * t11091 + 0.3071625e0 * t11094 + 0.15358125e0 * t11096;
    let t11100 = 0.99655555555555555557e-1 * t8605 + 0.66437037037037037038e-1 * t8607 - 0.26574814814814814816e0 * t8616 - 0.19931111111111111111e0 * t8618 - 0.18257037037037037037e0 * t8627 + 0.54771111111111111111e-1 * t8629 + 0.18257037037037037037e-1 * t8631 - 0.13287407407407407408e0 * t10980 + t10983 - 0.29896666666666666667e0 * t10986 + t11053 - 0.54771111111111111112e-1 * t11056 - 0.27385555555555555556e-1 * t11059 - 0.36514074074074074075e-1 * t11062 + 0.32862666666666666666e0 * t11065 + 0.16431333333333333333e0 * t11068 + 0.13287407407407407408e0 * t11002 - t11071 - 0.33218518518518518518e0 * t11010 + 0.11958666666666666667e1 * t11015 - 0.39862222222222222222e0 * t11020 + t11098;
    (t11086, t11089, t11091, t11094, t11096, t11100)
}
