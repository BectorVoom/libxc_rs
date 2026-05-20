//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1299/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1299<F: Float>(t2884: F, t302: F, t2887: F, t10727: F, t10817: F, t10655: F, t10731: F, t10661: F, t2836: F, t2845: F, t10697: F, t2792: F, t912: F) -> (F, F, F, F, F, F) {
    let t42224 = t2884 * t2884;
    let t42226 = t302 / t42224;
    let t42227 = t2887 * t2887;
    let t42228 = F::new(1.0) / t42227;
    let t42233 = F::new(24.0) * t10817 * t10727;
    let t42235 = F::cast_from(0.1929837539843104208e3_f64) * t10655 * t10731;
    let t42238 = F::cast_from(0.57895126195293126241e3_f64) * t10661 * t2845 * t2836;
    let t42241 = F::new(8.0) * t2792 * t10697 * t912;
    (t42226, t42228, t42233, t42235, t42238, t42241)
}
