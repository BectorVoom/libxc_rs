//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 771/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk771<F: Float>(t1072: F, t1535: F, t1080: F, t1543: F, t2836: F, t2893: F, t2981: F, t2988: F, t4044: F, t4049: F, t4054: F, t4058: F, t4072: F, t4080: F, t4088: F, t4090: F, t4093: F, t4096: F, t4099: F, t4102: F) -> (F, F, F) {
    let t4158 = t1535 * t1072;
    let t4163 = t1543 * t1080;
    let t4180 = -F::cast_from(0.1294625e1_f64) * t4072 + F::cast_from(0.258925e1_f64) * t4080 + t2981 - F::cast_from(0.10064166666666666667e0_f64) * t2836 - F::cast_from(0.10064166666666666667e0_f64) * t4044 - F::cast_from(0.20128333333333333333e0_f64) * t4049 + F::cast_from(0.60385e0_f64) * t4054 + F::cast_from(0.301925e0_f64) * t4058 + F::cast_from(0.82524375e-1_f64) * t4088 + F::cast_from(0.16504875e0_f64) * t4090 + t2988 - F::cast_from(0.5519e-1_f64) * t2893 - F::cast_from(0.5519e-1_f64) * t4093 - F::cast_from(0.27595e-1_f64) * t4096 + F::cast_from(0.16557e0_f64) * t4099 + F::cast_from(0.82785e-1_f64) * t4102;
    (t4158, t4163, t4180)
}
