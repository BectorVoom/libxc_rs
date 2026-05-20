//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1712/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1712<F: Float>(t12813: F, t510: F, t1458: F, t3652: F, t4098: F, t751: F, t2752: F, t4303: F, t172: F, t4095: F, t763: F, t1472: F, t2517: F) -> (F, F, F, F, F, F, F) {
    let t12835 = t510 * t12813;
    let t12841 = t3652 * t1458;
    let t12850 = F::new(2.0) * t4098 * t751;
    let t12854 = t4303 * t2752;
    let t12858 = t4095 * t172;
    let t12860 = F::cast_from(0.11696447245269292414e1_f64) * t12858 * t763;
    let t12861 = t1472 * t2517;
    (t12835, t12841, t12850, t12854, t12858, t12860, t12861)
}
