//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2181/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2181<F: Float>(t87729: F, t25325: F, t6547: F, t13390: F, t23016: F, t25255: F, t25262: F, t25295: F, t2679: F, t2684: F, t4162: F, t4166: F, t6660: F, t808: F, t812: F, t82028: F, t82032: F, t82047: F, t87699: F, t87705: F, t87708: F, t87710: F, t87714: F, t87718: F, t87726: F) -> F {
    let t87730 = F::cast_from(0.82246703342411321824e-2_f64) * t87729;
    let t87733 = t6547 * t25325;
    let t87734 = F::cast_from(0.38381794893125283518e-1_f64) * t87733;
    let t87735 = -t4166 * t23016 + F::cast_from(0.41123351671205660912e-2_f64) * t82028 + F::cast_from(0.9869604401089358619e-1_f64) * t87699 + F::cast_from(2.0_f64) * t808 * t25295 + F::cast_from(0.3289868133696452873e-1_f64) * t87705 - F::cast_from(0.52089578783527170488e-1_f64) * t82032 - t87708 + t87710 - F::cast_from(0.49348022005446793096e-1_f64) * t87714 - t82047 - t812 * t25255 * t2679 - F::cast_from(0.52089578783527170489e-1_f64) * t87718 - t812 * t25255 * t2684 + F::cast_from(2.0_f64) * t4162 * t6660 - F::cast_from(0.16449340668482264365e-1_f64) * t87726 + t87730 - F::cast_from(2.0_f64) * t13390 * t25262 - t87734;
    t87735
}
