//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1218/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1218<F: Float>(t2054: F, t24282: F, t24305: F, t2713: F, t2720: F, t40875: F, t81554: F, t81559: F, t82076: F, t82079: F, t82082: F, t82087: F, t82092: F, t82141: F, t82143: F, t82145: F, t82147: F, t82150: F, t82156: F, t82161: F, t82165: F, t84820: F, t85047: F, t85071: F, t85093: F, t85101: F, t85126: F, t85142: F, t85163: F) -> F {
    let t85166 = F::cast_from(6.0_f64) * t24305 * t2720 - F::cast_from(3.0_f64) * t2713 * t24282 - t40875 * t2054 + F::cast_from(0.16449340668482264365e-1_f64) * t81554 + t85163 + t85142 + t85126 - t85101 + t85093 + t85071 + t85047 + t84820 - F::cast_from(0.16449340668482264365e-1_f64) * t82165 + F::cast_from(0.19739208802178717238e0_f64) * t82161 - F::cast_from(0.49348022005446793095e-1_f64) * t82156 + F::cast_from(0.23029076935875170111e0_f64) * t82150 - F::cast_from(0.15626873635058151147e0_f64) * t82147 + F::cast_from(0.29608813203268075857e0_f64) * t82141 + F::cast_from(0.11514538467937585055e0_f64) * t82143 + F::cast_from(0.23029076935875170111e0_f64) * t82145 - F::cast_from(0.19739208802178717238e0_f64) * t82092 - F::cast_from(0.49348022005446793095e-1_f64) * t82087 + F::cast_from(0.49348022005446793095e-1_f64) * t82082 - F::cast_from(0.9869604401089358619e-1_f64) * t82076 + F::cast_from(0.24674011002723396548e-1_f64) * t82079 + F::cast_from(0.9869604401089358619e-1_f64) * t81559;
    t85166
}
