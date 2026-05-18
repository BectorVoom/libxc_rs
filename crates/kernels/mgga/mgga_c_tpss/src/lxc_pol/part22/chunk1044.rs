//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1044/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1044<F: Float>(t10980: F, t10986: F, t11004: F, t11010: F, t11015: F, t11020: F, t11056: F, t11059: F, t11062: F, t11065: F, t11068: F, t11309: F, t11321: F, t11328: F, t11345: F, t8605: F, t8607: F, t8616: F, t8618: F, t8627: F, t8629: F, t8631: F) -> F {
    let t11347 = F::new(0.17215833333333333333e0) * t8605 + F::new(0.11477222222222222222e0) * t8607 - F::new(0.45908888888888888888e0) * t8616 - F::new(0.34431666666666666666e0) * t8618 - F::new(0.23154444444444444444e0) * t8627 + F::new(0.69463333333333333333e-1) * t8629 + F::new(0.23154444444444444444e-1) * t8631 - F::new(0.22954444444444444444e0) * t10980 + t11309 - F::new(0.516475e0) * t10986 + t11321 - F::new(0.69463333333333333334e-1) * t11056 - F::new(0.34731666666666666667e-1) * t11059 - F::new(0.46308888888888888889e-1) * t11062 + F::new(0.41678e0) * t11065 + F::new(0.20839e0) * t11068 + t11328 - F::new(0.68863333333333333333e0) * t11004 - F::new(0.57386111111111111112e0) * t11010 + F::new(0.20659e1) * t11015 - F::new(0.68863333333333333334e0) * t11020 + t11345;
    t11347
}
