//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2123/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2123<F: Float>(t86895: F, t23035: F, t23241: F, t25224: F, t7480: F, t81632: F, t22975: F, t23191: F, t25184: F, t25330: F, t2597: F, t2713: F, t4147: F, t4268: F, t86844: F, t86847: F, t86852: F, t86857: F, t86862: F, t86866: F, t86869: F, t86870: F, t86875: F, t86881: F, t86884: F, t86887: F, t86891: F) -> F {
    let t86896 = F::cast_from(0.16449340668482264365e-1_f64) * t86895;
    let t86901 = t23035 * t25224 * t23241;
    let t86903 = t81632 * t7480;
    let t86905 = t86844 + F::cast_from(0.16449340668482264365e-1_f64) * t86847 + F::cast_from(0.3289868133696452873e-1_f64) * t86852 + F::cast_from(0.3289868133696452873e-1_f64) * t86857 + F::cast_from(0.3289868133696452873e-1_f64) * t86862 + F::cast_from(0.16449340668482264365e-1_f64) * t86866 + t86869 - F::cast_from(0.52089578783527170489e-1_f64) * t86870 + F::cast_from(0.3289868133696452873e-1_f64) * t86875 + F::cast_from(2.0_f64) * t4147 * t22975 - F::cast_from(0.49348022005446793095e-1_f64) * t86881 + F::cast_from(0.3289868133696452873e-1_f64) * t86884 + t86887 + F::cast_from(4.0_f64) * t2597 * t25184 - F::cast_from(0.16449340668482264365e-1_f64) * t86891 + t86896 - F::cast_from(2.0_f64) * t2713 * t25330 - t4268 * t23191 + F::cast_from(0.49348022005446793095e-1_f64) * t86901 - F::cast_from(0.12793931631041761173e0_f64) * t86903;
    t86905
}
