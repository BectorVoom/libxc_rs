//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 918/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk918<F: Float>(t24721: F, t27700: F, t1714: F, t2133: F, t2132: F, t6739: F, t8026: F, t7325: F, t25588: F, t2121: F, t2136: F, t24650: F, t24747: F, t24752: F, t24754: F, t27681: F, t27684: F, t27687: F, t27692: F, t27697: F, t4989: F, t7321: F, t7326: F, t7331: F, t7345: F, t8040: F) -> (F, F, F, F) {
    let t27701 = t24721 * t27700;
    let t27703 = t2133 * t1714;
    let t27704 = t2132 * t27703;
    let t27710 = t8026 * t6739;
    let t27711 = t27710 * t7325;
    let t27714 = t2132 * t25588;
    let t27719 = -0.80745512188280781712e-3 * t27681 - 0.10093189023535097714e-3 * t27684 * t7331 - t2121 * t27687 / 144.0 + 0.10093189023535097714e-3 * t7326 * t27692 - 0.10093189023535097714e-3 * t24650 * t8040 + t2121 * t27697 / 216.0 + 0.10093189023535097714e-3 * t27701 - 0.10093189023535097714e-3 * t27704 * t7321 + 5.0 / 6912.0 * t7345 * t4989 - 0.10093189023535097714e-3 * t24747 - 0.80745512188280781712e-3 * t27711 * t7331 - 0.10093189023535097714e-3 * t27714 * t2136 - t24752 / 3456.0 + t24754 / 2304.0;
    (t27704, t27711, t27714, t27719)
}
