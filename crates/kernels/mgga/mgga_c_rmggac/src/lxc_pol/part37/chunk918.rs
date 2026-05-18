//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 918/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk918<F: Float>(t73704: F, t3351: F, t498: F, t7231: F, t875: F, t9551: F, t3352: F, t9568: F, t3219: F, t38638: F, t73743: F, t73752: F) -> (F, F, F, F, F, F) {
    let t76608 = F::new(0.59127658063542114255e-5) * t73704;
    let t76612 = t3351 * t7231 * t875 * t9551 * t498;
    let t76613 = F::new(0.85129199786595678796e-5) * t76612;
    let t76616 = t3351 * t3352 * t875 * t9568;
    let t76617 = F::new(0.25538759935978703639e-4) * t76616;
    let t76618 = t38638 * t3219;
    let t76619 = F::new(0.99317399751028291929e-5) * t76618;
    let t76628 = F::new(0.19709219354514038085e-5) * t73743;
    let t76631 = F::new(0.3830813990396805546e-4) * t73752;
    (t76608, t76613, t76617, t76619, t76628, t76631)
}
