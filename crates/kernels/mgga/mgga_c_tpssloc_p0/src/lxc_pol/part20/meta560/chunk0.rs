//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2116/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2116<F: Float>(t10660: F, t888: F, t10810: F, t919: F, t2859: F, t2884: F, t302: F, t41654: F, t41961: F, t2887: F, t10619: F, t300: F) -> (F, F, F, F, F, F, F, F, F) {
    let t42143 = t888 * t10660;
    let t42149 = t919 * t10810;
    let t42154 = t302 / t2884 / t2859;
    let t42212 = F::cast_from(0.5356037037037037037e1_f64) * t41654;
    let t42213 = F::cast_from(0.16979925925925925926e1_f64) * t41961;
    let t42224 = t2884 * t2884;
    let t42226 = t302 / t42224;
    let t42227 = t2887 * t2887;
    let t42228 = F::new(1.0) / t42227;
    let t42245 = F::cast_from(0.17757530864197530864e0_f64) * t41654;
    let t42281 = t300 * t10619;
    (t42143, t42149, t42154, t42212, t42213, t42226, t42228, t42245, t42281)
}
