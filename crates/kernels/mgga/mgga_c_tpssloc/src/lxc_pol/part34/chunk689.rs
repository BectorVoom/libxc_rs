//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 689/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk689<F: Float>(t5: F, t109: F, t1860: F, t2032: F, t7026: F, t7034: F, t7428: F, t7432: F, t7435: F, t7782: F, t112: F, t1774: F, t2039: F, t7053: F, t7464: F) -> (F, F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t110 = F::new(1.0) < t109;
    let t7786 = piecewise3::<F>(t8, F::new(0.0), t7428 * t2032 / F::new(3.0) - F::new(5.0) / F::new(3.0) * t7026 * t7432 - F::new(2.0) / F::new(3.0) * t7435 * t2032 - t7034 + t1860 * t7782 / F::new(3.0));
    let t7787 = t7786 * t112;
    let t7796 = t1774 * t2039;
    let t7801 = piecewise3::<F>(t110, F::new(0.0), -t7053 - t7464 / F::new(4.0));
    (t7786, t7787, t7796, t7801)
}
