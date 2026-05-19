//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1272/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1272<F: Float>(t114255: F, t2007: F, t254: F, t114278: F, t32694: F, t6914: F, t114291: F, t32735: F, t6883: F, t114296: F, t114264: F, t2016: F, t26224: F, t26226: F, t26348: F, t26472: F, t26477: F, t32766: F, t3758: F, t40590: F, t5325: F, t6958: F, t6963: F, t6993: F, t8475: F, t91488: F, t91491: F) -> F {
    let t120590 = F::cast_from(0.76763589786250567036e-1_f64) * t114255;
    let t120591 = t2007 * t254;
    let t120594 = F::cast_from(0.16449340668482264365e-1_f64) * t114278;
    let t120605 = t6914 * t32694;
    let t120606 = F::cast_from(0.76763589786250567037e-1_f64) * t120605;
    let t120607 = F::cast_from(0.38381794893125283518e-1_f64) * t114291;
    let t120610 = t6883 * t32735;
    let t120611 = F::cast_from(0.38381794893125283518e-1_f64) * t120610;
    let t120612 = F::cast_from(0.38381794893125283518e-1_f64) * t114296;
    let t120613 = F::new(24.0) * t26224 * t40590 * t5325 * t8475 - F::new(12.0) * t120591 * t26226 - F::new(2.0) * t2016 * t91488 - F::new(2.0) * t2016 * t91491 + F::new(4.0) * t26348 * t6958 - F::new(2.0) * t26472 * t6958 + F::new(4.0) * t26477 * t6963 - F::new(2.0) * t26477 * t6993 + F::new(4.0) * t32766 * t3758 + t114264 - t120590 - t120594 - t120606 + t120607 + t120611 + t120612;
    t120613
}
