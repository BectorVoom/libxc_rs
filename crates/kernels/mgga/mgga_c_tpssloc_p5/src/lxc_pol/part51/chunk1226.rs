//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1226/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1226<F: Float>(t33379: F, t6646: F, t1888: F, t1894: F, t7823: F, t214: F, t1880: F, t1510: F, t31394: F, t31353: F, t31355: F, t31359: F, t32835: F, t32838: F, t32841: F, t32845: F, t32847: F) -> (F, F, F, F, F, F, F) {
    let t33380 = t6646 * t33379;
    let t33381 = t1888 * t33380;
    let t33383 = t1894 * t7823;
    let t33384 = t214 * t33383;
    let t33385 = t1880 * t33384;
    let t33388 = t31394 * t1510;
    let t33395 = -t31353 - F::cast_from(0.96894614625936938046e-2_f64) * t32835 - t31355 - F::cast_from(0.16149102437656156341e-2_f64) * t32838 + t32841 / F::cast_from(768.0_f64) - t32845 / F::cast_from(768.0_f64) - t31359 - t32847 / F::cast_from(192.0_f64);
    (t33380, t33381, t33383, t33384, t33385, t33388, t33395)
}
