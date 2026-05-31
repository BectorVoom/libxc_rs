//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2017/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2017<F: Float>(t90956: F, t90961: F, t90963: F, t90970: F, t90980: F, t90983: F, t90987: F, t90993: F, t1338: F, t27051: F, t12267: F, t1336: F, t1352: F, t1825: F, t24128: F, t27074: F, t27103: F, t3777: F, t3856: F, t5234: F, t5344: F, t7932: F, t81115: F, t81125: F, t84581: F, t90968: F) -> F {
    let t93588 = F::cast_from(0.76763589786250567036e-1_f64) * t90956;
    let t93589 = F::cast_from(0.3289868133696452873e-1_f64) * t90961;
    let t93590 = F::cast_from(0.15352717957250113407e0_f64) * t90963;
    let t93592 = F::cast_from(0.76763589786250567036e-1_f64) * t90970;
    let t93595 = F::cast_from(0.16449340668482264365e-1_f64) * t90980;
    let t93599 = F::cast_from(0.16449340668482264365e-1_f64) * t90983;
    let t93600 = F::cast_from(0.16449340668482264365e-1_f64) * t90987;
    let t93605 = F::cast_from(0.16449340668482264365e-1_f64) * t90993;
    let t93607 = t1338 * t27051;
    let t93612 = t93588 - t93589 - t93590 + F::cast_from(0.3289868133696452873e-1_f64) * t90968 + t93592 - F::cast_from(2.0_f64) * t3777 * t27103 + t93595 - t12267 * t7932 - t5344 * t27074 * t3856 + t93599 - t93600 - t1336 * t84581 * t1825 + F::cast_from(2.0_f64) * t5234 * t24128 - t93605 + F::cast_from(0.82246703342411321825e-2_f64) * t81115 - F::cast_from(2.0_f64) * t1336 * t93607 * t1352 + F::cast_from(0.82246703342411321825e-2_f64) * t81125;
    t93612
}
