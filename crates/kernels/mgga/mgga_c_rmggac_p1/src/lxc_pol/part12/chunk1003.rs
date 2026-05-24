//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1003/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1003<F: Float>(t38745: F, t5271: F, t39670: F, t5162: F, t39674: F, t4669: F, t305: F, t38674: F, t118: F, t25809: F, t39692: F, t2123: F, t558: F) -> (F, F, F, F, F, F, F) {
    let t41101 = t5271 * t38745;
    let t41106 = t5162 * t39670;
    let t41108 = t4669 * t39674;
    let t41114 = t305 * t38674;
    let t41115 = F::cast_from(0.79828278012425390426e-1_f64) * t41114;
    let t41116 = t118 * t25809;
    let t41120 = t5271 * t39692;
    let t41122 = t2123 * t558;
    (t41101, t41106, t41108, t41115, t41116, t41120, t41122)
}
