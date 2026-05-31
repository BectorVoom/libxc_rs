//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1951/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1951<F: Float>(t29473: F, t67: F, t1864: F, t7445: F, t7974: F, t2109: F, t27956: F, t1860: F, t2110: F, t24514: F, t26016: F, t27298: F, t27332: F, t27341: F, t27937: F, t27961: F, t27966: F, t27972: F, t27976: F, t27979: F, t27982: F, t7246: F, t7428: F, t7432: F, t7435: F, t7975: F, t7978: F) -> (F, F, F, F, F) {
    let t29474 = t29473 * t67;
    let t29475 = t29474 * t1864;
    let t29478 = t7974 * t7445;
    let t29481 = t2109 * t27956;
    let t29484 = -F::cast_from(5.0_f64) * t24514 * t27961 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t26016 * t27298 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t27341 * t7432 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t27966 * t2110 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t27332 * t7432 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7246 * t27972 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7246 * t27976 + t27979 * t2110 / F::cast_from(3.0_f64) + t27982 * t2110 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7435 * t7975 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7435 * t7978 - t27937 * t2110 / F::cast_from(6.0_f64) - t7428 * t7975 / F::cast_from(3.0_f64) - t7428 * t7978 / F::cast_from(3.0_f64) - t1860 * t29475 / F::cast_from(6.0_f64) - t1860 * t29478 / F::cast_from(3.0_f64) - t1860 * t29481 / F::cast_from(6.0_f64);
    (t29474, t29475, t29478, t29481, t29484)
}
