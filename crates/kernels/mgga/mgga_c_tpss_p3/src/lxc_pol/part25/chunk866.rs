//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 866/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk866<F: Float>(t2257: F, t704: F, t172: F, t2274: F, t182: F, t2209: F, t177: F, t2214: F, t7813: F, t7821: F, t7824: F, t7827: F, t7830: F, t7834: F, t7836: F, t7838: F, t7841: F) -> (F, F, F, F, F, F) {
    let t7850 = t2257 * t704;
    let t7852 = F::new(1.0) / t2274 / t172;
    let t7853 = t7850 * t7852;
    let t7857 = F::new(1.0) / t2209 / t182;
    let t7858 = t177 * t7857;
    let t7859 = t7813 * t2214;
    let t7870 = -F::cast_from(0.34523333333333333333e1_f64) * t7821 + F::cast_from(0.23015555555555555556e1_f64) * t7824 - F::cast_from(0.26851481481481481482e1_f64) * t7827 - F::cast_from(0.93932222222222222223e0_f64) * t7830 + F::new(0.73355e-1) * t7834 - F::new(0.14671e0) * t7836 - F::cast_from(0.17116166666666666667e0_f64) * t7838 - F::cast_from(0.36793333333333333333e0_f64) * t7841;
    (t7850, t7853, t7857, t7858, t7859, t7870)
}
