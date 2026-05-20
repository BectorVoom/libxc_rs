//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2401/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2401<F: Float>(t47761: F, t47765: F, t47769: F, t48112: F, t48114: F, t48116: F, t48119: F, t48122: F, t48125: F, t48128: F, t48131: F, t41887: F, t41889: F, t48134: F, t48137: F, t48142: F, t48145: F, t48148: F, t49009: F, t49012: F, t49015: F, t49018: F, t49021: F) -> (F, F) {
    let t49181 = F::new(0.181155e1) * t47761 + F::new(0.181155e1) * t47765 + F::new(0.60385e0) * t47769 + F::new(0.49671e0) * t48112 + F::new(0.16557e0) * t48114 + F::cast_from(0.73586666666666666668e-1_f64) * t48116 + F::new(0.49671e0) * t48119 + F::new(0.44152e0) * t48122 - F::new(0.149013e1) * t48125 - F::new(0.82785e-1) * t48128 - F::new(0.11038e0) * t48131;
    let t49194 = -F::new(0.27595e-1) * t48134 - F::cast_from(0.8585111111111111111e-1_f64) * t48137 + F::new(0.49671e0) * t48142 - F::new(0.149013e1) * t48145 - F::new(0.1294625e1) * t49009 - F::cast_from(0.1237865625e0_f64) * t49012 + F::new(0.58258125e1) * t49015 - F::cast_from(0.485484375e1_f64) * t49018 + F::cast_from(0.6189328125e-1_f64) * t49021 - F::new(0.11038e0) * t48148 - F::new(0.33114e0) * t41887 + F::cast_from(0.55190000000000000001e-1_f64) * t41889;
    (t49181, t49194)
}
