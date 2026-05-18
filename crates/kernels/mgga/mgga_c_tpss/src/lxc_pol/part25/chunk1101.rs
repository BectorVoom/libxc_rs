//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1101/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1101<F: Float>(t30: F, t259: F, t379: F, t14432: F, t14689: F, t15206: F, t1288: F, t1289: F, t13334: F, t13335: F, t14440: F, t1490: F, t3431: F, t3735: F, t381: F, t4028: F, t45: F, t4578: F, t4579: F, t4818: F, t5048: F, t580: F, t581: F, t826: F, t999: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> F {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t15208 = piecewise3::<f64>(t380, t14689 + t15206, t14432);
    let t15220 = piecewise3::<f64>(t120, t14432 * t30 / F::new(2.0) + t4818 * t580 / F::new(2.0) + t3735 * t1288 + t14440 + t826 * t4578 / F::new(2.0) + t259 * t13334 / F::new(2.0), t15208 * t45 / F::new(2.0) + t5048 * t581 / F::new(2.0) + t4028 * t1289 + t1490 * t3431 + t999 * t4579 / F::new(2.0) + t381 * t13335 / F::new(2.0));
    t15220
}
