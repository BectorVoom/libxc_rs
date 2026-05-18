//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 953/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk953<F: Float>(t3308: F, t8223: F, t8232: F, t1186: F, t3305: F, t30: F, t502: F, t33: F, t504: F, t1173: F, t3197: F, t1193: F, t8021: F) -> (F, F, F, F, F, F, F) {
    let t9846 = F::new(0.48159733137676571078e0) * t3308 * t8223;
    let t9848 = F::new(0.32530743900905219526e-1) * t3308 * t8232;
    let t9854 = F::new(60.0) * t3305 * t1186;
    let t9856 = F::new(1.0) / t502 / t30;
    let t9868 = F::new(1.0) / t504 / t33;
    let t9883 = t1173 * t3197;
    let t9886 = F::new(0.10389515463408878255e3) * t1193 * t8021;
    (t9846, t9848, t9854, t9856, t9868, t9883, t9886)
}
