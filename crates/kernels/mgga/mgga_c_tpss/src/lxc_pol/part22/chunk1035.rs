//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1035/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1035<F: Float>(t10980: F, t10986: F, t11004: F, t11010: F, t11015: F, t11020: F, t11056: F, t11059: F, t11062: F, t11065: F, t11068: F, t11169: F, t11181: F, t11188: F, t11205: F, t8605: F, t8607: F, t8616: F, t8618: F, t8627: F, t8629: F, t8631: F) -> F {
    let t11207 = F::new(0.10064166666666666667e0) * t8605 + F::new(0.67094444444444444447e-1) * t8607 - F::new(0.26837777777777777778e0) * t8616 - F::new(0.20128333333333333334e0) * t8618 - F::new(0.18396666666666666667e0) * t8627 + F::new(0.5519e-1) * t8629 + F::new(0.18396666666666666667e-1) * t8631 - F::new(0.13418888888888888889e0) * t10980 + t11169 - F::new(0.301925e0) * t10986 + t11181 - F::new(0.5519e-1) * t11056 - F::new(0.27595e-1) * t11059 - F::new(0.36793333333333333333e-1) * t11062 + F::new(0.33114e0) * t11065 + F::new(0.16557e0) * t11068 + t11188 - F::new(0.40256666666666666667e0) * t11004 - F::new(0.33547222222222222222e0) * t11010 + F::new(0.12077e1) * t11015 - F::new(0.40256666666666666666e0) * t11020 + t11205;
    t11207
}
