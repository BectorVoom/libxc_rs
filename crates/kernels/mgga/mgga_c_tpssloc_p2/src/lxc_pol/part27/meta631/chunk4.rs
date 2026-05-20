//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2126/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2126<F: Float>(t86942: F, t23168: F, t25338: F, t13059: F, t22979: F, t25184: F, t2713: F, t2718: F, t2742: F, t4268: F, t6627: F, t7537: F, t855: F, t86929: F, t86930: F, t86931: F, t86933: F, t86941: F) -> F {
    let t86943 = F::cast_from(0.38381794893125283518e-1_f64) * t86942;
    let t86950 = t23168 * t25338;
    let t86951 = F::cast_from(0.76763589786250567036e-1_f64) * t86950;
    let t86952 = -t86929 + t86930 - t86931 + F::cast_from(0.3289868133696452873e-1_f64) * t86933 + F::new(2.0) * t855 * t2718 * t7537 * t2742 + t86941 + t86943 + F::new(4.0) * t2713 * t25184 + F::new(2.0) * t6627 * t13059 + F::new(4.0) * t4268 * t22979 + t86951;
    t86952
}
