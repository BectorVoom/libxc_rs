//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 284/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk284<F: Float>(t2211: F, t352: F, t118: F, t2098: F, t2107: F, t2113: F, t2121: F, t2094: F, t2096: F, t2101: F, t2104: F, t2109: F, t2111: F, t2116: F, t2119: F) -> (F, F, F, F, F, F) {
    let t2212 = t2211 * t352;
    let t2213 = t118 * t2212;
    let t2217 = F::new(0.26609426004141796809e-1) * t2098;
    let t2220 = F::new(0.60610359231656314955e-2) * t2107;
    let t2223 = F::new(0.35403077613494883571e-3) * t2113;
    let t2226 = F::new(0.80640343452960568133e-4) * t2121;
    let t2227 = -F::new(0.19957069503106347607e-1) * t2094 + F::new(0.2993560425465952141e-1) * t2096 + t2217 + F::new(0.68186654135613354324e-2) * t2101 - F::new(0.90915538847484472432e-2) * t2104 - t2220 - F::new(0.66380770525302906695e-3) * t2109 + F::new(0.79656924630363488034e-3) * t2111 + t2223 + F::new(0.1814407727691612783e-3) * t2116 - F::new(0.21168090156402149135e-3) * t2119 - t2226;
    (t2213, t2217, t2220, t2223, t2226, t2227)
}
