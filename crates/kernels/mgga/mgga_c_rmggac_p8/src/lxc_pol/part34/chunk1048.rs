//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1048/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1048<F: Float>(t570: F, t71916: F, t8940: F, t14495: F, t551: F, t1587: F, t3204: F, t71983: F, t8626: F, t3839: F, t71982: F, t8632: F) -> (F, F, F, F, F) {
    let t78083 = F::cast_from(0.11974241701863808564e0_f64) * t8940 * t71916 * t570;
    let t78084 = t14495 * t551;
    let t78087 = t3204 * t1587;
    let t78090 = t71983 * t8626;
    let t78091 = F::cast_from(0.40911992481368012592e-1_f64) * t78090;
    let t78093 = t3839 * t71982 * t8632;
    (t78083, t78084, t78087, t78091, t78093)
}
