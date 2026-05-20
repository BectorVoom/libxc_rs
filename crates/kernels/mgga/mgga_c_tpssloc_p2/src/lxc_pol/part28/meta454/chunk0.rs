//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1651/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1651<F: Float>(t23095: F, t23105: F, t23107: F, t23140: F, t23143: F, t23100: F, t23114: F, t23117: F, t23119: F, t23125: F, t23128: F, t23130: F, t23134: F, t23136: F, t23147: F) -> (F, F, F, F, F, F) {
    let t24218 = F::cast_from(0.10541775202358879834e-2_f64) * t23095;
    let t24220 = F::cast_from(0.33643963411783659044e-4_f64) * t23105;
    let t24221 = F::new(119.0) / F::new(3456.0) * t23107;
    let t24230 = F::cast_from(0.22608743412718618878e-1_f64) * t23140;
    let t24231 = F::new(35.0) / F::new(216.0) * t23143;
    let t24233 = t24218 + F::cast_from(0.48447307312968469024e-2_f64) * t23100 - t24220 + t24221 + F::cast_from(0.13457585364713463618e-3_f64) * t23114 + t23117 / F::new(768.0) - F::new(7.0) / F::new(576.0) * t23119 + F::cast_from(0.80745512188280781706e-3_f64) * t23125 - t23128 / F::new(96.0) + F::new(5.0) / F::new(192.0) * t23130 + F::new(7.0) / F::new(144.0) * t23134 - t23136 / F::new(192.0) + t24230 + t24231 + t23147 / F::new(96.0);
    (t24218, t24220, t24221, t24230, t24231, t24233)
}
